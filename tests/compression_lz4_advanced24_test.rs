//! Advanced LZ4 compression tests for autonomous drone operations and UAV fleet management.
//!
//! Covers flight plan waypoints, geofence boundaries, MAVLink telemetry messages,
//! battery endurance calculations, obstacle avoidance maps, payload bay configs,
//! wind estimation, return-to-home triggers, airspace deconfliction zones,
//! ground control station commands, swarm coordination protocols, photogrammetry
//! survey grids, delivery confirmation receipts, maintenance logs, and regulatory
//! compliance (Part 107/BVLOS).

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
struct GpsCoordinate {
    latitude: f64,
    longitude: f64,
    altitude_m: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlightPlanWaypoint {
    sequence: u16,
    coordinate: GpsCoordinate,
    speed_m_s: f32,
    loiter_time_s: u16,
    heading_deg: f32,
    action: WaypointAction,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WaypointAction {
    FlyThrough,
    Hover { duration_s: u16 },
    TakePhoto,
    StartVideo,
    StopVideo,
    DropPayload,
    AdjustAltitude { target_m: f64 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlightPlan {
    plan_id: u64,
    drone_id: String,
    mission_name: String,
    waypoints: Vec<FlightPlanWaypoint>,
    total_distance_m: f64,
    estimated_flight_time_s: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeofenceBoundary {
    zone_id: u32,
    name: String,
    vertices: Vec<GpsCoordinate>,
    max_altitude_m: f64,
    min_altitude_m: f64,
    restriction: GeofenceRestriction,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GeofenceRestriction {
    NoFlyZone,
    AltitudeRestricted,
    SpeedRestricted {
        max_speed_m_s: f32,
    },
    TimeRestricted {
        allowed_start_utc: u64,
        allowed_end_utc: u64,
    },
    AuthorizationRequired {
        authority: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MavlinkTelemetry {
    system_id: u8,
    component_id: u8,
    message_id: u32,
    timestamp_us: u64,
    payload: MavlinkPayload,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MavlinkPayload {
    Heartbeat {
        autopilot_type: u8,
        system_status: u8,
        mavlink_version: u8,
    },
    GlobalPositionInt {
        lat_deg_e7: i32,
        lon_deg_e7: i32,
        alt_mm: i32,
        vx_cm_s: i16,
        vy_cm_s: i16,
        vz_cm_s: i16,
        hdg_cdeg: u16,
    },
    Attitude {
        roll_rad: f32,
        pitch_rad: f32,
        yaw_rad: f32,
        rollspeed: f32,
        pitchspeed: f32,
        yawspeed: f32,
    },
    BatteryStatus {
        voltage_mv: u16,
        current_ca: i16,
        remaining_pct: i8,
        temperature_cdeg: i16,
    },
    GpsRawInt {
        fix_type: u8,
        satellites_visible: u8,
        eph: u16,
        epv: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatteryEndurance {
    drone_id: String,
    cell_count: u8,
    nominal_voltage_v: f32,
    capacity_mah: u32,
    current_charge_pct: f32,
    estimated_remaining_s: u32,
    discharge_rate_c: f32,
    temperature_celsius: f32,
    cycle_count: u16,
    health_pct: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ObstacleCell {
    grid_x: u16,
    grid_y: u16,
    occupancy_probability: f32,
    elevation_m: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ObstacleAvoidanceMap {
    map_id: u32,
    origin: GpsCoordinate,
    resolution_m: f32,
    width_cells: u16,
    height_cells: u16,
    cells: Vec<ObstacleCell>,
    last_updated_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PayloadType {
    Camera {
        resolution_mp: f32,
        focal_length_mm: f32,
        stabilized: bool,
    },
    Lidar {
        points_per_second: u32,
        range_m: f32,
        channels: u8,
    },
    DeliveryBay {
        max_weight_kg: f32,
        dimensions_cm: (f32, f32, f32),
        release_mechanism: String,
    },
    Multispectral {
        bands: Vec<String>,
        spatial_resolution_m: f32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PayloadBayConfig {
    bay_id: u8,
    drone_id: String,
    payload: PayloadType,
    weight_kg: f32,
    powered: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WindEstimate {
    timestamp_epoch: u64,
    altitude_m: f64,
    speed_m_s: f32,
    direction_deg: f32,
    gust_speed_m_s: f32,
    confidence: f32,
    source: WindSource,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WindSource {
    Onboard,
    GroundStation,
    Metar { station_id: String },
    SwarmConsensus { contributing_drones: u8 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReturnToHomeTrigger {
    trigger_id: u32,
    drone_id: String,
    reason: RthReason,
    triggered_at_epoch: u64,
    home_coordinate: GpsCoordinate,
    estimated_return_s: u32,
    battery_at_trigger_pct: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RthReason {
    LowBattery { remaining_pct: f32 },
    GeofenceBreach { zone_id: u32 },
    SignalLost { last_rssi_dbm: i16 },
    OperatorCommand,
    WeatherAbort { wind_speed_m_s: f32 },
    ObstacleDetected,
    MissionComplete,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AirspaceDeconflictionZone {
    zone_id: u64,
    name: String,
    boundary: Vec<GpsCoordinate>,
    floor_altitude_m: f64,
    ceiling_altitude_m: f64,
    active_drones: Vec<String>,
    max_occupancy: u16,
    priority_level: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GcsCommand {
    Arm,
    Disarm,
    Takeoff { altitude_m: f64 },
    Land,
    GoTo { coordinate: GpsCoordinate },
    SetSpeed { speed_m_s: f32 },
    SetMode { mode: String },
    EmergencyStop,
    ReturnToHome,
    StartMission { plan_id: u64 },
    PauseMission,
    ResumeMission,
    SetGeofence { boundary: GeofenceBoundary },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GcsCommandMessage {
    command_id: u64,
    operator_id: String,
    drone_id: String,
    command: GcsCommand,
    issued_at_epoch: u64,
    acknowledged: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SwarmCoordinationMessage {
    swarm_id: u32,
    sender_drone_id: String,
    message_type: SwarmMessageType,
    timestamp_epoch: u64,
    sequence_number: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SwarmMessageType {
    PositionBroadcast {
        position: GpsCoordinate,
        velocity_m_s: f32,
    },
    TaskAssignment {
        task_id: u32,
        assigned_area: Vec<GpsCoordinate>,
    },
    FormationUpdate {
        slot_index: u8,
        offset_m: (f32, f32, f32),
    },
    CollisionWarning {
        conflicting_drone: String,
        time_to_conflict_s: f32,
    },
    LeaderElection {
        candidate_id: String,
        priority: u16,
    },
    MissionSync {
        waypoint_index: u16,
        eta_s: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PhotogrammetrySurveyGrid {
    survey_id: u64,
    area_name: String,
    grid_lines: Vec<Vec<GpsCoordinate>>,
    overlap_pct: f32,
    sidelap_pct: f32,
    altitude_m: f64,
    gsd_cm_per_px: f32,
    total_photos_expected: u32,
    camera_trigger_interval_m: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DeliveryConfirmation {
    delivery_id: String,
    drone_id: String,
    recipient_name: String,
    drop_coordinate: GpsCoordinate,
    delivered_at_epoch: u64,
    signature_hash: Vec<u8>,
    photo_proof_hash: Vec<u8>,
    weight_delivered_kg: f32,
    condition: DeliveryCondition,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DeliveryCondition {
    Nominal,
    MinorDelay { extra_seconds: u32 },
    ReRouted { reason: String },
    DamagedInTransit { description: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaintenanceLogEntry {
    log_id: u64,
    drone_id: String,
    technician_id: String,
    performed_at_epoch: u64,
    flight_hours_at_service: f64,
    action: MaintenanceAction,
    parts_replaced: Vec<String>,
    notes: String,
    next_service_hours: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MaintenanceAction {
    RoutineInspection,
    MotorReplacement {
        motor_index: u8,
    },
    BatteryReplacement,
    FirmwareUpdate {
        from_version: String,
        to_version: String,
    },
    PropellerChange {
        set_id: String,
    },
    SensorCalibration {
        sensor: String,
    },
    FrameRepair {
        component: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RegulatoryCompliance {
    drone_id: String,
    registration_number: String,
    rule_set: RuleSet,
    pilot_certificate_id: String,
    remote_id_enabled: bool,
    waiver_ids: Vec<String>,
    max_operating_altitude_ft: u32,
    allowed_operations: Vec<String>,
    insurance_policy_id: String,
    last_audit_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RuleSet {
    Part107,
    Part107Waiver {
        waiver_type: String,
    },
    Bvlos {
        authorization_id: String,
        corridor: Vec<GpsCoordinate>,
    },
    RecreationalCas,
    EasaOpen {
        subcategory: String,
    },
    EasaSpecific {
        authorization: String,
    },
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn lz4_roundtrip<T: Encode + Decode<()> + std::fmt::Debug + PartialEq>(
    value: &T,
    label: &str,
) -> (Vec<u8>, Vec<u8>) {
    let encoded = encode_to_vec(value).unwrap_or_else(|_| panic!("encode {label} failed"));
    let compressed =
        compress(&encoded, Compression::Lz4).unwrap_or_else(|_| panic!("compress {label} failed"));
    let decompressed =
        decompress(&compressed).unwrap_or_else(|_| panic!("decompress {label} failed"));
    let (decoded, _): (T, usize) =
        decode_from_slice(&decompressed).unwrap_or_else(|_| panic!("decode {label} failed"));
    assert_eq!(*value, decoded, "{label} roundtrip mismatch");
    (encoded, compressed)
}

// ---------------------------------------------------------------------------
// Test 1 – Flight plan waypoints roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_flight_plan_waypoints_roundtrip() {
    let plan = FlightPlan {
        plan_id: 90001,
        drone_id: "UAV-ALPHA-07".to_string(),
        mission_name: "Coastal survey mission".to_string(),
        waypoints: vec![
            FlightPlanWaypoint {
                sequence: 0,
                coordinate: GpsCoordinate {
                    latitude: 37.7749,
                    longitude: -122.4194,
                    altitude_m: 50.0,
                },
                speed_m_s: 0.0,
                loiter_time_s: 0,
                heading_deg: 0.0,
                action: WaypointAction::FlyThrough,
            },
            FlightPlanWaypoint {
                sequence: 1,
                coordinate: GpsCoordinate {
                    latitude: 37.7850,
                    longitude: -122.4094,
                    altitude_m: 80.0,
                },
                speed_m_s: 12.0,
                loiter_time_s: 0,
                heading_deg: 45.0,
                action: WaypointAction::TakePhoto,
            },
            FlightPlanWaypoint {
                sequence: 2,
                coordinate: GpsCoordinate {
                    latitude: 37.7950,
                    longitude: -122.3994,
                    altitude_m: 80.0,
                },
                speed_m_s: 12.0,
                loiter_time_s: 30,
                heading_deg: 90.0,
                action: WaypointAction::Hover { duration_s: 30 },
            },
            FlightPlanWaypoint {
                sequence: 3,
                coordinate: GpsCoordinate {
                    latitude: 37.7749,
                    longitude: -122.4194,
                    altitude_m: 50.0,
                },
                speed_m_s: 8.0,
                loiter_time_s: 0,
                heading_deg: 270.0,
                action: WaypointAction::AdjustAltitude { target_m: 30.0 },
            },
        ],
        total_distance_m: 4200.0,
        estimated_flight_time_s: 600,
    };

    lz4_roundtrip(&plan, "FlightPlan");
}

// ---------------------------------------------------------------------------
// Test 2 – Geofence boundary roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_geofence_boundary_roundtrip() {
    let geofence = GeofenceBoundary {
        zone_id: 501,
        name: "Airport exclusion zone KSFO".to_string(),
        vertices: vec![
            GpsCoordinate {
                latitude: 37.620,
                longitude: -122.395,
                altitude_m: 0.0,
            },
            GpsCoordinate {
                latitude: 37.625,
                longitude: -122.380,
                altitude_m: 0.0,
            },
            GpsCoordinate {
                latitude: 37.615,
                longitude: -122.370,
                altitude_m: 0.0,
            },
            GpsCoordinate {
                latitude: 37.610,
                longitude: -122.385,
                altitude_m: 0.0,
            },
        ],
        max_altitude_m: 0.0,
        min_altitude_m: 0.0,
        restriction: GeofenceRestriction::NoFlyZone,
    };

    lz4_roundtrip(&geofence, "GeofenceBoundary");
}

// ---------------------------------------------------------------------------
// Test 3 – MAVLink heartbeat telemetry roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_mavlink_heartbeat_roundtrip() {
    let telemetry = MavlinkTelemetry {
        system_id: 1,
        component_id: 1,
        message_id: 0,
        timestamp_us: 1_700_000_000_000_000,
        payload: MavlinkPayload::Heartbeat {
            autopilot_type: 3,
            system_status: 4,
            mavlink_version: 2,
        },
    };

    lz4_roundtrip(&telemetry, "MavlinkHeartbeat");
}

// ---------------------------------------------------------------------------
// Test 4 – MAVLink GPS and attitude telemetry roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_mavlink_gps_attitude_roundtrip() {
    let gps = MavlinkTelemetry {
        system_id: 1,
        component_id: 1,
        message_id: 33,
        timestamp_us: 1_700_000_001_000_000,
        payload: MavlinkPayload::GlobalPositionInt {
            lat_deg_e7: 377_749_000,
            lon_deg_e7: -1_224_194_000,
            alt_mm: 80_000,
            vx_cm_s: 120,
            vy_cm_s: -30,
            vz_cm_s: 0,
            hdg_cdeg: 4500,
        },
    };
    lz4_roundtrip(&gps, "MavlinkGPS");

    let attitude = MavlinkTelemetry {
        system_id: 1,
        component_id: 1,
        message_id: 30,
        timestamp_us: 1_700_000_001_100_000,
        payload: MavlinkPayload::Attitude {
            roll_rad: 0.02,
            pitch_rad: -0.01,
            yaw_rad: 1.57,
            rollspeed: 0.001,
            pitchspeed: -0.002,
            yawspeed: 0.0,
        },
    };
    lz4_roundtrip(&attitude, "MavlinkAttitude");
}

// ---------------------------------------------------------------------------
// Test 5 – Battery endurance calculation roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_battery_endurance_roundtrip() {
    let battery = BatteryEndurance {
        drone_id: "UAV-BRAVO-12".to_string(),
        cell_count: 6,
        nominal_voltage_v: 22.2,
        capacity_mah: 16000,
        current_charge_pct: 78.5,
        estimated_remaining_s: 1440,
        discharge_rate_c: 0.8,
        temperature_celsius: 32.1,
        cycle_count: 147,
        health_pct: 91.3,
    };

    lz4_roundtrip(&battery, "BatteryEndurance");
}

// ---------------------------------------------------------------------------
// Test 6 – Obstacle avoidance map roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_obstacle_avoidance_map_roundtrip() {
    let cells: Vec<ObstacleCell> = (0..64)
        .map(|i| ObstacleCell {
            grid_x: i % 8,
            grid_y: i / 8,
            occupancy_probability: if i % 5 == 0 { 0.9 } else { 0.1 },
            elevation_m: 10.0 + (i as f32) * 0.5,
        })
        .collect();

    let map = ObstacleAvoidanceMap {
        map_id: 3001,
        origin: GpsCoordinate {
            latitude: 37.77,
            longitude: -122.42,
            altitude_m: 0.0,
        },
        resolution_m: 2.0,
        width_cells: 8,
        height_cells: 8,
        cells,
        last_updated_epoch: 1_700_000_500,
    };

    lz4_roundtrip(&map, "ObstacleAvoidanceMap");
}

// ---------------------------------------------------------------------------
// Test 7 – Payload bay config: camera
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_payload_camera_roundtrip() {
    let config = PayloadBayConfig {
        bay_id: 1,
        drone_id: "UAV-CHARLIE-03".to_string(),
        payload: PayloadType::Camera {
            resolution_mp: 48.0,
            focal_length_mm: 24.0,
            stabilized: true,
        },
        weight_kg: 0.35,
        powered: true,
    };

    lz4_roundtrip(&config, "PayloadCamera");
}

// ---------------------------------------------------------------------------
// Test 8 – Payload bay config: LIDAR and delivery bay
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_payload_lidar_delivery_roundtrip() {
    let lidar = PayloadBayConfig {
        bay_id: 2,
        drone_id: "UAV-DELTA-09".to_string(),
        payload: PayloadType::Lidar {
            points_per_second: 300_000,
            range_m: 120.0,
            channels: 16,
        },
        weight_kg: 0.83,
        powered: true,
    };
    lz4_roundtrip(&lidar, "PayloadLidar");

    let delivery = PayloadBayConfig {
        bay_id: 1,
        drone_id: "UAV-ECHO-22".to_string(),
        payload: PayloadType::DeliveryBay {
            max_weight_kg: 2.5,
            dimensions_cm: (30.0, 25.0, 20.0),
            release_mechanism: "electromagnetic latch v3".to_string(),
        },
        weight_kg: 0.6,
        powered: false,
    };
    lz4_roundtrip(&delivery, "PayloadDeliveryBay");
}

// ---------------------------------------------------------------------------
// Test 9 – Wind estimation roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_wind_estimation_roundtrip() {
    let estimates = vec![
        WindEstimate {
            timestamp_epoch: 1_700_100_000,
            altitude_m: 50.0,
            speed_m_s: 4.2,
            direction_deg: 225.0,
            gust_speed_m_s: 7.1,
            confidence: 0.85,
            source: WindSource::Onboard,
        },
        WindEstimate {
            timestamp_epoch: 1_700_100_060,
            altitude_m: 50.0,
            speed_m_s: 5.0,
            direction_deg: 230.0,
            gust_speed_m_s: 8.3,
            confidence: 0.92,
            source: WindSource::SwarmConsensus {
                contributing_drones: 4,
            },
        },
        WindEstimate {
            timestamp_epoch: 1_700_100_120,
            altitude_m: 100.0,
            speed_m_s: 7.8,
            direction_deg: 210.0,
            gust_speed_m_s: 12.0,
            confidence: 0.78,
            source: WindSource::Metar {
                station_id: "KSFO".to_string(),
            },
        },
    ];

    lz4_roundtrip(&estimates, "WindEstimateVec");
}

// ---------------------------------------------------------------------------
// Test 10 – Return-to-home triggers roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_return_to_home_triggers_roundtrip() {
    let trigger = ReturnToHomeTrigger {
        trigger_id: 7001,
        drone_id: "UAV-FOXTROT-15".to_string(),
        reason: RthReason::LowBattery {
            remaining_pct: 18.5,
        },
        triggered_at_epoch: 1_700_200_000,
        home_coordinate: GpsCoordinate {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude_m: 5.0,
        },
        estimated_return_s: 120,
        battery_at_trigger_pct: 18.5,
    };

    lz4_roundtrip(&trigger, "ReturnToHomeTrigger");

    let signal_lost = ReturnToHomeTrigger {
        trigger_id: 7002,
        drone_id: "UAV-GOLF-08".to_string(),
        reason: RthReason::SignalLost { last_rssi_dbm: -95 },
        triggered_at_epoch: 1_700_200_300,
        home_coordinate: GpsCoordinate {
            latitude: 38.0,
            longitude: -121.9,
            altitude_m: 10.0,
        },
        estimated_return_s: 300,
        battery_at_trigger_pct: 55.0,
    };

    lz4_roundtrip(&signal_lost, "RTH-SignalLost");
}

// ---------------------------------------------------------------------------
// Test 11 – Airspace deconfliction zone roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_airspace_deconfliction_roundtrip() {
    let zone = AirspaceDeconflictionZone {
        zone_id: 88001,
        name: "Urban corridor segment A-7".to_string(),
        boundary: vec![
            GpsCoordinate {
                latitude: 37.78,
                longitude: -122.41,
                altitude_m: 0.0,
            },
            GpsCoordinate {
                latitude: 37.79,
                longitude: -122.40,
                altitude_m: 0.0,
            },
            GpsCoordinate {
                latitude: 37.79,
                longitude: -122.39,
                altitude_m: 0.0,
            },
            GpsCoordinate {
                latitude: 37.78,
                longitude: -122.39,
                altitude_m: 0.0,
            },
        ],
        floor_altitude_m: 30.0,
        ceiling_altitude_m: 120.0,
        active_drones: vec![
            "UAV-HOTEL-01".to_string(),
            "UAV-HOTEL-02".to_string(),
            "UAV-HOTEL-03".to_string(),
        ],
        max_occupancy: 5,
        priority_level: 2,
    };

    lz4_roundtrip(&zone, "AirspaceDeconflictionZone");
}

// ---------------------------------------------------------------------------
// Test 12 – Ground control station commands roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_gcs_commands_roundtrip() {
    let commands = vec![
        GcsCommandMessage {
            command_id: 10001,
            operator_id: "OP-LIMA-42".to_string(),
            drone_id: "UAV-INDIA-06".to_string(),
            command: GcsCommand::Takeoff { altitude_m: 50.0 },
            issued_at_epoch: 1_700_300_000,
            acknowledged: true,
        },
        GcsCommandMessage {
            command_id: 10002,
            operator_id: "OP-LIMA-42".to_string(),
            drone_id: "UAV-INDIA-06".to_string(),
            command: GcsCommand::GoTo {
                coordinate: GpsCoordinate {
                    latitude: 37.80,
                    longitude: -122.38,
                    altitude_m: 80.0,
                },
            },
            issued_at_epoch: 1_700_300_060,
            acknowledged: true,
        },
        GcsCommandMessage {
            command_id: 10003,
            operator_id: "OP-LIMA-42".to_string(),
            drone_id: "UAV-INDIA-06".to_string(),
            command: GcsCommand::EmergencyStop,
            issued_at_epoch: 1_700_300_120,
            acknowledged: false,
        },
    ];

    lz4_roundtrip(&commands, "GcsCommandVec");
}

// ---------------------------------------------------------------------------
// Test 13 – Swarm coordination protocol roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_swarm_coordination_roundtrip() {
    let messages = vec![
        SwarmCoordinationMessage {
            swarm_id: 200,
            sender_drone_id: "SWARM-J-01".to_string(),
            message_type: SwarmMessageType::PositionBroadcast {
                position: GpsCoordinate {
                    latitude: 37.775,
                    longitude: -122.418,
                    altitude_m: 60.0,
                },
                velocity_m_s: 8.5,
            },
            timestamp_epoch: 1_700_400_000,
            sequence_number: 1,
        },
        SwarmCoordinationMessage {
            swarm_id: 200,
            sender_drone_id: "SWARM-J-02".to_string(),
            message_type: SwarmMessageType::CollisionWarning {
                conflicting_drone: "SWARM-J-01".to_string(),
                time_to_conflict_s: 3.2,
            },
            timestamp_epoch: 1_700_400_001,
            sequence_number: 2,
        },
        SwarmCoordinationMessage {
            swarm_id: 200,
            sender_drone_id: "SWARM-J-03".to_string(),
            message_type: SwarmMessageType::LeaderElection {
                candidate_id: "SWARM-J-03".to_string(),
                priority: 100,
            },
            timestamp_epoch: 1_700_400_002,
            sequence_number: 3,
        },
    ];

    lz4_roundtrip(&messages, "SwarmCoordinationVec");
}

// ---------------------------------------------------------------------------
// Test 14 – Photogrammetry survey grid roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_photogrammetry_survey_grid_roundtrip() {
    let grid = PhotogrammetrySurveyGrid {
        survey_id: 55001,
        area_name: "Construction site sector B".to_string(),
        grid_lines: vec![
            vec![
                GpsCoordinate {
                    latitude: 37.780,
                    longitude: -122.410,
                    altitude_m: 100.0,
                },
                GpsCoordinate {
                    latitude: 37.780,
                    longitude: -122.400,
                    altitude_m: 100.0,
                },
            ],
            vec![
                GpsCoordinate {
                    latitude: 37.781,
                    longitude: -122.410,
                    altitude_m: 100.0,
                },
                GpsCoordinate {
                    latitude: 37.781,
                    longitude: -122.400,
                    altitude_m: 100.0,
                },
            ],
            vec![
                GpsCoordinate {
                    latitude: 37.782,
                    longitude: -122.410,
                    altitude_m: 100.0,
                },
                GpsCoordinate {
                    latitude: 37.782,
                    longitude: -122.400,
                    altitude_m: 100.0,
                },
            ],
        ],
        overlap_pct: 80.0,
        sidelap_pct: 70.0,
        altitude_m: 100.0,
        gsd_cm_per_px: 2.5,
        total_photos_expected: 450,
        camera_trigger_interval_m: 15.0,
    };

    lz4_roundtrip(&grid, "PhotogrammetrySurveyGrid");
}

// ---------------------------------------------------------------------------
// Test 15 – Delivery confirmation receipt roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_delivery_confirmation_roundtrip() {
    let receipt = DeliveryConfirmation {
        delivery_id: "DEL-2024-001847".to_string(),
        drone_id: "UAV-KILO-19".to_string(),
        recipient_name: "Taro Yamada".to_string(),
        drop_coordinate: GpsCoordinate {
            latitude: 35.6812,
            longitude: 139.7671,
            altitude_m: 2.0,
        },
        delivered_at_epoch: 1_700_500_000,
        signature_hash: vec![0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x23, 0x45, 0x67],
        photo_proof_hash: vec![0xCA, 0xFE, 0xBA, 0xBE, 0x89, 0xAB, 0xCD, 0xEF],
        weight_delivered_kg: 1.2,
        condition: DeliveryCondition::Nominal,
    };

    lz4_roundtrip(&receipt, "DeliveryConfirmation");
}

// ---------------------------------------------------------------------------
// Test 16 – Maintenance log entry roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_maintenance_log_roundtrip() {
    let entry = MaintenanceLogEntry {
        log_id: 44001,
        drone_id: "UAV-MIKE-05".to_string(),
        technician_id: "TECH-307".to_string(),
        performed_at_epoch: 1_700_600_000,
        flight_hours_at_service: 312.5,
        action: MaintenanceAction::MotorReplacement { motor_index: 3 },
        parts_replaced: vec![
            "Motor-2212-920KV".to_string(),
            "ESC-30A-BLHeli".to_string(),
        ],
        notes: "Motor 3 exhibited abnormal vibration pattern at 75% throttle. Replaced motor and ESC as preventive measure.".to_string(),
        next_service_hours: 362.5,
    };

    lz4_roundtrip(&entry, "MaintenanceLogEntry");
}

// ---------------------------------------------------------------------------
// Test 17 – Regulatory compliance Part 107 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_regulatory_compliance_part107_roundtrip() {
    let compliance = RegulatoryCompliance {
        drone_id: "UAV-NOVEMBER-11".to_string(),
        registration_number: "FA3WXYZ789".to_string(),
        rule_set: RuleSet::Part107,
        pilot_certificate_id: "RPIC-2024-55123".to_string(),
        remote_id_enabled: true,
        waiver_ids: vec![],
        max_operating_altitude_ft: 400,
        allowed_operations: vec![
            "Visual line of sight".to_string(),
            "Daylight operations".to_string(),
            "Max 55 lbs".to_string(),
        ],
        insurance_policy_id: "INS-DRONE-2024-8812".to_string(),
        last_audit_epoch: 1_700_000_000,
    };

    lz4_roundtrip(&compliance, "RegulatoryPart107");
}

// ---------------------------------------------------------------------------
// Test 18 – Regulatory compliance BVLOS roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_regulatory_compliance_bvlos_roundtrip() {
    let bvlos = RegulatoryCompliance {
        drone_id: "UAV-OSCAR-14".to_string(),
        registration_number: "FA3ABCD012".to_string(),
        rule_set: RuleSet::Bvlos {
            authorization_id: "BVLOS-AUTH-2024-0042".to_string(),
            corridor: vec![
                GpsCoordinate {
                    latitude: 37.77,
                    longitude: -122.42,
                    altitude_m: 60.0,
                },
                GpsCoordinate {
                    latitude: 37.85,
                    longitude: -122.35,
                    altitude_m: 60.0,
                },
                GpsCoordinate {
                    latitude: 37.90,
                    longitude: -122.30,
                    altitude_m: 60.0,
                },
            ],
        },
        pilot_certificate_id: "RPIC-2024-77001".to_string(),
        remote_id_enabled: true,
        waiver_ids: vec!["WAIVER-107-44".to_string(), "WAIVER-107-108".to_string()],
        max_operating_altitude_ft: 400,
        allowed_operations: vec![
            "Beyond visual line of sight".to_string(),
            "Corridor operations".to_string(),
            "Automated flight".to_string(),
        ],
        insurance_policy_id: "INS-DRONE-2024-9003".to_string(),
        last_audit_epoch: 1_700_700_000,
    };

    lz4_roundtrip(&bvlos, "RegulatoryBVLOS");
}

// ---------------------------------------------------------------------------
// Test 19 – Large obstacle map compresses smaller than raw encoding
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_large_obstacle_map_compresses_smaller() {
    let cells: Vec<ObstacleCell> = (0..1024)
        .map(|i| ObstacleCell {
            grid_x: (i % 32) as u16,
            grid_y: (i / 32) as u16,
            occupancy_probability: 0.1,
            elevation_m: 25.0,
        })
        .collect();

    let map = ObstacleAvoidanceMap {
        map_id: 9999,
        origin: GpsCoordinate {
            latitude: 35.68,
            longitude: 139.77,
            altitude_m: 0.0,
        },
        resolution_m: 1.0,
        width_cells: 32,
        height_cells: 32,
        cells,
        last_updated_epoch: 1_700_800_000,
    };

    let (encoded, compressed) = lz4_roundtrip(&map, "LargeObstacleMap");
    assert!(
        compressed.len() < encoded.len(),
        "compressed size {} should be smaller than uncompressed size {} for repetitive obstacle map",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// Test 20 – Repeated swarm broadcasts compress smaller
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_repeated_swarm_broadcasts_compress_smaller() {
    let messages: Vec<SwarmCoordinationMessage> = (0..100)
        .map(|i| SwarmCoordinationMessage {
            swarm_id: 300,
            sender_drone_id: format!("SWARM-P-{:02}", i % 10),
            message_type: SwarmMessageType::PositionBroadcast {
                position: GpsCoordinate {
                    latitude: 37.77 + (i as f64) * 0.0001,
                    longitude: -122.42 + (i as f64) * 0.0001,
                    altitude_m: 60.0,
                },
                velocity_m_s: 8.0,
            },
            timestamp_epoch: 1_700_900_000 + (i as u64),
            sequence_number: i as u32,
        })
        .collect();

    let (encoded, compressed) = lz4_roundtrip(&messages, "RepeatedSwarmBroadcasts");
    assert!(
        compressed.len() < encoded.len(),
        "compressed size {} should be smaller than uncompressed size {} for repeated swarm msgs",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// Test 21 – Mixed maintenance log batch compresses smaller
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_maintenance_log_batch_compresses_smaller() {
    let entries: Vec<MaintenanceLogEntry> = (0..50)
        .map(|i| MaintenanceLogEntry {
            log_id: 60000 + i as u64,
            drone_id: format!("UAV-QUEBEC-{:02}", i % 8),
            technician_id: "TECH-500".to_string(),
            performed_at_epoch: 1_701_000_000 + (i as u64) * 86400,
            flight_hours_at_service: 100.0 + (i as f64) * 5.0,
            action: MaintenanceAction::RoutineInspection,
            parts_replaced: vec![],
            notes: "Routine pre-flight inspection completed. All systems nominal.".to_string(),
            next_service_hours: 150.0 + (i as f64) * 5.0,
        })
        .collect();

    let (encoded, compressed) = lz4_roundtrip(&entries, "MaintenanceLogBatch");
    assert!(
        compressed.len() < encoded.len(),
        "compressed size {} should be smaller than uncompressed size {} for maintenance batch",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// Test 22 – Full mission scenario: plan + telemetry + delivery + compliance
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_full_mission_scenario_roundtrip() {
    let plan = FlightPlan {
        plan_id: 77001,
        drone_id: "UAV-ROMEO-20".to_string(),
        mission_name: "Package delivery mission PD-2024-100".to_string(),
        waypoints: vec![
            FlightPlanWaypoint {
                sequence: 0,
                coordinate: GpsCoordinate {
                    latitude: 35.6812,
                    longitude: 139.7671,
                    altitude_m: 10.0,
                },
                speed_m_s: 0.0,
                loiter_time_s: 0,
                heading_deg: 0.0,
                action: WaypointAction::FlyThrough,
            },
            FlightPlanWaypoint {
                sequence: 1,
                coordinate: GpsCoordinate {
                    latitude: 35.6900,
                    longitude: 139.7500,
                    altitude_m: 80.0,
                },
                speed_m_s: 15.0,
                loiter_time_s: 0,
                heading_deg: 315.0,
                action: WaypointAction::FlyThrough,
            },
            FlightPlanWaypoint {
                sequence: 2,
                coordinate: GpsCoordinate {
                    latitude: 35.7000,
                    longitude: 139.7400,
                    altitude_m: 15.0,
                },
                speed_m_s: 5.0,
                loiter_time_s: 10,
                heading_deg: 0.0,
                action: WaypointAction::DropPayload,
            },
        ],
        total_distance_m: 3100.0,
        estimated_flight_time_s: 480,
    };
    lz4_roundtrip(&plan, "MissionPlan");

    let telemetry_stream: Vec<MavlinkTelemetry> = (0..20)
        .map(|i| MavlinkTelemetry {
            system_id: 1,
            component_id: 1,
            message_id: 33,
            timestamp_us: 1_700_000_000_000_000 + (i as u64) * 500_000,
            payload: MavlinkPayload::GlobalPositionInt {
                lat_deg_e7: 356_812_000 + i * 1000,
                lon_deg_e7: 1_397_671_000 - i * 800,
                alt_mm: 80_000,
                vx_cm_s: 150,
                vy_cm_s: -80,
                vz_cm_s: 0,
                hdg_cdeg: 31500,
            },
        })
        .collect();
    lz4_roundtrip(&telemetry_stream, "MissionTelemetryStream");

    let delivery = DeliveryConfirmation {
        delivery_id: "DEL-2024-100".to_string(),
        drone_id: "UAV-ROMEO-20".to_string(),
        recipient_name: "Hanako Suzuki".to_string(),
        drop_coordinate: GpsCoordinate {
            latitude: 35.7000,
            longitude: 139.7400,
            altitude_m: 2.0,
        },
        delivered_at_epoch: 1_700_000_480,
        signature_hash: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        photo_proof_hash: vec![0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8],
        weight_delivered_kg: 0.8,
        condition: DeliveryCondition::Nominal,
    };
    lz4_roundtrip(&delivery, "MissionDelivery");

    let compliance = RegulatoryCompliance {
        drone_id: "UAV-ROMEO-20".to_string(),
        registration_number: "JA-UAS-2024-0200".to_string(),
        rule_set: RuleSet::Part107,
        pilot_certificate_id: "JP-RPIC-88001".to_string(),
        remote_id_enabled: true,
        waiver_ids: vec![],
        max_operating_altitude_ft: 400,
        allowed_operations: vec!["Urban delivery corridor".to_string()],
        insurance_policy_id: "INS-JP-2024-3001".to_string(),
        last_audit_epoch: 1_699_900_000,
    };
    lz4_roundtrip(&compliance, "MissionCompliance");
}
