//! Advanced file I/O tests for OxiCode using an autonomous vehicle / self-driving domain.

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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SensorType {
    LiDAR,
    Radar,
    Camera,
    Ultrasonic,
    GPS,
    IMU,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DrivingMode {
    Manual,
    Assisted,
    Autonomous,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SensorReading {
    sensor_type: SensorType,
    timestamp_us: u64,
    data: Vec<f32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VehicleState {
    vehicle_id: String,
    mode: DrivingMode,
    speed_ms: f32,
    heading_deg: f32,
    position: (f64, f64, f64),
    timestamp_us: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DrivingSession {
    session_id: u64,
    vehicle_id: String,
    readings: Vec<SensorReading>,
    states: Vec<VehicleState>,
    duration_s: f64,
    distance_m: f64,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn temp_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("oxicode_av17_{}_{}", name, std::process::id()))
}

// ---------------------------------------------------------------------------
// Tests (22)
// ---------------------------------------------------------------------------

/// 1. Round-trip a SensorReading (LiDAR) through a temp file.
#[test]
fn test_sensor_reading_lidar_file_roundtrip() {
    let path = temp_path("av17_lidar_reading.bin");
    let reading = SensorReading {
        sensor_type: SensorType::LiDAR,
        timestamp_us: 1_000_000,
        data: vec![1.0, 2.5, 3.7, 100.0],
    };
    encode_to_file(&reading, &path).expect("encode LiDAR reading to file");
    let decoded: SensorReading = decode_from_file(&path).expect("decode LiDAR reading from file");
    std::fs::remove_file(&path).expect("cleanup LiDAR reading file");
    assert_eq!(reading, decoded);
}

/// 2. Round-trip a SensorReading (Radar).
#[test]
fn test_sensor_reading_radar_file_roundtrip() {
    let path = temp_path("av17_radar_reading.bin");
    let reading = SensorReading {
        sensor_type: SensorType::Radar,
        timestamp_us: 2_000_000,
        data: vec![0.1, 0.2, 50.0, 200.0],
    };
    encode_to_file(&reading, &path).expect("encode Radar reading");
    let decoded: SensorReading = decode_from_file(&path).expect("decode Radar reading");
    std::fs::remove_file(&path).expect("cleanup Radar reading file");
    assert_eq!(reading, decoded);
}

/// 3. Round-trip a SensorReading (Camera).
#[test]
fn test_sensor_reading_camera_file_roundtrip() {
    let path = temp_path("av17_camera_reading.bin");
    let reading = SensorReading {
        sensor_type: SensorType::Camera,
        timestamp_us: 3_333_333,
        data: vec![255.0, 128.0, 0.0],
    };
    encode_to_file(&reading, &path).expect("encode Camera reading");
    let decoded: SensorReading = decode_from_file(&path).expect("decode Camera reading");
    std::fs::remove_file(&path).expect("cleanup Camera reading file");
    assert_eq!(reading, decoded);
}

/// 4. Round-trip a SensorReading (Ultrasonic).
#[test]
fn test_sensor_reading_ultrasonic_file_roundtrip() {
    let path = temp_path("av17_ultrasonic_reading.bin");
    let reading = SensorReading {
        sensor_type: SensorType::Ultrasonic,
        timestamp_us: 4_000_000,
        data: vec![0.5, 1.2, 2.8],
    };
    encode_to_file(&reading, &path).expect("encode Ultrasonic reading");
    let decoded: SensorReading = decode_from_file(&path).expect("decode Ultrasonic reading");
    std::fs::remove_file(&path).expect("cleanup Ultrasonic reading file");
    assert_eq!(reading, decoded);
}

/// 5. Round-trip a SensorReading (GPS).
#[test]
fn test_sensor_reading_gps_file_roundtrip() {
    let path = temp_path("av17_gps_reading.bin");
    let reading = SensorReading {
        sensor_type: SensorType::GPS,
        timestamp_us: 5_000_000,
        data: vec![37.7749_f32, -122.4194_f32, 10.0_f32],
    };
    encode_to_file(&reading, &path).expect("encode GPS reading");
    let decoded: SensorReading = decode_from_file(&path).expect("decode GPS reading");
    std::fs::remove_file(&path).expect("cleanup GPS reading file");
    assert_eq!(reading, decoded);
}

/// 6. Round-trip a SensorReading (IMU).
#[test]
fn test_sensor_reading_imu_file_roundtrip() {
    let path = temp_path("av17_imu_reading.bin");
    let reading = SensorReading {
        sensor_type: SensorType::IMU,
        timestamp_us: 6_000_000,
        data: vec![0.01, -0.02, 9.81, 0.0, 0.0, 0.0],
    };
    encode_to_file(&reading, &path).expect("encode IMU reading");
    let decoded: SensorReading = decode_from_file(&path).expect("decode IMU reading");
    std::fs::remove_file(&path).expect("cleanup IMU reading file");
    assert_eq!(reading, decoded);
}

/// 7. Large SensorReading with 200+ floats (LiDAR point cloud simulation).
#[test]
fn test_large_lidar_point_cloud_file_roundtrip() {
    let path = temp_path("av17_large_lidar.bin");
    let data: Vec<f32> = (0..210).map(|i| i as f32 * 0.1_f32).collect();
    let reading = SensorReading {
        sensor_type: SensorType::LiDAR,
        timestamp_us: 7_000_000,
        data,
    };
    encode_to_file(&reading, &path).expect("encode large LiDAR cloud");
    let decoded: SensorReading = decode_from_file(&path).expect("decode large LiDAR cloud");
    std::fs::remove_file(&path).expect("cleanup large LiDAR file");
    assert_eq!(reading, decoded);
    assert_eq!(decoded.data.len(), 210);
}

/// 8. VehicleState in Manual driving mode.
#[test]
fn test_vehicle_state_manual_mode_file_roundtrip() {
    let path = temp_path("av17_vehicle_manual.bin");
    let state = VehicleState {
        vehicle_id: "AV-001".to_string(),
        mode: DrivingMode::Manual,
        speed_ms: 0.0,
        heading_deg: 90.0,
        position: (37.7749, -122.4194, 5.0),
        timestamp_us: 8_000_000,
    };
    encode_to_file(&state, &path).expect("encode VehicleState manual");
    let decoded: VehicleState = decode_from_file(&path).expect("decode VehicleState manual");
    std::fs::remove_file(&path).expect("cleanup VehicleState manual file");
    assert_eq!(state, decoded);
}

/// 9. VehicleState in Assisted driving mode.
#[test]
fn test_vehicle_state_assisted_mode_file_roundtrip() {
    let path = temp_path("av17_vehicle_assisted.bin");
    let state = VehicleState {
        vehicle_id: "AV-002".to_string(),
        mode: DrivingMode::Assisted,
        speed_ms: 8.3,
        heading_deg: 180.0,
        position: (40.7128, -74.0060, 10.0),
        timestamp_us: 9_000_000,
    };
    encode_to_file(&state, &path).expect("encode VehicleState assisted");
    let decoded: VehicleState = decode_from_file(&path).expect("decode VehicleState assisted");
    std::fs::remove_file(&path).expect("cleanup VehicleState assisted file");
    assert_eq!(state, decoded);
}

/// 10. VehicleState in Autonomous driving mode.
#[test]
fn test_vehicle_state_autonomous_mode_file_roundtrip() {
    let path = temp_path("av17_vehicle_autonomous.bin");
    let state = VehicleState {
        vehicle_id: "AV-003".to_string(),
        mode: DrivingMode::Autonomous,
        speed_ms: 25.0,
        heading_deg: 270.0,
        position: (51.5074, -0.1278, 0.0),
        timestamp_us: 10_000_000,
    };
    encode_to_file(&state, &path).expect("encode VehicleState autonomous");
    let decoded: VehicleState = decode_from_file(&path).expect("decode VehicleState autonomous");
    std::fs::remove_file(&path).expect("cleanup VehicleState autonomous file");
    assert_eq!(state, decoded);
}

/// 11. VehicleState in Emergency driving mode.
#[test]
fn test_vehicle_state_emergency_mode_file_roundtrip() {
    let path = temp_path("av17_vehicle_emergency.bin");
    let state = VehicleState {
        vehicle_id: "AV-004".to_string(),
        mode: DrivingMode::Emergency,
        speed_ms: 0.0,
        heading_deg: 45.0,
        position: (48.8566, 2.3522, 35.0),
        timestamp_us: 11_000_000,
    };
    encode_to_file(&state, &path).expect("encode VehicleState emergency");
    let decoded: VehicleState = decode_from_file(&path).expect("decode VehicleState emergency");
    std::fs::remove_file(&path).expect("cleanup VehicleState emergency file");
    assert_eq!(state, decoded);
}

/// 12. Full DrivingSession with multiple readings and states.
#[test]
fn test_driving_session_full_file_roundtrip() {
    let path = temp_path("av17_session_full.bin");
    let session = DrivingSession {
        session_id: 1_234_567_890,
        vehicle_id: "AV-100".to_string(),
        readings: vec![
            SensorReading {
                sensor_type: SensorType::LiDAR,
                timestamp_us: 1_000,
                data: vec![1.0, 2.0, 3.0],
            },
            SensorReading {
                sensor_type: SensorType::GPS,
                timestamp_us: 2_000,
                data: vec![37.0, -122.0, 5.0],
            },
        ],
        states: vec![VehicleState {
            vehicle_id: "AV-100".to_string(),
            mode: DrivingMode::Autonomous,
            speed_ms: 15.0,
            heading_deg: 0.0,
            position: (37.0, -122.0, 5.0),
            timestamp_us: 1_500,
        }],
        duration_s: 3600.0,
        distance_m: 45_000.0,
    };
    encode_to_file(&session, &path).expect("encode full DrivingSession");
    let decoded: DrivingSession = decode_from_file(&path).expect("decode full DrivingSession");
    std::fs::remove_file(&path).expect("cleanup full DrivingSession file");
    assert_eq!(session, decoded);
}

/// 13. Empty DrivingSession (no readings, no states).
#[test]
fn test_driving_session_empty_file_roundtrip() {
    let path = temp_path("av17_session_empty.bin");
    let session = DrivingSession {
        session_id: 0,
        vehicle_id: String::new(),
        readings: vec![],
        states: vec![],
        duration_s: 0.0,
        distance_m: 0.0,
    };
    encode_to_file(&session, &path).expect("encode empty DrivingSession");
    let decoded: DrivingSession = decode_from_file(&path).expect("decode empty DrivingSession");
    std::fs::remove_file(&path).expect("cleanup empty DrivingSession file");
    assert_eq!(session, decoded);
}

/// 14. SensorReading with empty data vector.
#[test]
fn test_sensor_reading_empty_data_file_roundtrip() {
    let path = temp_path("av17_empty_data_reading.bin");
    let reading = SensorReading {
        sensor_type: SensorType::Ultrasonic,
        timestamp_us: 14_000_000,
        data: vec![],
    };
    encode_to_file(&reading, &path).expect("encode empty-data reading");
    let decoded: SensorReading = decode_from_file(&path).expect("decode empty-data reading");
    std::fs::remove_file(&path).expect("cleanup empty-data reading file");
    assert_eq!(reading, decoded);
    assert!(decoded.data.is_empty());
}

/// 15. File overwrite: encode a first value, then overwrite with a different value, verify the
///     second value is recovered.
#[test]
fn test_file_overwrite_produces_new_value() {
    let path = temp_path("av17_overwrite.bin");

    let first = VehicleState {
        vehicle_id: "AV-FIRST".to_string(),
        mode: DrivingMode::Manual,
        speed_ms: 0.0,
        heading_deg: 0.0,
        position: (0.0, 0.0, 0.0),
        timestamp_us: 1,
    };
    encode_to_file(&first, &path).expect("encode first VehicleState");

    let second = VehicleState {
        vehicle_id: "AV-SECOND".to_string(),
        mode: DrivingMode::Autonomous,
        speed_ms: 30.0,
        heading_deg: 135.0,
        position: (52.52, 13.405, 34.0),
        timestamp_us: 2,
    };
    encode_to_file(&second, &path).expect("encode second VehicleState (overwrite)");

    let decoded: VehicleState = decode_from_file(&path).expect("decode overwritten VehicleState");
    std::fs::remove_file(&path).expect("cleanup overwrite file");
    assert_eq!(
        second, decoded,
        "overwritten file should return the second value"
    );
    assert_ne!(
        first, decoded,
        "first value should not be present after overwrite"
    );
}

/// 16. Verify temp file is removed after use (cleanup verification).
#[test]
fn test_temp_file_cleanup_verification() {
    let path = temp_path("av17_cleanup_check.bin");
    let reading = SensorReading {
        sensor_type: SensorType::Camera,
        timestamp_us: 16_000_000,
        data: vec![0.0; 10],
    };
    encode_to_file(&reading, &path).expect("encode for cleanup check");
    assert!(path.exists(), "file should exist before removal");
    std::fs::remove_file(&path).expect("remove file");
    assert!(!path.exists(), "file should be gone after removal");
}

/// 17. Error returned when decoding from a non-existent file.
#[test]
fn test_decode_missing_file_returns_error() {
    let path = temp_path("av17_nonexistent_definitely_missing.bin");
    // Make sure the file does not exist.
    let _ = std::fs::remove_file(&path);
    let result: Result<SensorReading, _> = decode_from_file(&path);
    assert!(
        result.is_err(),
        "decoding a missing file must return an error"
    );
}

/// 18. DrivingSession with a large number of IMU readings (stress scenario).
#[test]
fn test_driving_session_many_imu_readings_file_roundtrip() {
    let path = temp_path("av17_session_many_imu.bin");
    let readings: Vec<SensorReading> = (0..50_u64)
        .map(|i| SensorReading {
            sensor_type: SensorType::IMU,
            timestamp_us: i * 10_000,
            data: vec![
                i as f32 * 0.01_f32,
                -(i as f32) * 0.005_f32,
                9.81_f32,
                0.0_f32,
                0.0_f32,
                0.0_f32,
            ],
        })
        .collect();
    let session = DrivingSession {
        session_id: 999,
        vehicle_id: "AV-IMU".to_string(),
        readings,
        states: vec![],
        duration_s: 0.5,
        distance_m: 0.0,
    };
    encode_to_file(&session, &path).expect("encode many-IMU session");
    let decoded: DrivingSession = decode_from_file(&path).expect("decode many-IMU session");
    std::fs::remove_file(&path).expect("cleanup many-IMU session file");
    assert_eq!(session.readings.len(), decoded.readings.len());
    assert_eq!(session, decoded);
}

/// 19. Encode-to-vec and encode-to-file produce identical bytes.
#[test]
fn test_encode_to_file_matches_encode_to_vec() {
    let path = temp_path("av17_vec_vs_file.bin");
    let state = VehicleState {
        vehicle_id: "AV-VEC".to_string(),
        mode: DrivingMode::Assisted,
        speed_ms: 12.5,
        heading_deg: 60.0,
        position: (35.6762, 139.6503, 40.0),
        timestamp_us: 19_000_000,
    };
    let vec_bytes = encode_to_vec(&state).expect("encode_to_vec VehicleState");
    encode_to_file(&state, &path).expect("encode_to_file VehicleState");
    let file_bytes = std::fs::read(&path).expect("read file bytes");
    std::fs::remove_file(&path).expect("cleanup vec-vs-file file");
    assert_eq!(
        vec_bytes, file_bytes,
        "encode_to_vec and encode_to_file must produce identical bytes"
    );
}

/// 20. decode_from_file and decode_from_slice agree on the same payload.
#[test]
fn test_decode_from_file_matches_decode_from_slice() {
    let path = temp_path("av17_file_vs_slice.bin");
    let reading = SensorReading {
        sensor_type: SensorType::Radar,
        timestamp_us: 20_000_000,
        data: vec![3.14, 2.71, 1.41],
    };
    encode_to_file(&reading, &path).expect("encode for file-vs-slice");
    let from_file: SensorReading = decode_from_file(&path).expect("decode_from_file");
    let raw = std::fs::read(&path).expect("read raw bytes");
    std::fs::remove_file(&path).expect("cleanup file-vs-slice file");
    let (from_slice, _): (SensorReading, _) = decode_from_slice(&raw).expect("decode_from_slice");
    assert_eq!(from_file, from_slice);
}

/// 21. DrivingSession spanning multiple sensor types (heterogeneous readings).
#[test]
fn test_driving_session_heterogeneous_sensors_file_roundtrip() {
    let path = temp_path("av17_session_hetero.bin");
    let session = DrivingSession {
        session_id: 7_777_777,
        vehicle_id: "AV-HETERO".to_string(),
        readings: vec![
            SensorReading {
                sensor_type: SensorType::LiDAR,
                timestamp_us: 100,
                data: (0..50).map(|x| x as f32).collect(),
            },
            SensorReading {
                sensor_type: SensorType::Radar,
                timestamp_us: 200,
                data: vec![10.0, 20.0, 30.0],
            },
            SensorReading {
                sensor_type: SensorType::Camera,
                timestamp_us: 300,
                data: vec![],
            },
            SensorReading {
                sensor_type: SensorType::GPS,
                timestamp_us: 400,
                data: vec![48.8566, 2.3522, 35.0],
            },
            SensorReading {
                sensor_type: SensorType::IMU,
                timestamp_us: 500,
                data: vec![0.0, 0.0, 9.81, 0.1, 0.0, -0.1],
            },
            SensorReading {
                sensor_type: SensorType::Ultrasonic,
                timestamp_us: 600,
                data: vec![0.3, 0.5, 1.1],
            },
        ],
        states: vec![VehicleState {
            vehicle_id: "AV-HETERO".to_string(),
            mode: DrivingMode::Autonomous,
            speed_ms: 20.0,
            heading_deg: 359.9,
            position: (48.8566, 2.3522, 35.0),
            timestamp_us: 350,
        }],
        duration_s: 0.001,
        distance_m: 0.02,
    };
    encode_to_file(&session, &path).expect("encode heterogeneous session");
    let decoded: DrivingSession = decode_from_file(&path).expect("decode heterogeneous session");
    std::fs::remove_file(&path).expect("cleanup heterogeneous session file");
    assert_eq!(session, decoded);
    assert_eq!(decoded.readings.len(), 6);
}

/// 22. Large DrivingSession with 200+ floats per LiDAR reading across multiple states.
#[test]
fn test_driving_session_large_lidar_readings_file_roundtrip() {
    let path = temp_path("av17_session_large_lidar.bin");
    let readings: Vec<SensorReading> = (0..5_u64)
        .map(|i| SensorReading {
            sensor_type: SensorType::LiDAR,
            timestamp_us: i * 100_000,
            data: (0..220).map(|j| (i as f32 * 1000.0) + j as f32).collect(),
        })
        .collect();
    let states: Vec<VehicleState> = (0..5_u64)
        .map(|i| VehicleState {
            vehicle_id: format!("AV-LARGE-{}", i),
            mode: DrivingMode::Autonomous,
            speed_ms: 20.0 + i as f32,
            heading_deg: (i as f32) * 72.0,
            position: (37.7749 + i as f64 * 0.001, -122.4194, 5.0),
            timestamp_us: i * 100_000 + 50_000,
        })
        .collect();
    let session = DrivingSession {
        session_id: 2_222_222_222,
        vehicle_id: "AV-LARGE".to_string(),
        readings,
        states,
        duration_s: 0.5,
        distance_m: 250.0,
    };
    encode_to_file(&session, &path).expect("encode large LiDAR session");
    let decoded: DrivingSession = decode_from_file(&path).expect("decode large LiDAR session");
    std::fs::remove_file(&path).expect("cleanup large LiDAR session file");
    assert_eq!(session.readings.len(), decoded.readings.len());
    for (orig, dec) in session.readings.iter().zip(decoded.readings.iter()) {
        assert_eq!(orig.data.len(), 220);
        assert_eq!(orig, dec);
    }
    assert_eq!(session, decoded);
}
