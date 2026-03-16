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

// ── Camera Metadata ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SensorSpec {
    width_mm: f64,
    height_mm: f64,
    photosites_h: u32,
    photosites_v: u32,
    dual_gain_iso: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LensInfo {
    manufacturer: String,
    model: String,
    focal_length_mm: u16,
    max_aperture_tenths: u16,
    is_anamorphic: bool,
    squeeze_factor_tenths: Option<u16>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CameraMetadata {
    camera_id: String,
    make: String,
    model: String,
    sensor: SensorSpec,
    lens: LensInfo,
    frame_rate_num: u32,
    frame_rate_den: u32,
    codec: String,
    bit_depth: u8,
    recording_format: String,
    color_space: String,
}

// ── Edit Decision List ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum EdlFormat {
    Cmx3600,
    Aaf,
    FcpXml,
    Otio,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EdlEvent {
    event_number: u32,
    reel_name: String,
    track_type: String,
    edit_type: String,
    src_in_frames: u64,
    src_out_frames: u64,
    rec_in_frames: u64,
    rec_out_frames: u64,
    comment: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EditDecisionList {
    title: String,
    format: EdlFormat,
    frame_rate_num: u32,
    frame_rate_den: u32,
    events: Vec<EdlEvent>,
    drop_frame: bool,
}

// ── Color Grading ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ColorWheelValues {
    red: f64,
    green: f64,
    blue: f64,
    master: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ColorGradingNode {
    node_id: u32,
    label: String,
    enabled: bool,
    lift: ColorWheelValues,
    gamma: ColorWheelValues,
    gain: ColorWheelValues,
    saturation: f64,
    lut_reference: Option<String>,
    lut_intensity: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GradingTimeline {
    project: String,
    colorist: String,
    nodes: Vec<ColorGradingNode>,
    output_color_space: String,
}

// ── VFX Shot Tracking ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum VfxShotStatus {
    Omitted,
    NotStarted,
    InProgress,
    InternalReview,
    ClientReview,
    Approved,
    Final,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VfxShot {
    shot_code: String,
    sequence: String,
    status: VfxShotStatus,
    frame_range_start: u32,
    frame_range_end: u32,
    assigned_artist: String,
    complexity_tier: u8,
    description: String,
    version: u16,
    vendor: Option<String>,
}

// ── Render Farm Job Queue ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum RenderPriority {
    Low,
    Normal,
    High,
    Rush,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RenderFarmJob {
    job_id: u64,
    project_name: String,
    shot_code: String,
    priority: RenderPriority,
    frame_start: u32,
    frame_end: u32,
    chunk_size: u16,
    renderer: String,
    estimated_seconds_per_frame: f64,
    output_path: String,
    dependencies: Vec<u64>,
    submitted_by: String,
}

// ── Subtitle / Closed Caption ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum CaptionType {
    OpenCaption,
    ClosedCaption,
    Sdh,
    ForcedNarrative,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SubtitleCue {
    index: u32,
    start_ms: u64,
    end_ms: u64,
    text: String,
    position_x_pct: f32,
    position_y_pct: f32,
    style: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SubtitleTrack {
    track_id: u32,
    language_code: String,
    caption_type: CaptionType,
    cues: Vec<SubtitleCue>,
    font_family: String,
    default_size_pt: u16,
}

// ── Audio Mixing ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AudioStem {
    stem_id: u32,
    label: String,
    channel_count: u8,
    sample_rate_hz: u32,
    bit_depth: u8,
    file_path: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AudioBus {
    bus_id: u32,
    name: String,
    stems: Vec<u32>,
    fader_db: f64,
    pan_position: f64,
    muted: bool,
    solo: bool,
    insert_plugins: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AudioMixSession {
    session_name: String,
    sample_rate_hz: u32,
    bit_depth: u8,
    stems: Vec<AudioStem>,
    buses: Vec<AudioBus>,
    master_fader_db: f64,
    loudness_target_lufs: f64,
}

// ── Content Delivery Manifest ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DeliveryProtocol {
    Hls,
    Dash,
    SmoothStreaming,
    Cmaf,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VariantStream {
    bandwidth_kbps: u32,
    resolution_w: u16,
    resolution_h: u16,
    codec_string: String,
    segment_duration_ms: u32,
    uri_template: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ContentDeliveryManifest {
    content_id: String,
    protocol: DeliveryProtocol,
    drm_system: Option<String>,
    variants: Vec<VariantStream>,
    audio_only_variants: Vec<VariantStream>,
    min_buffer_seconds: u16,
}

// ── Broadcast Scheduling ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AdBreak {
    break_id: u32,
    offset_from_start_ms: u64,
    duration_ms: u64,
    slot_count: u8,
    is_local: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PlaylistEntry {
    entry_id: u64,
    media_id: String,
    title: String,
    duration_ms: u64,
    start_offset_ms: u64,
    ad_breaks: Vec<AdBreak>,
    rating: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BroadcastSchedule {
    channel_name: String,
    date_ymd: String,
    entries: Vec<PlaylistEntry>,
    total_duration_ms: u64,
}

// ── MAM Metadata ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MamAsset {
    asset_id: u64,
    original_filename: String,
    proxy_path: String,
    hi_res_path: String,
    mime_type: String,
    file_size_bytes: u64,
    duration_ms: Option<u64>,
    tags: Vec<String>,
    created_epoch: u64,
    last_accessed_epoch: u64,
    owner: String,
}

// ── QC Report ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum QcSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct QcFinding {
    timecode_in: String,
    timecode_out: String,
    severity: QcSeverity,
    category: String,
    description: String,
    auto_detected: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct QcReport {
    report_id: u64,
    asset_id: u64,
    operator: String,
    pass: bool,
    findings: Vec<QcFinding>,
    overall_score: f64,
}

// ── Rights Management ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LicenseWindow {
    territory: String,
    start_epoch: u64,
    end_epoch: u64,
    exclusive: bool,
    platforms: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RightsRecord {
    content_id: String,
    title: String,
    rights_holder: String,
    contract_ref: String,
    windows: Vec<LicenseWindow>,
    residual_pct_hundredths: u32,
}

// ── Live Production Switching ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TransitionType {
    Cut,
    Dissolve,
    Wipe,
    Dve,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SwitcherInput {
    input_index: u8,
    label: String,
    source_type: String,
    is_tally: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SwitcherState {
    program_input: u8,
    preview_input: u8,
    transition: TransitionType,
    transition_rate_frames: u16,
    inputs: Vec<SwitcherInput>,
    dsk_on_air: Vec<bool>,
}

// ── Graphics Overlay ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GraphicElement {
    element_id: u32,
    layer: u8,
    template_name: String,
    fields: Vec<(String, String)>,
    x_pct: f32,
    y_pct: f32,
    width_pct: f32,
    height_pct: f32,
    opacity: f32,
    visible: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GraphicsOverlayConfig {
    scene_name: String,
    resolution_w: u16,
    resolution_h: u16,
    elements: Vec<GraphicElement>,
    safe_area_pct: f32,
}

// ── Archive / LTO Tape Tracking ──

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TapeRecord {
    barcode: String,
    generation: u8,
    capacity_tb: u16,
    used_tb_hundredths: u32,
    write_protect: bool,
    location: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ArchiveEntry {
    asset_id: u64,
    tape_barcode: String,
    offset_bytes: u64,
    length_bytes: u64,
    checksum_crc32: u32,
    archived_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ArchiveManifest {
    manifest_id: u64,
    tapes: Vec<TapeRecord>,
    entries: Vec<ArchiveEntry>,
    created_epoch: u64,
    verified: bool,
}

// ═══════════════════ Tests ═══════════════════

#[test]
fn test_camera_metadata_roundtrip() {
    let cfg = config::standard();
    let data = CameraMetadata {
        camera_id: "CAM-A01".into(),
        make: "ARRI".into(),
        model: "ALEXA 35".into(),
        sensor: SensorSpec {
            width_mm: 27.99,
            height_mm: 19.22,
            photosites_h: 4608,
            photosites_v: 3164,
            dual_gain_iso: true,
        },
        lens: LensInfo {
            manufacturer: "Cooke".into(),
            model: "S7/i 50mm".into(),
            focal_length_mm: 50,
            max_aperture_tenths: 14,
            is_anamorphic: false,
            squeeze_factor_tenths: None,
        },
        frame_rate_num: 24000,
        frame_rate_den: 1001,
        codec: "ARRIRAW".into(),
        bit_depth: 12,
        recording_format: "MXF".into(),
        color_space: "ARRI Wide Gamut 4".into(),
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode camera metadata");
    let (decoded, _): (CameraMetadata, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode camera metadata");
    assert_eq!(data, decoded);
}

#[test]
fn test_anamorphic_lens_metadata() {
    let cfg = config::standard();
    let data = CameraMetadata {
        camera_id: "CAM-B03".into(),
        make: "Panavision".into(),
        model: "Millennium DXL2".into(),
        sensor: SensorSpec {
            width_mm: 33.60,
            height_mm: 17.82,
            photosites_h: 8192,
            photosites_v: 4320,
            dual_gain_iso: false,
        },
        lens: LensInfo {
            manufacturer: "Panavision".into(),
            model: "Ultra Vista 40mm".into(),
            focal_length_mm: 40,
            max_aperture_tenths: 19,
            is_anamorphic: true,
            squeeze_factor_tenths: Some(20),
        },
        frame_rate_num: 24,
        frame_rate_den: 1,
        codec: "R3D".into(),
        bit_depth: 16,
        recording_format: "R3D".into(),
        color_space: "REDWideGamutRGB".into(),
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode anamorphic metadata");
    let (decoded, _): (CameraMetadata, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode anamorphic metadata");
    assert_eq!(data, decoded);
}

#[test]
fn test_edit_decision_list_roundtrip() {
    let cfg = config::standard();
    let data = EditDecisionList {
        title: "PILOT_EP101_V14".into(),
        format: EdlFormat::Cmx3600,
        frame_rate_num: 24,
        frame_rate_den: 1,
        drop_frame: false,
        events: vec![
            EdlEvent {
                event_number: 1,
                reel_name: "A001C003".into(),
                track_type: "V".into(),
                edit_type: "C".into(),
                src_in_frames: 86400,
                src_out_frames: 86520,
                rec_in_frames: 0,
                rec_out_frames: 120,
                comment: Some("VFX plate for comp".into()),
            },
            EdlEvent {
                event_number: 2,
                reel_name: "A002C011".into(),
                track_type: "V".into(),
                edit_type: "D".into(),
                src_in_frames: 43200,
                src_out_frames: 43440,
                rec_in_frames: 120,
                rec_out_frames: 360,
                comment: None,
            },
        ],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode EDL");
    let (decoded, _): (EditDecisionList, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EDL");
    assert_eq!(data, decoded);
}

#[test]
fn test_fcp_xml_edl_format() {
    let cfg = config::standard();
    let data = EditDecisionList {
        title: "DOC_MASTER_V2".into(),
        format: EdlFormat::FcpXml,
        frame_rate_num: 30000,
        frame_rate_den: 1001,
        drop_frame: true,
        events: vec![EdlEvent {
            event_number: 1,
            reel_name: "INT_001".into(),
            track_type: "VA".into(),
            edit_type: "C".into(),
            src_in_frames: 0,
            src_out_frames: 900,
            rec_in_frames: 0,
            rec_out_frames: 900,
            comment: Some("Interview segment A".into()),
        }],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode FCP XML EDL");
    let (decoded, _): (EditDecisionList, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FCP XML EDL");
    assert_eq!(data, decoded);
}

#[test]
fn test_color_grading_timeline_roundtrip() {
    let cfg = config::standard();
    let data = GradingTimeline {
        project: "FEATURE_2026_GRADE".into(),
        colorist: "J. Doe".into(),
        output_color_space: "Rec.2020 PQ".into(),
        nodes: vec![
            ColorGradingNode {
                node_id: 1,
                label: "Base Balance".into(),
                enabled: true,
                lift: ColorWheelValues {
                    red: -0.02,
                    green: 0.0,
                    blue: 0.01,
                    master: 0.0,
                },
                gamma: ColorWheelValues {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    master: 1.0,
                },
                gain: ColorWheelValues {
                    red: 1.05,
                    green: 1.0,
                    blue: 0.98,
                    master: 1.0,
                },
                saturation: 1.1,
                lut_reference: None,
                lut_intensity: 1.0,
            },
            ColorGradingNode {
                node_id: 2,
                label: "Film Emulation".into(),
                enabled: true,
                lift: ColorWheelValues {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    master: 0.0,
                },
                gamma: ColorWheelValues {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    master: 1.0,
                },
                gain: ColorWheelValues {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    master: 1.0,
                },
                saturation: 0.95,
                lut_reference: Some("Kodak_2383_D65.cube".into()),
                lut_intensity: 0.7,
            },
        ],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode grading timeline");
    let (decoded, _): (GradingTimeline, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode grading timeline");
    assert_eq!(data, decoded);
}

#[test]
fn test_vfx_shot_tracking_roundtrip() {
    let cfg = config::standard();
    let data = vec![
        VfxShot {
            shot_code: "SQ020_SH0310".into(),
            sequence: "SQ020".into(),
            status: VfxShotStatus::InProgress,
            frame_range_start: 1001,
            frame_range_end: 1120,
            assigned_artist: "artist_mko".into(),
            complexity_tier: 3,
            description: "Full CG environment replacement with hero digi-double".into(),
            version: 7,
            vendor: Some("Framestore".into()),
        },
        VfxShot {
            shot_code: "SQ020_SH0320".into(),
            sequence: "SQ020".into(),
            status: VfxShotStatus::Approved,
            frame_range_start: 1001,
            frame_range_end: 1048,
            assigned_artist: "artist_lnr".into(),
            complexity_tier: 1,
            description: "Wire removal and cleanup".into(),
            version: 3,
            vendor: None,
        },
    ];
    let bytes = encode_to_vec(&data, cfg).expect("encode VFX shots");
    let (decoded, _): (Vec<VfxShot>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VFX shots");
    assert_eq!(data, decoded);
}

#[test]
fn test_vfx_shot_final_status() {
    let cfg = config::standard();
    let data = VfxShot {
        shot_code: "SQ055_SH0010".into(),
        sequence: "SQ055".into(),
        status: VfxShotStatus::Final,
        frame_range_start: 1001,
        frame_range_end: 1200,
        assigned_artist: "lead_comp".into(),
        complexity_tier: 5,
        description: "Hero explosion with full destruction sim".into(),
        version: 22,
        vendor: Some("Weta FX".into()),
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode final VFX shot");
    let (decoded, _): (VfxShot, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode final VFX shot");
    assert_eq!(data, decoded);
}

#[test]
fn test_render_farm_job_roundtrip() {
    let cfg = config::standard();
    let data = RenderFarmJob {
        job_id: 99201,
        project_name: "MOONRISE_S01E03".into(),
        shot_code: "SQ010_SH0050".into(),
        priority: RenderPriority::Rush,
        frame_start: 1001,
        frame_end: 1180,
        chunk_size: 10,
        renderer: "Arnold 7.3".into(),
        estimated_seconds_per_frame: 420.5,
        output_path: "/renders/moonrise/s01e03/sq010_sh0050/v12/".into(),
        dependencies: vec![99198, 99199, 99200],
        submitted_by: "sup_jt".into(),
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode render farm job");
    let (decoded, _): (RenderFarmJob, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode render farm job");
    assert_eq!(data, decoded);
}

#[test]
fn test_render_farm_no_dependencies() {
    let cfg = config::standard();
    let data = RenderFarmJob {
        job_id: 10001,
        project_name: "PROMO_2026Q2".into(),
        shot_code: "PROMO_HERO".into(),
        priority: RenderPriority::Low,
        frame_start: 0,
        frame_end: 149,
        chunk_size: 50,
        renderer: "Nuke 15.1".into(),
        estimated_seconds_per_frame: 12.0,
        output_path: "/renders/promo/hero/v01/".into(),
        dependencies: vec![],
        submitted_by: "jr_comp".into(),
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode render job no deps");
    let (decoded, _): (RenderFarmJob, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode render job no deps");
    assert_eq!(data, decoded);
}

#[test]
fn test_subtitle_track_roundtrip() {
    let cfg = config::standard();
    let data = SubtitleTrack {
        track_id: 1,
        language_code: "en-US".into(),
        caption_type: CaptionType::ClosedCaption,
        font_family: "Arial".into(),
        default_size_pt: 24,
        cues: vec![
            SubtitleCue {
                index: 1,
                start_ms: 1200,
                end_ms: 4500,
                text: "Previously on Moonrise...".into(),
                position_x_pct: 50.0,
                position_y_pct: 90.0,
                style: "italic".into(),
            },
            SubtitleCue {
                index: 2,
                start_ms: 5000,
                end_ms: 7800,
                text: "We need to get to the observatory\nbefore dawn.".into(),
                position_x_pct: 50.0,
                position_y_pct: 90.0,
                style: "default".into(),
            },
        ],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode subtitle track");
    let (decoded, _): (SubtitleTrack, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode subtitle track");
    assert_eq!(data, decoded);
}

#[test]
fn test_sdh_subtitle_track() {
    let cfg = config::standard();
    let data = SubtitleTrack {
        track_id: 3,
        language_code: "en-US".into(),
        caption_type: CaptionType::Sdh,
        font_family: "Helvetica Neue".into(),
        default_size_pt: 22,
        cues: vec![SubtitleCue {
            index: 1,
            start_ms: 0,
            end_ms: 3000,
            text: "[suspenseful music playing]".into(),
            position_x_pct: 50.0,
            position_y_pct: 85.0,
            style: "sdh_description".into(),
        }],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode SDH track");
    let (decoded, _): (SubtitleTrack, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SDH track");
    assert_eq!(data, decoded);
}

#[test]
fn test_audio_mix_session_roundtrip() {
    let cfg = config::standard();
    let data = AudioMixSession {
        session_name: "MOONRISE_S01E03_FINAL_MIX".into(),
        sample_rate_hz: 48000,
        bit_depth: 24,
        stems: vec![
            AudioStem {
                stem_id: 1,
                label: "DX".into(),
                channel_count: 2,
                sample_rate_hz: 48000,
                bit_depth: 24,
                file_path: "/audio/stems/dx_premix.wav".into(),
            },
            AudioStem {
                stem_id: 2,
                label: "MX".into(),
                channel_count: 6,
                sample_rate_hz: 48000,
                bit_depth: 24,
                file_path: "/audio/stems/mx_5_1.wav".into(),
            },
            AudioStem {
                stem_id: 3,
                label: "FX".into(),
                channel_count: 6,
                sample_rate_hz: 48000,
                bit_depth: 24,
                file_path: "/audio/stems/fx_5_1.wav".into(),
            },
        ],
        buses: vec![
            AudioBus {
                bus_id: 1,
                name: "Dialogue Bus".into(),
                stems: vec![1],
                fader_db: 0.0,
                pan_position: 0.0,
                muted: false,
                solo: false,
                insert_plugins: vec!["iZotope RX".into(), "FabFilter Pro-Q".into()],
            },
            AudioBus {
                bus_id: 2,
                name: "Music Bus".into(),
                stems: vec![2],
                fader_db: -3.5,
                pan_position: 0.0,
                muted: false,
                solo: false,
                insert_plugins: vec!["Waves L2".into()],
            },
        ],
        master_fader_db: -0.5,
        loudness_target_lufs: -24.0,
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode audio mix session");
    let (decoded, _): (AudioMixSession, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode audio mix session");
    assert_eq!(data, decoded);
}

#[test]
fn test_content_delivery_manifest_roundtrip() {
    let cfg = config::standard();
    let data = ContentDeliveryManifest {
        content_id: "MOONRISE_S01E03_DELIVERY".into(),
        protocol: DeliveryProtocol::Hls,
        drm_system: Some("FairPlay".into()),
        min_buffer_seconds: 6,
        variants: vec![
            VariantStream {
                bandwidth_kbps: 12000,
                resolution_w: 3840,
                resolution_h: 2160,
                codec_string: "avc1.640033".into(),
                segment_duration_ms: 6000,
                uri_template: "/hls/4k/seg_%05d.ts".into(),
            },
            VariantStream {
                bandwidth_kbps: 6000,
                resolution_w: 1920,
                resolution_h: 1080,
                codec_string: "avc1.64001f".into(),
                segment_duration_ms: 6000,
                uri_template: "/hls/1080p/seg_%05d.ts".into(),
            },
            VariantStream {
                bandwidth_kbps: 2000,
                resolution_w: 1280,
                resolution_h: 720,
                codec_string: "avc1.64001f".into(),
                segment_duration_ms: 6000,
                uri_template: "/hls/720p/seg_%05d.ts".into(),
            },
        ],
        audio_only_variants: vec![VariantStream {
            bandwidth_kbps: 128,
            resolution_w: 0,
            resolution_h: 0,
            codec_string: "mp4a.40.2".into(),
            segment_duration_ms: 6000,
            uri_template: "/hls/audio/seg_%05d.aac".into(),
        }],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode delivery manifest");
    let (decoded, _): (ContentDeliveryManifest, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode delivery manifest");
    assert_eq!(data, decoded);
}

#[test]
fn test_dash_delivery_no_drm() {
    let cfg = config::standard();
    let data = ContentDeliveryManifest {
        content_id: "TRAILER_2026_DASH".into(),
        protocol: DeliveryProtocol::Dash,
        drm_system: None,
        min_buffer_seconds: 3,
        variants: vec![VariantStream {
            bandwidth_kbps: 5000,
            resolution_w: 1920,
            resolution_h: 1080,
            codec_string: "avc1.64001f".into(),
            segment_duration_ms: 4000,
            uri_template: "/dash/1080p/chunk_$Number$.m4s".into(),
        }],
        audio_only_variants: vec![],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode DASH manifest");
    let (decoded, _): (ContentDeliveryManifest, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DASH manifest");
    assert_eq!(data, decoded);
}

#[test]
fn test_broadcast_schedule_roundtrip() {
    let cfg = config::standard();
    let data = BroadcastSchedule {
        channel_name: "NHK BS Premium".into(),
        date_ymd: "2026-03-15".into(),
        total_duration_ms: 86_400_000,
        entries: vec![
            PlaylistEntry {
                entry_id: 1,
                media_id: "NHK-2026-0315-0600".into(),
                title: "Morning News".into(),
                duration_ms: 3_600_000,
                start_offset_ms: 21_600_000,
                rating: "G".into(),
                ad_breaks: vec![
                    AdBreak {
                        break_id: 1,
                        offset_from_start_ms: 900_000,
                        duration_ms: 120_000,
                        slot_count: 4,
                        is_local: false,
                    },
                    AdBreak {
                        break_id: 2,
                        offset_from_start_ms: 1_800_000,
                        duration_ms: 180_000,
                        slot_count: 6,
                        is_local: true,
                    },
                ],
            },
            PlaylistEntry {
                entry_id: 2,
                media_id: "NHK-2026-0315-0700".into(),
                title: "Documentary: Ocean Depths".into(),
                duration_ms: 5_400_000,
                start_offset_ms: 25_200_000,
                rating: "G".into(),
                ad_breaks: vec![],
            },
        ],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode broadcast schedule");
    let (decoded, _): (BroadcastSchedule, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode broadcast schedule");
    assert_eq!(data, decoded);
}

#[test]
fn test_mam_asset_roundtrip() {
    let cfg = config::standard();
    let data = MamAsset {
        asset_id: 500_123,
        original_filename: "A001C003_220415_R1AB.mxf".into(),
        proxy_path: "/proxies/500123_proxy.mp4".into(),
        hi_res_path: "/vault/originals/500123/A001C003_220415_R1AB.mxf".into(),
        mime_type: "application/mxf".into(),
        file_size_bytes: 42_949_672_960,
        duration_ms: Some(312_000),
        tags: vec![
            "ep103".into(),
            "scene12".into(),
            "ext_night".into(),
            "hero_closeup".into(),
        ],
        created_epoch: 1_710_504_000,
        last_accessed_epoch: 1_710_590_400,
        owner: "post_dept".into(),
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode MAM asset");
    let (decoded, _): (MamAsset, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MAM asset");
    assert_eq!(data, decoded);
}

#[test]
fn test_qc_report_roundtrip() {
    let cfg = config::standard();
    let data = QcReport {
        report_id: 7890,
        asset_id: 500_123,
        operator: "qc_tech_01".into(),
        pass: false,
        overall_score: 72.5,
        findings: vec![
            QcFinding {
                timecode_in: "01:02:15:08".into(),
                timecode_out: "01:02:15:12".into(),
                severity: QcSeverity::Error,
                category: "VIDEO".into(),
                description: "Flash frame detected, 4 frames of black".into(),
                auto_detected: true,
            },
            QcFinding {
                timecode_in: "00:45:30:00".into(),
                timecode_out: "00:45:32:00".into(),
                severity: QcSeverity::Warning,
                category: "AUDIO".into(),
                description: "Dialogue level exceeds -24 LUFS target by 3 dB".into(),
                auto_detected: true,
            },
            QcFinding {
                timecode_in: "01:15:00:00".into(),
                timecode_out: "01:15:00:00".into(),
                severity: QcSeverity::Info,
                category: "METADATA".into(),
                description: "Closed caption track missing for Spanish locale".into(),
                auto_detected: false,
            },
        ],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode QC report");
    let (decoded, _): (QcReport, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode QC report");
    assert_eq!(data, decoded);
}

#[test]
fn test_rights_record_roundtrip() {
    let cfg = config::standard();
    let data = RightsRecord {
        content_id: "MOONRISE_S01".into(),
        title: "Moonrise Season 1".into(),
        rights_holder: "Stellar Productions Ltd".into(),
        contract_ref: "CTR-2025-00432".into(),
        residual_pct_hundredths: 250,
        windows: vec![
            LicenseWindow {
                territory: "US".into(),
                start_epoch: 1_710_000_000,
                end_epoch: 1_741_536_000,
                exclusive: true,
                platforms: vec!["SVOD".into(), "AVOD".into()],
            },
            LicenseWindow {
                territory: "JP".into(),
                start_epoch: 1_712_592_000,
                end_epoch: 1_744_128_000,
                exclusive: false,
                platforms: vec!["SVOD".into()],
            },
            LicenseWindow {
                territory: "EMEA".into(),
                start_epoch: 1_715_184_000,
                end_epoch: 1_746_720_000,
                exclusive: false,
                platforms: vec!["SVOD".into(), "Linear".into(), "EST".into()],
            },
        ],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode rights record");
    let (decoded, _): (RightsRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode rights record");
    assert_eq!(data, decoded);
}

#[test]
fn test_live_production_switcher_roundtrip() {
    let cfg = config::standard();
    let data = SwitcherState {
        program_input: 1,
        preview_input: 3,
        transition: TransitionType::Dissolve,
        transition_rate_frames: 30,
        inputs: vec![
            SwitcherInput {
                input_index: 1,
                label: "Camera 1 - Wide".into(),
                source_type: "SDI".into(),
                is_tally: true,
            },
            SwitcherInput {
                input_index: 2,
                label: "Camera 2 - Close".into(),
                source_type: "SDI".into(),
                is_tally: false,
            },
            SwitcherInput {
                input_index: 3,
                label: "Graphics In".into(),
                source_type: "NDI".into(),
                is_tally: false,
            },
            SwitcherInput {
                input_index: 4,
                label: "Replay Server".into(),
                source_type: "SDI".into(),
                is_tally: false,
            },
        ],
        dsk_on_air: vec![true, false],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode switcher state");
    let (decoded, _): (SwitcherState, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode switcher state");
    assert_eq!(data, decoded);
}

#[test]
fn test_graphics_overlay_config_roundtrip() {
    let cfg = config::standard();
    let data = GraphicsOverlayConfig {
        scene_name: "LIVE_SCORE_OVERLAY".into(),
        resolution_w: 1920,
        resolution_h: 1080,
        safe_area_pct: 90.0,
        elements: vec![
            GraphicElement {
                element_id: 1,
                layer: 10,
                template_name: "lower_third".into(),
                fields: vec![
                    ("name".into(), "Presenter Name".into()),
                    ("title".into(), "Senior Correspondent".into()),
                ],
                x_pct: 5.0,
                y_pct: 80.0,
                width_pct: 40.0,
                height_pct: 12.0,
                opacity: 0.95,
                visible: true,
            },
            GraphicElement {
                element_id: 2,
                layer: 20,
                template_name: "bug_logo".into(),
                fields: vec![("channel".into(), "NHK".into())],
                x_pct: 90.0,
                y_pct: 5.0,
                width_pct: 8.0,
                height_pct: 6.0,
                opacity: 0.8,
                visible: true,
            },
            GraphicElement {
                element_id: 3,
                layer: 15,
                template_name: "ticker".into(),
                fields: vec![
                    ("text".into(), "Breaking: Market update...".into()),
                    ("speed".into(), "medium".into()),
                ],
                x_pct: 0.0,
                y_pct: 92.0,
                width_pct: 100.0,
                height_pct: 8.0,
                opacity: 1.0,
                visible: false,
            },
        ],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode graphics overlay");
    let (decoded, _): (GraphicsOverlayConfig, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode graphics overlay");
    assert_eq!(data, decoded);
}

#[test]
fn test_archive_manifest_roundtrip() {
    let cfg = config::standard();
    let data = ArchiveManifest {
        manifest_id: 42,
        created_epoch: 1_710_504_000,
        verified: true,
        tapes: vec![
            TapeRecord {
                barcode: "LTO9-00042A".into(),
                generation: 9,
                capacity_tb: 18,
                used_tb_hundredths: 1523,
                write_protect: false,
                location: "Vault-A Rack 3 Slot 12".into(),
            },
            TapeRecord {
                barcode: "LTO9-00042B".into(),
                generation: 9,
                capacity_tb: 18,
                used_tb_hundredths: 890,
                write_protect: true,
                location: "Offsite Iron Mountain".into(),
            },
        ],
        entries: vec![
            ArchiveEntry {
                asset_id: 500_123,
                tape_barcode: "LTO9-00042A".into(),
                offset_bytes: 0,
                length_bytes: 42_949_672_960,
                checksum_crc32: 0xABCD_1234,
                archived_epoch: 1_710_504_100,
            },
            ArchiveEntry {
                asset_id: 500_124,
                tape_barcode: "LTO9-00042A".into(),
                offset_bytes: 42_949_672_960,
                length_bytes: 21_474_836_480,
                checksum_crc32: 0xDEAD_BEEF,
                archived_epoch: 1_710_504_200,
            },
            ArchiveEntry {
                asset_id: 500_123,
                tape_barcode: "LTO9-00042B".into(),
                offset_bytes: 0,
                length_bytes: 42_949_672_960,
                checksum_crc32: 0xABCD_1234,
                archived_epoch: 1_710_504_300,
            },
        ],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode archive manifest");
    let (decoded, _): (ArchiveManifest, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode archive manifest");
    assert_eq!(data, decoded);
}

#[test]
fn test_qc_report_all_pass() {
    let cfg = config::standard();
    let data = QcReport {
        report_id: 7891,
        asset_id: 500_200,
        operator: "qc_tech_02".into(),
        pass: true,
        overall_score: 100.0,
        findings: vec![QcFinding {
            timecode_in: "00:00:00:00".into(),
            timecode_out: "01:32:00:00".into(),
            severity: QcSeverity::Info,
            category: "GENERAL".into(),
            description: "Full automated scan completed with no issues".into(),
            auto_detected: true,
        }],
    };
    let bytes = encode_to_vec(&data, cfg).expect("encode passing QC report");
    let (decoded, _): (QcReport, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode passing QC report");
    assert_eq!(data, decoded);
}
