#![cfg(feature = "versioning")]

//! Autonomous drone / UAV systems domain — versioning feature tests.
//!
//! 22 #[test] functions covering flight plans, waypoints, telemetry data,
//! mission types, battery management, obstacle avoidance, swarm coordination,
//! payload delivery, and geofencing using encode_versioned_value /
//! decode_versioned_value.

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
use oxicode::{
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value, Decode,
    Encode,
};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FlightMode {
    Manual,
    Stabilized,
    AltitudeHold,
    PositionHold,
    Mission,
    ReturnToHome,
    Land,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MissionType {
    Surveillance,
    Delivery,
    Search,
    Mapping,
    Inspection,
    SwarmFormation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ObstacleAction {
    Stop,
    Avoid,
    ReturnToHome,
    Land,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GeofenceAction {
    Warn,
    Hover,
    ReturnToHome,
    Land,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Waypoint {
    waypoint_id: u32,
    latitude_deg_e7: i64,
    longitude_deg_e7: i64,
    altitude_mm: i32,
    speed_cms: u16,
    hover_time_ms: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlightPlanV1 {
    plan_id: u64,
    drone_id: String,
    mission_type: MissionType,
    waypoints: Vec<Waypoint>,
    home_latitude_deg_e7: i64,
    home_longitude_deg_e7: i64,
    max_altitude_mm: i32,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlightPlanV2 {
    plan_id: u64,
    drone_id: String,
    mission_type: MissionType,
    waypoints: Vec<Waypoint>,
    home_latitude_deg_e7: i64,
    home_longitude_deg_e7: i64,
    max_altitude_mm: i32,
    active: bool,
    priority: u8,
    geofence_radius_m: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TelemetryFrame {
    drone_id: String,
    timestamp_ms: u64,
    latitude_deg_e7: i64,
    longitude_deg_e7: i64,
    altitude_mm: i32,
    roll_mrad: i16,
    pitch_mrad: i16,
    yaw_mrad: i16,
    ground_speed_cms: u16,
    vertical_speed_cms: i16,
    flight_mode: FlightMode,
    battery_percent: u8,
    gps_fix_satellites: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatteryStatus {
    drone_id: String,
    cell_count: u8,
    voltage_mv: u32,
    current_ma: i32,
    capacity_mah: u32,
    remaining_mah: u32,
    temperature_celsius_tenths: i16,
    cycle_count: u16,
    critical: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ObstacleEvent {
    drone_id: String,
    timestamp_ms: u64,
    distance_cm: u16,
    bearing_mrad: u16,
    action_taken: ObstacleAction,
    avoided: bool,
    sensor_id: Option<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SwarmCoordinationMessage {
    swarm_id: String,
    leader_drone_id: String,
    member_drone_ids: Vec<String>,
    formation_name: String,
    target_spacing_m: f32,
    synchronised_at_ms: u64,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PayloadDelivery {
    delivery_id: u64,
    drone_id: String,
    payload_mass_g: u32,
    destination_latitude_deg_e7: i64,
    destination_longitude_deg_e7: i64,
    release_altitude_mm: i32,
    delivered: bool,
    actual_delivery_timestamp_ms: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeofenceZone {
    zone_id: u32,
    name: String,
    center_latitude_deg_e7: i64,
    center_longitude_deg_e7: i64,
    radius_m: u32,
    min_altitude_mm: i32,
    max_altitude_mm: i32,
    action: GeofenceAction,
    enabled: bool,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn test_waypoint_basic_roundtrip() {
    let wp = Waypoint {
        waypoint_id: 1,
        latitude_deg_e7: 377_749_000,
        longitude_deg_e7: -1_224_194_000,
        altitude_mm: 50_000,
        speed_cms: 500,
        hover_time_ms: 2_000,
    };

    let encoded = encode_to_vec(&wp).expect("encode Waypoint failed");
    let (decoded, _): (Waypoint, _) = decode_from_slice(&encoded).expect("decode Waypoint failed");

    assert_eq!(wp, decoded);
}

#[test]
fn test_flight_plan_v1_versioned_encode_decode() {
    let plan = FlightPlanV1 {
        plan_id: 1001,
        drone_id: String::from("UAV-ALPHA-01"),
        mission_type: MissionType::Surveillance,
        waypoints: vec![
            Waypoint {
                waypoint_id: 1,
                latitude_deg_e7: 377_749_000,
                longitude_deg_e7: -1_224_194_000,
                altitude_mm: 30_000,
                speed_cms: 800,
                hover_time_ms: 0,
            },
            Waypoint {
                waypoint_id: 2,
                latitude_deg_e7: 377_800_000,
                longitude_deg_e7: -1_224_100_000,
                altitude_mm: 30_000,
                speed_cms: 800,
                hover_time_ms: 5_000,
            },
        ],
        home_latitude_deg_e7: 377_700_000,
        home_longitude_deg_e7: -1_224_300_000,
        max_altitude_mm: 120_000,
        active: true,
    };
    let ver = Version::new(1, 0, 0);

    let bytes = encode_versioned_value(&plan, ver).expect("versioned encode FlightPlanV1 failed");
    let (decoded, decoded_ver, _): (FlightPlanV1, Version, usize) =
        decode_versioned_value::<FlightPlanV1>(&bytes)
            .expect("versioned decode FlightPlanV1 failed");

    assert_eq!(plan, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
}

#[test]
fn test_version_field_access_major_minor_patch() {
    let ver = Version::new(3, 5, 2);

    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 5);
    assert_eq!(ver.patch, 2);
}

#[test]
fn test_flight_plan_v2_with_geofence_radius_some() {
    let plan = FlightPlanV2 {
        plan_id: 2002,
        drone_id: String::from("UAV-BETA-07"),
        mission_type: MissionType::Delivery,
        waypoints: vec![Waypoint {
            waypoint_id: 1,
            latitude_deg_e7: 518_520_000,
            longitude_deg_e7: -1_242_000,
            altitude_mm: 20_000,
            speed_cms: 1_200,
            hover_time_ms: 3_000,
        }],
        home_latitude_deg_e7: 518_500_000,
        home_longitude_deg_e7: -1_240_000,
        max_altitude_mm: 100_000,
        active: true,
        priority: 5,
        geofence_radius_m: Some(500),
    };
    let ver = Version::new(2, 0, 0);

    let bytes = encode_versioned_value(&plan, ver).expect("versioned encode FlightPlanV2 failed");
    let (decoded, decoded_ver, consumed): (FlightPlanV2, Version, usize) =
        decode_versioned_value::<FlightPlanV2>(&bytes)
            .expect("versioned decode FlightPlanV2 failed");

    assert_eq!(plan, decoded);
    assert_eq!(decoded.geofence_radius_m, Some(500));
    assert_eq!(decoded_ver.major, 2);
    assert!(consumed > 0);
}

#[test]
fn test_flight_plan_v2_geofence_none() {
    let plan = FlightPlanV2 {
        plan_id: 3003,
        drone_id: String::from("UAV-GAMMA-12"),
        mission_type: MissionType::Mapping,
        waypoints: vec![],
        home_latitude_deg_e7: 484_000_000,
        home_longitude_deg_e7: 22_500_000,
        max_altitude_mm: 80_000,
        active: false,
        priority: 1,
        geofence_radius_m: None,
    };
    let ver = Version::new(2, 1, 0);

    let bytes = encode_versioned_value(&plan, ver).expect("encode FlightPlanV2 no geofence failed");
    let (decoded, decoded_ver, _): (FlightPlanV2, Version, usize) =
        decode_versioned_value::<FlightPlanV2>(&bytes)
            .expect("decode FlightPlanV2 no geofence failed");

    assert!(decoded.geofence_radius_m.is_none());
    assert!(!decoded.active);
    assert_eq!(decoded_ver.minor, 1);
}

#[test]
fn test_telemetry_frame_versioned_encode_decode() {
    let frame = TelemetryFrame {
        drone_id: String::from("UAV-DELTA-03"),
        timestamp_ms: 1_700_000_000_000,
        latitude_deg_e7: 355_890_000,
        longitude_deg_e7: 1_397_820_000,
        altitude_mm: 45_000,
        roll_mrad: -120,
        pitch_mrad: 50,
        yaw_mrad: 3_141,
        ground_speed_cms: 1_500,
        vertical_speed_cms: -100,
        flight_mode: FlightMode::Mission,
        battery_percent: 72,
        gps_fix_satellites: 14,
    };
    let ver = Version::new(1, 3, 0);

    let bytes =
        encode_versioned_value(&frame, ver).expect("versioned encode TelemetryFrame failed");
    let (decoded, decoded_ver, _): (TelemetryFrame, Version, usize) =
        decode_versioned_value::<TelemetryFrame>(&bytes)
            .expect("versioned decode TelemetryFrame failed");

    assert_eq!(frame, decoded);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 3);
    assert_eq!(decoded.flight_mode, FlightMode::Mission);
}

#[test]
fn test_telemetry_frame_return_to_home_mode() {
    let frame = TelemetryFrame {
        drone_id: String::from("UAV-ECHO-09"),
        timestamp_ms: 1_700_000_050_000,
        latitude_deg_e7: 355_890_100,
        longitude_deg_e7: 1_397_820_100,
        altitude_mm: 60_000,
        roll_mrad: 0,
        pitch_mrad: 0,
        yaw_mrad: 0,
        ground_speed_cms: 600,
        vertical_speed_cms: 0,
        flight_mode: FlightMode::ReturnToHome,
        battery_percent: 15,
        gps_fix_satellites: 10,
    };

    let encoded = encode_to_vec(&frame).expect("encode RTH telemetry failed");
    let (decoded, _): (TelemetryFrame, _) =
        decode_from_slice(&encoded).expect("decode RTH telemetry failed");

    assert_eq!(decoded.flight_mode, FlightMode::ReturnToHome);
    assert_eq!(decoded.battery_percent, 15);
}

#[test]
fn test_battery_status_critical_versioned() {
    let bat = BatteryStatus {
        drone_id: String::from("UAV-FOXTROT-02"),
        cell_count: 6,
        voltage_mv: 18_500,
        current_ma: 25_000,
        capacity_mah: 10_000,
        remaining_mah: 800,
        temperature_celsius_tenths: 420,
        cycle_count: 312,
        critical: true,
    };
    let ver = Version::new(1, 0, 0);

    let bytes = encode_versioned_value(&bat, ver).expect("versioned encode BatteryStatus failed");
    let (decoded, decoded_ver, consumed): (BatteryStatus, Version, usize) =
        decode_versioned_value::<BatteryStatus>(&bytes)
            .expect("versioned decode BatteryStatus failed");

    assert_eq!(bat, decoded);
    assert!(decoded.critical);
    assert_eq!(decoded.remaining_mah, 800);
    assert_eq!(decoded_ver.major, 1);
    assert!(consumed > 0);
}

#[test]
fn test_battery_status_healthy_roundtrip() {
    let bat = BatteryStatus {
        drone_id: String::from("UAV-GOLF-05"),
        cell_count: 4,
        voltage_mv: 16_800,
        current_ma: 8_000,
        capacity_mah: 5_000,
        remaining_mah: 4_200,
        temperature_celsius_tenths: 250,
        cycle_count: 45,
        critical: false,
    };

    let encoded = encode_to_vec(&bat).expect("encode healthy battery failed");
    let (decoded, _): (BatteryStatus, _) =
        decode_from_slice(&encoded).expect("decode healthy battery failed");

    assert!(!decoded.critical);
    assert_eq!(decoded.cell_count, 4);
    assert_eq!(decoded.remaining_mah, 4_200);
}

#[test]
fn test_obstacle_event_with_sensor_id_some() {
    let event = ObstacleEvent {
        drone_id: String::from("UAV-HOTEL-11"),
        timestamp_ms: 1_700_000_100_000,
        distance_cm: 350,
        bearing_mrad: 1_570,
        action_taken: ObstacleAction::Avoid,
        avoided: true,
        sensor_id: Some(3),
    };
    let ver = Version::new(1, 2, 0);

    let bytes = encode_versioned_value(&event, ver).expect("versioned encode ObstacleEvent failed");
    let (decoded, decoded_ver, _): (ObstacleEvent, Version, usize) =
        decode_versioned_value::<ObstacleEvent>(&bytes)
            .expect("versioned decode ObstacleEvent failed");

    assert_eq!(event, decoded);
    assert_eq!(decoded.sensor_id, Some(3));
    assert!(decoded.avoided);
    assert_eq!(decoded_ver.minor, 2);
}

#[test]
fn test_obstacle_event_land_no_sensor() {
    let event = ObstacleEvent {
        drone_id: String::from("UAV-INDIA-04"),
        timestamp_ms: 1_700_000_200_000,
        distance_cm: 80,
        bearing_mrad: 0,
        action_taken: ObstacleAction::Land,
        avoided: false,
        sensor_id: None,
    };

    let encoded = encode_to_vec(&event).expect("encode land obstacle event failed");
    let (decoded, _): (ObstacleEvent, _) =
        decode_from_slice(&encoded).expect("decode land obstacle event failed");

    assert_eq!(decoded.action_taken, ObstacleAction::Land);
    assert!(!decoded.avoided);
    assert!(decoded.sensor_id.is_none());
}

#[test]
fn test_swarm_coordination_message_versioned() {
    let msg = SwarmCoordinationMessage {
        swarm_id: String::from("SWARM-ALPHA"),
        leader_drone_id: String::from("UAV-LEAD-01"),
        member_drone_ids: vec![
            String::from("UAV-MEM-02"),
            String::from("UAV-MEM-03"),
            String::from("UAV-MEM-04"),
            String::from("UAV-MEM-05"),
        ],
        formation_name: String::from("Delta-V"),
        target_spacing_m: 15.0_f32,
        synchronised_at_ms: 1_700_000_300_000,
        active: true,
    };
    let ver = Version::new(2, 0, 0);

    let bytes = encode_versioned_value(&msg, ver)
        .expect("versioned encode SwarmCoordinationMessage failed");
    let (decoded, decoded_ver, consumed): (SwarmCoordinationMessage, Version, usize) =
        decode_versioned_value::<SwarmCoordinationMessage>(&bytes)
            .expect("versioned decode SwarmCoordinationMessage failed");

    assert_eq!(msg, decoded);
    assert_eq!(decoded.member_drone_ids.len(), 4);
    assert_eq!(decoded_ver.major, 2);
    assert!(consumed > 0);
    assert!(consumed <= bytes.len());
}

#[test]
fn test_swarm_coordination_no_members() {
    let msg = SwarmCoordinationMessage {
        swarm_id: String::from("SWARM-EMPTY"),
        leader_drone_id: String::from("UAV-SOLO-01"),
        member_drone_ids: vec![],
        formation_name: String::from("Solo"),
        target_spacing_m: 0.0_f32,
        synchronised_at_ms: 1_700_000_400_000,
        active: false,
    };

    let encoded = encode_to_vec(&msg).expect("encode solo swarm failed");
    let (decoded, _): (SwarmCoordinationMessage, _) =
        decode_from_slice(&encoded).expect("decode solo swarm failed");

    assert!(decoded.member_drone_ids.is_empty());
    assert!(!decoded.active);
    assert_eq!(decoded.formation_name, "Solo");
}

#[test]
fn test_payload_delivery_delivered_versioned() {
    let delivery = PayloadDelivery {
        delivery_id: 7_001,
        drone_id: String::from("UAV-JULIET-08"),
        payload_mass_g: 2_500,
        destination_latitude_deg_e7: 356_000_000,
        destination_longitude_deg_e7: 1_398_000_000,
        release_altitude_mm: 5_000,
        delivered: true,
        actual_delivery_timestamp_ms: Some(1_700_000_500_000),
    };
    let ver = Version::new(1, 0, 0);

    let bytes =
        encode_versioned_value(&delivery, ver).expect("versioned encode PayloadDelivery failed");
    let (decoded, decoded_ver, _): (PayloadDelivery, Version, usize) =
        decode_versioned_value::<PayloadDelivery>(&bytes)
            .expect("versioned decode PayloadDelivery failed");

    assert_eq!(delivery, decoded);
    assert!(decoded.delivered);
    assert_eq!(
        decoded.actual_delivery_timestamp_ms,
        Some(1_700_000_500_000)
    );
    assert_eq!(decoded_ver.major, 1);
}

#[test]
fn test_payload_delivery_pending_no_timestamp() {
    let delivery = PayloadDelivery {
        delivery_id: 7_002,
        drone_id: String::from("UAV-KILO-06"),
        payload_mass_g: 1_000,
        destination_latitude_deg_e7: 486_000_000,
        destination_longitude_deg_e7: 2_350_000_000,
        release_altitude_mm: 8_000,
        delivered: false,
        actual_delivery_timestamp_ms: None,
    };

    let encoded = encode_to_vec(&delivery).expect("encode pending delivery failed");
    let (decoded, _): (PayloadDelivery, _) =
        decode_from_slice(&encoded).expect("decode pending delivery failed");

    assert!(!decoded.delivered);
    assert!(decoded.actual_delivery_timestamp_ms.is_none());
    assert_eq!(decoded.payload_mass_g, 1_000);
}

#[test]
fn test_geofence_zone_versioned_hover_action() {
    let zone = GeofenceZone {
        zone_id: 42,
        name: String::from("AirportExclusion"),
        center_latitude_deg_e7: 356_500_000,
        center_longitude_deg_e7: 1_399_000_000,
        radius_m: 5_000,
        min_altitude_mm: 0,
        max_altitude_mm: 300_000,
        action: GeofenceAction::Hover,
        enabled: true,
    };
    let ver = Version::new(1, 1, 0);

    let bytes = encode_versioned_value(&zone, ver).expect("versioned encode GeofenceZone failed");
    let (decoded, decoded_ver, consumed): (GeofenceZone, Version, usize) =
        decode_versioned_value::<GeofenceZone>(&bytes)
            .expect("versioned decode GeofenceZone failed");

    assert_eq!(zone, decoded);
    assert_eq!(decoded.action, GeofenceAction::Hover);
    assert!(decoded.enabled);
    assert_eq!(decoded_ver.minor, 1);
    assert!(consumed > 0);
}

#[test]
fn test_geofence_zone_disabled_warn_action() {
    let zone = GeofenceZone {
        zone_id: 99,
        name: String::from("AdvisoryZone"),
        center_latitude_deg_e7: 510_000_000,
        center_longitude_deg_e7: -1_250_000,
        radius_m: 1_000,
        min_altitude_mm: 10_000,
        max_altitude_mm: 50_000,
        action: GeofenceAction::Warn,
        enabled: false,
    };

    let encoded = encode_to_vec(&zone).expect("encode disabled geofence failed");
    let (decoded, _): (GeofenceZone, _) =
        decode_from_slice(&encoded).expect("decode disabled geofence failed");

    assert_eq!(decoded.action, GeofenceAction::Warn);
    assert!(!decoded.enabled);
    assert_eq!(decoded.radius_m, 1_000);
}

#[test]
fn test_multiple_versions_same_telemetry_struct() {
    let frame = TelemetryFrame {
        drone_id: String::from("UAV-LIMA-10"),
        timestamp_ms: 1_700_001_000_000,
        latitude_deg_e7: 340_000_000,
        longitude_deg_e7: 1_360_000_000,
        altitude_mm: 25_000,
        roll_mrad: 10,
        pitch_mrad: -5,
        yaw_mrad: 1_571,
        ground_speed_cms: 900,
        vertical_speed_cms: 0,
        flight_mode: FlightMode::AltitudeHold,
        battery_percent: 88,
        gps_fix_satellites: 16,
    };

    let ver_a = Version::new(1, 0, 0);
    let ver_b = Version::new(2, 4, 1);

    let bytes_a = encode_versioned_value(&frame, ver_a).expect("encode telemetry v1.0.0 failed");
    let bytes_b = encode_versioned_value(&frame, ver_b).expect("encode telemetry v2.4.1 failed");

    let (_, dver_a, _): (TelemetryFrame, Version, usize) =
        decode_versioned_value::<TelemetryFrame>(&bytes_a).expect("decode telemetry v1.0.0 failed");
    let (_, dver_b, _): (TelemetryFrame, Version, usize) =
        decode_versioned_value::<TelemetryFrame>(&bytes_b).expect("decode telemetry v2.4.1 failed");

    assert_eq!(dver_a.major, 1);
    assert_eq!(dver_a.minor, 0);
    assert_eq!(dver_a.patch, 0);

    assert_eq!(dver_b.major, 2);
    assert_eq!(dver_b.minor, 4);
    assert_eq!(dver_b.patch, 1);
}

#[test]
fn test_vec_of_waypoints_roundtrip() {
    let waypoints = vec![
        Waypoint {
            waypoint_id: 1,
            latitude_deg_e7: 375_000_000,
            longitude_deg_e7: 1_270_000_000,
            altitude_mm: 10_000,
            speed_cms: 600,
            hover_time_ms: 0,
        },
        Waypoint {
            waypoint_id: 2,
            latitude_deg_e7: 375_100_000,
            longitude_deg_e7: 1_270_100_000,
            altitude_mm: 15_000,
            speed_cms: 600,
            hover_time_ms: 10_000,
        },
        Waypoint {
            waypoint_id: 3,
            latitude_deg_e7: 375_200_000,
            longitude_deg_e7: 1_270_200_000,
            altitude_mm: 10_000,
            speed_cms: 400,
            hover_time_ms: 0,
        },
    ];
    let ver = Version::new(1, 0, 0);

    let bytes = encode_versioned_value(&waypoints, ver).expect("encode Vec<Waypoint> failed");
    let (decoded, decoded_ver, consumed): (Vec<Waypoint>, Version, usize) =
        decode_versioned_value::<Vec<Waypoint>>(&bytes).expect("decode Vec<Waypoint> failed");

    assert_eq!(waypoints, decoded);
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded_ver.major, 1);
    assert!(consumed > 0);
    assert!(consumed <= bytes.len());
}

#[test]
fn test_bytes_consumed_less_than_encoded_length() {
    let bat = BatteryStatus {
        drone_id: String::from("UAV-MIKE-13"),
        cell_count: 3,
        voltage_mv: 12_600,
        current_ma: 5_000,
        capacity_mah: 3_000,
        remaining_mah: 2_700,
        temperature_celsius_tenths: 300,
        cycle_count: 10,
        critical: false,
    };
    let ver = Version::new(1, 0, 0);

    let bytes = encode_versioned_value(&bat, ver).expect("encode for consumed check failed");
    let (_, _, consumed): (BatteryStatus, Version, usize) =
        decode_versioned_value::<BatteryStatus>(&bytes).expect("decode for consumed check failed");

    assert!(consumed > 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_mission_type_variants_all_roundtrip() {
    let missions = vec![
        MissionType::Surveillance,
        MissionType::Delivery,
        MissionType::Search,
        MissionType::Mapping,
        MissionType::Inspection,
        MissionType::SwarmFormation,
    ];

    let encoded = encode_to_vec(&missions).expect("encode mission types failed");
    let (decoded, _): (Vec<MissionType>, _) =
        decode_from_slice(&encoded).expect("decode mission types failed");

    assert_eq!(missions, decoded);
    assert_eq!(decoded.len(), 6);
}

#[test]
fn test_flight_plan_v2_patch_version_inspect_mission() {
    let plan = FlightPlanV2 {
        plan_id: 9_999,
        drone_id: String::from("UAV-NOVEMBER-15"),
        mission_type: MissionType::Inspection,
        waypoints: vec![Waypoint {
            waypoint_id: 1,
            latitude_deg_e7: 352_000_000,
            longitude_deg_e7: 1_350_000_000,
            altitude_mm: 5_000,
            speed_cms: 200,
            hover_time_ms: 15_000,
        }],
        home_latitude_deg_e7: 351_900_000,
        home_longitude_deg_e7: 1_349_900_000,
        max_altitude_mm: 50_000,
        active: true,
        priority: 9,
        geofence_radius_m: Some(200),
    };
    let ver = Version::new(2, 0, 3);

    let bytes = encode_versioned_value(&plan, ver).expect("encode inspection plan v2.0.3 failed");
    let (decoded, decoded_ver, _): (FlightPlanV2, Version, usize) =
        decode_versioned_value::<FlightPlanV2>(&bytes)
            .expect("decode inspection plan v2.0.3 failed");

    assert_eq!(plan, decoded);
    assert_eq!(decoded.mission_type, MissionType::Inspection);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 3);
    assert_eq!(decoded.priority, 9);
}
