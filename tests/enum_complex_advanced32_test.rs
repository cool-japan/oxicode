//! Advanced tests for autonomous vehicle manufacturing QA system domain types.
//! 22 test functions covering sensor fusion, path planning, V2X, safety levels, and more.

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

// ---------------------------------------------------------------------------
// Test 1: Sensor fusion events
// ---------------------------------------------------------------------------
#[test]
fn test_av_sensor_fusion_event() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum SensorFusionEvent {
        LidarDetection {
            object_id: u64,
            distance_cm: u32,
            confidence: u8,
        },
        CameraFrame {
            timestamp_ns: u64,
            width: u16,
            height: u16,
        },
        RadarReturn {
            velocity_mps: i32,
            azimuth_deg: i16,
        },
    }

    let cases = vec![
        SensorFusionEvent::LidarDetection {
            object_id: 42,
            distance_cm: 1500,
            confidence: 95,
        },
        SensorFusionEvent::CameraFrame {
            timestamp_ns: 1_700_000_000_000,
            width: 1920,
            height: 1080,
        },
        SensorFusionEvent::RadarReturn {
            velocity_mps: -12,
            azimuth_deg: 45,
        },
    ];
    for val in cases {
        let bytes = encode_to_vec(&val).expect("encode sensor fusion event");
        let (decoded, _): (SensorFusionEvent, usize) =
            decode_from_slice(&bytes).expect("decode sensor fusion event");
        assert_eq!(val, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 2: LIDAR point cloud classification
// ---------------------------------------------------------------------------
#[test]
fn test_av_lidar_point_classification() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum PointClassification {
        Ground,
        Vegetation { height_cm: u16 },
        Building { floors_estimated: u8 },
        Vehicle { length_cm: u16, width_cm: u16 },
        Pedestrian { heading_deg: u16 },
        Unknown { intensity: u16 },
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct LidarPoint {
        x_mm: i32,
        y_mm: i32,
        z_mm: i32,
        return_number: u8,
        classification: PointClassification,
    }

    let point = LidarPoint {
        x_mm: -34_521,
        y_mm: 128_900,
        z_mm: 1_450,
        return_number: 1,
        classification: PointClassification::Vehicle {
            length_cm: 450,
            width_cm: 185,
        },
    };
    let bytes = encode_to_vec(&point).expect("encode lidar point");
    let (decoded, _): (LidarPoint, usize) = decode_from_slice(&bytes).expect("decode lidar point");
    assert_eq!(point, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Lane detection results
// ---------------------------------------------------------------------------
#[test]
fn test_av_lane_detection_result() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum LaneMarkingType {
        SolidWhite,
        DashedWhite,
        SolidYellow,
        DashedYellow,
        DoubleSolid,
        BotsDots,
        NoMarking,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum LaneDetectionResult {
        Detected {
            left_marking: LaneMarkingType,
            right_marking: LaneMarkingType,
            lane_width_cm: u16,
            curvature_radius_m: u32,
            confidence_pct: u8,
        },
        PartialDetection {
            side_detected: bool,
            marking: LaneMarkingType,
            confidence_pct: u8,
        },
        NoLane {
            reason_code: u16,
        },
    }

    let val = LaneDetectionResult::Detected {
        left_marking: LaneMarkingType::SolidWhite,
        right_marking: LaneMarkingType::DashedWhite,
        lane_width_cm: 365,
        curvature_radius_m: 500,
        confidence_pct: 88,
    };
    let bytes = encode_to_vec(&val).expect("encode lane detection");
    let (decoded, _): (LaneDetectionResult, usize) =
        decode_from_slice(&bytes).expect("decode lane detection");
    assert_eq!(val, decoded);

    let partial = LaneDetectionResult::PartialDetection {
        side_detected: true,
        marking: LaneMarkingType::DoubleSolid,
        confidence_pct: 62,
    };
    let bytes2 = encode_to_vec(&partial).expect("encode partial lane");
    let (decoded2, _): (LaneDetectionResult, usize) =
        decode_from_slice(&bytes2).expect("decode partial lane");
    assert_eq!(partial, decoded2);
}

// ---------------------------------------------------------------------------
// Test 4: Obstacle classification
// ---------------------------------------------------------------------------
#[test]
fn test_av_obstacle_classification() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum ObstacleThreatLevel {
        None,
        Low,
        Medium,
        High,
        Critical,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum ObstacleClassification {
        PassengerCar {
            estimated_speed_kmh: u16,
            threat: ObstacleThreatLevel,
        },
        Truck {
            axle_count: u8,
            estimated_length_m: u8,
            threat: ObstacleThreatLevel,
        },
        Cyclist {
            helmet_detected: bool,
            threat: ObstacleThreatLevel,
        },
        Pedestrian {
            age_group: u8,
            carrying_object: bool,
            threat: ObstacleThreatLevel,
        },
        Animal {
            size_class: u8,
            threat: ObstacleThreatLevel,
        },
        Debris {
            width_cm: u16,
            height_cm: u16,
            threat: ObstacleThreatLevel,
        },
        Construction {
            cone_count: u16,
            barricade: bool,
            threat: ObstacleThreatLevel,
        },
    }

    let obstacle = ObstacleClassification::Pedestrian {
        age_group: 2,
        carrying_object: true,
        threat: ObstacleThreatLevel::High,
    };
    let bytes = encode_to_vec(&obstacle).expect("encode obstacle");
    let (decoded, _): (ObstacleClassification, usize) =
        decode_from_slice(&bytes).expect("decode obstacle");
    assert_eq!(obstacle, decoded);

    let debris = ObstacleClassification::Debris {
        width_cm: 120,
        height_cm: 35,
        threat: ObstacleThreatLevel::Medium,
    };
    let bytes2 = encode_to_vec(&debris).expect("encode debris obstacle");
    let (decoded2, _): (ObstacleClassification, usize) =
        decode_from_slice(&bytes2).expect("decode debris");
    assert_eq!(debris, decoded2);
}

// ---------------------------------------------------------------------------
// Test 5: Path planning waypoints
// ---------------------------------------------------------------------------
#[test]
fn test_av_path_planning_waypoint() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum WaypointAction {
        Continue,
        SlowDown { target_speed_kmh: u16 },
        Stop { hold_duration_ms: u32 },
        LaneChangeLeft { urgency: u8 },
        LaneChangeRight { urgency: u8 },
        UTurn,
        MergeOnto { lane_id: u8 },
        YieldAndProceed { gap_threshold_ms: u32 },
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Waypoint {
        sequence: u32,
        lat_microdeg: i64,
        lon_microdeg: i64,
        altitude_cm: i32,
        action: WaypointAction,
    }

    let wp = Waypoint {
        sequence: 17,
        lat_microdeg: 37_774_929_000,
        lon_microdeg: -122_419_416_000,
        altitude_cm: 1_520,
        action: WaypointAction::LaneChangeLeft { urgency: 3 },
    };
    let bytes = encode_to_vec(&wp).expect("encode waypoint");
    let (decoded, _): (Waypoint, usize) = decode_from_slice(&bytes).expect("decode waypoint");
    assert_eq!(wp, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Vehicle state machine
// ---------------------------------------------------------------------------
#[test]
fn test_av_vehicle_state_machine() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum DrivingMode {
        Manual,
        AssistedSteering,
        AdaptiveCruise { set_speed_kmh: u16 },
        HighwayAutopilot { max_speed_kmh: u16 },
        UrbanAutopilot,
        Valet,
        EmergencyStop { trigger_code: u16 },
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum VehicleState {
        Parked,
        Initializing {
            subsystems_ready: u8,
            total_subsystems: u8,
        },
        ReadyToDrive {
            mode: DrivingMode,
        },
        Driving {
            mode: DrivingMode,
            speed_kmh: u16,
        },
        Degraded {
            fault_code: u32,
            fallback_mode: DrivingMode,
        },
        EmergencyStopping {
            deceleration_mps2: u16,
        },
        Shutdown {
            reason_code: u16,
        },
    }

    let state = VehicleState::Driving {
        mode: DrivingMode::HighwayAutopilot { max_speed_kmh: 130 },
        speed_kmh: 112,
    };
    let bytes = encode_to_vec(&state).expect("encode vehicle state");
    let (decoded, _): (VehicleState, usize) =
        decode_from_slice(&bytes).expect("decode vehicle state");
    assert_eq!(state, decoded);

    let degraded = VehicleState::Degraded {
        fault_code: 0xDEAD_BEEF,
        fallback_mode: DrivingMode::AssistedSteering,
    };
    let bytes2 = encode_to_vec(&degraded).expect("encode degraded state");
    let (decoded2, _): (VehicleState, usize) =
        decode_from_slice(&bytes2).expect("decode degraded state");
    assert_eq!(degraded, decoded2);
}

// ---------------------------------------------------------------------------
// Test 7: Brake and throttle commands
// ---------------------------------------------------------------------------
#[test]
fn test_av_brake_throttle_command() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum BrakeCommand {
        Release,
        Apply { pressure_kpa: u16 },
        EmergencyBrake { source_id: u8 },
        AbsActivation { wheel_id: u8, cycle_count: u16 },
        HoldPressure,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum ThrottleCommand {
        Idle,
        SetPosition { percent_x10: u16 },
        TorqueRequest { nm_x10: i32 },
        RegenBraking { level: u8 },
        CreepMode,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ActuatorCommand {
        timestamp_us: u64,
        brake: BrakeCommand,
        throttle: ThrottleCommand,
        steering_angle_x100: i32,
    }

    let cmd = ActuatorCommand {
        timestamp_us: 1_000_000_042,
        brake: BrakeCommand::Apply { pressure_kpa: 3200 },
        throttle: ThrottleCommand::Idle,
        steering_angle_x100: -450,
    };
    let bytes = encode_to_vec(&cmd).expect("encode actuator command");
    let (decoded, _): (ActuatorCommand, usize) =
        decode_from_slice(&bytes).expect("decode actuator command");
    assert_eq!(cmd, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: V2X communication messages
// ---------------------------------------------------------------------------
#[test]
fn test_av_v2x_communication() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum V2xMessage {
        BasicSafetyMessage {
            sender_id: u32,
            lat_microdeg: i64,
            lon_microdeg: i64,
            speed_cmps: u32,
            heading_x100: u16,
        },
        SignalPhaseAndTiming {
            intersection_id: u32,
            phase: u8,
            time_to_change_ms: u32,
        },
        RoadSideAlert {
            alert_type: u16,
            severity: u8,
            distance_m: u32,
        },
        EmergencyVehicleAlert {
            vehicle_type: u8,
            direction: u8,
            eta_seconds: u16,
        },
        PlatooningRequest {
            platoon_id: u32,
            position_in_platoon: u8,
            gap_distance_cm: u16,
        },
    }

    let msg = V2xMessage::SignalPhaseAndTiming {
        intersection_id: 9001,
        phase: 3,
        time_to_change_ms: 14_500,
    };
    let bytes = encode_to_vec(&msg).expect("encode v2x message");
    let (decoded, _): (V2xMessage, usize) = decode_from_slice(&bytes).expect("decode v2x message");
    assert_eq!(msg, decoded);

    let alert = V2xMessage::EmergencyVehicleAlert {
        vehicle_type: 1,
        direction: 2,
        eta_seconds: 45,
    };
    let bytes2 = encode_to_vec(&alert).expect("encode emergency alert");
    let (decoded2, _): (V2xMessage, usize) =
        decode_from_slice(&bytes2).expect("decode emergency alert");
    assert_eq!(alert, decoded2);
}

// ---------------------------------------------------------------------------
// Test 9: Operational design domain boundaries
// ---------------------------------------------------------------------------
#[test]
fn test_av_odd_boundary() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum WeatherCondition {
        Clear,
        LightRain,
        HeavyRain,
        Snow { accumulation_mm: u16 },
        Fog { visibility_m: u16 },
        Hail,
        Sandstorm,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum RoadType {
        ControlledAccessHighway,
        ArterialRoad,
        ResidentialStreet,
        ParkingLot,
        UnpavedRoad,
        ConstructionZone { speed_limit_kmh: u8 },
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum OddBoundary {
        WithinDomain {
            weather: WeatherCondition,
            road: RoadType,
            max_speed_kmh: u16,
            gps_accuracy_cm: u16,
        },
        ApproachingLimit {
            parameter_code: u16,
            margin_pct: u8,
        },
        OutsideDomain {
            violated_param: u16,
            severity: u8,
            action_required: bool,
        },
    }

    let within = OddBoundary::WithinDomain {
        weather: WeatherCondition::Fog { visibility_m: 200 },
        road: RoadType::ControlledAccessHighway,
        max_speed_kmh: 80,
        gps_accuracy_cm: 15,
    };
    let bytes = encode_to_vec(&within).expect("encode odd boundary");
    let (decoded, _): (OddBoundary, usize) =
        decode_from_slice(&bytes).expect("decode odd boundary");
    assert_eq!(within, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Safety integrity levels (ASIL)
// ---------------------------------------------------------------------------
#[test]
fn test_av_safety_integrity_level() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum AsilLevel {
        Qm,
        AsilA,
        AsilB,
        AsilC,
        AsilD,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum SafetyAssessment {
        ComponentRating {
            component_id: u32,
            asil: AsilLevel,
            failure_rate_fit: u64,
            diagnostic_coverage_pct: u8,
        },
        SystemDecomposition {
            parent_asil: AsilLevel,
            child_a_asil: AsilLevel,
            child_b_asil: AsilLevel,
        },
        FaultTreeNode {
            node_id: u32,
            gate_type: u8,
            probability_x1e9: u64,
            asil: AsilLevel,
        },
    }

    let rating = SafetyAssessment::ComponentRating {
        component_id: 0x0042_00FF,
        asil: AsilLevel::AsilD,
        failure_rate_fit: 10,
        diagnostic_coverage_pct: 99,
    };
    let bytes = encode_to_vec(&rating).expect("encode safety assessment");
    let (decoded, _): (SafetyAssessment, usize) =
        decode_from_slice(&bytes).expect("decode safety assessment");
    assert_eq!(rating, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Perception pipeline stages
// ---------------------------------------------------------------------------
#[test]
fn test_av_perception_pipeline_stage() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum PerceptionStage {
        RawCapture {
            sensor_id: u16,
            frame_number: u64,
            data_size_bytes: u32,
        },
        Preprocessing {
            noise_reduction_applied: bool,
            distortion_corrected: bool,
        },
        ObjectDetection {
            detections_count: u16,
            model_version: u32,
            inference_time_us: u32,
        },
        Tracking {
            active_tracks: u16,
            new_tracks: u16,
            lost_tracks: u16,
        },
        Prediction {
            horizon_ms: u32,
            predicted_paths: u16,
        },
        Fusion {
            sources_merged: u8,
            total_objects: u16,
            conflict_count: u8,
        },
    }

    let stage = PerceptionStage::Fusion {
        sources_merged: 4,
        total_objects: 23,
        conflict_count: 1,
    };
    let bytes = encode_to_vec(&stage).expect("encode perception stage");
    let (decoded, _): (PerceptionStage, usize) =
        decode_from_slice(&bytes).expect("decode perception stage");
    assert_eq!(stage, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Map data elements
// ---------------------------------------------------------------------------
#[test]
fn test_av_hd_map_element() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum TrafficSignType {
        SpeedLimit { value_kmh: u16 },
        StopSign,
        YieldSign,
        NoEntry,
        OneWay,
        SchoolZone { speed_kmh: u8 },
        Custom { code: u32 },
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum HdMapElement {
        RoadSegment {
            segment_id: u64,
            lane_count: u8,
            speed_limit_kmh: u16,
            gradient_permille: i16,
        },
        Intersection {
            intersection_id: u64,
            arm_count: u8,
            has_traffic_light: bool,
        },
        TrafficSign {
            sign_id: u64,
            sign_type: TrafficSignType,
            lat_microdeg: i64,
            lon_microdeg: i64,
        },
        SpeedBump {
            height_mm: u16,
            width_cm: u16,
        },
    }

    let sign = HdMapElement::TrafficSign {
        sign_id: 88_001,
        sign_type: TrafficSignType::SchoolZone { speed_kmh: 30 },
        lat_microdeg: 35_689_487_000,
        lon_microdeg: 139_691_706_000,
    };
    let bytes = encode_to_vec(&sign).expect("encode map element");
    let (decoded, _): (HdMapElement, usize) =
        decode_from_slice(&bytes).expect("decode map element");
    assert_eq!(sign, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Battery management for EV autonomous vehicles
// ---------------------------------------------------------------------------
#[test]
fn test_av_battery_management() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum BatteryAlert {
        Normal,
        HighTemperature { cell_id: u16, temp_x10_c: i16 },
        LowVoltage { cell_id: u16, mv: u16 },
        CellImbalance { delta_mv: u16 },
        ThermalRunawayRisk { cell_id: u16 },
        DegradedCapacity { soh_pct: u8 },
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct BatteryStatus {
        pack_voltage_mv: u32,
        pack_current_ma: i32,
        soc_pct_x10: u16,
        cell_count: u16,
        alert: BatteryAlert,
    }

    let status = BatteryStatus {
        pack_voltage_mv: 400_000,
        pack_current_ma: -125_000,
        soc_pct_x10: 723,
        cell_count: 96,
        alert: BatteryAlert::HighTemperature {
            cell_id: 42,
            temp_x10_c: 485,
        },
    };
    let bytes = encode_to_vec(&status).expect("encode battery status");
    let (decoded, _): (BatteryStatus, usize) =
        decode_from_slice(&bytes).expect("decode battery status");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Localization method
// ---------------------------------------------------------------------------
#[test]
fn test_av_localization_method() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum LocalizationMethod {
        GnssRtk {
            fix_type: u8,
            satellites: u8,
            hdop_x100: u16,
            lat_nanodeg: i64,
            lon_nanodeg: i64,
        },
        VisualOdometry {
            features_tracked: u32,
            translation_mm: (i32, i32, i32),
        },
        LidarSlam {
            map_points: u32,
            drift_mm: u32,
            loop_closures: u16,
        },
        ImuDeadReckoning {
            accel_bias_ug: (i32, i32, i32),
            gyro_bias_mdps: (i32, i32, i32),
        },
        Fused {
            method_count: u8,
            confidence_pct: u8,
            covariance_mm2: u64,
        },
    }

    let loc = LocalizationMethod::LidarSlam {
        map_points: 2_500_000,
        drift_mm: 12,
        loop_closures: 7,
    };
    let bytes = encode_to_vec(&loc).expect("encode localization");
    let (decoded, _): (LocalizationMethod, usize) =
        decode_from_slice(&bytes).expect("decode localization");
    assert_eq!(loc, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Manufacturing QA test result
// ---------------------------------------------------------------------------
#[test]
fn test_av_qa_test_result() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum QaTestOutcome {
        Pass,
        Fail {
            error_code: u32,
        },
        Warning {
            code: u32,
            threshold_exceeded_pct: u8,
        },
        Skipped {
            reason_code: u16,
        },
        Inconclusive {
            retry_count: u8,
        },
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum QaTestCategory {
        SensorCalibration {
            sensor_id: u16,
            deviation_ppm: u32,
            outcome: QaTestOutcome,
        },
        ActuatorResponse {
            actuator_id: u16,
            latency_us: u32,
            overshoot_pct_x10: u16,
            outcome: QaTestOutcome,
        },
        SoftwareChecksum {
            module_id: u32,
            checksum: u64,
            outcome: QaTestOutcome,
        },
        EnduranceCycle {
            cycle_number: u32,
            total_cycles: u32,
            outcome: QaTestOutcome,
        },
    }

    let test_result = QaTestCategory::ActuatorResponse {
        actuator_id: 7,
        latency_us: 2_400,
        overshoot_pct_x10: 32,
        outcome: QaTestOutcome::Pass,
    };
    let bytes = encode_to_vec(&test_result).expect("encode qa test");
    let (decoded, _): (QaTestCategory, usize) = decode_from_slice(&bytes).expect("decode qa test");
    assert_eq!(test_result, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Diagnostic trouble codes (DTC)
// ---------------------------------------------------------------------------
#[test]
fn test_av_diagnostic_trouble_code() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum DtcSeverity {
        Information,
        Maintenance,
        CheckSoon,
        CheckNow,
        StopImmediately,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum DiagnosticCode {
        Powertrain {
            code: u16,
            severity: DtcSeverity,
            freeze_frame_id: u32,
        },
        Chassis {
            code: u16,
            severity: DtcSeverity,
            subsystem: u8,
        },
        Body {
            code: u16,
            severity: DtcSeverity,
        },
        Network {
            code: u16,
            severity: DtcSeverity,
            bus_id: u8,
            node_id: u8,
        },
        AutonomousSystem {
            code: u16,
            severity: DtcSeverity,
            module_hash: u64,
            stack_depth: u8,
        },
    }

    let dtc = DiagnosticCode::AutonomousSystem {
        code: 0x0A42,
        severity: DtcSeverity::CheckNow,
        module_hash: 0xCAFE_BABE_DEAD_BEEF,
        stack_depth: 12,
    };
    let bytes = encode_to_vec(&dtc).expect("encode dtc");
    let (decoded, _): (DiagnosticCode, usize) = decode_from_slice(&bytes).expect("decode dtc");
    assert_eq!(dtc, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Motion planning constraint
// ---------------------------------------------------------------------------
#[test]
fn test_av_motion_planning_constraint() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum MotionConstraint {
        SpeedLimit {
            max_kmh: u16,
            source: u8,
        },
        AccelerationLimit {
            max_longitudinal_mg: u16,
            max_lateral_mg: u16,
        },
        JerkLimit {
            max_mps3_x100: u16,
        },
        SteeringRateLimit {
            max_deg_per_s_x10: u16,
        },
        KeepOutZone {
            zone_id: u32,
            min_distance_cm: u16,
        },
        TimeGap {
            min_gap_ms: u32,
            target_gap_ms: u32,
        },
        OccupancyGridCell {
            grid_x: u16,
            grid_y: u16,
            cost: u8,
        },
    }

    let constraint = MotionConstraint::TimeGap {
        min_gap_ms: 1_500,
        target_gap_ms: 2_500,
    };
    let bytes = encode_to_vec(&constraint).expect("encode motion constraint");
    let (decoded, _): (MotionConstraint, usize) =
        decode_from_slice(&bytes).expect("decode motion constraint");
    assert_eq!(constraint, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: OTA update management
// ---------------------------------------------------------------------------
#[test]
fn test_av_ota_update_status() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum OtaUpdateStatus {
        Idle,
        Checking {
            server_url_hash: u64,
        },
        Downloading {
            total_bytes: u64,
            downloaded_bytes: u64,
            checksum_partial: u32,
        },
        Verifying {
            signature_alg: u8,
            hash_computed: u64,
        },
        Installing {
            partition: u8,
            progress_pct: u8,
            rollback_available: bool,
        },
        RollbackRequired {
            error_code: u32,
            previous_version: u32,
        },
        Complete {
            new_version: u32,
            install_duration_s: u32,
        },
    }

    let status = OtaUpdateStatus::Installing {
        partition: 2,
        progress_pct: 67,
        rollback_available: true,
    };
    let bytes = encode_to_vec(&status).expect("encode ota status");
    let (decoded, _): (OtaUpdateStatus, usize) =
        decode_from_slice(&bytes).expect("decode ota status");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Functional safety monitor events
// ---------------------------------------------------------------------------
#[test]
fn test_av_functional_safety_monitor() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum SafetyMonitorEvent {
        Heartbeat {
            module_id: u16,
            sequence: u64,
            alive_counter: u8,
        },
        WatchdogTimeout {
            module_id: u16,
            last_sequence: u64,
            elapsed_ms: u32,
        },
        RedundancyMismatch {
            channel_a_value: u64,
            channel_b_value: u64,
            tolerance: u32,
        },
        MemoryCorruption {
            address: u64,
            expected_crc: u32,
            actual_crc: u32,
        },
        VotingDisagreement {
            voter_count: u8,
            majority_value: u64,
            minority_value: u64,
        },
        SafeStateEntry {
            trigger_event_id: u32,
            response_time_us: u32,
        },
    }

    let event = SafetyMonitorEvent::RedundancyMismatch {
        channel_a_value: 0x0000_1234_5678_ABCD,
        channel_b_value: 0x0000_1234_5678_ABCE,
        tolerance: 2,
    };
    let bytes = encode_to_vec(&event).expect("encode safety event");
    let (decoded, _): (SafetyMonitorEvent, usize) =
        decode_from_slice(&bytes).expect("decode safety event");
    assert_eq!(event, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Traffic participant prediction
// ---------------------------------------------------------------------------
#[test]
fn test_av_traffic_participant_prediction() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum PredictedBehavior {
        MaintainLane {
            speed_change_pct: i8,
        },
        LaneChangeLeft {
            probability_pct: u8,
            time_to_start_ms: u32,
        },
        LaneChangeRight {
            probability_pct: u8,
            time_to_start_ms: u32,
        },
        Braking {
            deceleration_mps2_x10: u16,
        },
        Turning {
            direction_deg: i16,
            radius_m: u16,
        },
        Jaywalking {
            crossing_speed_cmps: u16,
            crossing_angle_deg: i16,
        },
        Stationary {
            expected_duration_s: u32,
        },
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ParticipantPrediction {
        track_id: u32,
        prediction_horizon_ms: u32,
        top_behavior: PredictedBehavior,
        top_confidence_pct: u8,
        alt_behavior: PredictedBehavior,
        alt_confidence_pct: u8,
    }

    let pred = ParticipantPrediction {
        track_id: 299,
        prediction_horizon_ms: 3_000,
        top_behavior: PredictedBehavior::LaneChangeLeft {
            probability_pct: 72,
            time_to_start_ms: 800,
        },
        top_confidence_pct: 72,
        alt_behavior: PredictedBehavior::MaintainLane {
            speed_change_pct: -5,
        },
        alt_confidence_pct: 28,
    };
    let bytes = encode_to_vec(&pred).expect("encode prediction");
    let (decoded, _): (ParticipantPrediction, usize) =
        decode_from_slice(&bytes).expect("decode prediction");
    assert_eq!(pred, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: CAN bus message routing
// ---------------------------------------------------------------------------
#[test]
fn test_av_can_bus_message() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum CanBusType {
        HighSpeed,
        LowSpeed,
        CanFd,
        FlexRay,
        Ethernet100Base,
        Ethernet1000Base,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum CanMessage {
        Standard {
            bus: CanBusType,
            arbitration_id: u16,
            dlc: u8,
            payload: (u64,),
            timestamp_us: u64,
        },
        Extended {
            bus: CanBusType,
            arbitration_id: u32,
            dlc: u8,
            payload_high: u64,
            payload_low: u64,
            timestamp_us: u64,
        },
        Error {
            bus: CanBusType,
            error_type: u8,
            error_count: u32,
        },
        BusOff {
            bus: CanBusType,
            tec: u8,
            rec: u8,
        },
    }

    let msg = CanMessage::Extended {
        bus: CanBusType::CanFd,
        arbitration_id: 0x18FF_00FE,
        dlc: 64,
        payload_high: 0xDEAD_BEEF_CAFE_BABE,
        payload_low: 0x1234_5678_9ABC_DEF0,
        timestamp_us: 42_000_000,
    };
    let bytes = encode_to_vec(&msg).expect("encode can message");
    let (decoded, _): (CanMessage, usize) = decode_from_slice(&bytes).expect("decode can message");
    assert_eq!(msg, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: Simulation scenario configuration
// ---------------------------------------------------------------------------
#[test]
fn test_av_simulation_scenario() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum ScenarioWeather {
        Sunny,
        Overcast,
        Rain { intensity_mmph: u16 },
        Snow { intensity_mmph: u16 },
        Night { ambient_lux: u16 },
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum ScenarioTrafficDensity {
        Empty,
        Light { vehicles_per_km: u16 },
        Moderate { vehicles_per_km: u16 },
        Heavy { vehicles_per_km: u16 },
        Gridlock,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    enum InjectedFault {
        None,
        SensorDegradation { sensor_id: u16, degradation_pct: u8 },
        ActuatorDelay { actuator_id: u16, delay_ms: u32 },
        GpsJamming { strength_db: u8 },
        CommunicationLoss { duration_ms: u32 },
        SpoofedObject { object_type: u8, bearing_deg: i16 },
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SimulationScenario {
        scenario_id: u64,
        seed: u64,
        duration_s: u32,
        weather: ScenarioWeather,
        traffic: ScenarioTrafficDensity,
        fault: InjectedFault,
        ego_start_speed_kmh: u16,
        pass_criteria_code: u32,
    }

    let scenario = SimulationScenario {
        scenario_id: 100_042,
        seed: 0xBADC0FFEE,
        duration_s: 600,
        weather: ScenarioWeather::Rain { intensity_mmph: 25 },
        traffic: ScenarioTrafficDensity::Heavy {
            vehicles_per_km: 120,
        },
        fault: InjectedFault::GpsJamming { strength_db: 40 },
        ego_start_speed_kmh: 60,
        pass_criteria_code: 0x0001_0003,
    };
    let bytes = encode_to_vec(&scenario).expect("encode simulation scenario");
    let (decoded, _): (SimulationScenario, usize) =
        decode_from_slice(&bytes).expect("decode simulation scenario");
    assert_eq!(scenario, decoded);

    let clean_scenario = SimulationScenario {
        scenario_id: 100_043,
        seed: 12345,
        duration_s: 1200,
        weather: ScenarioWeather::Night { ambient_lux: 5 },
        traffic: ScenarioTrafficDensity::Empty,
        fault: InjectedFault::None,
        ego_start_speed_kmh: 0,
        pass_criteria_code: 0x0002_0001,
    };
    let bytes2 = encode_to_vec(&clean_scenario).expect("encode clean scenario");
    let (decoded2, _): (SimulationScenario, usize) =
        decode_from_slice(&bytes2).expect("decode clean scenario");
    assert_eq!(clean_scenario, decoded2);
}
