//! Advanced property-based tests (set 41) using proptest.
//!
//! Theme: Robotics / motion planning — JointType, JointState, RobotPose,
//! Waypoint, TrajectorySegment.
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Tests cover roundtrips, consumed bytes, determinism, Vec types,
//! all JointType variants, nested Waypoint / TrajectorySegment,
//! and boundary values.

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
enum JointType {
    Revolute,
    Prismatic,
    Fixed,
    Continuous,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct JointState {
    joint_type: JointType,
    position: f64,
    velocity: f64,
    effort: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RobotPose {
    x: f64,
    y: f64,
    z: f64,
    roll: f64,
    pitch: f64,
    yaw: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Waypoint {
    pose: RobotPose,
    time_s: f64,
    velocity_scale: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrajectorySegment {
    start: Waypoint,
    end: Waypoint,
    joints: Vec<JointState>,
}

// ── Strategy helpers ──────────────────────────────────────────────────────────

fn joint_type_strategy() -> impl Strategy<Value = JointType> {
    prop_oneof![
        Just(JointType::Revolute),
        Just(JointType::Prismatic),
        Just(JointType::Fixed),
        Just(JointType::Continuous),
    ]
}

fn joint_state_strategy() -> impl Strategy<Value = JointState> {
    (
        joint_type_strategy(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
    )
        .prop_map(|(joint_type, position, velocity, effort)| JointState {
            joint_type,
            position,
            velocity,
            effort,
        })
}

fn robot_pose_strategy() -> impl Strategy<Value = RobotPose> {
    (
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
    )
        .prop_map(|(x, y, z, roll, pitch, yaw)| RobotPose {
            x,
            y,
            z,
            roll,
            pitch,
            yaw,
        })
}

fn waypoint_strategy() -> impl Strategy<Value = Waypoint> {
    (robot_pose_strategy(), any::<f64>(), any::<f32>()).prop_map(
        |(pose, time_s, velocity_scale)| Waypoint {
            pose,
            time_s,
            velocity_scale,
        },
    )
}

fn trajectory_segment_strategy() -> impl Strategy<Value = TrajectorySegment> {
    (
        waypoint_strategy(),
        waypoint_strategy(),
        prop::collection::vec(joint_state_strategy(), 0..8usize),
    )
        .prop_map(|(start, end, joints)| TrajectorySegment { start, end, joints })
}

// ── 1. JointType roundtrip ────────────────────────────────────────────────────

#[test]
fn test_joint_type_roundtrip() {
    proptest!(|(jt in joint_type_strategy())| {
        let enc = encode_to_vec(&jt).expect("encode JointType failed");
        let (decoded, consumed): (JointType, usize) =
            decode_from_slice(&enc).expect("decode JointType failed");
        prop_assert_eq!(jt, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 2. JointType consumed bytes equal encoded length ─────────────────────────

#[test]
fn test_joint_type_consumed_eq_len() {
    proptest!(|(jt in joint_type_strategy())| {
        let enc = encode_to_vec(&jt).expect("encode JointType failed");
        let (_decoded, consumed): (JointType, usize) =
            decode_from_slice(&enc).expect("decode JointType failed");
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 3. JointType deterministic encoding ──────────────────────────────────────

#[test]
fn test_joint_type_deterministic_encoding() {
    proptest!(|(jt in joint_type_strategy())| {
        let enc1 = encode_to_vec(&jt).expect("first encode JointType failed");
        let enc2 = encode_to_vec(&jt).expect("second encode JointType failed");
        prop_assert_eq!(enc1, enc2, "JointType encoding must be deterministic");
    });
}

// ── 4. All four JointType variants encode and decode correctly ────────────────

#[test]
fn test_all_joint_type_variants_roundtrip() {
    let variants = [
        JointType::Revolute,
        JointType::Prismatic,
        JointType::Fixed,
        JointType::Continuous,
    ];
    for jt in &variants {
        let enc = encode_to_vec(jt).expect("encode JointType variant failed");
        let (decoded, consumed): (JointType, usize) =
            decode_from_slice(&enc).expect("decode JointType variant failed");
        assert_eq!(jt, &decoded, "JointType variant mismatch");
        assert_eq!(consumed, enc.len(), "JointType consumed bytes mismatch");
    }
    proptest!(|(_dummy: u8)| {
        prop_assert!(true);
    });
}

// ── 5. JointState roundtrip ───────────────────────────────────────────────────

#[test]
fn test_joint_state_roundtrip() {
    proptest!(|(js in joint_state_strategy())| {
        let enc = encode_to_vec(&js).expect("encode JointState failed");
        let (decoded, consumed): (JointState, usize) =
            decode_from_slice(&enc).expect("decode JointState failed");
        prop_assert_eq!(js, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 6. JointState re-encode is idempotent ────────────────────────────────────

#[test]
fn test_joint_state_reencode_idempotent() {
    proptest!(|(js in joint_state_strategy())| {
        let enc1 = encode_to_vec(&js).expect("first encode JointState failed");
        let (decoded, _): (JointState, usize) =
            decode_from_slice(&enc1).expect("decode JointState failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode JointState failed");
        prop_assert_eq!(enc1, enc2, "JointState re-encoding must be idempotent");
    });
}

// ── 7. JointState with zero position (boundary) ───────────────────────────────

#[test]
fn test_joint_state_zero_position_roundtrip() {
    proptest!(|(
        jt in joint_type_strategy(),
        velocity: f64,
        effort: f64,
    )| {
        let js = JointState { joint_type: jt, position: 0.0, velocity, effort };
        let enc = encode_to_vec(&js).expect("encode zero-position JointState failed");
        let (decoded, consumed): (JointState, usize) =
            decode_from_slice(&enc).expect("decode zero-position JointState failed");
        prop_assert_eq!(js, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 8. Vec<JointState> roundtrip ─────────────────────────────────────────────

#[test]
fn test_vec_joint_state_roundtrip() {
    proptest!(|(joints in prop::collection::vec(joint_state_strategy(), 0..12usize))| {
        let enc = encode_to_vec(&joints).expect("encode Vec<JointState> failed");
        let (decoded, consumed): (Vec<JointState>, usize) =
            decode_from_slice(&enc).expect("decode Vec<JointState> failed");
        prop_assert_eq!(joints, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 9. RobotPose roundtrip ────────────────────────────────────────────────────

#[test]
fn test_robot_pose_roundtrip() {
    proptest!(|(pose in robot_pose_strategy())| {
        let enc = encode_to_vec(&pose).expect("encode RobotPose failed");
        let (decoded, consumed): (RobotPose, usize) =
            decode_from_slice(&enc).expect("decode RobotPose failed");
        prop_assert_eq!(pose, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 10. RobotPose deterministic encoding ─────────────────────────────────────

#[test]
fn test_robot_pose_deterministic_encoding() {
    proptest!(|(pose in robot_pose_strategy())| {
        let enc1 = encode_to_vec(&pose).expect("first encode RobotPose failed");
        let enc2 = encode_to_vec(&pose).expect("second encode RobotPose failed");
        prop_assert_eq!(enc1, enc2, "RobotPose encoding must be deterministic");
    });
}

// ── 11. RobotPose at origin (all zeros boundary) ──────────────────────────────

#[test]
fn test_robot_pose_origin_roundtrip() {
    proptest!(|(_dummy: u8)| {
        let pose = RobotPose { x: 0.0, y: 0.0, z: 0.0, roll: 0.0, pitch: 0.0, yaw: 0.0 };
        let enc = encode_to_vec(&pose).expect("encode origin RobotPose failed");
        let (decoded, consumed): (RobotPose, usize) =
            decode_from_slice(&enc).expect("decode origin RobotPose failed");
        prop_assert_eq!(pose, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 12. RobotPose re-encode is idempotent ────────────────────────────────────

#[test]
fn test_robot_pose_reencode_idempotent() {
    proptest!(|(pose in robot_pose_strategy())| {
        let enc1 = encode_to_vec(&pose).expect("first encode RobotPose failed");
        let (decoded, _): (RobotPose, usize) =
            decode_from_slice(&enc1).expect("decode RobotPose failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode RobotPose failed");
        prop_assert_eq!(enc1, enc2, "RobotPose re-encoding must be idempotent");
    });
}

// ── 13. Waypoint roundtrip ────────────────────────────────────────────────────

#[test]
fn test_waypoint_roundtrip() {
    proptest!(|(wp in waypoint_strategy())| {
        let enc = encode_to_vec(&wp).expect("encode Waypoint failed");
        let (decoded, consumed): (Waypoint, usize) =
            decode_from_slice(&enc).expect("decode Waypoint failed");
        prop_assert_eq!(wp, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 14. Waypoint with zero time_s (boundary) ──────────────────────────────────

#[test]
fn test_waypoint_zero_time_roundtrip() {
    proptest!(|(
        pose in robot_pose_strategy(),
        velocity_scale: f32,
    )| {
        let wp = Waypoint { pose, time_s: 0.0, velocity_scale };
        let enc = encode_to_vec(&wp).expect("encode zero-time Waypoint failed");
        let (decoded, consumed): (Waypoint, usize) =
            decode_from_slice(&enc).expect("decode zero-time Waypoint failed");
        prop_assert_eq!(wp, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 15. Waypoint re-encode is idempotent ─────────────────────────────────────

#[test]
fn test_waypoint_reencode_idempotent() {
    proptest!(|(wp in waypoint_strategy())| {
        let enc1 = encode_to_vec(&wp).expect("first encode Waypoint failed");
        let (decoded, _): (Waypoint, usize) =
            decode_from_slice(&enc1).expect("decode Waypoint failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode Waypoint failed");
        prop_assert_eq!(enc1, enc2, "Waypoint re-encoding must be idempotent");
    });
}

// ── 16. Vec<Waypoint> roundtrip ───────────────────────────────────────────────

#[test]
fn test_vec_waypoint_roundtrip() {
    proptest!(|(wps in prop::collection::vec(waypoint_strategy(), 0..10usize))| {
        let enc = encode_to_vec(&wps).expect("encode Vec<Waypoint> failed");
        let (decoded, consumed): (Vec<Waypoint>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Waypoint> failed");
        prop_assert_eq!(wps, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 17. TrajectorySegment roundtrip ───────────────────────────────────────────

#[test]
fn test_trajectory_segment_roundtrip() {
    proptest!(|(seg in trajectory_segment_strategy())| {
        let enc = encode_to_vec(&seg).expect("encode TrajectorySegment failed");
        let (decoded, consumed): (TrajectorySegment, usize) =
            decode_from_slice(&enc).expect("decode TrajectorySegment failed");
        prop_assert_eq!(seg, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 18. TrajectorySegment with empty joints list ──────────────────────────────

#[test]
fn test_trajectory_segment_empty_joints_roundtrip() {
    proptest!(|(
        start in waypoint_strategy(),
        end in waypoint_strategy(),
    )| {
        let seg = TrajectorySegment { start, end, joints: vec![] };
        let enc = encode_to_vec(&seg).expect("encode empty-joints TrajectorySegment failed");
        let (decoded, consumed): (TrajectorySegment, usize) =
            decode_from_slice(&enc).expect("decode empty-joints TrajectorySegment failed");
        prop_assert_eq!(seg, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 19. TrajectorySegment joint count preserved after roundtrip ───────────────

#[test]
fn test_trajectory_segment_joint_count_preserved() {
    proptest!(|(seg in trajectory_segment_strategy())| {
        let original_count = seg.joints.len();
        let enc = encode_to_vec(&seg).expect("encode TrajectorySegment failed");
        let (decoded, _): (TrajectorySegment, usize) =
            decode_from_slice(&enc).expect("decode TrajectorySegment failed");
        prop_assert_eq!(decoded.joints.len(), original_count,
            "joint count must survive encode/decode");
    });
}

// ── 20. TrajectorySegment re-encode is idempotent ─────────────────────────────

#[test]
fn test_trajectory_segment_reencode_idempotent() {
    proptest!(|(seg in trajectory_segment_strategy())| {
        let enc1 = encode_to_vec(&seg).expect("first encode TrajectorySegment failed");
        let (decoded, _): (TrajectorySegment, usize) =
            decode_from_slice(&enc1).expect("decode TrajectorySegment failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode TrajectorySegment failed");
        prop_assert_eq!(enc1, enc2, "TrajectorySegment re-encoding must be idempotent");
    });
}

// ── 21. Vec<TrajectorySegment> roundtrip ──────────────────────────────────────

#[test]
fn test_vec_trajectory_segment_roundtrip() {
    proptest!(|(segs in prop::collection::vec(trajectory_segment_strategy(), 0..5usize))| {
        let enc = encode_to_vec(&segs).expect("encode Vec<TrajectorySegment> failed");
        let (decoded, consumed): (Vec<TrajectorySegment>, usize) =
            decode_from_slice(&enc).expect("decode Vec<TrajectorySegment> failed");
        prop_assert_eq!(segs, decoded);
        prop_assert_eq!(consumed, enc.len());
    });
}

// ── 22. TrajectorySegment encoding grows monotonically with joint count ────────

#[test]
fn test_trajectory_segment_encoding_grows_with_joints() {
    proptest!(|(
        start in waypoint_strategy(),
        end in waypoint_strategy(),
        base_joint in joint_state_strategy(),
        extra_joint in joint_state_strategy(),
    )| {
        let seg_fewer = TrajectorySegment {
            start: start.clone(),
            end: end.clone(),
            joints: vec![base_joint.clone()],
        };
        let seg_more = TrajectorySegment {
            start,
            end,
            joints: vec![base_joint, extra_joint],
        };
        let enc_fewer = encode_to_vec(&seg_fewer).expect("encode fewer-joints segment failed");
        let enc_more = encode_to_vec(&seg_more).expect("encode more-joints segment failed");
        prop_assert!(
            enc_more.len() >= enc_fewer.len(),
            "more joints should produce >= encoded bytes"
        );
    });
}
