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

// ── Domain types: Autonomous Mining Operations ──────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TruckOperatingMode {
    Autonomous,
    RemoteControl,
    Manual,
    Standby,
    EmergencyStop,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DrillPatternType {
    Rectangular,
    Staggered,
    TriangularOffset,
    FanDrill,
    Contour,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OreGradeClass {
    HighGrade,
    MediumGrade,
    LowGrade,
    Waste,
    Marginal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AlertSeverity {
    Critical,
    Warning,
    Advisory,
    Information,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PumpState {
    Running,
    Idle,
    Priming,
    Faulted,
    Maintenance,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConveyorStatus {
    Operating,
    Stopped,
    Slipping,
    Overloaded,
    Misaligned,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HaulTruckTelemetry {
    truck_id: u32,
    fleet_name: String,
    operating_mode: TruckOperatingMode,
    payload_tonnes: u32,
    speed_mm_per_sec: u32,
    heading_deg_x100: u16,
    engine_rpm: u16,
    fuel_level_pct: u8,
    gps_lat_x1e7: i64,
    gps_lon_x1e7: i64,
    elevation_mm: i32,
    odometer_metres: u64,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DrillPatternAutomation {
    pattern_id: u64,
    pit_zone: String,
    pattern_type: DrillPatternType,
    hole_count: u32,
    spacing_mm: u32,
    burden_mm: u32,
    depth_mm: u32,
    subdrill_mm: u32,
    collar_offset_x_mm: i32,
    collar_offset_y_mm: i32,
    designed_by: String,
    approved: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OreBodyGradeControl {
    sample_id: u64,
    bench_name: String,
    block_x: u32,
    block_y: u32,
    block_z: u32,
    grade_class: OreGradeClass,
    copper_pct_x1000: u32,
    gold_ppm_x100: u32,
    silver_ppm_x100: u32,
    density_kg_m3: u32,
    tonnage_estimate: u64,
    assay_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CollisionAvoidanceRadar {
    radar_unit_id: u32,
    mounted_on_truck: u32,
    detection_range_mm: u32,
    closest_object_mm: u32,
    object_bearing_deg_x100: u16,
    relative_speed_mm_per_sec: i32,
    alert_severity: AlertSeverity,
    alert_active: bool,
    scan_interval_ms: u16,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FatigueDetectionEvent {
    event_id: u64,
    operator_id: String,
    truck_id: u32,
    microsleep_count: u16,
    perclos_pct_x100: u16,
    yawn_frequency_per_hour: u16,
    head_pose_deviation_deg_x100: u16,
    alert_severity: AlertSeverity,
    shift_elapsed_minutes: u32,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StockpileVolumeSurvey {
    survey_id: u64,
    stockpile_name: String,
    volume_m3_x1000: u64,
    tonnage_estimate: u64,
    ore_grade_class: OreGradeClass,
    base_elevation_mm: i32,
    peak_elevation_mm: i32,
    survey_method: String,
    surveyed_at: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrusherThroughput {
    crusher_id: u32,
    crusher_name: String,
    throughput_tph: u32,
    feed_size_p80_mm: u32,
    product_size_p80_mm: u32,
    power_draw_kw: u32,
    css_mm_x10: u16,
    liner_wear_pct: u8,
    operating_hours: u64,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConveyorBeltMonitoring {
    conveyor_id: u32,
    belt_name: String,
    status: ConveyorStatus,
    speed_m_per_sec_x100: u32,
    load_tonnes_per_hour: u32,
    belt_tension_kn_x10: u32,
    splice_count: u16,
    temperature_c_x10: i16,
    cumulative_run_hours: u64,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DewateringPumpControl {
    pump_id: u32,
    pump_location: String,
    state: PumpState,
    flow_rate_lpm: u32,
    discharge_pressure_kpa: u32,
    suction_pressure_kpa: u32,
    motor_current_ma: u32,
    water_level_mm: i32,
    setpoint_level_mm: i32,
    runtime_hours: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BlastFragmentationAnalysis {
    blast_id: u64,
    blast_zone: String,
    p50_fragment_mm: u32,
    p80_fragment_mm: u32,
    max_fragment_mm: u32,
    oversize_pct_x100: u16,
    fines_pct_x100: u16,
    powder_factor_kg_m3_x1000: u32,
    image_count: u32,
    analysis_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TailingsPipelinePressure {
    pipeline_id: u32,
    segment_name: String,
    inlet_pressure_kpa: u32,
    outlet_pressure_kpa: u32,
    flow_rate_m3_per_hour_x10: u32,
    slurry_density_kg_m3: u32,
    velocity_m_per_sec_x100: u32,
    wear_thickness_mm_x100: u16,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DustMonitoringStation {
    station_id: u32,
    station_name: String,
    pm10_ug_m3: u32,
    pm25_ug_m3: u32,
    tsp_ug_m3: u32,
    wind_speed_mm_per_sec: u32,
    wind_direction_deg_x10: u16,
    humidity_pct_x10: u16,
    suppression_active: bool,
    reading_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FleetDispatchOptimization {
    dispatch_id: u64,
    truck_id: u32,
    assigned_loader: String,
    assigned_dump: String,
    estimated_cycle_sec: u32,
    priority_score: u32,
    queue_position: u16,
    haul_distance_m: u32,
    grade_resistance_pct_x100: i16,
    dispatch_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PitSlopeRadar {
    radar_id: u32,
    wall_sector: String,
    displacement_mm_x100: i64,
    velocity_mm_per_day_x100: i64,
    acceleration_mm_per_day2_x100: i64,
    alert_severity: AlertSeverity,
    prism_reflector_count: u16,
    scan_area_m2: u32,
    last_scan_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaintenancePrediction {
    asset_id: u32,
    asset_name: String,
    component: String,
    remaining_useful_life_hours: u32,
    confidence_pct_x100: u16,
    failure_mode: String,
    vibration_rms_mm_per_sec_x100: u32,
    oil_analysis_ppm_iron: u32,
    temperature_c_x10: i16,
    prediction_timestamp: u64,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn test_haul_truck_telemetry_roundtrip() {
    let truck = HaulTruckTelemetry {
        truck_id: 801,
        fleet_name: "Pit-A-Fleet".to_string(),
        operating_mode: TruckOperatingMode::Autonomous,
        payload_tonnes: 220,
        speed_mm_per_sec: 8333,
        heading_deg_x100: 27450,
        engine_rpm: 1850,
        fuel_level_pct: 73,
        gps_lat_x1e7: -235678900,
        gps_lon_x1e7: 1189432100,
        elevation_mm: 485_000,
        odometer_metres: 1_284_390,
        timestamp_ms: 1_742_000_000_000,
    };
    let bytes = encode_to_vec(&truck).expect("encode HaulTruckTelemetry failed");
    let (decoded, consumed): (HaulTruckTelemetry, usize) =
        decode_from_slice(&bytes).expect("decode HaulTruckTelemetry failed");
    assert_eq!(truck, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_haul_truck_telemetry_versioned_v1_0_0() {
    let truck = HaulTruckTelemetry {
        truck_id: 802,
        fleet_name: "Pit-B-Fleet".to_string(),
        operating_mode: TruckOperatingMode::EmergencyStop,
        payload_tonnes: 0,
        speed_mm_per_sec: 0,
        heading_deg_x100: 9000,
        engine_rpm: 0,
        fuel_level_pct: 45,
        gps_lat_x1e7: -236001200,
        gps_lon_x1e7: 1188870300,
        elevation_mm: 502_000,
        odometer_metres: 987_120,
        timestamp_ms: 1_742_000_060_000,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&truck, version)
        .expect("encode versioned HaulTruckTelemetry v1.0.0 failed");
    let (decoded, ver, _consumed): (HaulTruckTelemetry, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned HaulTruckTelemetry v1.0.0 failed");
    assert_eq!(truck, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_drill_pattern_automation_roundtrip() {
    let pattern = DrillPatternAutomation {
        pattern_id: 55_001,
        pit_zone: "Zone-NW-12".to_string(),
        pattern_type: DrillPatternType::Staggered,
        hole_count: 144,
        spacing_mm: 6500,
        burden_mm: 7500,
        depth_mm: 12_000,
        subdrill_mm: 1500,
        collar_offset_x_mm: -250,
        collar_offset_y_mm: 180,
        designed_by: "GeoTech-AutoDesign-v3".to_string(),
        approved: true,
    };
    let bytes = encode_to_vec(&pattern).expect("encode DrillPatternAutomation failed");
    let (decoded, consumed): (DrillPatternAutomation, usize) =
        decode_from_slice(&bytes).expect("decode DrillPatternAutomation failed");
    assert_eq!(pattern, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_drill_pattern_versioned_v2_1_0() {
    let pattern = DrillPatternAutomation {
        pattern_id: 55_002,
        pit_zone: "Zone-SE-07".to_string(),
        pattern_type: DrillPatternType::FanDrill,
        hole_count: 36,
        spacing_mm: 4000,
        burden_mm: 5000,
        depth_mm: 18_000,
        subdrill_mm: 2000,
        collar_offset_x_mm: 0,
        collar_offset_y_mm: 0,
        designed_by: "SeniorBlaster-M.Chen".to_string(),
        approved: false,
    };
    let version = Version::new(2, 1, 0);
    let bytes = encode_versioned_value(&pattern, version)
        .expect("encode versioned DrillPattern v2.1.0 failed");
    let (decoded, ver, consumed): (DrillPatternAutomation, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned DrillPattern v2.1.0 failed");
    assert_eq!(pattern, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_ore_body_grade_control_high_grade() {
    let sample = OreBodyGradeControl {
        sample_id: 100_301,
        bench_name: "Bench-1045".to_string(),
        block_x: 42,
        block_y: 18,
        block_z: 3,
        grade_class: OreGradeClass::HighGrade,
        copper_pct_x1000: 3200,
        gold_ppm_x100: 85,
        silver_ppm_x100: 420,
        density_kg_m3: 2750,
        tonnage_estimate: 85_000,
        assay_timestamp: 1_742_100_000,
    };
    let bytes = encode_to_vec(&sample).expect("encode OreBodyGradeControl failed");
    let (decoded, consumed): (OreBodyGradeControl, usize) =
        decode_from_slice(&bytes).expect("decode OreBodyGradeControl failed");
    assert_eq!(sample, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_ore_body_grade_versioned_upgrade_v1_to_v2() {
    let sample = OreBodyGradeControl {
        sample_id: 100_302,
        bench_name: "Bench-1050".to_string(),
        block_x: 50,
        block_y: 22,
        block_z: 5,
        grade_class: OreGradeClass::Waste,
        copper_pct_x1000: 120,
        gold_ppm_x100: 2,
        silver_ppm_x100: 15,
        density_kg_m3: 2600,
        tonnage_estimate: 200_000,
        assay_timestamp: 1_742_100_500,
    };
    let v1 = Version::new(1, 0, 0);
    let bytes_v1 = encode_versioned_value(&sample, v1).expect("encode grade v1 failed");
    let (decoded_v1, ver_v1, _): (OreBodyGradeControl, Version, usize) =
        decode_versioned_value(&bytes_v1).expect("decode grade v1 failed");
    assert_eq!(ver_v1.major, 1);

    let v2 = Version::new(2, 0, 0);
    let bytes_v2 = encode_versioned_value(&decoded_v1, v2).expect("re-encode grade v2 failed");
    let (decoded_v2, ver_v2, _): (OreBodyGradeControl, Version, usize) =
        decode_versioned_value(&bytes_v2).expect("decode grade v2 failed");
    assert_eq!(sample, decoded_v2);
    assert_eq!(ver_v2.major, 2);
}

#[test]
fn test_collision_avoidance_radar_critical_alert() {
    let radar = CollisionAvoidanceRadar {
        radar_unit_id: 4010,
        mounted_on_truck: 801,
        detection_range_mm: 200_000,
        closest_object_mm: 3_500,
        object_bearing_deg_x100: 4500,
        relative_speed_mm_per_sec: -2200,
        alert_severity: AlertSeverity::Critical,
        alert_active: true,
        scan_interval_ms: 100,
        timestamp_ms: 1_742_001_000_000,
    };
    let bytes = encode_to_vec(&radar).expect("encode CollisionAvoidanceRadar failed");
    let (decoded, consumed): (CollisionAvoidanceRadar, usize) =
        decode_from_slice(&bytes).expect("decode CollisionAvoidanceRadar failed");
    assert_eq!(radar, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_collision_avoidance_radar_versioned_v3_0_2() {
    let radar = CollisionAvoidanceRadar {
        radar_unit_id: 4011,
        mounted_on_truck: 803,
        detection_range_mm: 250_000,
        closest_object_mm: 150_000,
        object_bearing_deg_x100: 31500,
        relative_speed_mm_per_sec: 500,
        alert_severity: AlertSeverity::Information,
        alert_active: false,
        scan_interval_ms: 200,
        timestamp_ms: 1_742_002_000_000,
    };
    let version = Version::new(3, 0, 2);
    let bytes =
        encode_versioned_value(&radar, version).expect("encode versioned radar v3.0.2 failed");
    let (decoded, ver, consumed): (CollisionAvoidanceRadar, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned radar v3.0.2 failed");
    assert_eq!(radar, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.patch, 2);
    assert!(consumed > 0);
}

#[test]
fn test_fatigue_detection_event_roundtrip() {
    let event = FatigueDetectionEvent {
        event_id: 9900,
        operator_id: "OP-2847".to_string(),
        truck_id: 805,
        microsleep_count: 3,
        perclos_pct_x100: 1250,
        yawn_frequency_per_hour: 8,
        head_pose_deviation_deg_x100: 1520,
        alert_severity: AlertSeverity::Warning,
        shift_elapsed_minutes: 540,
        timestamp_ms: 1_742_010_000_000,
    };
    let bytes = encode_to_vec(&event).expect("encode FatigueDetectionEvent failed");
    let (decoded, consumed): (FatigueDetectionEvent, usize) =
        decode_from_slice(&bytes).expect("decode FatigueDetectionEvent failed");
    assert_eq!(event, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_stockpile_volume_survey_versioned_v1_5_0() {
    let survey = StockpileVolumeSurvey {
        survey_id: 7700,
        stockpile_name: "ROM-Pad-North".to_string(),
        volume_m3_x1000: 45_200_000,
        tonnage_estimate: 120_000,
        ore_grade_class: OreGradeClass::MediumGrade,
        base_elevation_mm: 450_000,
        peak_elevation_mm: 468_500,
        survey_method: "LiDAR-Drone".to_string(),
        surveyed_at: 1_742_050_000,
    };
    let version = Version::new(1, 5, 0);
    let bytes = encode_versioned_value(&survey, version)
        .expect("encode versioned StockpileSurvey v1.5.0 failed");
    let (decoded, ver, _consumed): (StockpileVolumeSurvey, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned StockpileSurvey v1.5.0 failed");
    assert_eq!(survey, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 5);
}

#[test]
fn test_crusher_throughput_roundtrip() {
    let crusher = CrusherThroughput {
        crusher_id: 201,
        crusher_name: "Primary-Gyratory-01".to_string(),
        throughput_tph: 4500,
        feed_size_p80_mm: 850,
        product_size_p80_mm: 175,
        power_draw_kw: 2200,
        css_mm_x10: 1750,
        liner_wear_pct: 42,
        operating_hours: 18_720,
        timestamp_ms: 1_742_020_000_000,
    };
    let bytes = encode_to_vec(&crusher).expect("encode CrusherThroughput failed");
    let (decoded, consumed): (CrusherThroughput, usize) =
        decode_from_slice(&bytes).expect("decode CrusherThroughput failed");
    assert_eq!(crusher, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_crusher_throughput_versioned_v4_2_1() {
    let crusher = CrusherThroughput {
        crusher_id: 202,
        crusher_name: "Secondary-Cone-03".to_string(),
        throughput_tph: 1800,
        feed_size_p80_mm: 175,
        product_size_p80_mm: 38,
        power_draw_kw: 750,
        css_mm_x10: 380,
        liner_wear_pct: 78,
        operating_hours: 6_210,
        timestamp_ms: 1_742_020_060_000,
    };
    let version = Version::new(4, 2, 1);
    let bytes =
        encode_versioned_value(&crusher, version).expect("encode versioned crusher v4.2.1 failed");
    let (decoded, ver, consumed): (CrusherThroughput, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned crusher v4.2.1 failed");
    assert_eq!(crusher, decoded);
    assert_eq!(ver.major, 4);
    assert_eq!(ver.minor, 2);
    assert_eq!(ver.patch, 1);
    assert!(consumed > 0);
}

#[test]
fn test_conveyor_belt_monitoring_overloaded() {
    let belt = ConveyorBeltMonitoring {
        conveyor_id: 310,
        belt_name: "CV-310-Overland".to_string(),
        status: ConveyorStatus::Overloaded,
        speed_m_per_sec_x100: 420,
        load_tonnes_per_hour: 6200,
        belt_tension_kn_x10: 8500,
        splice_count: 14,
        temperature_c_x10: 385,
        cumulative_run_hours: 42_000,
        timestamp_ms: 1_742_030_000_000,
    };
    let bytes = encode_to_vec(&belt).expect("encode ConveyorBeltMonitoring failed");
    let (decoded, consumed): (ConveyorBeltMonitoring, usize) =
        decode_from_slice(&bytes).expect("decode ConveyorBeltMonitoring failed");
    assert_eq!(belt, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_dewatering_pump_control_versioned_v2_0_0() {
    let pump = DewateringPumpControl {
        pump_id: 501,
        pump_location: "Sump-NE-Level3".to_string(),
        state: PumpState::Running,
        flow_rate_lpm: 4500,
        discharge_pressure_kpa: 850,
        suction_pressure_kpa: 35,
        motor_current_ma: 125_000,
        water_level_mm: 2800,
        setpoint_level_mm: 2000,
        runtime_hours: 14_300,
    };
    let version = Version::new(2, 0, 0);
    let bytes =
        encode_versioned_value(&pump, version).expect("encode versioned pump v2.0.0 failed");
    let (decoded, ver, _consumed): (DewateringPumpControl, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned pump v2.0.0 failed");
    assert_eq!(pump, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
}

#[test]
fn test_blast_fragmentation_analysis_roundtrip() {
    let blast = BlastFragmentationAnalysis {
        blast_id: 88_010,
        blast_zone: "Pit-A-Bench1040-Shot7".to_string(),
        p50_fragment_mm: 220,
        p80_fragment_mm: 480,
        max_fragment_mm: 1800,
        oversize_pct_x100: 850,
        fines_pct_x100: 1200,
        powder_factor_kg_m3_x1000: 650,
        image_count: 48,
        analysis_timestamp: 1_742_060_000,
    };
    let bytes = encode_to_vec(&blast).expect("encode BlastFragmentationAnalysis failed");
    let (decoded, consumed): (BlastFragmentationAnalysis, usize) =
        decode_from_slice(&bytes).expect("decode BlastFragmentationAnalysis failed");
    assert_eq!(blast, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_tailings_pipeline_pressure_roundtrip() {
    let pipeline = TailingsPipelinePressure {
        pipeline_id: 601,
        segment_name: "TSF-Main-Seg-04".to_string(),
        inlet_pressure_kpa: 2100,
        outlet_pressure_kpa: 1850,
        flow_rate_m3_per_hour_x10: 4500,
        slurry_density_kg_m3: 1450,
        velocity_m_per_sec_x100: 280,
        wear_thickness_mm_x100: 920,
        timestamp_ms: 1_742_070_000_000,
    };
    let bytes = encode_to_vec(&pipeline).expect("encode TailingsPipelinePressure failed");
    let (decoded, consumed): (TailingsPipelinePressure, usize) =
        decode_from_slice(&bytes).expect("decode TailingsPipelinePressure failed");
    assert_eq!(pipeline, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_dust_monitoring_station_versioned_v1_2_3() {
    let station = DustMonitoringStation {
        station_id: 701,
        station_name: "DMS-Haul-Road-03".to_string(),
        pm10_ug_m3: 185,
        pm25_ug_m3: 42,
        tsp_ug_m3: 320,
        wind_speed_mm_per_sec: 8500,
        wind_direction_deg_x10: 2250,
        humidity_pct_x10: 380,
        suppression_active: true,
        reading_timestamp: 1_742_080_000,
    };
    let version = Version::new(1, 2, 3);
    let bytes = encode_versioned_value(&station, version)
        .expect("encode versioned dust station v1.2.3 failed");
    let (decoded, ver, consumed): (DustMonitoringStation, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned dust station v1.2.3 failed");
    assert_eq!(station, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 2);
    assert_eq!(ver.patch, 3);
    assert!(consumed > 0);
}

#[test]
fn test_fleet_dispatch_optimization_roundtrip() {
    let dispatch = FleetDispatchOptimization {
        dispatch_id: 330_001,
        truck_id: 807,
        assigned_loader: "EX-5600-Alpha".to_string(),
        assigned_dump: "Crusher-Primary-01".to_string(),
        estimated_cycle_sec: 1320,
        priority_score: 8750,
        queue_position: 2,
        haul_distance_m: 3_400,
        grade_resistance_pct_x100: -850,
        dispatch_timestamp: 1_742_090_000,
    };
    let bytes = encode_to_vec(&dispatch).expect("encode FleetDispatchOptimization failed");
    let (decoded, consumed): (FleetDispatchOptimization, usize) =
        decode_from_slice(&bytes).expect("decode FleetDispatchOptimization failed");
    assert_eq!(dispatch, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_pit_slope_radar_versioned_upgrade_v1_to_v3() {
    let radar = PitSlopeRadar {
        radar_id: 901,
        wall_sector: "West-Wall-Sector-B".to_string(),
        displacement_mm_x100: 4520,
        velocity_mm_per_day_x100: 125,
        acceleration_mm_per_day2_x100: 8,
        alert_severity: AlertSeverity::Advisory,
        prism_reflector_count: 24,
        scan_area_m2: 15_000,
        last_scan_timestamp: 1_742_095_000,
    };

    let v1 = Version::new(1, 0, 0);
    let bytes_v1 = encode_versioned_value(&radar, v1).expect("encode pit slope radar v1 failed");
    let (decoded_v1, ver_v1, _): (PitSlopeRadar, Version, usize) =
        decode_versioned_value(&bytes_v1).expect("decode pit slope radar v1 failed");
    assert_eq!(ver_v1.major, 1);
    assert_eq!(radar, decoded_v1);

    let v2 = Version::new(2, 0, 0);
    let bytes_v2 =
        encode_versioned_value(&decoded_v1, v2).expect("re-encode pit slope radar v2 failed");
    let (decoded_v2, ver_v2, _): (PitSlopeRadar, Version, usize) =
        decode_versioned_value(&bytes_v2).expect("decode pit slope radar v2 failed");
    assert_eq!(ver_v2.major, 2);

    let v3 = Version::new(3, 0, 0);
    let bytes_v3 =
        encode_versioned_value(&decoded_v2, v3).expect("re-encode pit slope radar v3 failed");
    let (decoded_v3, ver_v3, consumed_v3): (PitSlopeRadar, Version, usize) =
        decode_versioned_value(&bytes_v3).expect("decode pit slope radar v3 failed");
    assert_eq!(radar, decoded_v3);
    assert_eq!(ver_v3.major, 3);
    assert!(consumed_v3 > 0);
}

#[test]
fn test_maintenance_prediction_roundtrip() {
    let prediction = MaintenancePrediction {
        asset_id: 801,
        asset_name: "HaulTruck-801".to_string(),
        component: "Final-Drive-LH".to_string(),
        remaining_useful_life_hours: 1250,
        confidence_pct_x100: 8720,
        failure_mode: "Gear-Tooth-Fatigue".to_string(),
        vibration_rms_mm_per_sec_x100: 480,
        oil_analysis_ppm_iron: 165,
        temperature_c_x10: 920,
        prediction_timestamp: 1_742_099_000,
    };
    let bytes = encode_to_vec(&prediction).expect("encode MaintenancePrediction failed");
    let (decoded, consumed): (MaintenancePrediction, usize) =
        decode_from_slice(&bytes).expect("decode MaintenancePrediction failed");
    assert_eq!(prediction, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_maintenance_prediction_versioned_v5_0_0() {
    let prediction = MaintenancePrediction {
        asset_id: 202,
        asset_name: "Crusher-Secondary-03".to_string(),
        component: "Mantle-Liner".to_string(),
        remaining_useful_life_hours: 320,
        confidence_pct_x100: 9150,
        failure_mode: "Abrasive-Wear-Limit".to_string(),
        vibration_rms_mm_per_sec_x100: 1250,
        oil_analysis_ppm_iron: 0,
        temperature_c_x10: 650,
        prediction_timestamp: 1_742_099_500,
    };
    let version = Version::new(5, 0, 0);
    let bytes = encode_versioned_value(&prediction, version)
        .expect("encode versioned maintenance v5.0.0 failed");
    let (decoded, ver, consumed): (MaintenancePrediction, Version, usize) =
        decode_versioned_value(&bytes).expect("decode versioned maintenance v5.0.0 failed");
    assert_eq!(prediction, decoded);
    assert_eq!(ver.major, 5);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_dewatering_pump_faulted_roundtrip() {
    let pump = DewateringPumpControl {
        pump_id: 502,
        pump_location: "Sump-SW-Level5".to_string(),
        state: PumpState::Faulted,
        flow_rate_lpm: 0,
        discharge_pressure_kpa: 0,
        suction_pressure_kpa: 12,
        motor_current_ma: 0,
        water_level_mm: 4200,
        setpoint_level_mm: 2000,
        runtime_hours: 22_450,
    };
    let bytes = encode_to_vec(&pump).expect("encode DewateringPumpControl faulted failed");
    let (decoded, consumed): (DewateringPumpControl, usize) =
        decode_from_slice(&bytes).expect("decode DewateringPumpControl faulted failed");
    assert_eq!(pump, decoded);
    assert_eq!(consumed, bytes.len());
}
