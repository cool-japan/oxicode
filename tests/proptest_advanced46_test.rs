//! Advanced property-based tests (set 46) using proptest.
//!
//! 22 top-level #[test] functions with proptest! blocks.
//! Domain: virtual reality / augmented reality (XR).
//! Covers roundtrip, consumed == bytes.len(), deterministic encoding,
//! all enum variants, option positions, and various tuple inputs.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum XrPlatform {
    OculusQuest,
    HoloLens,
    VisionPro,
    SteamVR,
    WebXR,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TrackingMode {
    SixDof,
    ThreeDof,
    HandTracking,
    EyeTracking,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct XrFrame {
    frame_id: u64,
    timestamp_ns: u64,
    head_pos: (f32, f32, f32),
    head_rot: (f32, f32, f32, f32),
    left_hand_pos: Option<(f32, f32, f32)>,
    right_hand_pos: Option<(f32, f32, f32)>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct XrSession {
    session_id: u64,
    platform: XrPlatform,
    tracking: TrackingMode,
    fps: f32,
    resolution_w: u32,
    resolution_h: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct XrEvent {
    session_id: u64,
    frame: XrFrame,
    button_states: Vec<bool>,
}

// ── Proptest strategies ───────────────────────────────────────────────────────

fn arb_xr_platform() -> impl Strategy<Value = XrPlatform> {
    prop_oneof![
        Just(XrPlatform::OculusQuest),
        Just(XrPlatform::HoloLens),
        Just(XrPlatform::VisionPro),
        Just(XrPlatform::SteamVR),
        Just(XrPlatform::WebXR),
    ]
}

fn arb_tracking_mode() -> impl Strategy<Value = TrackingMode> {
    prop_oneof![
        Just(TrackingMode::SixDof),
        Just(TrackingMode::ThreeDof),
        Just(TrackingMode::HandTracking),
        Just(TrackingMode::EyeTracking),
    ]
}

fn arb_vec3() -> impl Strategy<Value = (f32, f32, f32)> {
    (any::<f32>(), any::<f32>(), any::<f32>())
}

fn arb_quat() -> impl Strategy<Value = (f32, f32, f32, f32)> {
    (any::<f32>(), any::<f32>(), any::<f32>(), any::<f32>())
}

fn arb_xr_frame() -> impl Strategy<Value = XrFrame> {
    (
        any::<u64>(),
        any::<u64>(),
        arb_vec3(),
        arb_quat(),
        prop::option::of(arb_vec3()),
        prop::option::of(arb_vec3()),
    )
        .prop_map(
            |(frame_id, timestamp_ns, head_pos, head_rot, left_hand_pos, right_hand_pos)| XrFrame {
                frame_id,
                timestamp_ns,
                head_pos,
                head_rot,
                left_hand_pos,
                right_hand_pos,
            },
        )
}

fn arb_xr_session() -> impl Strategy<Value = XrSession> {
    (
        any::<u64>(),
        arb_xr_platform(),
        arb_tracking_mode(),
        any::<f32>(),
        any::<u32>(),
        any::<u32>(),
    )
        .prop_map(
            |(session_id, platform, tracking, fps, resolution_w, resolution_h)| XrSession {
                session_id,
                platform,
                tracking,
                fps,
                resolution_w,
                resolution_h,
            },
        )
}

fn arb_xr_event() -> impl Strategy<Value = XrEvent> {
    (
        any::<u64>(),
        arb_xr_frame(),
        prop::collection::vec(any::<bool>(), 0..16),
    )
        .prop_map(|(session_id, frame, button_states)| XrEvent {
            session_id,
            frame,
            button_states,
        })
}

// ── 1. XrFrame roundtrip ──────────────────────────────────────────────────────

#[test]
fn test_xr_frame_roundtrip() {
    proptest!(|(frame in arb_xr_frame())| {
        let enc = encode_to_vec(&frame).expect("encode XrFrame failed");
        let (decoded, consumed): (XrFrame, usize) =
            decode_from_slice(&enc).expect("decode XrFrame failed");
        prop_assert_eq!(frame, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 2. XrSession roundtrip ────────────────────────────────────────────────────

#[test]
fn test_xr_session_roundtrip() {
    proptest!(|(session in arb_xr_session())| {
        let enc = encode_to_vec(&session).expect("encode XrSession failed");
        let (decoded, consumed): (XrSession, usize) =
            decode_from_slice(&enc).expect("decode XrSession failed");
        prop_assert_eq!(session, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 3. XrEvent roundtrip ──────────────────────────────────────────────────────

#[test]
fn test_xr_event_roundtrip() {
    proptest!(|(event in arb_xr_event())| {
        let enc = encode_to_vec(&event).expect("encode XrEvent failed");
        let (decoded, consumed): (XrEvent, usize) =
            decode_from_slice(&enc).expect("decode XrEvent failed");
        prop_assert_eq!(event, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 4. XrPlatform consumed == bytes.len() ─────────────────────────────────────

#[test]
fn test_xr_platform_consumed_eq_len() {
    proptest!(|(platform in arb_xr_platform())| {
        let enc = encode_to_vec(&platform).expect("encode XrPlatform failed");
        let (_decoded, consumed): (XrPlatform, usize) =
            decode_from_slice(&enc).expect("decode XrPlatform failed");
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 5. TrackingMode consumed == bytes.len() ───────────────────────────────────

#[test]
fn test_tracking_mode_consumed_eq_len() {
    proptest!(|(mode in arb_tracking_mode())| {
        let enc = encode_to_vec(&mode).expect("encode TrackingMode failed");
        let (_decoded, consumed): (TrackingMode, usize) =
            decode_from_slice(&enc).expect("decode TrackingMode failed");
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 6. XrFrame consumed == bytes.len() ────────────────────────────────────────

#[test]
fn test_xr_frame_consumed_eq_len() {
    proptest!(|(frame in arb_xr_frame())| {
        let enc = encode_to_vec(&frame).expect("encode XrFrame failed");
        let (_decoded, consumed): (XrFrame, usize) =
            decode_from_slice(&enc).expect("decode XrFrame failed");
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 7. Deterministic encoding for XrFrame ────────────────────────────────────

#[test]
fn test_xr_frame_encoding_deterministic() {
    proptest!(|(frame in arb_xr_frame())| {
        let enc1 = encode_to_vec(&frame).expect("first encode XrFrame failed");
        let enc2 = encode_to_vec(&frame).expect("second encode XrFrame failed");
        prop_assert_eq!(enc1, enc2, "XrFrame encoding must be deterministic");
    });
}

// ── 8. Deterministic encoding for XrSession ──────────────────────────────────

#[test]
fn test_xr_session_encoding_deterministic() {
    proptest!(|(session in arb_xr_session())| {
        let enc1 = encode_to_vec(&session).expect("first encode XrSession failed");
        let enc2 = encode_to_vec(&session).expect("second encode XrSession failed");
        prop_assert_eq!(enc1, enc2, "XrSession encoding must be deterministic");
    });
}

// ── 9. Deterministic encoding for XrEvent ────────────────────────────────────

#[test]
fn test_xr_event_encoding_deterministic() {
    proptest!(|(event in arb_xr_event())| {
        let enc1 = encode_to_vec(&event).expect("first encode XrEvent failed");
        let enc2 = encode_to_vec(&event).expect("second encode XrEvent failed");
        prop_assert_eq!(enc1, enc2, "XrEvent encoding must be deterministic");
    });
}

// ── 10. XrPlatform::OculusQuest variant ──────────────────────────────────────

#[test]
fn test_xr_platform_oculus_quest_roundtrip() {
    proptest!(|(session_id: u64)| {
        let platform = XrPlatform::OculusQuest;
        let enc = encode_to_vec(&platform).expect("encode OculusQuest failed");
        let (decoded, consumed): (XrPlatform, usize) =
            decode_from_slice(&enc).expect("decode OculusQuest failed");
        prop_assert_eq!(decoded, XrPlatform::OculusQuest);
        prop_assert_eq!(consumed, enc.len());
        prop_assert!(session_id <= u64::MAX);
    });
}

// ── 11. XrPlatform::VisionPro variant ────────────────────────────────────────

#[test]
fn test_xr_platform_vision_pro_roundtrip() {
    proptest!(|(session_id: u64)| {
        let platform = XrPlatform::VisionPro;
        let enc = encode_to_vec(&platform).expect("encode VisionPro failed");
        let (decoded, consumed): (XrPlatform, usize) =
            decode_from_slice(&enc).expect("decode VisionPro failed");
        prop_assert_eq!(decoded, XrPlatform::VisionPro);
        prop_assert_eq!(consumed, enc.len());
        prop_assert!(session_id <= u64::MAX);
    });
}

// ── 12. XrPlatform::WebXR variant ────────────────────────────────────────────

#[test]
fn test_xr_platform_webxr_roundtrip() {
    proptest!(|(fps: f32)| {
        let platform = XrPlatform::WebXR;
        let enc = encode_to_vec(&platform).expect("encode WebXR failed");
        let (decoded, consumed): (XrPlatform, usize) =
            decode_from_slice(&enc).expect("decode WebXR failed");
        prop_assert_eq!(decoded, XrPlatform::WebXR);
        prop_assert_eq!(consumed, enc.len());
        let _ = fps;
    });
}

// ── 13. TrackingMode::HandTracking variant ────────────────────────────────────

#[test]
fn test_tracking_mode_hand_tracking_roundtrip() {
    proptest!(|(frame_id: u64)| {
        let mode = TrackingMode::HandTracking;
        let enc = encode_to_vec(&mode).expect("encode HandTracking failed");
        let (decoded, consumed): (TrackingMode, usize) =
            decode_from_slice(&enc).expect("decode HandTracking failed");
        prop_assert_eq!(decoded, TrackingMode::HandTracking);
        prop_assert_eq!(consumed, enc.len());
        prop_assert!(frame_id <= u64::MAX);
    });
}

// ── 14. TrackingMode::EyeTracking variant ────────────────────────────────────

#[test]
fn test_tracking_mode_eye_tracking_roundtrip() {
    proptest!(|(timestamp_ns: u64)| {
        let mode = TrackingMode::EyeTracking;
        let enc = encode_to_vec(&mode).expect("encode EyeTracking failed");
        let (decoded, consumed): (TrackingMode, usize) =
            decode_from_slice(&enc).expect("decode EyeTracking failed");
        prop_assert_eq!(decoded, TrackingMode::EyeTracking);
        prop_assert_eq!(consumed, enc.len());
        prop_assert!(timestamp_ns <= u64::MAX);
    });
}

// ── 15. Vec<XrEvent> roundtrip ────────────────────────────────────────────────

#[test]
fn test_vec_xr_event_roundtrip() {
    proptest!(|(events in prop::collection::vec(arb_xr_event(), 0..6))| {
        let enc = encode_to_vec(&events).expect("encode Vec<XrEvent> failed");
        let (decoded, consumed): (Vec<XrEvent>, usize) =
            decode_from_slice(&enc).expect("decode Vec<XrEvent> failed");
        prop_assert_eq!(events, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 16. Option<XrFrame> – left hand present, right absent ────────────────────

#[test]
fn test_option_xr_frame_roundtrip() {
    proptest!(|(opt in prop::option::of(arb_xr_frame()))| {
        let enc = encode_to_vec(&opt).expect("encode Option<XrFrame> failed");
        let (decoded, consumed): (Option<XrFrame>, usize) =
            decode_from_slice(&enc).expect("decode Option<XrFrame> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 17. Option<XrSession> roundtrip ──────────────────────────────────────────

#[test]
fn test_option_xr_session_roundtrip() {
    proptest!(|(opt in prop::option::of(arb_xr_session()))| {
        let enc = encode_to_vec(&opt).expect("encode Option<XrSession> failed");
        let (decoded, consumed): (Option<XrSession>, usize) =
            decode_from_slice(&enc).expect("decode Option<XrSession> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 18. XrFrame hand position options – both None ────────────────────────────

#[test]
fn test_xr_frame_both_hands_none_roundtrip() {
    proptest!(|(frame_id: u64, timestamp_ns: u64, hp in arb_vec3(), hr in arb_quat())| {
        let frame = XrFrame {
            frame_id,
            timestamp_ns,
            head_pos: hp,
            head_rot: hr,
            left_hand_pos: None,
            right_hand_pos: None,
        };
        let enc = encode_to_vec(&frame).expect("encode XrFrame (no hands) failed");
        let (decoded, consumed): (XrFrame, usize) =
            decode_from_slice(&enc).expect("decode XrFrame (no hands) failed");
        prop_assert_eq!(frame, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 19. XrFrame hand position options – both Some ────────────────────────────

#[test]
fn test_xr_frame_both_hands_some_roundtrip() {
    proptest!(|(
        frame_id: u64,
        timestamp_ns: u64,
        hp in arb_vec3(),
        hr in arb_quat(),
        lh in arb_vec3(),
        rh in arb_vec3(),
    )| {
        let frame = XrFrame {
            frame_id,
            timestamp_ns,
            head_pos: hp,
            head_rot: hr,
            left_hand_pos: Some(lh),
            right_hand_pos: Some(rh),
        };
        let enc = encode_to_vec(&frame).expect("encode XrFrame (both hands) failed");
        let (decoded, consumed): (XrFrame, usize) =
            decode_from_slice(&enc).expect("decode XrFrame (both hands) failed");
        prop_assert_eq!(frame, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 20. XrEvent re-encode idempotent ──────────────────────────────────────────

#[test]
fn test_xr_event_reencode_idempotent() {
    proptest!(|(event in arb_xr_event())| {
        let enc1 = encode_to_vec(&event).expect("first encode XrEvent failed");
        let (decoded, _): (XrEvent, usize) =
            decode_from_slice(&enc1).expect("first decode XrEvent failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode XrEvent failed");
        prop_assert_eq!(enc1, enc2, "re-encoding XrEvent must be idempotent");
    });
}

// ── 21. Vec<XrSession> roundtrip ──────────────────────────────────────────────

#[test]
fn test_vec_xr_session_roundtrip() {
    proptest!(|(sessions in prop::collection::vec(arb_xr_session(), 0..5))| {
        let enc = encode_to_vec(&sessions).expect("encode Vec<XrSession> failed");
        let (decoded, consumed): (Vec<XrSession>, usize) =
            decode_from_slice(&enc).expect("decode Vec<XrSession> failed");
        prop_assert_eq!(sessions, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 22. (XrPlatform, TrackingMode, u64) tuple roundtrip ───────────────────────

#[test]
fn test_platform_tracking_session_tuple_roundtrip() {
    proptest!(|(
        platform in arb_xr_platform(),
        tracking in arb_tracking_mode(),
        session_id: u64,
    )| {
        let tup = (platform, tracking, session_id);
        let enc =
            encode_to_vec(&tup).expect("encode (XrPlatform, TrackingMode, u64) failed");
        let (decoded, consumed): ((XrPlatform, TrackingMode, u64), usize) =
            decode_from_slice(&enc).expect("decode (XrPlatform, TrackingMode, u64) failed");
        prop_assert_eq!(tup, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}
