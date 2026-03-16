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

// ── Domain types: autonomous vehicles / self-driving systems ─────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DrivingMode {
    Manual,
    Assisted,
    SemiAutonomous,
    FullyAutonomous,
    EmergencyStop,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TrafficSignal {
    Red,
    Yellow,
    Green,
    BlinkingRed,
    BlinkingYellow,
    Off,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ObstacleClass {
    Pedestrian,
    Cyclist,
    Vehicle,
    Animal,
    StaticObject,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LanePosition {
    LeftLane,
    CenterLane,
    RightLane,
    MergingLeft,
    MergingRight,
    Shoulder,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EmergencyManeuver {
    HardBrake,
    EvasiveSteerLeft,
    EvasiveSteerRight,
    PullOver,
    Honk,
    HazardLights,
    None,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PerceptionSensor {
    sensor_id: u64,
    lidar_range_cm: u32,
    radar_velocity_cms: i32,
    camera_resolution_mp: u16,
    active: bool,
    confidence_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VehicleState {
    vehicle_id: u64,
    speed_cms: u32,
    heading_deg: u16,
    latitude: f64,
    longitude: f64,
    altitude_m: f32,
    driving_mode: DrivingMode,
    lane: LanePosition,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ObstacleDetection {
    obstacle_id: u64,
    class: ObstacleClass,
    distance_cm: u32,
    relative_velocity_cms: i32,
    azimuth_deg: i16,
    elevation_deg: i16,
    confidence_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PathWaypoint {
    sequence: u32,
    latitude: f64,
    longitude: f64,
    target_speed_cms: u32,
    signal: TrafficSignal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmergencyEvent {
    event_id: u64,
    maneuver: EmergencyManeuver,
    trigger_obstacle_id: Option<u64>,
    vehicle_state_at_trigger: VehicleState,
    resolved: bool,
}

// ── Prop strategies ──────────────────────────────────────────────────────────

fn driving_mode_strategy() -> impl Strategy<Value = DrivingMode> {
    (0u8..5).prop_map(|v| match v {
        0 => DrivingMode::Manual,
        1 => DrivingMode::Assisted,
        2 => DrivingMode::SemiAutonomous,
        3 => DrivingMode::FullyAutonomous,
        _ => DrivingMode::EmergencyStop,
    })
}

fn traffic_signal_strategy() -> impl Strategy<Value = TrafficSignal> {
    (0u8..6).prop_map(|v| match v {
        0 => TrafficSignal::Red,
        1 => TrafficSignal::Yellow,
        2 => TrafficSignal::Green,
        3 => TrafficSignal::BlinkingRed,
        4 => TrafficSignal::BlinkingYellow,
        _ => TrafficSignal::Off,
    })
}

fn obstacle_class_strategy() -> impl Strategy<Value = ObstacleClass> {
    (0u8..6).prop_map(|v| match v {
        0 => ObstacleClass::Pedestrian,
        1 => ObstacleClass::Cyclist,
        2 => ObstacleClass::Vehicle,
        3 => ObstacleClass::Animal,
        4 => ObstacleClass::StaticObject,
        _ => ObstacleClass::Unknown,
    })
}

fn lane_position_strategy() -> impl Strategy<Value = LanePosition> {
    (0u8..6).prop_map(|v| match v {
        0 => LanePosition::LeftLane,
        1 => LanePosition::CenterLane,
        2 => LanePosition::RightLane,
        3 => LanePosition::MergingLeft,
        4 => LanePosition::MergingRight,
        _ => LanePosition::Shoulder,
    })
}

fn emergency_maneuver_strategy() -> impl Strategy<Value = EmergencyManeuver> {
    (0u8..7).prop_map(|v| match v {
        0 => EmergencyManeuver::HardBrake,
        1 => EmergencyManeuver::EvasiveSteerLeft,
        2 => EmergencyManeuver::EvasiveSteerRight,
        3 => EmergencyManeuver::PullOver,
        4 => EmergencyManeuver::Honk,
        5 => EmergencyManeuver::HazardLights,
        _ => EmergencyManeuver::None,
    })
}

fn perception_sensor_strategy() -> impl Strategy<Value = PerceptionSensor> {
    (
        any::<u64>(),
        any::<u32>(),
        any::<i32>(),
        any::<u16>(),
        any::<bool>(),
        any::<u8>(),
    )
        .prop_map(
            |(
                sensor_id,
                lidar_range_cm,
                radar_velocity_cms,
                camera_resolution_mp,
                active,
                confidence_pct,
            )| {
                PerceptionSensor {
                    sensor_id,
                    lidar_range_cm,
                    radar_velocity_cms,
                    camera_resolution_mp,
                    active,
                    confidence_pct,
                }
            },
        )
}

fn vehicle_state_strategy() -> impl Strategy<Value = VehicleState> {
    (
        any::<u64>(),
        any::<u32>(),
        any::<u16>(),
        any::<f64>(),
        any::<f64>(),
        any::<f32>(),
        driving_mode_strategy(),
        lane_position_strategy(),
    )
        .prop_map(
            |(
                vehicle_id,
                speed_cms,
                heading_deg,
                latitude,
                longitude,
                altitude_m,
                driving_mode,
                lane,
            )| {
                VehicleState {
                    vehicle_id,
                    speed_cms,
                    heading_deg,
                    latitude,
                    longitude,
                    altitude_m,
                    driving_mode,
                    lane,
                }
            },
        )
}

fn obstacle_detection_strategy() -> impl Strategy<Value = ObstacleDetection> {
    (
        any::<u64>(),
        obstacle_class_strategy(),
        any::<u32>(),
        any::<i32>(),
        any::<i16>(),
        any::<i16>(),
        any::<u8>(),
    )
        .prop_map(
            |(
                obstacle_id,
                class,
                distance_cm,
                relative_velocity_cms,
                azimuth_deg,
                elevation_deg,
                confidence_pct,
            )| {
                ObstacleDetection {
                    obstacle_id,
                    class,
                    distance_cm,
                    relative_velocity_cms,
                    azimuth_deg,
                    elevation_deg,
                    confidence_pct,
                }
            },
        )
}

fn path_waypoint_strategy() -> impl Strategy<Value = PathWaypoint> {
    (
        any::<u32>(),
        any::<f64>(),
        any::<f64>(),
        any::<u32>(),
        traffic_signal_strategy(),
    )
        .prop_map(
            |(sequence, latitude, longitude, target_speed_cms, signal)| PathWaypoint {
                sequence,
                latitude,
                longitude,
                target_speed_cms,
                signal,
            },
        )
}

fn emergency_event_strategy() -> impl Strategy<Value = EmergencyEvent> {
    (
        any::<u64>(),
        emergency_maneuver_strategy(),
        prop::option::of(any::<u64>()),
        vehicle_state_strategy(),
        any::<bool>(),
    )
        .prop_map(
            |(event_id, maneuver, trigger_obstacle_id, vehicle_state_at_trigger, resolved)| {
                EmergencyEvent {
                    event_id,
                    maneuver,
                    trigger_obstacle_id,
                    vehicle_state_at_trigger,
                    resolved,
                }
            },
        )
}

// ── 22 property-based tests ──────────────────────────────────────────────────

proptest! {
    #[test]
    fn test_f64_coordinate_roundtrip(val in any::<f64>()) {
        let encoded = encode_to_vec(&val).expect("encode f64 coordinate failed");
        let (decoded, _): (f64, usize) = decode_from_slice(&encoded).expect("decode f64 coordinate failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_perception_sensor_roundtrip(sensor in perception_sensor_strategy()) {
        let encoded = encode_to_vec(&sensor).expect("encode PerceptionSensor failed");
        let (decoded, _): (PerceptionSensor, usize) = decode_from_slice(&encoded).expect("decode PerceptionSensor failed");
        prop_assert_eq!(sensor, decoded);
    }

    #[test]
    fn test_vehicle_state_roundtrip(state in vehicle_state_strategy()) {
        let encoded = encode_to_vec(&state).expect("encode VehicleState failed");
        let (decoded, _): (VehicleState, usize) = decode_from_slice(&encoded).expect("decode VehicleState failed");
        prop_assert_eq!(state, decoded);
    }

    #[test]
    fn test_obstacle_detection_roundtrip(obs in obstacle_detection_strategy()) {
        let encoded = encode_to_vec(&obs).expect("encode ObstacleDetection failed");
        let (decoded, _): (ObstacleDetection, usize) = decode_from_slice(&encoded).expect("decode ObstacleDetection failed");
        prop_assert_eq!(obs, decoded);
    }

    #[test]
    fn test_driving_mode_enum_roundtrip(index in 0u8..5) {
        let mode = match index {
            0 => DrivingMode::Manual,
            1 => DrivingMode::Assisted,
            2 => DrivingMode::SemiAutonomous,
            3 => DrivingMode::FullyAutonomous,
            _ => DrivingMode::EmergencyStop,
        };
        let encoded = encode_to_vec(&mode).expect("encode DrivingMode failed");
        let (decoded, _): (DrivingMode, usize) = decode_from_slice(&encoded).expect("decode DrivingMode failed");
        prop_assert_eq!(mode, decoded);
    }

    #[test]
    fn test_traffic_signal_enum_roundtrip(index in 0u8..6) {
        let signal = match index {
            0 => TrafficSignal::Red,
            1 => TrafficSignal::Yellow,
            2 => TrafficSignal::Green,
            3 => TrafficSignal::BlinkingRed,
            4 => TrafficSignal::BlinkingYellow,
            _ => TrafficSignal::Off,
        };
        let encoded = encode_to_vec(&signal).expect("encode TrafficSignal failed");
        let (decoded, _): (TrafficSignal, usize) = decode_from_slice(&encoded).expect("decode TrafficSignal failed");
        prop_assert_eq!(signal, decoded);
    }

    #[test]
    fn test_vec_obstacle_detections_roundtrip(obstacles in prop::collection::vec(obstacle_detection_strategy(), 0..8)) {
        let encoded = encode_to_vec(&obstacles).expect("encode Vec<ObstacleDetection> failed");
        let (decoded, _): (Vec<ObstacleDetection>, usize) = decode_from_slice(&encoded).expect("decode Vec<ObstacleDetection> failed");
        prop_assert_eq!(obstacles, decoded);
    }

    #[test]
    fn test_vec_path_waypoints_roundtrip(waypoints in prop::collection::vec(path_waypoint_strategy(), 0..10)) {
        let encoded = encode_to_vec(&waypoints).expect("encode Vec<PathWaypoint> failed");
        let (decoded, _): (Vec<PathWaypoint>, usize) = decode_from_slice(&encoded).expect("decode Vec<PathWaypoint> failed");
        prop_assert_eq!(waypoints, decoded);
    }

    #[test]
    fn test_option_vehicle_state_roundtrip(maybe_state in prop::option::of(vehicle_state_strategy())) {
        let encoded = encode_to_vec(&maybe_state).expect("encode Option<VehicleState> failed");
        let (decoded, _): (Option<VehicleState>, usize) = decode_from_slice(&encoded).expect("decode Option<VehicleState> failed");
        prop_assert_eq!(maybe_state, decoded);
    }

    #[test]
    fn test_option_obstacle_detection_roundtrip(maybe_obs in prop::option::of(obstacle_detection_strategy())) {
        let encoded = encode_to_vec(&maybe_obs).expect("encode Option<ObstacleDetection> failed");
        let (decoded, _): (Option<ObstacleDetection>, usize) = decode_from_slice(&encoded).expect("decode Option<ObstacleDetection> failed");
        prop_assert_eq!(maybe_obs, decoded);
    }

    #[test]
    fn test_perception_sensor_consumed_bytes(sensor in perception_sensor_strategy()) {
        let encoded = encode_to_vec(&sensor).expect("encode PerceptionSensor for bytes check failed");
        let (_, consumed): (PerceptionSensor, usize) = decode_from_slice(&encoded).expect("decode PerceptionSensor for bytes check failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_vehicle_state_consumed_bytes(state in vehicle_state_strategy()) {
        let encoded = encode_to_vec(&state).expect("encode VehicleState for bytes check failed");
        let (_, consumed): (VehicleState, usize) = decode_from_slice(&encoded).expect("decode VehicleState for bytes check failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_vehicle_state_encode_deterministic(state in vehicle_state_strategy()) {
        let encoded_first = encode_to_vec(&state).expect("first encode VehicleState failed");
        let encoded_second = encode_to_vec(&state).expect("second encode VehicleState failed");
        prop_assert_eq!(encoded_first, encoded_second);
    }

    #[test]
    fn test_perception_sensor_encode_deterministic(sensor in perception_sensor_strategy()) {
        let encoded_first = encode_to_vec(&sensor).expect("first encode PerceptionSensor failed");
        let encoded_second = encode_to_vec(&sensor).expect("second encode PerceptionSensor failed");
        prop_assert_eq!(encoded_first, encoded_second);
    }

    #[test]
    fn test_emergency_event_nested_struct_roundtrip(event in emergency_event_strategy()) {
        let encoded = encode_to_vec(&event).expect("encode EmergencyEvent failed");
        let (decoded, consumed): (EmergencyEvent, usize) = decode_from_slice(&encoded).expect("decode EmergencyEvent failed");
        prop_assert_eq!(&event, &decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_emergency_maneuver_all_variants(index in 0u8..7) {
        let maneuver = match index {
            0 => EmergencyManeuver::HardBrake,
            1 => EmergencyManeuver::EvasiveSteerLeft,
            2 => EmergencyManeuver::EvasiveSteerRight,
            3 => EmergencyManeuver::PullOver,
            4 => EmergencyManeuver::Honk,
            5 => EmergencyManeuver::HazardLights,
            _ => EmergencyManeuver::None,
        };
        let encoded = encode_to_vec(&maneuver).expect("encode EmergencyManeuver failed");
        let (decoded, _): (EmergencyManeuver, usize) = decode_from_slice(&encoded).expect("decode EmergencyManeuver failed");
        prop_assert_eq!(maneuver, decoded);
    }

    #[test]
    fn test_distinct_vehicle_states_bytes_reflect_inequality(
        state_a in vehicle_state_strategy(),
        state_b in vehicle_state_strategy()
    ) {
        let encoded_a = encode_to_vec(&state_a).expect("encode VehicleState A failed");
        let encoded_b = encode_to_vec(&state_b).expect("encode VehicleState B failed");
        if state_a == state_b {
            prop_assert_eq!(&encoded_a, &encoded_b);
        } else {
            prop_assert_ne!(&encoded_a, &encoded_b);
        }
    }

    #[test]
    fn test_i32_sensor_velocity_roundtrip(val in any::<i32>()) {
        let encoded = encode_to_vec(&val).expect("encode i32 sensor velocity failed");
        let (decoded, _): (i32, usize) = decode_from_slice(&encoded).expect("decode i32 sensor velocity failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_u64_vehicle_id_roundtrip(val in any::<u64>()) {
        let encoded = encode_to_vec(&val).expect("encode u64 vehicle id failed");
        let (decoded, _): (u64, usize) = decode_from_slice(&encoded).expect("decode u64 vehicle id failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_bool_sensor_active_roundtrip(val in any::<bool>()) {
        let encoded = encode_to_vec(&val).expect("encode bool sensor active failed");
        let (decoded, _): (bool, usize) = decode_from_slice(&encoded).expect("decode bool sensor active failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_string_vehicle_label_roundtrip(val in "\\PC*") {
        let encoded = encode_to_vec(&val).expect("encode String vehicle label failed");
        let (decoded, _): (String, usize) = decode_from_slice(&encoded).expect("decode String vehicle label failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_f32_altitude_roundtrip(val in any::<f32>()) {
        let encoded = encode_to_vec(&val).expect("encode f32 altitude failed");
        let (decoded, _): (f32, usize) = decode_from_slice(&encoded).expect("decode f32 altitude failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }
}
