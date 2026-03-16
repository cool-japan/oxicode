#![cfg(feature = "versioning")]
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
use oxicode::versioning::Version;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

// ── Domain types: Autonomous Vehicle Safety Systems ─────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AsilLevel {
    QmNonSafety,
    AsilA,
    AsilB,
    AsilC,
    AsilD,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SensorType {
    LidarFront,
    LidarRear,
    LidarLeftFlank,
    LidarRightFlank,
    CameraFrontWide,
    CameraNarrowTelephoto,
    CameraRearView,
    RadarLongRange,
    RadarShortRange,
    Ultrasonic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LaneMarkingType {
    SolidWhite,
    DashedWhite,
    SolidYellow,
    DashedYellow,
    DoubleSolid,
    BotsDots,
    NoneDetected,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TrafficSignClass {
    SpeedLimit,
    StopSign,
    YieldSign,
    NoEntry,
    PedestrianCrossing,
    SchoolZone,
    ConstructionZone,
    RailwayCrossing,
    RoundaboutAhead,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EmergencyBrakeReason {
    PedestrianInPath,
    VehicleCutIn,
    StaticObstacle,
    SensorDegradation,
    OddBoundaryViolation,
    SystemFault,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum V2xMessageType {
    BasicSafetyMessage,
    SignalPhaseAndTiming,
    MapData,
    TravelerInformation,
    EmergencyVehicleAlert,
    PersonalSafetyMessage,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OddCondition {
    WithinBounds,
    WeatherExceeded,
    SpeedExceeded,
    GeofenceViolation,
    SensorDegraded,
    MapDataStale,
    ConnectivityLost,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DtcSeverity {
    Information,
    Warning,
    Fault,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LidarPointCloud {
    sensor_id: u32,
    sensor_type: SensorType,
    frame_sequence: u64,
    timestamp_ns: u64,
    num_points: u32,
    min_range_mm: u32,
    max_range_mm: u32,
    angular_resolution_mrad: u16,
    intensity_mean: u16,
    reflectivity_threshold: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CameraFrameMeta {
    camera_id: u32,
    sensor_type: SensorType,
    frame_number: u64,
    exposure_us: u32,
    gain_db_x10: u16,
    resolution_width: u16,
    resolution_height: u16,
    timestamp_ns: u64,
    distortion_corrected: bool,
    hdr_enabled: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadarReturn {
    radar_id: u32,
    sensor_type: SensorType,
    track_id: u32,
    range_cm: u32,
    range_rate_cm_per_s: i32,
    azimuth_mrad: i16,
    elevation_mrad: i16,
    rcs_dbsm_x10: i16,
    timestamp_ns: u64,
    snr_db_x10: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VehicleDynamics {
    timestamp_ns: u64,
    speed_mm_per_s: u32,
    lateral_speed_mm_per_s: i32,
    yaw_rate_mrad_per_s: i32,
    steering_angle_mrad: i32,
    brake_pressure_kpa: u16,
    throttle_percent_x10: u16,
    longitudinal_accel_mg: i16,
    lateral_accel_mg: i16,
    gear_position: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SafetyEnvelopeViolation {
    violation_id: u64,
    timestamp_ns: u64,
    safety_metric: String,
    threshold_value: u32,
    actual_value: u32,
    duration_ms: u32,
    asil_level: AsilLevel,
    mitigation_action: String,
    resolved: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmergencyBrakeEvent {
    event_id: u64,
    timestamp_ns: u64,
    reason: EmergencyBrakeReason,
    ttc_ms: u32,
    initial_speed_mm_per_s: u32,
    decel_mg: i16,
    stopping_distance_cm: u32,
    object_distance_cm: u32,
    asil_level: AsilLevel,
    driver_override: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LaneDetection {
    detection_id: u64,
    timestamp_ns: u64,
    left_marking: LaneMarkingType,
    right_marking: LaneMarkingType,
    lane_width_mm: u32,
    lateral_offset_mm: i32,
    heading_error_mrad: i16,
    curvature_inv_m_x1e6: i32,
    confidence_percent: u8,
    num_lanes_detected: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrafficSignRecognition {
    detection_id: u64,
    timestamp_ns: u64,
    sign_class: TrafficSignClass,
    sign_value: u16,
    confidence_percent: u8,
    bounding_box_x: u16,
    bounding_box_y: u16,
    bounding_box_w: u16,
    bounding_box_h: u16,
    distance_cm: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PedestrianDetection {
    detection_id: u64,
    timestamp_ns: u64,
    confidence_percent: u8,
    position_x_cm: i32,
    position_y_cm: i32,
    velocity_x_cm_per_s: i32,
    velocity_y_cm_per_s: i32,
    bounding_box_area: u32,
    is_child: bool,
    crossing_intent: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct V2xMessage {
    message_id: u64,
    timestamp_ns: u64,
    message_type: V2xMessageType,
    sender_id: u32,
    rssi_dbm: i8,
    payload_length: u16,
    latitude_x1e7: i32,
    longitude_x1e7: i32,
    heading_mrad: u16,
    speed_mm_per_s: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OddBoundary {
    boundary_id: u32,
    condition: OddCondition,
    max_speed_mm_per_s: u32,
    min_visibility_m: u16,
    max_precipitation_mm_per_h: u16,
    geofence_polygon_hash: u64,
    map_version_required: u32,
    connectivity_timeout_ms: u32,
    last_evaluated_ns: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiagnosticTroubleCode {
    dtc_code: u32,
    severity: DtcSeverity,
    subsystem: String,
    description: String,
    occurrence_count: u32,
    first_seen_ns: u64,
    last_seen_ns: u64,
    frozen_frame_available: bool,
    mil_active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SensorFusionSnapshot {
    snapshot_id: u64,
    timestamp_ns: u64,
    num_lidar_points: u32,
    num_radar_tracks: u16,
    num_camera_objects: u16,
    num_fused_objects: u16,
    processing_latency_us: u32,
    fusion_confidence_percent: u8,
    asil_level: AsilLevel,
    odd_status: OddCondition,
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn test_lidar_point_cloud_roundtrip() {
    let cloud = LidarPointCloud {
        sensor_id: 1,
        sensor_type: SensorType::LidarFront,
        frame_sequence: 1_000_001,
        timestamp_ns: 1_710_000_000_000_000_000,
        num_points: 128_000,
        min_range_mm: 500,
        max_range_mm: 200_000,
        angular_resolution_mrad: 2,
        intensity_mean: 4500,
        reflectivity_threshold: 10,
    };
    let bytes = encode_to_vec(&cloud).expect("encode LidarPointCloud failed");
    let (decoded, consumed) =
        decode_from_slice::<LidarPointCloud>(&bytes).expect("decode LidarPointCloud failed");
    assert_eq!(cloud, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_lidar_point_cloud_versioned_v1_0_0() {
    let cloud = LidarPointCloud {
        sensor_id: 2,
        sensor_type: SensorType::LidarRear,
        frame_sequence: 2_500_000,
        timestamp_ns: 1_710_000_050_000_000_000,
        num_points: 64_000,
        min_range_mm: 300,
        max_range_mm: 150_000,
        angular_resolution_mrad: 4,
        intensity_mean: 3200,
        reflectivity_threshold: 15,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&cloud, version)
        .expect("encode versioned LidarPointCloud v1.0.0 failed");
    let (decoded, ver, _consumed): (LidarPointCloud, Version, usize) =
        decode_versioned_value::<LidarPointCloud>(&bytes)
            .expect("decode versioned LidarPointCloud v1.0.0 failed");
    assert_eq!(cloud, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_camera_frame_meta_roundtrip() {
    let frame = CameraFrameMeta {
        camera_id: 10,
        sensor_type: SensorType::CameraFrontWide,
        frame_number: 5_400_000,
        exposure_us: 8333,
        gain_db_x10: 60,
        resolution_width: 1920,
        resolution_height: 1200,
        timestamp_ns: 1_710_000_100_000_000_000,
        distortion_corrected: true,
        hdr_enabled: false,
    };
    let bytes = encode_to_vec(&frame).expect("encode CameraFrameMeta failed");
    let (decoded, consumed) =
        decode_from_slice::<CameraFrameMeta>(&bytes).expect("decode CameraFrameMeta failed");
    assert_eq!(frame, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_camera_frame_meta_versioned_v2_1_0() {
    let frame = CameraFrameMeta {
        camera_id: 11,
        sensor_type: SensorType::CameraNarrowTelephoto,
        frame_number: 9_999_999,
        exposure_us: 5000,
        gain_db_x10: 120,
        resolution_width: 3840,
        resolution_height: 2160,
        timestamp_ns: 1_710_001_000_000_000_000,
        distortion_corrected: true,
        hdr_enabled: true,
    };
    let version = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&frame, version)
        .expect("encode versioned CameraFrameMeta v2.1.0 failed");
    let (decoded, ver, consumed): (CameraFrameMeta, Version, usize) =
        decode_versioned_value::<CameraFrameMeta>(&bytes)
            .expect("decode versioned CameraFrameMeta v2.1.0 failed");
    assert_eq!(frame, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_radar_return_roundtrip() {
    let ret = RadarReturn {
        radar_id: 20,
        sensor_type: SensorType::RadarLongRange,
        track_id: 42,
        range_cm: 15_000,
        range_rate_cm_per_s: -1_200,
        azimuth_mrad: 50,
        elevation_mrad: -10,
        rcs_dbsm_x10: 150,
        timestamp_ns: 1_710_000_200_000_000_000,
        snr_db_x10: 250,
    };
    let bytes = encode_to_vec(&ret).expect("encode RadarReturn failed");
    let (decoded, consumed) =
        decode_from_slice::<RadarReturn>(&bytes).expect("decode RadarReturn failed");
    assert_eq!(ret, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_radar_return_versioned_v3_0_0() {
    let ret = RadarReturn {
        radar_id: 21,
        sensor_type: SensorType::RadarShortRange,
        track_id: 99,
        range_cm: 500,
        range_rate_cm_per_s: 0,
        azimuth_mrad: -300,
        elevation_mrad: 20,
        rcs_dbsm_x10: -50,
        timestamp_ns: 1_710_000_250_000_000_000,
        snr_db_x10: 180,
    };
    let version = Version::new(3, 0, 0);
    let bytes =
        encode_versioned_value(&ret, version).expect("encode versioned RadarReturn v3.0.0 failed");
    let (decoded, ver, _consumed): (RadarReturn, Version, usize) =
        decode_versioned_value::<RadarReturn>(&bytes)
            .expect("decode versioned RadarReturn v3.0.0 failed");
    assert_eq!(ret, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_vehicle_dynamics_braking_scenario() {
    let dynamics = VehicleDynamics {
        timestamp_ns: 1_710_000_300_000_000_000,
        speed_mm_per_s: 22_222,
        lateral_speed_mm_per_s: -150,
        yaw_rate_mrad_per_s: 30,
        steering_angle_mrad: -500,
        brake_pressure_kpa: 8500,
        throttle_percent_x10: 0,
        longitudinal_accel_mg: -8000,
        lateral_accel_mg: -200,
        gear_position: 3,
    };
    let version = Version::new(1, 2, 0);
    let bytes = encode_versioned_value(&dynamics, version)
        .expect("encode versioned VehicleDynamics braking failed");
    let (decoded, ver, consumed): (VehicleDynamics, Version, usize) =
        decode_versioned_value::<VehicleDynamics>(&bytes)
            .expect("decode versioned VehicleDynamics braking failed");
    assert_eq!(dynamics, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 2);
    assert!(consumed > 0);
}

#[test]
fn test_safety_envelope_violation_asil_d() {
    let violation = SafetyEnvelopeViolation {
        violation_id: 7001,
        timestamp_ns: 1_710_000_400_000_000_000,
        safety_metric: "minimum_following_distance".to_string(),
        threshold_value: 3000,
        actual_value: 1800,
        duration_ms: 450,
        asil_level: AsilLevel::AsilD,
        mitigation_action: "adaptive_cruise_decel".to_string(),
        resolved: true,
    };
    let version = Version::new(4, 0, 1);
    let bytes = encode_versioned_value(&violation, version)
        .expect("encode versioned SafetyEnvelopeViolation ASIL-D failed");
    let (decoded, ver, _consumed): (SafetyEnvelopeViolation, Version, usize) =
        decode_versioned_value::<SafetyEnvelopeViolation>(&bytes)
            .expect("decode versioned SafetyEnvelopeViolation ASIL-D failed");
    assert_eq!(violation, decoded);
    assert_eq!(ver.major, 4);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 1);
}

#[test]
fn test_emergency_brake_pedestrian_in_path() {
    let event = EmergencyBrakeEvent {
        event_id: 8001,
        timestamp_ns: 1_710_000_500_000_000_000,
        reason: EmergencyBrakeReason::PedestrianInPath,
        ttc_ms: 1200,
        initial_speed_mm_per_s: 13_889,
        decel_mg: -9800,
        stopping_distance_cm: 1_450,
        object_distance_cm: 2_000,
        asil_level: AsilLevel::AsilD,
        driver_override: false,
    };
    let bytes = encode_to_vec(&event).expect("encode EmergencyBrakeEvent failed");
    let (decoded, consumed) = decode_from_slice::<EmergencyBrakeEvent>(&bytes)
        .expect("decode EmergencyBrakeEvent failed");
    assert_eq!(event, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_emergency_brake_versioned_sensor_degradation() {
    let event = EmergencyBrakeEvent {
        event_id: 8002,
        timestamp_ns: 1_710_000_550_000_000_000,
        reason: EmergencyBrakeReason::SensorDegradation,
        ttc_ms: 0,
        initial_speed_mm_per_s: 30_556,
        decel_mg: -5000,
        stopping_distance_cm: 5_800,
        object_distance_cm: 0,
        asil_level: AsilLevel::AsilC,
        driver_override: true,
    };
    let version = Version::new(2, 5, 0);
    let bytes = encode_versioned_value(&event, version)
        .expect("encode versioned EmergencyBrakeEvent SensorDeg failed");
    let (decoded, ver, consumed): (EmergencyBrakeEvent, Version, usize) =
        decode_versioned_value::<EmergencyBrakeEvent>(&bytes)
            .expect("decode versioned EmergencyBrakeEvent SensorDeg failed");
    assert_eq!(event, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 5);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_lane_detection_highway() {
    let detection = LaneDetection {
        detection_id: 9001,
        timestamp_ns: 1_710_000_600_000_000_000,
        left_marking: LaneMarkingType::DashedWhite,
        right_marking: LaneMarkingType::SolidWhite,
        lane_width_mm: 3_650,
        lateral_offset_mm: -120,
        heading_error_mrad: 5,
        curvature_inv_m_x1e6: 200,
        confidence_percent: 95,
        num_lanes_detected: 4,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&detection, version)
        .expect("encode versioned LaneDetection highway failed");
    let (decoded, ver, _consumed): (LaneDetection, Version, usize) =
        decode_versioned_value::<LaneDetection>(&bytes)
            .expect("decode versioned LaneDetection highway failed");
    assert_eq!(detection, decoded);
    assert_eq!(ver.major, 1);
}

#[test]
fn test_traffic_sign_recognition_speed_limit() {
    let sign = TrafficSignRecognition {
        detection_id: 10_001,
        timestamp_ns: 1_710_000_700_000_000_000,
        sign_class: TrafficSignClass::SpeedLimit,
        sign_value: 65,
        confidence_percent: 98,
        bounding_box_x: 820,
        bounding_box_y: 310,
        bounding_box_w: 45,
        bounding_box_h: 55,
        distance_cm: 8_500,
    };
    let bytes = encode_to_vec(&sign).expect("encode TrafficSignRecognition failed");
    let (decoded, consumed) = decode_from_slice::<TrafficSignRecognition>(&bytes)
        .expect("decode TrafficSignRecognition failed");
    assert_eq!(sign, decoded);
    assert_eq!(consumed, bytes.len());

    // Also do versioned roundtrip
    let version = Version::new(3, 2, 1);
    let vbytes = encode_versioned_value(&sign, version)
        .expect("encode versioned TrafficSignRecognition failed");
    let (vdecoded, ver, _consumed): (TrafficSignRecognition, Version, usize) =
        decode_versioned_value::<TrafficSignRecognition>(&vbytes)
            .expect("decode versioned TrafficSignRecognition failed");
    assert_eq!(sign, vdecoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 2);
    assert_eq!(ver.patch, 1);
}

#[test]
fn test_pedestrian_detection_child_crossing() {
    let ped = PedestrianDetection {
        detection_id: 11_001,
        timestamp_ns: 1_710_000_800_000_000_000,
        confidence_percent: 92,
        position_x_cm: 1_200,
        position_y_cm: -350,
        velocity_x_cm_per_s: -80,
        velocity_y_cm_per_s: 150,
        bounding_box_area: 12_000,
        is_child: true,
        crossing_intent: true,
    };
    let version = Version::new(5, 0, 0);
    let bytes = encode_versioned_value(&ped, version)
        .expect("encode versioned PedestrianDetection child failed");
    let (decoded, ver, consumed): (PedestrianDetection, Version, usize) =
        decode_versioned_value::<PedestrianDetection>(&bytes)
            .expect("decode versioned PedestrianDetection child failed");
    assert_eq!(ped, decoded);
    assert_eq!(ver.major, 5);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_v2x_emergency_vehicle_alert() {
    let msg = V2xMessage {
        message_id: 12_001,
        timestamp_ns: 1_710_000_900_000_000_000,
        message_type: V2xMessageType::EmergencyVehicleAlert,
        sender_id: 0x00FF_ABCD,
        rssi_dbm: -65,
        payload_length: 256,
        latitude_x1e7: 374_219_876,
        longitude_x1e7: -1_220_847_123,
        heading_mrad: 1_571,
        speed_mm_per_s: 27_778,
    };
    let version = Version::new(1, 1, 0);
    let bytes =
        encode_versioned_value(&msg, version).expect("encode versioned V2xMessage EVA failed");
    let (decoded, ver, _consumed): (V2xMessage, Version, usize) =
        decode_versioned_value::<V2xMessage>(&bytes)
            .expect("decode versioned V2xMessage EVA failed");
    assert_eq!(msg, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_odd_boundary_geofence_violation() {
    let odd = OddBoundary {
        boundary_id: 500,
        condition: OddCondition::GeofenceViolation,
        max_speed_mm_per_s: 16_667,
        min_visibility_m: 100,
        max_precipitation_mm_per_h: 25,
        geofence_polygon_hash: 0xDEAD_BEEF_CAFE_BABE,
        map_version_required: 20240315,
        connectivity_timeout_ms: 5000,
        last_evaluated_ns: 1_710_001_000_000_000_000,
    };
    let bytes = encode_to_vec(&odd).expect("encode OddBoundary failed");
    let (decoded, consumed) =
        decode_from_slice::<OddBoundary>(&bytes).expect("decode OddBoundary failed");
    assert_eq!(odd, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_diagnostic_trouble_code_critical() {
    let dtc = DiagnosticTroubleCode {
        dtc_code: 0x00C1_0045,
        severity: DtcSeverity::Critical,
        subsystem: "AEB_PRIMARY".to_string(),
        description: "Autonomous Emergency Braking primary actuator communication lost".to_string(),
        occurrence_count: 1,
        first_seen_ns: 1_710_001_100_000_000_000,
        last_seen_ns: 1_710_001_100_000_000_000,
        frozen_frame_available: true,
        mil_active: true,
    };
    let version = Version::new(2, 0, 3);
    let bytes = encode_versioned_value(&dtc, version)
        .expect("encode versioned DiagnosticTroubleCode critical failed");
    let (decoded, ver, consumed): (DiagnosticTroubleCode, Version, usize) =
        decode_versioned_value::<DiagnosticTroubleCode>(&bytes)
            .expect("decode versioned DiagnosticTroubleCode critical failed");
    assert_eq!(dtc, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 3);
    assert!(consumed > 0);
}

#[test]
fn test_sensor_fusion_snapshot_full_health() {
    let snapshot = SensorFusionSnapshot {
        snapshot_id: 99_001,
        timestamp_ns: 1_710_001_200_000_000_000,
        num_lidar_points: 256_000,
        num_radar_tracks: 48,
        num_camera_objects: 32,
        num_fused_objects: 55,
        processing_latency_us: 12_500,
        fusion_confidence_percent: 97,
        asil_level: AsilLevel::AsilD,
        odd_status: OddCondition::WithinBounds,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&snapshot, version)
        .expect("encode versioned SensorFusionSnapshot failed");
    let (decoded, ver, _consumed): (SensorFusionSnapshot, Version, usize) =
        decode_versioned_value::<SensorFusionSnapshot>(&bytes)
            .expect("decode versioned SensorFusionSnapshot failed");
    assert_eq!(snapshot, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_version_upgrade_lidar_v1_to_v2() {
    let cloud = LidarPointCloud {
        sensor_id: 3,
        sensor_type: SensorType::LidarLeftFlank,
        frame_sequence: 100_000,
        timestamp_ns: 1_710_002_000_000_000_000,
        num_points: 32_000,
        min_range_mm: 200,
        max_range_mm: 100_000,
        angular_resolution_mrad: 8,
        intensity_mean: 2100,
        reflectivity_threshold: 5,
    };

    let v1 = Version::new(1, 0, 0);
    let v1_bytes =
        encode_versioned_value(&cloud, v1).expect("encode versioned LidarPointCloud v1 failed");
    let (decoded_v1, ver_v1, _): (LidarPointCloud, Version, usize) =
        decode_versioned_value::<LidarPointCloud>(&v1_bytes)
            .expect("decode versioned LidarPointCloud v1 failed");
    assert_eq!(cloud, decoded_v1);
    assert_eq!(ver_v1.major, 1);

    let v2 = Version::new(2, 0, 0);
    let v2_bytes =
        encode_versioned_value(&cloud, v2).expect("encode versioned LidarPointCloud v2 failed");
    let (decoded_v2, ver_v2, _): (LidarPointCloud, Version, usize) =
        decode_versioned_value::<LidarPointCloud>(&v2_bytes)
            .expect("decode versioned LidarPointCloud v2 failed");
    assert_eq!(cloud, decoded_v2);
    assert_eq!(ver_v2.major, 2);

    assert_eq!(decoded_v1, decoded_v2);
}

#[test]
fn test_mixed_version_sensor_collection() {
    let lidar = LidarPointCloud {
        sensor_id: 100,
        sensor_type: SensorType::LidarFront,
        frame_sequence: 50_000,
        timestamp_ns: 1_710_003_000_000_000_000,
        num_points: 128_000,
        min_range_mm: 500,
        max_range_mm: 200_000,
        angular_resolution_mrad: 2,
        intensity_mean: 4000,
        reflectivity_threshold: 12,
    };
    let radar = RadarReturn {
        radar_id: 200,
        sensor_type: SensorType::RadarLongRange,
        track_id: 7,
        range_cm: 25_000,
        range_rate_cm_per_s: -500,
        azimuth_mrad: 100,
        elevation_mrad: 0,
        rcs_dbsm_x10: 200,
        timestamp_ns: 1_710_003_000_000_000_000,
        snr_db_x10: 300,
    };
    let camera = CameraFrameMeta {
        camera_id: 300,
        sensor_type: SensorType::CameraFrontWide,
        frame_number: 1_000_000,
        exposure_us: 10_000,
        gain_db_x10: 30,
        resolution_width: 1920,
        resolution_height: 1200,
        timestamp_ns: 1_710_003_000_000_000_000,
        distortion_corrected: true,
        hdr_enabled: false,
    };

    let lidar_bytes = encode_versioned_value(&lidar, Version::new(1, 0, 0))
        .expect("encode versioned lidar in collection failed");
    let radar_bytes = encode_versioned_value(&radar, Version::new(2, 3, 0))
        .expect("encode versioned radar in collection failed");
    let camera_bytes = encode_versioned_value(&camera, Version::new(1, 5, 2))
        .expect("encode versioned camera in collection failed");

    let (dec_lidar, ver_lidar, _): (LidarPointCloud, Version, usize) =
        decode_versioned_value::<LidarPointCloud>(&lidar_bytes)
            .expect("decode versioned lidar in collection failed");
    let (dec_radar, ver_radar, _): (RadarReturn, Version, usize) =
        decode_versioned_value::<RadarReturn>(&radar_bytes)
            .expect("decode versioned radar in collection failed");
    let (dec_camera, ver_camera, _): (CameraFrameMeta, Version, usize) =
        decode_versioned_value::<CameraFrameMeta>(&camera_bytes)
            .expect("decode versioned camera in collection failed");

    assert_eq!(lidar, dec_lidar);
    assert_eq!(radar, dec_radar);
    assert_eq!(camera, dec_camera);

    assert_eq!(ver_lidar.major, 1);
    assert_eq!(ver_radar.major, 2);
    assert_eq!(ver_radar.minor, 3);
    assert_eq!(ver_camera.major, 1);
    assert_eq!(ver_camera.minor, 5);
    assert_eq!(ver_camera.patch, 2);
}

#[test]
fn test_version_compatibility_asil_levels() {
    let violation_a = SafetyEnvelopeViolation {
        violation_id: 20_001,
        timestamp_ns: 1_710_004_000_000_000_000,
        safety_metric: "lateral_jerk_limit".to_string(),
        threshold_value: 500,
        actual_value: 620,
        duration_ms: 80,
        asil_level: AsilLevel::AsilA,
        mitigation_action: "reduce_lateral_acceleration".to_string(),
        resolved: true,
    };
    let violation_d = SafetyEnvelopeViolation {
        violation_id: 20_002,
        timestamp_ns: 1_710_004_001_000_000_000,
        safety_metric: "time_to_collision".to_string(),
        threshold_value: 2000,
        actual_value: 900,
        duration_ms: 200,
        asil_level: AsilLevel::AsilD,
        mitigation_action: "emergency_braking_full".to_string(),
        resolved: false,
    };

    let v_old = Version::new(1, 0, 0);
    let v_new = Version::new(5, 3, 7);

    let bytes_a = encode_versioned_value(&violation_a, v_old)
        .expect("encode versioned ASIL-A old version failed");
    let bytes_d = encode_versioned_value(&violation_d, v_new)
        .expect("encode versioned ASIL-D new version failed");

    let (dec_a, ver_a, _): (SafetyEnvelopeViolation, Version, usize) =
        decode_versioned_value::<SafetyEnvelopeViolation>(&bytes_a)
            .expect("decode versioned ASIL-A old version failed");
    let (dec_d, ver_d, _): (SafetyEnvelopeViolation, Version, usize) =
        decode_versioned_value::<SafetyEnvelopeViolation>(&bytes_d)
            .expect("decode versioned ASIL-D new version failed");

    assert_eq!(violation_a, dec_a);
    assert_eq!(violation_d, dec_d);
    assert_eq!(ver_a.major, 1);
    assert_eq!(ver_d.major, 5);
    assert_eq!(ver_d.minor, 3);
    assert_eq!(ver_d.patch, 7);
}

#[test]
fn test_v2x_bsm_and_spat_roundtrips() {
    let bsm = V2xMessage {
        message_id: 30_001,
        timestamp_ns: 1_710_005_000_000_000_000,
        message_type: V2xMessageType::BasicSafetyMessage,
        sender_id: 0x0001_2345,
        rssi_dbm: -72,
        payload_length: 128,
        latitude_x1e7: 408_523_100,
        longitude_x1e7: -739_785_600,
        heading_mrad: 785,
        speed_mm_per_s: 15_000,
    };
    let spat = V2xMessage {
        message_id: 30_002,
        timestamp_ns: 1_710_005_000_100_000_000,
        message_type: V2xMessageType::SignalPhaseAndTiming,
        sender_id: 0x0000_FF01,
        rssi_dbm: -58,
        payload_length: 512,
        latitude_x1e7: 408_523_200,
        longitude_x1e7: -739_785_500,
        heading_mrad: 0,
        speed_mm_per_s: 0,
    };

    let bsm_bytes =
        encode_versioned_value(&bsm, Version::new(3, 0, 0)).expect("encode versioned BSM failed");
    let spat_bytes =
        encode_versioned_value(&spat, Version::new(3, 0, 0)).expect("encode versioned SPAT failed");

    let (dec_bsm, bsm_ver, _): (V2xMessage, Version, usize) =
        decode_versioned_value::<V2xMessage>(&bsm_bytes).expect("decode versioned BSM failed");
    let (dec_spat, spat_ver, _): (V2xMessage, Version, usize) =
        decode_versioned_value::<V2xMessage>(&spat_bytes).expect("decode versioned SPAT failed");

    assert_eq!(bsm, dec_bsm);
    assert_eq!(spat, dec_spat);
    assert_eq!(bsm_ver.major, spat_ver.major);
    assert_eq!(bsm_ver.major, 3);
}

#[test]
fn test_odd_boundary_versioned_weather_exceeded() {
    let odd = OddBoundary {
        boundary_id: 601,
        condition: OddCondition::WeatherExceeded,
        max_speed_mm_per_s: 8_333,
        min_visibility_m: 50,
        max_precipitation_mm_per_h: 80,
        geofence_polygon_hash: 0x1234_5678_9ABC_DEF0,
        map_version_required: 20260101,
        connectivity_timeout_ms: 3000,
        last_evaluated_ns: 1_710_006_000_000_000_000,
    };
    let version = Version::new(2, 4, 0);
    let bytes = encode_versioned_value(&odd, version)
        .expect("encode versioned OddBoundary WeatherExceeded failed");
    let (decoded, ver, consumed): (OddBoundary, Version, usize) =
        decode_versioned_value::<OddBoundary>(&bytes)
            .expect("decode versioned OddBoundary WeatherExceeded failed");
    assert_eq!(odd, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 4);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);

    // Also verify plain roundtrip
    let plain_bytes = encode_to_vec(&odd).expect("encode OddBoundary plain failed");
    let (plain_decoded, plain_consumed) =
        decode_from_slice::<OddBoundary>(&plain_bytes).expect("decode OddBoundary plain failed");
    assert_eq!(odd, plain_decoded);
    assert_eq!(plain_consumed, plain_bytes.len());
}
