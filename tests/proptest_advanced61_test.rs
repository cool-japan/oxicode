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

// ── Domain types: robotics / SLAM ─────────────────────────────────────────────

/// Robot 2-D pose in the world frame (x, y in metres; yaw in radians).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Pose2D {
    x: f64,
    y: f64,
    yaw: f64,
}

/// Robot 3-D pose (translation + quaternion orientation).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Pose3D {
    tx: f64,
    ty: f64,
    tz: f64,
    qw: f64,
    qx: f64,
    qy: f64,
    qz: f64,
}

/// A landmark observed in the map (position + unique id + confidence).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MapLandmark {
    landmark_id: u64,
    position_x: f64,
    position_y: f64,
    position_z: f64,
    confidence: f32,
    observed_count: u32,
    visible: bool,
}

/// 6×6 upper-triangular covariance matrix stored as 21 f64 entries (row-major).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CovarianceMatrix6x6 {
    entries: Vec<f64>,
}

/// Type of loop-closure detection result.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LoopClosureStatus {
    NotDetected,
    Candidate,
    Verified,
    Rejected,
    Integrated,
}

/// Sensor modality used for a keyframe.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SensorModality {
    MonoCamera,
    StereoCamera,
    Lidar2D,
    Lidar3D,
    RgbdCamera,
    Imu,
    Wheel,
    GpsRtk,
}

/// Quality level for a feature descriptor match.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MatchQuality {
    Poor,
    Fair,
    Good,
    Excellent,
}

/// Occupancy state for a grid cell.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OccupancyState {
    Unknown,
    Free,
    Occupied,
    Inflated,
}

/// A SLAM keyframe: pose + sensor info + loop closure status.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Keyframe {
    keyframe_id: u64,
    timestamp_us: u64,
    pose: Pose3D,
    modality: SensorModality,
    loop_status: LoopClosureStatus,
    is_marginalised: bool,
}

/// A feature descriptor (e.g. ORB / BRIEF) stored as raw bytes.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeatureDescriptor {
    feature_id: u64,
    descriptor_bytes: Vec<u8>,
    keypoint_x: f32,
    keypoint_y: f32,
    response: f32,
    octave: u8,
    quality: MatchQuality,
}

/// An edge in the pose-graph used for trajectory optimisation.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PoseGraphEdge {
    from_id: u64,
    to_id: u64,
    relative_pose: Pose3D,
    information_diag: Vec<f64>,
    edge_type: LoopClosureStatus,
    weight: f64,
}

/// A 2-D occupancy grid tile (compressed representation).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OccupancyGridTile {
    tile_x: i32,
    tile_y: i32,
    resolution_mm: u16,
    cells: Vec<OccupancyState>,
    update_seq: u64,
}

/// Sensor fusion result combining IMU pre-integration with visual odometry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FusedOdometry {
    seq: u64,
    pose: Pose3D,
    linear_velocity_x: f64,
    linear_velocity_y: f64,
    linear_velocity_z: f64,
    covariance: Option<CovarianceMatrix6x6>,
    reliable: bool,
}

// ── Prop strategies ───────────────────────────────────────────────────────────

fn pose2d_strategy() -> impl Strategy<Value = Pose2D> {
    (any::<f64>(), any::<f64>(), any::<f64>()).prop_map(|(x, y, yaw)| Pose2D { x, y, yaw })
}

fn pose3d_strategy() -> impl Strategy<Value = Pose3D> {
    (
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
    )
        .prop_map(|(tx, ty, tz, qw, qx, qy, qz)| Pose3D {
            tx,
            ty,
            tz,
            qw,
            qx,
            qy,
            qz,
        })
}

fn map_landmark_strategy() -> impl Strategy<Value = MapLandmark> {
    (
        any::<u64>(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        any::<f32>(),
        any::<u32>(),
        any::<bool>(),
    )
        .prop_map(
            |(
                landmark_id,
                position_x,
                position_y,
                position_z,
                confidence,
                observed_count,
                visible,
            )| {
                MapLandmark {
                    landmark_id,
                    position_x,
                    position_y,
                    position_z,
                    confidence,
                    observed_count,
                    visible,
                }
            },
        )
}

fn covariance_matrix_strategy() -> impl Strategy<Value = CovarianceMatrix6x6> {
    prop::collection::vec(any::<f64>(), 21).prop_map(|entries| CovarianceMatrix6x6 { entries })
}

fn loop_closure_status_strategy() -> impl Strategy<Value = LoopClosureStatus> {
    (0u8..5).prop_map(|v| match v {
        0 => LoopClosureStatus::NotDetected,
        1 => LoopClosureStatus::Candidate,
        2 => LoopClosureStatus::Verified,
        3 => LoopClosureStatus::Rejected,
        _ => LoopClosureStatus::Integrated,
    })
}

fn sensor_modality_strategy() -> impl Strategy<Value = SensorModality> {
    (0u8..8).prop_map(|v| match v {
        0 => SensorModality::MonoCamera,
        1 => SensorModality::StereoCamera,
        2 => SensorModality::Lidar2D,
        3 => SensorModality::Lidar3D,
        4 => SensorModality::RgbdCamera,
        5 => SensorModality::Imu,
        6 => SensorModality::Wheel,
        _ => SensorModality::GpsRtk,
    })
}

fn match_quality_strategy() -> impl Strategy<Value = MatchQuality> {
    (0u8..4).prop_map(|v| match v {
        0 => MatchQuality::Poor,
        1 => MatchQuality::Fair,
        2 => MatchQuality::Good,
        _ => MatchQuality::Excellent,
    })
}

fn occupancy_state_strategy() -> impl Strategy<Value = OccupancyState> {
    (0u8..4).prop_map(|v| match v {
        0 => OccupancyState::Unknown,
        1 => OccupancyState::Free,
        2 => OccupancyState::Occupied,
        _ => OccupancyState::Inflated,
    })
}

fn keyframe_strategy() -> impl Strategy<Value = Keyframe> {
    (
        any::<u64>(),
        any::<u64>(),
        pose3d_strategy(),
        sensor_modality_strategy(),
        loop_closure_status_strategy(),
        any::<bool>(),
    )
        .prop_map(
            |(keyframe_id, timestamp_us, pose, modality, loop_status, is_marginalised)| Keyframe {
                keyframe_id,
                timestamp_us,
                pose,
                modality,
                loop_status,
                is_marginalised,
            },
        )
}

fn feature_descriptor_strategy() -> impl Strategy<Value = FeatureDescriptor> {
    (
        any::<u64>(),
        prop::collection::vec(any::<u8>(), 32),
        any::<f32>(),
        any::<f32>(),
        any::<f32>(),
        any::<u8>(),
        match_quality_strategy(),
    )
        .prop_map(
            |(feature_id, descriptor_bytes, keypoint_x, keypoint_y, response, octave, quality)| {
                FeatureDescriptor {
                    feature_id,
                    descriptor_bytes,
                    keypoint_x,
                    keypoint_y,
                    response,
                    octave,
                    quality,
                }
            },
        )
}

fn pose_graph_edge_strategy() -> impl Strategy<Value = PoseGraphEdge> {
    (
        any::<u64>(),
        any::<u64>(),
        pose3d_strategy(),
        prop::collection::vec(any::<f64>(), 6),
        loop_closure_status_strategy(),
        any::<f64>(),
    )
        .prop_map(
            |(from_id, to_id, relative_pose, information_diag, edge_type, weight)| PoseGraphEdge {
                from_id,
                to_id,
                relative_pose,
                information_diag,
                edge_type,
                weight,
            },
        )
}

fn occupancy_grid_tile_strategy() -> impl Strategy<Value = OccupancyGridTile> {
    (
        any::<i32>(),
        any::<i32>(),
        any::<u16>(),
        prop::collection::vec(occupancy_state_strategy(), 0..16),
        any::<u64>(),
    )
        .prop_map(
            |(tile_x, tile_y, resolution_mm, cells, update_seq)| OccupancyGridTile {
                tile_x,
                tile_y,
                resolution_mm,
                cells,
                update_seq,
            },
        )
}

fn fused_odometry_strategy() -> impl Strategy<Value = FusedOdometry> {
    (
        any::<u64>(),
        pose3d_strategy(),
        any::<f64>(),
        any::<f64>(),
        any::<f64>(),
        prop::option::of(covariance_matrix_strategy()),
        any::<bool>(),
    )
        .prop_map(
            |(
                seq,
                pose,
                linear_velocity_x,
                linear_velocity_y,
                linear_velocity_z,
                covariance,
                reliable,
            )| {
                FusedOdometry {
                    seq,
                    pose,
                    linear_velocity_x,
                    linear_velocity_y,
                    linear_velocity_z,
                    covariance,
                    reliable,
                }
            },
        )
}

// ── 22 property-based tests ───────────────────────────────────────────────────

proptest! {
    // 1 – Pose2D struct roundtrip
    #[test]
    fn test_pose2d_roundtrip(pose in pose2d_strategy()) {
        let encoded = encode_to_vec(&pose).expect("encode Pose2D failed");
        let (decoded, _): (Pose2D, usize) = decode_from_slice(&encoded).expect("decode Pose2D failed");
        prop_assert_eq!(pose.x.to_bits(), decoded.x.to_bits());
        prop_assert_eq!(pose.y.to_bits(), decoded.y.to_bits());
        prop_assert_eq!(pose.yaw.to_bits(), decoded.yaw.to_bits());
    }

    // 2 – Pose3D (7-DOF) struct roundtrip
    #[test]
    fn test_pose3d_roundtrip(pose in pose3d_strategy()) {
        let encoded = encode_to_vec(&pose).expect("encode Pose3D failed");
        let (decoded, _): (Pose3D, usize) = decode_from_slice(&encoded).expect("decode Pose3D failed");
        prop_assert_eq!(pose.qw.to_bits(), decoded.qw.to_bits());
        prop_assert_eq!(pose.tx.to_bits(), decoded.tx.to_bits());
    }

    // 3 – MapLandmark struct roundtrip
    #[test]
    fn test_map_landmark_roundtrip(lm in map_landmark_strategy()) {
        let encoded = encode_to_vec(&lm).expect("encode MapLandmark failed");
        let (decoded, _): (MapLandmark, usize) = decode_from_slice(&encoded).expect("decode MapLandmark failed");
        prop_assert_eq!(lm, decoded);
    }

    // 4 – LoopClosureStatus enum roundtrip covering all variants
    #[test]
    fn test_loop_closure_status_all_variants(index in 0u8..5) {
        let status = match index {
            0 => LoopClosureStatus::NotDetected,
            1 => LoopClosureStatus::Candidate,
            2 => LoopClosureStatus::Verified,
            3 => LoopClosureStatus::Rejected,
            _ => LoopClosureStatus::Integrated,
        };
        let encoded = encode_to_vec(&status).expect("encode LoopClosureStatus failed");
        let (decoded, _): (LoopClosureStatus, usize) =
            decode_from_slice(&encoded).expect("decode LoopClosureStatus failed");
        prop_assert_eq!(status, decoded);
    }

    // 5 – SensorModality enum roundtrip covering all variants
    #[test]
    fn test_sensor_modality_all_variants(index in 0u8..8) {
        let modality = match index {
            0 => SensorModality::MonoCamera,
            1 => SensorModality::StereoCamera,
            2 => SensorModality::Lidar2D,
            3 => SensorModality::Lidar3D,
            4 => SensorModality::RgbdCamera,
            5 => SensorModality::Imu,
            6 => SensorModality::Wheel,
            _ => SensorModality::GpsRtk,
        };
        let encoded = encode_to_vec(&modality).expect("encode SensorModality failed");
        let (decoded, _): (SensorModality, usize) =
            decode_from_slice(&encoded).expect("decode SensorModality failed");
        prop_assert_eq!(modality, decoded);
    }

    // 6 – Keyframe nested struct roundtrip
    #[test]
    fn test_keyframe_roundtrip(kf in keyframe_strategy()) {
        let encoded = encode_to_vec(&kf).expect("encode Keyframe failed");
        let (decoded, consumed): (Keyframe, usize) =
            decode_from_slice(&encoded).expect("decode Keyframe failed");
        prop_assert_eq!(&kf, &decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 7 – FeatureDescriptor with Vec<u8> descriptor roundtrip
    #[test]
    fn test_feature_descriptor_roundtrip(fd in feature_descriptor_strategy()) {
        let encoded = encode_to_vec(&fd).expect("encode FeatureDescriptor failed");
        let (decoded, _): (FeatureDescriptor, usize) =
            decode_from_slice(&encoded).expect("decode FeatureDescriptor failed");
        prop_assert_eq!(fd, decoded);
    }

    // 8 – Vec<MapLandmark> roundtrip
    #[test]
    fn test_vec_map_landmarks_roundtrip(
        landmarks in prop::collection::vec(map_landmark_strategy(), 0..10)
    ) {
        let encoded = encode_to_vec(&landmarks).expect("encode Vec<MapLandmark> failed");
        let (decoded, _): (Vec<MapLandmark>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<MapLandmark> failed");
        prop_assert_eq!(landmarks, decoded);
    }

    // 9 – Vec<Keyframe> roundtrip
    #[test]
    fn test_vec_keyframes_roundtrip(
        keyframes in prop::collection::vec(keyframe_strategy(), 0..6)
    ) {
        let encoded = encode_to_vec(&keyframes).expect("encode Vec<Keyframe> failed");
        let (decoded, _): (Vec<Keyframe>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Keyframe> failed");
        prop_assert_eq!(keyframes, decoded);
    }

    // 10 – Option<Pose3D> roundtrip (Some and None paths)
    #[test]
    fn test_option_pose3d_roundtrip(maybe_pose in prop::option::of(pose3d_strategy())) {
        let encoded = encode_to_vec(&maybe_pose).expect("encode Option<Pose3D> failed");
        let (decoded, _): (Option<Pose3D>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Pose3D> failed");
        match (maybe_pose, decoded) {
            (None, None) => {}
            (Some(a), Some(b)) => {
                prop_assert_eq!(a.tx.to_bits(), b.tx.to_bits());
                prop_assert_eq!(a.qw.to_bits(), b.qw.to_bits());
            }
            _ => prop_assert!(false, "Option<Pose3D> Some/None mismatch after roundtrip"),
        }
    }

    // 11 – Option<MapLandmark> roundtrip
    #[test]
    fn test_option_map_landmark_roundtrip(
        maybe_lm in prop::option::of(map_landmark_strategy())
    ) {
        let encoded = encode_to_vec(&maybe_lm).expect("encode Option<MapLandmark> failed");
        let (decoded, _): (Option<MapLandmark>, usize) =
            decode_from_slice(&encoded).expect("decode Option<MapLandmark> failed");
        prop_assert_eq!(maybe_lm, decoded);
    }

    // 12 – Consumed bytes match encoded length for MapLandmark
    #[test]
    fn test_map_landmark_consumed_bytes(lm in map_landmark_strategy()) {
        let encoded = encode_to_vec(&lm).expect("encode MapLandmark for byte check failed");
        let (_, consumed): (MapLandmark, usize) =
            decode_from_slice(&encoded).expect("decode MapLandmark for byte check failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    // 13 – Consumed bytes match encoded length for FeatureDescriptor
    #[test]
    fn test_feature_descriptor_consumed_bytes(fd in feature_descriptor_strategy()) {
        let encoded =
            encode_to_vec(&fd).expect("encode FeatureDescriptor for byte check failed");
        let (_, consumed): (FeatureDescriptor, usize) =
            decode_from_slice(&encoded).expect("decode FeatureDescriptor for byte check failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    // 14 – Deterministic encoding of Keyframe
    #[test]
    fn test_keyframe_encode_deterministic(kf in keyframe_strategy()) {
        let enc1 = encode_to_vec(&kf).expect("first encode Keyframe failed");
        let enc2 = encode_to_vec(&kf).expect("second encode Keyframe failed");
        prop_assert_eq!(enc1, enc2);
    }

    // 15 – Deterministic encoding of FeatureDescriptor
    #[test]
    fn test_feature_descriptor_encode_deterministic(fd in feature_descriptor_strategy()) {
        let enc1 = encode_to_vec(&fd).expect("first encode FeatureDescriptor failed");
        let enc2 = encode_to_vec(&fd).expect("second encode FeatureDescriptor failed");
        prop_assert_eq!(enc1, enc2);
    }

    // 16 – PoseGraphEdge nested struct roundtrip
    #[test]
    fn test_pose_graph_edge_roundtrip(edge in pose_graph_edge_strategy()) {
        let encoded = encode_to_vec(&edge).expect("encode PoseGraphEdge failed");
        let (decoded, consumed): (PoseGraphEdge, usize) =
            decode_from_slice(&encoded).expect("decode PoseGraphEdge failed");
        prop_assert_eq!(&edge, &decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 17 – OccupancyGridTile with Vec<OccupancyState> roundtrip
    #[test]
    fn test_occupancy_grid_tile_roundtrip(tile in occupancy_grid_tile_strategy()) {
        let encoded = encode_to_vec(&tile).expect("encode OccupancyGridTile failed");
        let (decoded, _): (OccupancyGridTile, usize) =
            decode_from_slice(&encoded).expect("decode OccupancyGridTile failed");
        prop_assert_eq!(tile, decoded);
    }

    // 18 – FusedOdometry with optional covariance roundtrip
    #[test]
    fn test_fused_odometry_roundtrip(odom in fused_odometry_strategy()) {
        let encoded = encode_to_vec(&odom).expect("encode FusedOdometry failed");
        let (decoded, consumed): (FusedOdometry, usize) =
            decode_from_slice(&encoded).expect("decode FusedOdometry failed");
        prop_assert_eq!(odom.seq, decoded.seq);
        prop_assert_eq!(odom.reliable, decoded.reliable);
        prop_assert_eq!(consumed, encoded.len());
    }

    // 19 – CovarianceMatrix6x6 (21 f64 entries) roundtrip
    #[test]
    fn test_covariance_matrix_roundtrip(cov in covariance_matrix_strategy()) {
        let encoded = encode_to_vec(&cov).expect("encode CovarianceMatrix6x6 failed");
        let (decoded, _): (CovarianceMatrix6x6, usize) =
            decode_from_slice(&encoded).expect("decode CovarianceMatrix6x6 failed");
        prop_assert_eq!(cov.entries.len(), decoded.entries.len());
        for (a, b) in cov.entries.iter().zip(decoded.entries.iter()) {
            prop_assert_eq!(a.to_bits(), b.to_bits());
        }
    }

    // 20 – Primitive i64 (signed odometry delta) roundtrip
    #[test]
    fn test_i64_odometry_delta_roundtrip(val in any::<i64>()) {
        let encoded = encode_to_vec(&val).expect("encode i64 odometry delta failed");
        let (decoded, _): (i64, usize) =
            decode_from_slice(&encoded).expect("decode i64 odometry delta failed");
        prop_assert_eq!(val, decoded);
    }

    // 21 – Primitive u8 (descriptor byte) roundtrip
    #[test]
    fn test_u8_descriptor_byte_roundtrip(val in any::<u8>()) {
        let encoded = encode_to_vec(&val).expect("encode u8 descriptor byte failed");
        let (decoded, _): (u8, usize) =
            decode_from_slice(&encoded).expect("decode u8 descriptor byte failed");
        prop_assert_eq!(val, decoded);
    }

    // 22 – String landmark label roundtrip
    #[test]
    fn test_string_landmark_label_roundtrip(label in "\\PC*") {
        let encoded = encode_to_vec(&label).expect("encode String landmark label failed");
        let (decoded, consumed): (String, usize) =
            decode_from_slice(&encoded).expect("decode String landmark label failed");
        prop_assert_eq!(&label, &decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}
