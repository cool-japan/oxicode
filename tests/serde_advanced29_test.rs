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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum SensorType {
    Lidar,
    Radar,
    Camera,
    Ultrasonic,
    Gps,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ObstacleClass {
    Pedestrian,
    Vehicle,
    Cyclist,
    StaticObject,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SensorReading {
    sensor_id: u32,
    sensor_type: SensorType,
    timestamp_us: u64,
    range_cm: u32,
    confidence_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Obstacle {
    obstacle_id: u32,
    class: ObstacleClass,
    x_cm: i32,
    y_cm: i32,
    velocity_cms: i32,
    width_cm: u32,
    height_cm: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WaypointNav {
    x_cm: i64,
    y_cm: i64,
    speed_cms: u32,
    heading_mdeg: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VehicleState {
    vehicle_id: u64,
    speed_cms: i32,
    heading_mdeg: u32,
    x_cm: i64,
    y_cm: i64,
    obstacles: Vec<Obstacle>,
    path: Vec<WaypointNav>,
}

// Test 1: SensorType::Lidar variant roundtrip
#[test]
fn test_sensor_type_lidar() {
    let cfg = config::standard();
    let val = SensorType::Lidar;
    let bytes = encode_to_vec(&val, cfg).expect("encode SensorType::Lidar");
    let (decoded, _): (SensorType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SensorType::Lidar");
    assert_eq!(val, decoded);
}

// Test 2: SensorType::Radar variant roundtrip
#[test]
fn test_sensor_type_radar() {
    let cfg = config::standard();
    let val = SensorType::Radar;
    let bytes = encode_to_vec(&val, cfg).expect("encode SensorType::Radar");
    let (decoded, _): (SensorType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SensorType::Radar");
    assert_eq!(val, decoded);
}

// Test 3: SensorType::Camera variant roundtrip
#[test]
fn test_sensor_type_camera() {
    let cfg = config::standard();
    let val = SensorType::Camera;
    let bytes = encode_to_vec(&val, cfg).expect("encode SensorType::Camera");
    let (decoded, _): (SensorType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SensorType::Camera");
    assert_eq!(val, decoded);
}

// Test 4: SensorType::Ultrasonic variant roundtrip
#[test]
fn test_sensor_type_ultrasonic() {
    let cfg = config::standard();
    let val = SensorType::Ultrasonic;
    let bytes = encode_to_vec(&val, cfg).expect("encode SensorType::Ultrasonic");
    let (decoded, _): (SensorType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SensorType::Ultrasonic");
    assert_eq!(val, decoded);
}

// Test 5: SensorType::Gps variant roundtrip
#[test]
fn test_sensor_type_gps() {
    let cfg = config::standard();
    let val = SensorType::Gps;
    let bytes = encode_to_vec(&val, cfg).expect("encode SensorType::Gps");
    let (decoded, _): (SensorType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SensorType::Gps");
    assert_eq!(val, decoded);
}

// Test 6: ObstacleClass::Pedestrian variant roundtrip
#[test]
fn test_obstacle_class_pedestrian() {
    let cfg = config::standard();
    let val = ObstacleClass::Pedestrian;
    let bytes = encode_to_vec(&val, cfg).expect("encode ObstacleClass::Pedestrian");
    let (decoded, _): (ObstacleClass, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ObstacleClass::Pedestrian");
    assert_eq!(val, decoded);
}

// Test 7: ObstacleClass::Vehicle variant roundtrip
#[test]
fn test_obstacle_class_vehicle() {
    let cfg = config::standard();
    let val = ObstacleClass::Vehicle;
    let bytes = encode_to_vec(&val, cfg).expect("encode ObstacleClass::Vehicle");
    let (decoded, _): (ObstacleClass, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ObstacleClass::Vehicle");
    assert_eq!(val, decoded);
}

// Test 8: ObstacleClass::Cyclist variant roundtrip
#[test]
fn test_obstacle_class_cyclist() {
    let cfg = config::standard();
    let val = ObstacleClass::Cyclist;
    let bytes = encode_to_vec(&val, cfg).expect("encode ObstacleClass::Cyclist");
    let (decoded, _): (ObstacleClass, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ObstacleClass::Cyclist");
    assert_eq!(val, decoded);
}

// Test 9: ObstacleClass::StaticObject variant roundtrip
#[test]
fn test_obstacle_class_static_object() {
    let cfg = config::standard();
    let val = ObstacleClass::StaticObject;
    let bytes = encode_to_vec(&val, cfg).expect("encode ObstacleClass::StaticObject");
    let (decoded, _): (ObstacleClass, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ObstacleClass::StaticObject");
    assert_eq!(val, decoded);
}

// Test 10: ObstacleClass::Unknown variant roundtrip
#[test]
fn test_obstacle_class_unknown() {
    let cfg = config::standard();
    let val = ObstacleClass::Unknown;
    let bytes = encode_to_vec(&val, cfg).expect("encode ObstacleClass::Unknown");
    let (decoded, _): (ObstacleClass, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ObstacleClass::Unknown");
    assert_eq!(val, decoded);
}

// Test 11: SensorReading roundtrip standard config
#[test]
fn test_sensor_reading_roundtrip_standard() {
    let cfg = config::standard();
    let val = SensorReading {
        sensor_id: 42,
        sensor_type: SensorType::Lidar,
        timestamp_us: 1_700_000_000_000_u64,
        range_cm: 5000,
        confidence_pct: 95,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode SensorReading");
    let (decoded, _): (SensorReading, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SensorReading");
    assert_eq!(val, decoded);
}

// Test 12: Obstacle roundtrip
#[test]
fn test_obstacle_roundtrip() {
    let cfg = config::standard();
    let val = Obstacle {
        obstacle_id: 7,
        class: ObstacleClass::Pedestrian,
        x_cm: -300,
        y_cm: 1500,
        velocity_cms: -50,
        width_cm: 60,
        height_cm: 170,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode Obstacle");
    let (decoded, _): (Obstacle, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Obstacle");
    assert_eq!(val, decoded);
}

// Test 13: WaypointNav roundtrip with consumed bytes assertion
#[test]
fn test_waypoint_nav_roundtrip() {
    let cfg = config::standard();
    let val = WaypointNav {
        x_cm: 100_000_i64,
        y_cm: -50_000_i64,
        speed_cms: 1388,
        heading_mdeg: 90_000,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode WaypointNav");
    let (decoded, consumed): (WaypointNav, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode WaypointNav");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes should equal total encoded length"
    );
}

// Test 14: VehicleState with empty obstacles and path
#[test]
fn test_vehicle_state_empty_obstacles_and_path() {
    let cfg = config::standard();
    let val = VehicleState {
        vehicle_id: 1001,
        speed_cms: 0,
        heading_mdeg: 0,
        x_cm: 0,
        y_cm: 0,
        obstacles: vec![],
        path: vec![],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode VehicleState empty");
    let (decoded, _): (VehicleState, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VehicleState empty");
    assert_eq!(val, decoded);
}

// Test 15: VehicleState with multiple obstacles
#[test]
fn test_vehicle_state_multiple_obstacles() {
    let cfg = config::standard();
    let obstacles = vec![
        Obstacle {
            obstacle_id: 1,
            class: ObstacleClass::Vehicle,
            x_cm: 500,
            y_cm: 200,
            velocity_cms: 100,
            width_cm: 200,
            height_cm: 150,
        },
        Obstacle {
            obstacle_id: 2,
            class: ObstacleClass::Pedestrian,
            x_cm: -100,
            y_cm: 300,
            velocity_cms: 10,
            width_cm: 60,
            height_cm: 170,
        },
        Obstacle {
            obstacle_id: 3,
            class: ObstacleClass::Cyclist,
            x_cm: 800,
            y_cm: -100,
            velocity_cms: 40,
            width_cm: 80,
            height_cm: 180,
        },
    ];
    let val = VehicleState {
        vehicle_id: 2002,
        speed_cms: 1000,
        heading_mdeg: 45_000,
        x_cm: 10_000,
        y_cm: 20_000,
        obstacles,
        path: vec![],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode VehicleState multiple obstacles");
    let (decoded, _): (VehicleState, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VehicleState multiple obstacles");
    assert_eq!(val, decoded);
}

// Test 16: VehicleState with long path (10 waypoints)
#[test]
fn test_vehicle_state_long_path_10_waypoints() {
    let cfg = config::standard();
    let path: Vec<WaypointNav> = (0..10)
        .map(|i| WaypointNav {
            x_cm: i as i64 * 5000,
            y_cm: i as i64 * 2000,
            speed_cms: 800 + i as u32 * 50,
            heading_mdeg: (i as u32 * 18_000) % 360_000,
        })
        .collect();
    let val = VehicleState {
        vehicle_id: 3003,
        speed_cms: 800,
        heading_mdeg: 0,
        x_cm: 0,
        y_cm: 0,
        obstacles: vec![],
        path,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode VehicleState 10 waypoints");
    let (decoded, _): (VehicleState, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VehicleState 10 waypoints");
    assert_eq!(val, decoded);
}

// Test 17: Vec<SensorReading> roundtrip
#[test]
fn test_vec_sensor_reading_roundtrip() {
    let cfg = config::standard();
    let val = vec![
        SensorReading {
            sensor_id: 1,
            sensor_type: SensorType::Lidar,
            timestamp_us: 1_000_000,
            range_cm: 3000,
            confidence_pct: 90,
        },
        SensorReading {
            sensor_id: 2,
            sensor_type: SensorType::Radar,
            timestamp_us: 1_000_050,
            range_cm: 12000,
            confidence_pct: 75,
        },
        SensorReading {
            sensor_id: 3,
            sensor_type: SensorType::Camera,
            timestamp_us: 1_000_033,
            range_cm: 800,
            confidence_pct: 85,
        },
        SensorReading {
            sensor_id: 4,
            sensor_type: SensorType::Ultrasonic,
            timestamp_us: 1_000_010,
            range_cm: 50,
            confidence_pct: 99,
        },
        SensorReading {
            sensor_id: 5,
            sensor_type: SensorType::Gps,
            timestamp_us: 1_000_000,
            range_cm: 0,
            confidence_pct: 70,
        },
    ];
    let bytes = encode_to_vec(&val, cfg).expect("encode Vec<SensorReading>");
    let (decoded, _): (Vec<SensorReading>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<SensorReading>");
    assert_eq!(val, decoded);
}

// Test 18: big_endian config roundtrip
#[test]
fn test_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let val = SensorReading {
        sensor_id: 99,
        sensor_type: SensorType::Radar,
        timestamp_us: 9_999_999_999_u64,
        range_cm: 25000,
        confidence_pct: 60,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode big_endian SensorReading");
    let (decoded, _): (SensorReading, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode big_endian SensorReading");
    assert_eq!(val, decoded);
}

// Test 19: fixed_int config roundtrip
#[test]
fn test_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = Obstacle {
        obstacle_id: 55,
        class: ObstacleClass::Cyclist,
        x_cm: i32::MAX,
        y_cm: i32::MIN,
        velocity_cms: -1000,
        width_cm: u32::MAX,
        height_cm: 200,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode fixed_int Obstacle");
    let (decoded, _): (Obstacle, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode fixed_int Obstacle");
    assert_eq!(val, decoded);
}

// Test 20: big_endian + fixed_int combined config roundtrip
#[test]
fn test_big_endian_fixed_int_combined() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let val = VehicleState {
        vehicle_id: u64::MAX,
        speed_cms: -5000,
        heading_mdeg: 359_999,
        x_cm: i64::MAX / 2,
        y_cm: i64::MIN / 2,
        obstacles: vec![Obstacle {
            obstacle_id: 1,
            class: ObstacleClass::Vehicle,
            x_cm: 300,
            y_cm: 400,
            velocity_cms: 500,
            width_cm: 200,
            height_cm: 150,
        }],
        path: vec![WaypointNav {
            x_cm: 100_000,
            y_cm: 200_000,
            speed_cms: 1000,
            heading_mdeg: 180_000,
        }],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode big_endian+fixed_int VehicleState");
    let (decoded, _): (VehicleState, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode big_endian+fixed_int VehicleState");
    assert_eq!(val, decoded);
}

// Test 22: large obstacle count — 20 obstacles roundtrip
#[test]
fn test_large_obstacle_count_20() {
    let cfg = config::standard();
    let classes = [
        ObstacleClass::Pedestrian,
        ObstacleClass::Vehicle,
        ObstacleClass::Cyclist,
        ObstacleClass::StaticObject,
        ObstacleClass::Unknown,
    ];
    let obstacles: Vec<Obstacle> = (0..20)
        .map(|i| Obstacle {
            obstacle_id: i as u32,
            class: classes[i % 5].clone(),
            x_cm: (i as i32 - 10) * 200,
            y_cm: (i as i32) * 150,
            velocity_cms: (i as i32 % 5) * 100 - 200,
            width_cm: 50 + i as u32 * 10,
            height_cm: 100 + i as u32 * 5,
        })
        .collect();
    let val = VehicleState {
        vehicle_id: 4004,
        speed_cms: 1388,
        heading_mdeg: 30_000,
        x_cm: 500_000,
        y_cm: 750_000,
        obstacles,
        path: vec![],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode VehicleState 20 obstacles");
    let (decoded, _): (VehicleState, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VehicleState 20 obstacles");
    assert_eq!(val.obstacles.len(), 20);
    assert_eq!(val, decoded);
}

// Test 23: path planning with 15 waypoints roundtrip
#[test]
fn test_path_planning_15_waypoints() {
    let cfg = config::standard();
    let path: Vec<WaypointNav> = (0..15)
        .map(|i| WaypointNav {
            x_cm: (i as i64) * 10_000 - 70_000,
            y_cm: (i as i64 * i as i64) * 500,
            speed_cms: 500 + (i as u32 % 5) * 200,
            heading_mdeg: (i as u32 * 24_000) % 360_000,
        })
        .collect();
    let val = VehicleState {
        vehicle_id: 5005,
        speed_cms: 500,
        heading_mdeg: 0,
        x_cm: -70_000,
        y_cm: 0,
        obstacles: vec![Obstacle {
            obstacle_id: 100,
            class: ObstacleClass::StaticObject,
            x_cm: 20_000,
            y_cm: 30_000,
            velocity_cms: 0,
            width_cm: 500,
            height_cm: 300,
        }],
        path,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode VehicleState 15 waypoints");
    let (decoded, _): (VehicleState, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode VehicleState 15 waypoints");
    assert_eq!(val.path.len(), 15);
    assert_eq!(val, decoded);
}
