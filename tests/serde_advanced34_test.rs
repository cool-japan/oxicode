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

// ── Domain types: Augmented Reality / AR/XR Systems ──────────────────────────

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TrackingState {
    Lost,
    Initializing,
    Tracking,
    Limited,
    Relocating,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum GestureKind {
    Pinch,
    Grab,
    Point,
    OpenPalm,
    ThumbsUp,
    PeaceSign,
    Custom(u16),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum MixedRealityLayer {
    Holographic,
    PassThrough,
    Overlay,
    Underlay,
    WorldLocked,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DepthSensorMode {
    Disabled,
    Near,
    Far,
    Full,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SpatialTransform {
    position: Vector3,
    rotation: Quaternion,
    scale: Vector3,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SpatialAnchor {
    anchor_id: u64,
    name: String,
    transform: SpatialTransform,
    tracking_state: TrackingState,
    confidence: f32,
    is_persistent: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HolographicObject {
    object_id: u64,
    label: String,
    transform: SpatialTransform,
    layer: MixedRealityLayer,
    visible: bool,
    opacity: f32,
    anchor_id: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TrackingMarker {
    marker_id: u32,
    payload: String,
    transform: SpatialTransform,
    tracking_state: TrackingState,
    size_meters: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GestureEvent {
    hand_id: u8,
    kind: GestureKind,
    confidence: f32,
    position: Vector3,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EyeTrackingFrame {
    frame_id: u64,
    left_gaze: Vector3,
    right_gaze: Vector3,
    combined_gaze: Vector3,
    left_open: f32,
    right_open: f32,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HandJoint {
    joint_id: u8,
    position: Vector3,
    rotation: Quaternion,
    radius: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HandTrackingFrame {
    hand_id: u8,
    joints: Vec<HandJoint>,
    tracking_state: TrackingState,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DepthSample {
    u: u16,
    v: u16,
    depth_mm: u16,
    confidence: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DepthFrame {
    frame_id: u64,
    width: u16,
    height: u16,
    mode: DepthSensorMode,
    samples: Vec<DepthSample>,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SceneUnderstandingMesh {
    mesh_id: u32,
    anchor_id: u64,
    vertices: Vec<Vector3>,
    triangle_indices: Vec<u32>,
    label: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct XrSession {
    session_id: u64,
    device_name: String,
    anchors: Vec<SpatialAnchor>,
    holographic_objects: Vec<HolographicObject>,
    depth_mode: DepthSensorMode,
    active: bool,
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn identity_transform() -> SpatialTransform {
    SpatialTransform {
        position: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation: Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        },
        scale: Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    }
}

fn make_anchor(id: u64, name: &str, state: TrackingState) -> SpatialAnchor {
    SpatialAnchor {
        anchor_id: id,
        name: name.to_string(),
        transform: identity_transform(),
        tracking_state: state,
        confidence: 0.95,
        is_persistent: true,
    }
}

fn make_holographic(id: u64, label: &str, layer: MixedRealityLayer) -> HolographicObject {
    HolographicObject {
        object_id: id,
        label: label.to_string(),
        transform: identity_transform(),
        layer,
        visible: true,
        opacity: 1.0,
        anchor_id: None,
    }
}

fn make_hand_joint(joint_id: u8) -> HandJoint {
    HandJoint {
        joint_id,
        position: Vector3 {
            x: joint_id as f32 * 0.01,
            y: 0.0,
            z: 0.0,
        },
        rotation: Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        },
        radius: 0.005,
    }
}

// ── 1. SpatialAnchor basic roundtrip ─────────────────────────────────────────
#[test]
fn test_spatial_anchor_basic_roundtrip() {
    let cfg = config::standard();
    let anchor = make_anchor(1001, "TableSurface", TrackingState::Tracking);
    let bytes = encode_to_vec(&anchor, cfg).expect("encode SpatialAnchor");
    let (decoded, _): (SpatialAnchor, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SpatialAnchor");
    assert_eq!(anchor, decoded);
}

// ── 2. TrackingState: all variants ───────────────────────────────────────────
#[test]
fn test_tracking_state_all_variants() {
    let cfg = config::standard();
    let states = [
        TrackingState::Lost,
        TrackingState::Initializing,
        TrackingState::Tracking,
        TrackingState::Limited,
        TrackingState::Relocating,
    ];
    for state in &states {
        let bytes = encode_to_vec(state, cfg).expect("encode TrackingState");
        let (decoded, _): (TrackingState, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode TrackingState");
        assert_eq!(state, &decoded);
    }
}

// ── 3. GestureKind: unit and newtype variants ─────────────────────────────────
#[test]
fn test_gesture_kind_variants() {
    let cfg = config::standard();
    let gestures = vec![
        GestureKind::Pinch,
        GestureKind::Grab,
        GestureKind::Point,
        GestureKind::OpenPalm,
        GestureKind::ThumbsUp,
        GestureKind::PeaceSign,
        GestureKind::Custom(42),
        GestureKind::Custom(0xFFFF),
    ];
    for gesture in &gestures {
        let bytes = encode_to_vec(gesture, cfg).expect("encode GestureKind");
        let (decoded, _): (GestureKind, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode GestureKind");
        assert_eq!(gesture, &decoded);
    }
}

// ── 4. SpatialTransform nested struct roundtrip ───────────────────────────────
#[test]
fn test_spatial_transform_nested_roundtrip() {
    let cfg = config::standard();
    let transform = SpatialTransform {
        position: Vector3 {
            x: 1.5,
            y: -0.3,
            z: 2.7,
        },
        rotation: Quaternion {
            x: 0.0,
            y: 0.7071068,
            z: 0.0,
            w: 0.7071068,
        },
        scale: Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        },
    };
    let bytes = encode_to_vec(&transform, cfg).expect("encode SpatialTransform");
    let (decoded, _): (SpatialTransform, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SpatialTransform");
    assert_eq!(transform, decoded);
}

// ── 5. HolographicObject with Option<u64> anchor_id = Some ───────────────────
#[test]
fn test_holographic_object_with_anchor() {
    let cfg = config::standard();
    let mut obj = make_holographic(2001, "InstructionPanel", MixedRealityLayer::WorldLocked);
    obj.anchor_id = Some(1001);
    let bytes = encode_to_vec(&obj, cfg).expect("encode HolographicObject with anchor");
    let (decoded, _): (HolographicObject, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HolographicObject with anchor");
    assert_eq!(obj, decoded);
    assert_eq!(decoded.anchor_id, Some(1001));
}

// ── 6. HolographicObject with Option<u64> anchor_id = None ───────────────────
#[test]
fn test_holographic_object_without_anchor() {
    let cfg = config::standard();
    let obj = make_holographic(2002, "FloatingLabel", MixedRealityLayer::Overlay);
    let bytes = encode_to_vec(&obj, cfg).expect("encode HolographicObject without anchor");
    let (decoded, _): (HolographicObject, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HolographicObject without anchor");
    assert_eq!(obj, decoded);
    assert_eq!(decoded.anchor_id, None);
}

// ── 7. Vec<SpatialAnchor> roundtrip ──────────────────────────────────────────
#[test]
fn test_vec_spatial_anchor_roundtrip() {
    let cfg = config::standard();
    let anchors = vec![
        make_anchor(10, "WallLeft", TrackingState::Tracking),
        make_anchor(11, "WallRight", TrackingState::Tracking),
        make_anchor(12, "Floor", TrackingState::Limited),
        make_anchor(13, "Ceiling", TrackingState::Initializing),
    ];
    let bytes = encode_to_vec(&anchors, cfg).expect("encode Vec<SpatialAnchor>");
    let (decoded, _): (Vec<SpatialAnchor>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<SpatialAnchor>");
    assert_eq!(anchors, decoded);
    assert_eq!(decoded.len(), 4);
}

// ── 8. GestureEvent roundtrip ─────────────────────────────────────────────────
#[test]
fn test_gesture_event_roundtrip() {
    let cfg = config::standard();
    let event = GestureEvent {
        hand_id: 0,
        kind: GestureKind::Pinch,
        confidence: 0.98,
        position: Vector3 {
            x: 0.1,
            y: 1.2,
            z: -0.5,
        },
        timestamp_ms: 1_700_000_000_000,
    };
    let bytes = encode_to_vec(&event, cfg).expect("encode GestureEvent");
    let (decoded, _): (GestureEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode GestureEvent");
    assert_eq!(event, decoded);
}

// ── 9. EyeTrackingFrame roundtrip ────────────────────────────────────────────
#[test]
fn test_eye_tracking_frame_roundtrip() {
    let cfg = config::standard();
    let frame = EyeTrackingFrame {
        frame_id: 9001,
        left_gaze: Vector3 {
            x: -0.02,
            y: 0.01,
            z: -1.0,
        },
        right_gaze: Vector3 {
            x: 0.02,
            y: 0.01,
            z: -1.0,
        },
        combined_gaze: Vector3 {
            x: 0.0,
            y: 0.01,
            z: -1.0,
        },
        left_open: 0.85,
        right_open: 0.90,
        timestamp_ms: 1_700_000_001_000,
    };
    let bytes = encode_to_vec(&frame, cfg).expect("encode EyeTrackingFrame");
    let (decoded, _): (EyeTrackingFrame, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EyeTrackingFrame");
    assert_eq!(frame, decoded);
}

// ── 10. HandTrackingFrame with 26 joints ─────────────────────────────────────
#[test]
fn test_hand_tracking_frame_twenty_six_joints() {
    let cfg = config::standard();
    let joints: Vec<HandJoint> = (0u8..26).map(make_hand_joint).collect();
    let frame = HandTrackingFrame {
        hand_id: 1,
        joints,
        tracking_state: TrackingState::Tracking,
        timestamp_ms: 1_700_000_002_000,
    };
    let bytes = encode_to_vec(&frame, cfg).expect("encode HandTrackingFrame");
    let (decoded, _): (HandTrackingFrame, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HandTrackingFrame");
    assert_eq!(frame, decoded);
    assert_eq!(decoded.joints.len(), 26);
}

// ── 11. DepthFrame with samples ───────────────────────────────────────────────
#[test]
fn test_depth_frame_with_samples() {
    let cfg = config::standard();
    let samples: Vec<DepthSample> = (0u16..8)
        .map(|i| DepthSample {
            u: i,
            v: i * 2,
            depth_mm: 500 + i * 10,
            confidence: 200,
        })
        .collect();
    let frame = DepthFrame {
        frame_id: 3001,
        width: 64,
        height: 48,
        mode: DepthSensorMode::Near,
        samples,
        timestamp_ms: 1_700_000_003_000,
    };
    let bytes = encode_to_vec(&frame, cfg).expect("encode DepthFrame");
    let (decoded, _): (DepthFrame, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DepthFrame");
    assert_eq!(frame, decoded);
    assert_eq!(decoded.samples.len(), 8);
}

// ── 12. MixedRealityLayer: all variants ──────────────────────────────────────
#[test]
fn test_mixed_reality_layer_all_variants() {
    let cfg = config::standard();
    let layers = [
        MixedRealityLayer::Holographic,
        MixedRealityLayer::PassThrough,
        MixedRealityLayer::Overlay,
        MixedRealityLayer::Underlay,
        MixedRealityLayer::WorldLocked,
    ];
    for layer in &layers {
        let bytes = encode_to_vec(layer, cfg).expect("encode MixedRealityLayer");
        let (decoded, _): (MixedRealityLayer, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode MixedRealityLayer");
        assert_eq!(layer, &decoded);
    }
}

// ── 13. TrackingMarker roundtrip ──────────────────────────────────────────────
#[test]
fn test_tracking_marker_roundtrip() {
    let cfg = config::standard();
    let marker = TrackingMarker {
        marker_id: 42,
        payload: "AR_MARKER_ROOM_42".to_string(),
        transform: SpatialTransform {
            position: Vector3 {
                x: 0.5,
                y: 0.0,
                z: 1.0,
            },
            rotation: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            scale: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        },
        tracking_state: TrackingState::Tracking,
        size_meters: 0.15,
    };
    let bytes = encode_to_vec(&marker, cfg).expect("encode TrackingMarker");
    let (decoded, _): (TrackingMarker, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TrackingMarker");
    assert_eq!(marker, decoded);
}

// ── 14. SceneUnderstandingMesh with vertices and indices ─────────────────────
#[test]
fn test_scene_understanding_mesh_roundtrip() {
    let cfg = config::standard();
    let vertices = vec![
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.5,
            y: 1.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.5,
            y: 0.5,
            z: 1.0,
        },
    ];
    let mesh = SceneUnderstandingMesh {
        mesh_id: 7001,
        anchor_id: 1001,
        vertices,
        triangle_indices: vec![0, 1, 2, 0, 1, 3, 1, 2, 3, 0, 2, 3],
        label: "FloorMesh".to_string(),
    };
    let bytes = encode_to_vec(&mesh, cfg).expect("encode SceneUnderstandingMesh");
    let (decoded, _): (SceneUnderstandingMesh, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SceneUnderstandingMesh");
    assert_eq!(mesh, decoded);
    assert_eq!(decoded.vertices.len(), 4);
    assert_eq!(decoded.triangle_indices.len(), 12);
}

// ── 15. XrSession with anchors and holographic objects ───────────────────────
#[test]
fn test_xr_session_roundtrip() {
    let cfg = config::standard();
    let session = XrSession {
        session_id: 100,
        device_name: "HoloLens 3".to_string(),
        anchors: vec![
            make_anchor(1, "DeskAnchor", TrackingState::Tracking),
            make_anchor(2, "WindowAnchor", TrackingState::Limited),
        ],
        holographic_objects: vec![
            make_holographic(10, "Calendar", MixedRealityLayer::WorldLocked),
            make_holographic(11, "Email", MixedRealityLayer::Overlay),
        ],
        depth_mode: DepthSensorMode::Near,
        active: true,
    };
    let bytes = encode_to_vec(&session, cfg).expect("encode XrSession");
    let (decoded, _): (XrSession, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode XrSession");
    assert_eq!(session, decoded);
    assert_eq!(decoded.anchors.len(), 2);
    assert_eq!(decoded.holographic_objects.len(), 2);
}

// ── 16. Big-endian config roundtrip ──────────────────────────────────────────
#[test]
fn test_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let anchor = SpatialAnchor {
        anchor_id: 0xDEAD_BEEF_CAFE_F00D,
        name: "BigEndianAnchor".to_string(),
        transform: identity_transform(),
        tracking_state: TrackingState::Tracking,
        confidence: 0.99,
        is_persistent: false,
    };
    let bytes = encode_to_vec(&anchor, cfg).expect("encode with big_endian");
    let (decoded, _): (SpatialAnchor, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode with big_endian");
    assert_eq!(anchor, decoded);
}

// ── 17. Fixed-int config roundtrip ───────────────────────────────────────────
#[test]
fn test_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let frame = EyeTrackingFrame {
        frame_id: u64::MAX,
        left_gaze: Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        right_gaze: Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        combined_gaze: Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        left_open: 1.0,
        right_open: 1.0,
        timestamp_ms: u64::MAX,
    };
    let bytes = encode_to_vec(&frame, cfg).expect("encode with fixed_int");
    let (decoded, _): (EyeTrackingFrame, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode with fixed_int");
    assert_eq!(frame, decoded);
}

// ── 18. Combined big-endian + fixed-int config ────────────────────────────────
#[test]
fn test_combined_big_endian_fixed_int_config() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let marker = TrackingMarker {
        marker_id: u32::MAX,
        payload: "COMBINED_CONFIG_MARKER".to_string(),
        transform: identity_transform(),
        tracking_state: TrackingState::Lost,
        size_meters: 0.05,
    };
    let bytes = encode_to_vec(&marker, cfg).expect("encode combined config");
    let (decoded, _): (TrackingMarker, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode combined config");
    assert_eq!(marker, decoded);
}

// ── 19. DepthSensorMode: all variants ────────────────────────────────────────
#[test]
fn test_depth_sensor_mode_all_variants() {
    let cfg = config::standard();
    let modes = [
        DepthSensorMode::Disabled,
        DepthSensorMode::Near,
        DepthSensorMode::Far,
        DepthSensorMode::Full,
    ];
    for mode in &modes {
        let bytes = encode_to_vec(mode, cfg).expect("encode DepthSensorMode");
        let (decoded, _): (DepthSensorMode, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode DepthSensorMode");
        assert_eq!(mode, &decoded);
    }
}

// ── 20. Large DepthFrame with many samples ────────────────────────────────────
#[test]
fn test_large_depth_frame() {
    let cfg = config::standard();
    let width: u16 = 256;
    let height: u16 = 192;
    let samples: Vec<DepthSample> = (0u32..(width as u32 * height as u32))
        .map(|i| DepthSample {
            u: (i % width as u32) as u16,
            v: (i / width as u32) as u16,
            depth_mm: (1000 + (i % 3000)) as u16,
            confidence: (i % 256) as u8,
        })
        .collect();
    let expected_len = samples.len();
    let frame = DepthFrame {
        frame_id: 99999,
        width,
        height,
        mode: DepthSensorMode::Full,
        samples,
        timestamp_ms: 1_700_999_000_000,
    };
    let bytes = encode_to_vec(&frame, cfg).expect("encode large DepthFrame");
    let (decoded, _): (DepthFrame, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode large DepthFrame");
    assert_eq!(frame, decoded);
    assert_eq!(decoded.samples.len(), expected_len);
}

// ── 21. Consumed-bytes equals encoded length ──────────────────────────────────
#[test]
fn test_consumed_bytes_equals_encoded_length() {
    let cfg = config::standard();
    let session = XrSession {
        session_id: 555,
        device_name: "Magic Leap 3".to_string(),
        anchors: vec![
            make_anchor(20, "LabTable", TrackingState::Tracking),
            make_anchor(21, "Whiteboard", TrackingState::Tracking),
            make_anchor(22, "DoorFrame", TrackingState::Limited),
        ],
        holographic_objects: vec![
            make_holographic(30, "3DModel_Heart", MixedRealityLayer::Holographic),
            make_holographic(31, "Annotations", MixedRealityLayer::Overlay),
        ],
        depth_mode: DepthSensorMode::Far,
        active: true,
    };
    let bytes = encode_to_vec(&session, cfg).expect("encode for consumed-bytes check");
    let (decoded, consumed): (XrSession, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode for consumed-bytes check");
    assert_eq!(session, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ── 22. Multi-frame gesture sequence ─────────────────────────────────────────
#[test]
fn test_multi_frame_gesture_sequence() {
    let cfg = config::standard();
    let gestures = vec![
        GestureEvent {
            hand_id: 0,
            kind: GestureKind::OpenPalm,
            confidence: 0.91,
            position: Vector3 {
                x: 0.1,
                y: 1.3,
                z: -0.4,
            },
            timestamp_ms: 1_700_000_010_000,
        },
        GestureEvent {
            hand_id: 0,
            kind: GestureKind::Pinch,
            confidence: 0.97,
            position: Vector3 {
                x: 0.12,
                y: 1.28,
                z: -0.42,
            },
            timestamp_ms: 1_700_000_010_100,
        },
        GestureEvent {
            hand_id: 1,
            kind: GestureKind::Point,
            confidence: 0.88,
            position: Vector3 {
                x: -0.15,
                y: 1.1,
                z: -0.3,
            },
            timestamp_ms: 1_700_000_010_200,
        },
        GestureEvent {
            hand_id: 1,
            kind: GestureKind::Custom(512),
            confidence: 0.75,
            position: Vector3 {
                x: -0.2,
                y: 1.05,
                z: -0.35,
            },
            timestamp_ms: 1_700_000_010_300,
        },
        GestureEvent {
            hand_id: 0,
            kind: GestureKind::ThumbsUp,
            confidence: 0.99,
            position: Vector3 {
                x: 0.05,
                y: 1.4,
                z: -0.5,
            },
            timestamp_ms: 1_700_000_010_400,
        },
    ];
    let bytes = encode_to_vec(&gestures, cfg).expect("encode gesture sequence");
    let (decoded, _): (Vec<GestureEvent>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode gesture sequence");
    assert_eq!(gestures, decoded);
    assert_eq!(decoded.len(), 5);
    assert_eq!(decoded[3].kind, GestureKind::Custom(512));
}
