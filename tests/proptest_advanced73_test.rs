//! Advanced property-based tests (set 73) — Autonomous Robotics domain.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers LIDAR point clouds, robot joint angles, gripper force measurements,
//! IMU readings, SLAM map cells, path planning waypoints, obstacle detection
//! bounding boxes, motor PID parameters, battery charge cycles, collision
//! avoidance distances, terrain classification, task completion status, and
//! manipulator kinematics.

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

/// A single LIDAR return point in 3-D space.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LidarPoint {
    /// X coordinate in metres (robot frame).
    x_m: f32,
    /// Y coordinate in metres (robot frame).
    y_m: f32,
    /// Z coordinate in metres (robot frame).
    z_m: f32,
    /// Return intensity (0–255).
    intensity: u8,
    /// Ring / channel index on the sensor.
    ring: u16,
}

/// A sparse LIDAR point cloud scan.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LidarScan {
    /// Scan sequence number.
    seq: u64,
    /// Timestamp in nanoseconds since boot.
    stamp_ns: u64,
    /// Points captured in this scan.
    points: Vec<LidarPoint>,
}

/// Joint-angle snapshot for a single revolute joint.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct JointAngle {
    /// Joint identifier (0-based index).
    joint_id: u8,
    /// Position in radians.
    position_rad: f32,
    /// Velocity in radians per second.
    velocity_rad_s: f32,
    /// Torque in Newton-metres.
    torque_nm: f32,
}

/// Gripper force measurement from a tactile sensor array.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GripperForce {
    /// Sensor pad identifier.
    pad_id: u8,
    /// Normal force in Newtons.
    normal_force_n: f32,
    /// Shear force in Newtons.
    shear_force_n: f32,
    /// Contact area in mm^2.
    contact_area_mm2: f32,
    /// Whether slip has been detected.
    slip_detected: bool,
}

/// Inertial Measurement Unit reading (6-DoF).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImuReading {
    /// Timestamp in microseconds since boot.
    stamp_us: u64,
    /// Linear acceleration X in m/s^2.
    accel_x: f32,
    /// Linear acceleration Y in m/s^2.
    accel_y: f32,
    /// Linear acceleration Z in m/s^2.
    accel_z: f32,
    /// Angular velocity X in rad/s.
    gyro_x: f32,
    /// Angular velocity Y in rad/s.
    gyro_y: f32,
    /// Angular velocity Z in rad/s.
    gyro_z: f32,
}

/// Occupancy state of a single SLAM map cell.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SlamCellState {
    /// Free space — traversable.
    Free,
    /// Occupied by an obstacle.
    Occupied,
    /// Not yet observed.
    Unknown,
    /// Probabilistic occupancy (0.0 = free, 1.0 = occupied).
    Probabilistic { probability: f32 },
}

/// A cell in a 2-D SLAM occupancy grid.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SlamMapCell {
    /// Grid column index.
    col: u32,
    /// Grid row index.
    row: u32,
    /// Cell occupancy state.
    state: SlamCellState,
    /// Last-update timestamp in milliseconds.
    last_update_ms: u64,
}

/// A waypoint in a planned robot path.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PathWaypoint {
    /// Waypoint sequence index.
    index: u32,
    /// X position in metres (world frame).
    x_m: f32,
    /// Y position in metres (world frame).
    y_m: f32,
    /// Target heading in radians.
    heading_rad: f32,
    /// Desired linear velocity in m/s at this waypoint.
    velocity_m_s: f32,
}

/// Axis-aligned bounding box for obstacle detection.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ObstacleBbox {
    /// Detection confidence (0.0 – 1.0).
    confidence: f32,
    /// Centre X in metres.
    cx_m: f32,
    /// Centre Y in metres.
    cy_m: f32,
    /// Centre Z in metres.
    cz_m: f32,
    /// Half-extent X in metres.
    hx_m: f32,
    /// Half-extent Y in metres.
    hy_m: f32,
    /// Half-extent Z in metres.
    hz_m: f32,
    /// Object class label.
    label: String,
}

/// Motor PID controller parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MotorPidParams {
    /// Motor identifier.
    motor_id: u8,
    /// Proportional gain.
    kp: f32,
    /// Integral gain.
    ki: f32,
    /// Derivative gain.
    kd: f32,
    /// Output saturation limit.
    output_limit: f32,
}

/// Battery charge/discharge cycle record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatteryChargeCycle {
    /// Cycle number.
    cycle_number: u32,
    /// State-of-charge at start (0.0 – 1.0).
    soc_start: f32,
    /// State-of-charge at end (0.0 – 1.0).
    soc_end: f32,
    /// Duration of the cycle in seconds.
    duration_s: u32,
    /// Peak current in amperes.
    peak_current_a: f32,
    /// Average cell temperature in degrees Celsius.
    avg_temp_c: f32,
}

/// Collision avoidance distance measurement from a proximity sensor.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CollisionDistance {
    /// Sensor identifier.
    sensor_id: u8,
    /// Measured distance in metres.
    distance_m: f32,
    /// Whether the reading is within the emergency-stop threshold.
    emergency: bool,
    /// Minimum safe distance for this sensor zone in metres.
    min_safe_m: f32,
}

/// Terrain classification result from visual/proprioceptive analysis.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TerrainClass {
    /// Smooth indoor floor.
    IndoorFlat,
    /// Rough outdoor terrain.
    OutdoorRough,
    /// Gravel surface.
    Gravel { grain_size_mm: f32 },
    /// Grassy area.
    Grass { moisture_fraction: f32 },
    /// Ramp or incline.
    Incline { slope_deg: f32 },
}

/// Task completion status for an autonomous mission segment.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TaskStatus {
    /// Queued but not started.
    Pending,
    /// Currently executing — progress as a fraction (0.0 – 1.0).
    InProgress { progress: f32 },
    /// Completed successfully — elapsed time in milliseconds.
    Completed { elapsed_ms: u64 },
    /// Failed with an error code.
    Failed { error_code: u16, message: String },
    /// Cancelled by operator.
    Cancelled,
}

/// Forward kinematics result for a serial manipulator.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ManipulatorPose {
    /// End-effector position X in metres.
    ee_x_m: f32,
    /// End-effector position Y in metres.
    ee_y_m: f32,
    /// End-effector position Z in metres.
    ee_z_m: f32,
    /// End-effector orientation quaternion w.
    qw: f32,
    /// End-effector orientation quaternion x.
    qx: f32,
    /// End-effector orientation quaternion y.
    qy: f32,
    /// End-effector orientation quaternion z.
    qz: f32,
    /// Joint angles that produced this pose.
    joint_angles: Vec<f32>,
}

// ── prop_compose! strategies ────────────────────────────────────────────────

prop_compose! {
    fn arb_lidar_point()(
        x_m in -50.0f32..50.0f32,
        y_m in -50.0f32..50.0f32,
        z_m in -5.0f32..20.0f32,
        intensity: u8,
        ring in 0u16..128u16,
    ) -> LidarPoint {
        LidarPoint { x_m, y_m, z_m, intensity, ring }
    }
}

prop_compose! {
    fn arb_joint_angle()(
        joint_id in 0u8..12u8,
        position_rad in (-3.15f32)..3.15f32,
        velocity_rad_s in (-6.28f32)..6.28f32,
        torque_nm in (-100.0f32)..100.0f32,
    ) -> JointAngle {
        JointAngle { joint_id, position_rad, velocity_rad_s, torque_nm }
    }
}

prop_compose! {
    fn arb_waypoint()(
        index in 0u32..1000u32,
        x_m in -100.0f32..100.0f32,
        y_m in -100.0f32..100.0f32,
        heading_rad in (-3.15f32)..3.15f32,
        velocity_m_s in 0.0f32..3.0f32,
    ) -> PathWaypoint {
        PathWaypoint { index, x_m, y_m, heading_rad, velocity_m_s }
    }
}

prop_compose! {
    fn arb_slam_cell()(
        col in 0u32..1024u32,
        row in 0u32..1024u32,
        state_kind in 0u8..4u8,
        prob in 0.0f32..1.0f32,
        last_update_ms: u64,
    ) -> SlamMapCell {
        let state = match state_kind {
            0 => SlamCellState::Free,
            1 => SlamCellState::Occupied,
            2 => SlamCellState::Unknown,
            _ => SlamCellState::Probabilistic { probability: prob },
        };
        SlamMapCell { col, row, state, last_update_ms }
    }
}

prop_compose! {
    fn arb_collision_distance()(
        sensor_id in 0u8..32u8,
        distance_m in 0.01f32..10.0f32,
        emergency: bool,
        min_safe_m in 0.1f32..2.0f32,
    ) -> CollisionDistance {
        CollisionDistance { sensor_id, distance_m, emergency, min_safe_m }
    }
}

// ── Tests 1–22 ────────────────────────────────────────────────────────────────

// ── 1. LidarPoint roundtrip ──────────────────────────────────────────────────

#[test]
fn test_lidar_point_roundtrip() {
    proptest!(|(point in arb_lidar_point())| {
        let enc = encode_to_vec(&point).expect("encode LidarPoint failed");
        let (dec, consumed): (LidarPoint, usize) =
            decode_from_slice(&enc).expect("decode LidarPoint failed");
        prop_assert_eq!(&point, &dec, "LidarPoint roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 2. LidarScan roundtrip ──────────────────────────────────────────────────

#[test]
fn test_lidar_scan_roundtrip() {
    proptest!(|(
        seq: u64,
        stamp_ns: u64,
        points in prop::collection::vec(arb_lidar_point(), 0..16usize),
    )| {
        let val = LidarScan { seq, stamp_ns, points };
        let enc = encode_to_vec(&val).expect("encode LidarScan failed");
        let (dec, consumed): (LidarScan, usize) =
            decode_from_slice(&enc).expect("decode LidarScan failed");
        prop_assert_eq!(&val, &dec, "LidarScan roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 3. JointAngle roundtrip ─────────────────────────────────────────────────

#[test]
fn test_joint_angle_roundtrip() {
    proptest!(|(angle in arb_joint_angle())| {
        let enc = encode_to_vec(&angle).expect("encode JointAngle failed");
        let (dec, consumed): (JointAngle, usize) =
            decode_from_slice(&enc).expect("decode JointAngle failed");
        prop_assert_eq!(&angle, &dec, "JointAngle roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 4. Vec<JointAngle> roundtrip (robot arm snapshot) ───────────────────────

#[test]
fn test_vec_joint_angle_roundtrip() {
    proptest!(|(
        joints in prop::collection::vec(arb_joint_angle(), 1..8usize),
    )| {
        let enc = encode_to_vec(&joints).expect("encode Vec<JointAngle> failed");
        let (dec, consumed): (Vec<JointAngle>, usize) =
            decode_from_slice(&enc).expect("decode Vec<JointAngle> failed");
        prop_assert_eq!(&joints, &dec, "Vec<JointAngle> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 5. GripperForce roundtrip ───────────────────────────────────────────────

#[test]
fn test_gripper_force_roundtrip() {
    proptest!(|(
        pad_id in 0u8..8u8,
        normal_force_n in 0.0f32..50.0f32,
        shear_force_n in 0.0f32..20.0f32,
        contact_area_mm2 in 0.0f32..400.0f32,
        slip_detected: bool,
    )| {
        let val = GripperForce { pad_id, normal_force_n, shear_force_n, contact_area_mm2, slip_detected };
        let enc = encode_to_vec(&val).expect("encode GripperForce failed");
        let (dec, consumed): (GripperForce, usize) =
            decode_from_slice(&enc).expect("decode GripperForce failed");
        prop_assert_eq!(&val, &dec, "GripperForce roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 6. GripperForce re-encode determinism ───────────────────────────────────

#[test]
fn test_gripper_force_determinism() {
    proptest!(|(
        pad_id in 0u8..8u8,
        normal_force_n in 0.0f32..50.0f32,
        shear_force_n in 0.0f32..20.0f32,
        contact_area_mm2 in 0.0f32..400.0f32,
        slip_detected: bool,
    )| {
        let val = GripperForce { pad_id, normal_force_n, shear_force_n, contact_area_mm2, slip_detected };
        let enc1 = encode_to_vec(&val).expect("first encode GripperForce failed");
        let enc2 = encode_to_vec(&val).expect("second encode GripperForce failed");
        prop_assert_eq!(enc1, enc2, "encoding must be deterministic");
    });
}

// ── 7. ImuReading roundtrip ─────────────────────────────────────────────────

#[test]
fn test_imu_reading_roundtrip() {
    proptest!(|(
        stamp_us: u64,
        accel_x in (-20.0f32)..20.0f32,
        accel_y in (-20.0f32)..20.0f32,
        accel_z in (-20.0f32)..20.0f32,
        gyro_x in (-35.0f32)..35.0f32,
        gyro_y in (-35.0f32)..35.0f32,
        gyro_z in (-35.0f32)..35.0f32,
    )| {
        let val = ImuReading { stamp_us, accel_x, accel_y, accel_z, gyro_x, gyro_y, gyro_z };
        let enc = encode_to_vec(&val).expect("encode ImuReading failed");
        let (dec, consumed): (ImuReading, usize) =
            decode_from_slice(&enc).expect("decode ImuReading failed");
        prop_assert_eq!(&val, &dec, "ImuReading roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 8. ImuReading re-encode idempotency ─────────────────────────────────────

#[test]
fn test_imu_reading_reencode_idempotent() {
    proptest!(|(
        stamp_us: u64,
        accel_x in (-20.0f32)..20.0f32,
        accel_y in (-20.0f32)..20.0f32,
        accel_z in (-20.0f32)..20.0f32,
        gyro_x in (-35.0f32)..35.0f32,
        gyro_y in (-35.0f32)..35.0f32,
        gyro_z in (-35.0f32)..35.0f32,
    )| {
        let val = ImuReading { stamp_us, accel_x, accel_y, accel_z, gyro_x, gyro_y, gyro_z };
        let enc1 = encode_to_vec(&val).expect("first encode ImuReading failed");
        let (decoded, _): (ImuReading, usize) =
            decode_from_slice(&enc1).expect("decode ImuReading failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode ImuReading failed");
        prop_assert_eq!(enc1, enc2, "re-encoding must produce identical bytes");
    });
}

// ── 9. SlamMapCell roundtrip ────────────────────────────────────────────────

#[test]
fn test_slam_map_cell_roundtrip() {
    proptest!(|(cell in arb_slam_cell())| {
        let enc = encode_to_vec(&cell).expect("encode SlamMapCell failed");
        let (dec, consumed): (SlamMapCell, usize) =
            decode_from_slice(&enc).expect("decode SlamMapCell failed");
        prop_assert_eq!(&cell, &dec, "SlamMapCell roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 10. Vec<SlamMapCell> roundtrip (occupancy grid patch) ───────────────────

#[test]
fn test_vec_slam_map_cell_roundtrip() {
    proptest!(|(
        cells in prop::collection::vec(arb_slam_cell(), 0..20usize),
    )| {
        let enc = encode_to_vec(&cells).expect("encode Vec<SlamMapCell> failed");
        let (dec, consumed): (Vec<SlamMapCell>, usize) =
            decode_from_slice(&enc).expect("decode Vec<SlamMapCell> failed");
        prop_assert_eq!(&cells, &dec, "Vec<SlamMapCell> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 11. PathWaypoint roundtrip ──────────────────────────────────────────────

#[test]
fn test_path_waypoint_roundtrip() {
    proptest!(|(wp in arb_waypoint())| {
        let enc = encode_to_vec(&wp).expect("encode PathWaypoint failed");
        let (dec, consumed): (PathWaypoint, usize) =
            decode_from_slice(&enc).expect("decode PathWaypoint failed");
        prop_assert_eq!(&wp, &dec, "PathWaypoint roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 12. Vec<PathWaypoint> roundtrip (planned path) ──────────────────────────

#[test]
fn test_vec_path_waypoint_roundtrip() {
    proptest!(|(
        waypoints in prop::collection::vec(arb_waypoint(), 2..16usize),
    )| {
        let enc = encode_to_vec(&waypoints).expect("encode Vec<PathWaypoint> failed");
        let (dec, consumed): (Vec<PathWaypoint>, usize) =
            decode_from_slice(&enc).expect("decode Vec<PathWaypoint> failed");
        prop_assert_eq!(&waypoints, &dec, "Vec<PathWaypoint> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 13. ObstacleBbox roundtrip ──────────────────────────────────────────────

#[test]
fn test_obstacle_bbox_roundtrip() {
    proptest!(|(
        confidence in 0.0f32..1.0f32,
        cx_m in -30.0f32..30.0f32,
        cy_m in -30.0f32..30.0f32,
        cz_m in 0.0f32..5.0f32,
        hx_m in 0.05f32..5.0f32,
        hy_m in 0.05f32..5.0f32,
        hz_m in 0.05f32..3.0f32,
        label in "(person|vehicle|cone|wall|door|box)",
    )| {
        let val = ObstacleBbox { confidence, cx_m, cy_m, cz_m, hx_m, hy_m, hz_m, label };
        let enc = encode_to_vec(&val).expect("encode ObstacleBbox failed");
        let (dec, consumed): (ObstacleBbox, usize) =
            decode_from_slice(&enc).expect("decode ObstacleBbox failed");
        prop_assert_eq!(&val, &dec, "ObstacleBbox roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 14. MotorPidParams roundtrip ────────────────────────────────────────────

#[test]
fn test_motor_pid_params_roundtrip() {
    proptest!(|(
        motor_id in 0u8..16u8,
        kp in 0.0f32..100.0f32,
        ki in 0.0f32..50.0f32,
        kd in 0.0f32..20.0f32,
        output_limit in 0.1f32..1000.0f32,
    )| {
        let val = MotorPidParams { motor_id, kp, ki, kd, output_limit };
        let enc = encode_to_vec(&val).expect("encode MotorPidParams failed");
        let (dec, consumed): (MotorPidParams, usize) =
            decode_from_slice(&enc).expect("decode MotorPidParams failed");
        prop_assert_eq!(&val, &dec, "MotorPidParams roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 15. BatteryChargeCycle roundtrip ────────────────────────────────────────

#[test]
fn test_battery_charge_cycle_roundtrip() {
    proptest!(|(
        cycle_number in 0u32..50_000u32,
        soc_start in 0.0f32..1.0f32,
        soc_end in 0.0f32..1.0f32,
        duration_s in 60u32..36_000u32,
        peak_current_a in 0.0f32..200.0f32,
        avg_temp_c in (-10.0f32)..60.0f32,
    )| {
        let val = BatteryChargeCycle {
            cycle_number, soc_start, soc_end, duration_s, peak_current_a, avg_temp_c,
        };
        let enc = encode_to_vec(&val).expect("encode BatteryChargeCycle failed");
        let (dec, consumed): (BatteryChargeCycle, usize) =
            decode_from_slice(&enc).expect("decode BatteryChargeCycle failed");
        prop_assert_eq!(&val, &dec, "BatteryChargeCycle roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 16. CollisionDistance roundtrip ─────────────────────────────────────────

#[test]
fn test_collision_distance_roundtrip() {
    proptest!(|(dist in arb_collision_distance())| {
        let enc = encode_to_vec(&dist).expect("encode CollisionDistance failed");
        let (dec, consumed): (CollisionDistance, usize) =
            decode_from_slice(&enc).expect("decode CollisionDistance failed");
        prop_assert_eq!(&dist, &dec, "CollisionDistance roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 17. Vec<CollisionDistance> roundtrip (proximity ring) ───────────────────

#[test]
fn test_vec_collision_distance_roundtrip() {
    proptest!(|(
        sensors in prop::collection::vec(arb_collision_distance(), 1..12usize),
    )| {
        let enc = encode_to_vec(&sensors).expect("encode Vec<CollisionDistance> failed");
        let (dec, consumed): (Vec<CollisionDistance>, usize) =
            decode_from_slice(&enc).expect("decode Vec<CollisionDistance> failed");
        prop_assert_eq!(&sensors, &dec, "Vec<CollisionDistance> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 18. TerrainClass roundtrip ──────────────────────────────────────────────

#[test]
fn test_terrain_class_roundtrip() {
    proptest!(|(
        variant in 0u8..5u8,
        grain_size_mm in 0.5f32..50.0f32,
        moisture_fraction in 0.0f32..1.0f32,
        slope_deg in 1.0f32..45.0f32,
    )| {
        let val = match variant {
            0 => TerrainClass::IndoorFlat,
            1 => TerrainClass::OutdoorRough,
            2 => TerrainClass::Gravel { grain_size_mm },
            3 => TerrainClass::Grass { moisture_fraction },
            _ => TerrainClass::Incline { slope_deg },
        };
        let enc = encode_to_vec(&val).expect("encode TerrainClass failed");
        let (dec, consumed): (TerrainClass, usize) =
            decode_from_slice(&enc).expect("decode TerrainClass failed");
        prop_assert_eq!(&val, &dec, "TerrainClass roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 19. TaskStatus roundtrip ────────────────────────────────────────────────

#[test]
fn test_task_status_roundtrip() {
    proptest!(|(
        variant in 0u8..5u8,
        progress in 0.0f32..1.0f32,
        elapsed_ms: u64,
        error_code: u16,
        message in "[a-z]{3,20}",
    )| {
        let val = match variant {
            0 => TaskStatus::Pending,
            1 => TaskStatus::InProgress { progress },
            2 => TaskStatus::Completed { elapsed_ms },
            3 => TaskStatus::Failed { error_code, message },
            _ => TaskStatus::Cancelled,
        };
        let enc = encode_to_vec(&val).expect("encode TaskStatus failed");
        let (dec, consumed): (TaskStatus, usize) =
            decode_from_slice(&enc).expect("decode TaskStatus failed");
        prop_assert_eq!(&val, &dec, "TaskStatus roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 20. ManipulatorPose roundtrip ───────────────────────────────────────────

#[test]
fn test_manipulator_pose_roundtrip() {
    proptest!(|(
        ee_x_m in -2.0f32..2.0f32,
        ee_y_m in -2.0f32..2.0f32,
        ee_z_m in 0.0f32..2.5f32,
        qw in (-1.0f32)..1.0f32,
        qx in (-1.0f32)..1.0f32,
        qy in (-1.0f32)..1.0f32,
        qz in (-1.0f32)..1.0f32,
        joint_angles in prop::collection::vec((-3.15f32)..3.15f32, 3..8usize),
    )| {
        let val = ManipulatorPose { ee_x_m, ee_y_m, ee_z_m, qw, qx, qy, qz, joint_angles };
        let enc = encode_to_vec(&val).expect("encode ManipulatorPose failed");
        let (dec, consumed): (ManipulatorPose, usize) =
            decode_from_slice(&enc).expect("decode ManipulatorPose failed");
        prop_assert_eq!(&val, &dec, "ManipulatorPose roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 21. ManipulatorPose re-encode determinism ───────────────────────────────

#[test]
fn test_manipulator_pose_determinism() {
    proptest!(|(
        ee_x_m in -2.0f32..2.0f32,
        ee_y_m in -2.0f32..2.0f32,
        ee_z_m in 0.0f32..2.5f32,
        qw in (-1.0f32)..1.0f32,
        qx in (-1.0f32)..1.0f32,
        qy in (-1.0f32)..1.0f32,
        qz in (-1.0f32)..1.0f32,
        joint_angles in prop::collection::vec((-3.15f32)..3.15f32, 3..8usize),
    )| {
        let val = ManipulatorPose { ee_x_m, ee_y_m, ee_z_m, qw, qx, qy, qz, joint_angles };
        let enc1 = encode_to_vec(&val).expect("first encode ManipulatorPose failed");
        let enc2 = encode_to_vec(&val).expect("second encode ManipulatorPose failed");
        prop_assert_eq!(enc1, enc2, "encoding must be deterministic");
    });
}

// ── 22. Mixed robotics snapshot — consumed bytes == bytes.len() ─────────────

#[test]
fn test_mixed_robotics_snapshot_consumed_bytes() {
    proptest!(|(
        stamp_us: u64,
        accel_x in (-20.0f32)..20.0f32,
        accel_y in (-20.0f32)..20.0f32,
        accel_z in (-20.0f32)..20.0f32,
        gyro_x in (-35.0f32)..35.0f32,
        motor_id in 0u8..16u8,
        kp in 0.0f32..100.0f32,
        cycle_number in 0u32..50_000u32,
        soc_start in 0.0f32..1.0f32,
    )| {
        let imu = ImuReading {
            stamp_us, accel_x, accel_y, accel_z,
            gyro_x, gyro_y: 0.0, gyro_z: 0.0,
        };
        let pid = MotorPidParams {
            motor_id, kp, ki: 0.1, kd: 0.01, output_limit: 255.0,
        };
        let battery = BatteryChargeCycle {
            cycle_number, soc_start, soc_end: 1.0,
            duration_s: 3600, peak_current_a: 10.0, avg_temp_c: 25.0,
        };
        let status = TaskStatus::InProgress { progress: 0.5 };

        let enc_imu = encode_to_vec(&imu).expect("encode imu snapshot failed");
        let enc_pid = encode_to_vec(&pid).expect("encode pid snapshot failed");
        let enc_bat = encode_to_vec(&battery).expect("encode battery snapshot failed");
        let enc_status = encode_to_vec(&status).expect("encode status snapshot failed");

        let (_, c_imu): (ImuReading, usize) =
            decode_from_slice(&enc_imu).expect("decode imu snapshot failed");
        let (_, c_pid): (MotorPidParams, usize) =
            decode_from_slice(&enc_pid).expect("decode pid snapshot failed");
        let (_, c_bat): (BatteryChargeCycle, usize) =
            decode_from_slice(&enc_bat).expect("decode battery snapshot failed");
        let (_, c_status): (TaskStatus, usize) =
            decode_from_slice(&enc_status).expect("decode status snapshot failed");

        prop_assert_eq!(c_imu, enc_imu.len(), "imu consumed bytes mismatch");
        prop_assert_eq!(c_pid, enc_pid.len(), "pid consumed bytes mismatch");
        prop_assert_eq!(c_bat, enc_bat.len(), "battery consumed bytes mismatch");
        prop_assert_eq!(c_status, enc_status.len(), "status consumed bytes mismatch");
    });
}
