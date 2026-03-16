//! Checksum tests for OxiCode -- autonomous maritime shipping domain.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced27_test

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum, HEADER_SIZE};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types -- Autonomous Maritime Shipping
// ---------------------------------------------------------------------------

/// AIS (Automatic Identification System) message from a vessel transponder.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AisMessage {
    mmsi: u32,
    navigation_status: u8,
    rate_of_turn: i16,
    speed_over_ground: u16,
    longitude: i32,
    latitude: i32,
    course_over_ground: u16,
    true_heading: u16,
    timestamp_seconds: u8,
    vessel_name: String,
}

/// ECDIS (Electronic Chart Display and Information System) chart cell.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EcdisChartCell {
    cell_id: String,
    edition_number: u16,
    update_number: u16,
    scale: u32,
    depth_contours: Vec<f64>,
    hazard_count: u32,
    compilation_date: String,
}

/// COLREG collision avoidance decision.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ColregAction {
    StandOn,
    GiveWay { alteration_degrees: i16 },
    OvertakingKeepClear,
    HeadOnAlterStarboard { degrees: i16 },
    CrossingGiveWay { target_mmsi: u32 },
    RestrictedVisibility { speed_reduction_pct: u8 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ColregDecision {
    own_mmsi: u32,
    target_mmsi: u32,
    cpa_nm: f64,
    tcpa_minutes: f64,
    action: ColregAction,
    confidence: f64,
}

/// Engine room telemetry snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EngineRoomTelemetry {
    main_engine_rpm: u16,
    main_engine_load_pct: u8,
    exhaust_temp_celsius: Vec<u16>,
    lube_oil_pressure_bar: f64,
    coolant_temp_celsius: f64,
    fuel_oil_temp_celsius: f64,
    turbocharger_rpm: u32,
    scavenge_air_pressure_bar: f64,
    vibration_mm_s: f64,
    alarm_codes: Vec<u32>,
}

/// Ballast water management record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BallastWaterRecord {
    tank_id: String,
    capacity_m3: f64,
    current_volume_m3: f64,
    salinity_ppt: f64,
    temperature_celsius: f64,
    treatment_method: String,
    exchange_latitude: i32,
    exchange_longitude: i32,
    uv_dose_mj_cm2: f64,
    compliant: bool,
}

/// Weather routing waypoint with meteo data.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherRoutingWaypoint {
    waypoint_id: u16,
    latitude: f64,
    longitude: f64,
    eta_epoch_seconds: u64,
    wind_speed_knots: f64,
    wind_direction_deg: u16,
    wave_height_m: f64,
    wave_period_s: f64,
    current_speed_knots: f64,
    current_direction_deg: u16,
    visibility_nm: f64,
}

/// Cargo loading plan for a container vessel.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CargoLoadingPlan {
    voyage_number: String,
    port_of_loading: String,
    total_teu: u32,
    reefer_count: u16,
    dangerous_goods_count: u16,
    max_stack_weight_tonnes: f64,
    gm_metacentric_height_m: f64,
    shear_force_limit_pct: f64,
    bending_moment_limit_pct: f64,
    bay_plans: Vec<BayPlan>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BayPlan {
    bay_number: u16,
    tier_count: u8,
    row_count: u8,
    container_ids: Vec<String>,
    stack_weights_tonnes: Vec<f64>,
}

/// Port approach procedure step.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PortApproachStep {
    sequence: u8,
    waypoint_name: String,
    latitude: f64,
    longitude: f64,
    speed_limit_knots: f64,
    course_deg: u16,
    under_keel_clearance_m: f64,
    pilot_required: bool,
    vts_reporting_point: bool,
    notes: String,
}

/// Hull stress monitoring reading.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HullStressReading {
    sensor_location: String,
    timestamp_epoch_ms: u64,
    longitudinal_stress_mpa: f64,
    transverse_stress_mpa: f64,
    shear_stress_mpa: f64,
    bending_moment_kn_m: f64,
    fatigue_damage_index: f64,
    exceeded_threshold: bool,
}

/// Fuel consumption optimization record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FuelOptimization {
    voyage_leg: String,
    distance_nm: f64,
    optimal_speed_knots: f64,
    actual_speed_knots: f64,
    fuel_consumption_mt_day: f64,
    trim_optimization_m: f64,
    hull_fouling_factor: f64,
    weather_factor: f64,
    co2_emissions_tonnes: f64,
    cii_rating: String,
}

/// GMDSS (Global Maritime Distress and Safety System) alert.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GmdssAlertType {
    DistressMayday,
    UrgencyPanPan,
    SafetySecurite,
    SarCoordination { sar_id: String },
    EpfsPiracyAlert,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GmdssAlert {
    alert_type: GmdssAlertType,
    mmsi: u32,
    latitude: f64,
    longitude: f64,
    nature_of_distress: String,
    persons_on_board: u16,
    timestamp_utc: String,
    dsc_acknowledgement: bool,
}

/// Navigation waypoint for an autonomous passage.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NavigationWaypoint {
    name: String,
    latitude: f64,
    longitude: f64,
    planned_speed_knots: f64,
    leg_distance_nm: f64,
    wheel_over_point: bool,
    radius_of_turn_nm: f64,
    cross_track_limit_nm: f64,
}

/// VTS (Vessel Traffic Service) zone definition.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VtsZone {
    zone_id: String,
    zone_name: String,
    authority: String,
    vhf_channel: u8,
    boundary_points: Vec<(f64, f64)>,
    speed_limit_knots: Option<f64>,
    mandatory_reporting: bool,
    tss_active: bool,
}

/// Sea state measurement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeaStateMeasurement {
    timestamp_epoch_ms: u64,
    significant_wave_height_m: f64,
    max_wave_height_m: f64,
    dominant_period_s: f64,
    mean_direction_deg: u16,
    sea_surface_temp_celsius: f64,
    beaufort_scale: u8,
    douglas_sea_state: u8,
    swell_height_m: f64,
    swell_direction_deg: u16,
}

/// Crew fatigue management entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrewFatigueEntry {
    crew_id: String,
    role: String,
    rest_hours_last_24h: f64,
    rest_hours_last_7d: f64,
    watch_start_epoch: u64,
    watch_duration_hours: f64,
    fatigue_risk_score: u8,
    compliant_with_stcw: bool,
    override_reason: Option<String>,
}

// ---------------------------------------------------------------------------
// Test 1: AIS message roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ais_message_roundtrip() {
    let msg = AisMessage {
        mmsi: 211_234_567,
        navigation_status: 0,
        rate_of_turn: -127,
        speed_over_ground: 142,
        longitude: 6_100_000,
        latitude: 51_300_000,
        course_over_ground: 2110,
        true_heading: 211,
        timestamp_seconds: 34,
        vessel_name: "AURORA SPIRIT".to_string(),
    };
    let encoded = encode_with_checksum(&msg).expect("encode AIS message");
    let (decoded, consumed): (AisMessage, _) =
        decode_with_checksum(&encoded).expect("decode AIS message");
    assert_eq!(decoded, msg);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: ECDIS chart cell roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ecdis_chart_cell_roundtrip() {
    let cell = EcdisChartCell {
        cell_id: "GB301245".to_string(),
        edition_number: 14,
        update_number: 3,
        scale: 22_000,
        depth_contours: vec![2.0, 5.0, 10.0, 20.0, 50.0, 100.0],
        hazard_count: 7,
        compilation_date: "2026-01-15".to_string(),
    };
    let encoded = encode_with_checksum(&cell).expect("encode ECDIS cell");
    let (decoded, consumed): (EcdisChartCell, _) =
        decode_with_checksum(&encoded).expect("decode ECDIS cell");
    assert_eq!(decoded, cell);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: COLREG head-on decision roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_colreg_head_on_decision_roundtrip() {
    let decision = ColregDecision {
        own_mmsi: 244_010_123,
        target_mmsi: 636_091_456,
        cpa_nm: 0.35,
        tcpa_minutes: 8.2,
        action: ColregAction::HeadOnAlterStarboard { degrees: 15 },
        confidence: 0.92,
    };
    let encoded = encode_with_checksum(&decision).expect("encode COLREG decision");
    let (decoded, consumed): (ColregDecision, _) =
        decode_with_checksum(&encoded).expect("decode COLREG decision");
    assert_eq!(decoded, decision);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 4: Engine room telemetry roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_engine_room_telemetry_roundtrip() {
    let telemetry = EngineRoomTelemetry {
        main_engine_rpm: 95,
        main_engine_load_pct: 72,
        exhaust_temp_celsius: vec![380, 375, 390, 385, 382, 378],
        lube_oil_pressure_bar: 4.2,
        coolant_temp_celsius: 78.5,
        fuel_oil_temp_celsius: 135.0,
        turbocharger_rpm: 18_500,
        scavenge_air_pressure_bar: 2.8,
        vibration_mm_s: 3.2,
        alarm_codes: vec![],
    };
    let encoded = encode_with_checksum(&telemetry).expect("encode engine telemetry");
    let (decoded, consumed): (EngineRoomTelemetry, _) =
        decode_with_checksum(&encoded).expect("decode engine telemetry");
    assert_eq!(decoded, telemetry);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: Ballast water management roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ballast_water_management_roundtrip() {
    let record = BallastWaterRecord {
        tank_id: "WBT-3P".to_string(),
        capacity_m3: 1250.0,
        current_volume_m3: 980.0,
        salinity_ppt: 35.2,
        temperature_celsius: 18.7,
        treatment_method: "UV_ELECTROLYSIS".to_string(),
        exchange_latitude: 48_500_000,
        exchange_longitude: -5_200_000,
        uv_dose_mj_cm2: 40.0,
        compliant: true,
    };
    let encoded = encode_with_checksum(&record).expect("encode ballast water");
    let (decoded, consumed): (BallastWaterRecord, _) =
        decode_with_checksum(&encoded).expect("decode ballast water");
    assert_eq!(decoded, record);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 6: Weather routing waypoints roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_weather_routing_waypoints_roundtrip() {
    let waypoints: Vec<WeatherRoutingWaypoint> = vec![
        WeatherRoutingWaypoint {
            waypoint_id: 1,
            latitude: 51.45,
            longitude: 3.57,
            eta_epoch_seconds: 1_710_000_000,
            wind_speed_knots: 15.0,
            wind_direction_deg: 225,
            wave_height_m: 1.8,
            wave_period_s: 7.5,
            current_speed_knots: 0.8,
            current_direction_deg: 180,
            visibility_nm: 8.0,
        },
        WeatherRoutingWaypoint {
            waypoint_id: 2,
            latitude: 50.80,
            longitude: 1.10,
            eta_epoch_seconds: 1_710_043_200,
            wind_speed_knots: 22.0,
            wind_direction_deg: 240,
            wave_height_m: 3.2,
            wave_period_s: 9.0,
            current_speed_knots: 1.2,
            current_direction_deg: 200,
            visibility_nm: 5.0,
        },
    ];
    let encoded = encode_with_checksum(&waypoints).expect("encode weather waypoints");
    let (decoded, consumed): (Vec<WeatherRoutingWaypoint>, _) =
        decode_with_checksum(&encoded).expect("decode weather waypoints");
    assert_eq!(decoded, waypoints);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: Cargo loading plan with bay plans roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_cargo_loading_plan_roundtrip() {
    let plan = CargoLoadingPlan {
        voyage_number: "VY2026-0042".to_string(),
        port_of_loading: "NLRTM".to_string(),
        total_teu: 8456,
        reefer_count: 312,
        dangerous_goods_count: 45,
        max_stack_weight_tonnes: 120.0,
        gm_metacentric_height_m: 1.85,
        shear_force_limit_pct: 78.5,
        bending_moment_limit_pct: 82.3,
        bay_plans: vec![
            BayPlan {
                bay_number: 1,
                tier_count: 8,
                row_count: 13,
                container_ids: vec!["MSCU1234567".to_string(), "CMAU7654321".to_string()],
                stack_weights_tonnes: vec![68.5, 72.1],
            },
            BayPlan {
                bay_number: 3,
                tier_count: 10,
                row_count: 15,
                container_ids: vec!["HLCU9988776".to_string()],
                stack_weights_tonnes: vec![95.0],
            },
        ],
    };
    let encoded = encode_with_checksum(&plan).expect("encode cargo loading plan");
    let (decoded, consumed): (CargoLoadingPlan, _) =
        decode_with_checksum(&encoded).expect("decode cargo loading plan");
    assert_eq!(decoded, plan);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: Port approach procedure roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_port_approach_procedure_roundtrip() {
    let steps = vec![
        PortApproachStep {
            sequence: 1,
            waypoint_name: "PILOT_BOARDING".to_string(),
            latitude: 51.9925,
            longitude: 4.0130,
            speed_limit_knots: 12.0,
            course_deg: 85,
            under_keel_clearance_m: 3.5,
            pilot_required: true,
            vts_reporting_point: true,
            notes: "Report to Europoort VTS on VHF Ch 11".to_string(),
        },
        PortApproachStep {
            sequence: 2,
            waypoint_name: "MAAS_CENTER".to_string(),
            latitude: 51.9650,
            longitude: 4.0890,
            speed_limit_knots: 8.0,
            course_deg: 110,
            under_keel_clearance_m: 2.8,
            pilot_required: true,
            vts_reporting_point: false,
            notes: "Tug connection point".to_string(),
        },
    ];
    let encoded = encode_with_checksum(&steps).expect("encode port approach");
    let (decoded, consumed): (Vec<PortApproachStep>, _) =
        decode_with_checksum(&encoded).expect("decode port approach");
    assert_eq!(decoded, steps);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: Hull stress monitoring roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_hull_stress_monitoring_roundtrip() {
    let reading = HullStressReading {
        sensor_location: "FRAME_87_DECK_STBD".to_string(),
        timestamp_epoch_ms: 1_710_000_000_000,
        longitudinal_stress_mpa: 125.4,
        transverse_stress_mpa: 45.8,
        shear_stress_mpa: 32.1,
        bending_moment_kn_m: 4_500_000.0,
        fatigue_damage_index: 0.0032,
        exceeded_threshold: false,
    };
    let encoded = encode_with_checksum(&reading).expect("encode hull stress");
    let (decoded, consumed): (HullStressReading, _) =
        decode_with_checksum(&encoded).expect("decode hull stress");
    assert_eq!(decoded, reading);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: Fuel consumption optimization roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_fuel_optimization_roundtrip() {
    let record = FuelOptimization {
        voyage_leg: "NLRTM_to_SGSIN_leg3".to_string(),
        distance_nm: 1_245.0,
        optimal_speed_knots: 13.5,
        actual_speed_knots: 14.2,
        fuel_consumption_mt_day: 42.8,
        trim_optimization_m: -0.3,
        hull_fouling_factor: 1.12,
        weather_factor: 1.08,
        co2_emissions_tonnes: 135.6,
        cii_rating: "B".to_string(),
    };
    let encoded = encode_with_checksum(&record).expect("encode fuel optimization");
    let (decoded, consumed): (FuelOptimization, _) =
        decode_with_checksum(&encoded).expect("decode fuel optimization");
    assert_eq!(decoded, record);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: GMDSS distress alert roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_gmdss_distress_alert_roundtrip() {
    let alert = GmdssAlert {
        alert_type: GmdssAlertType::DistressMayday,
        mmsi: 311_000_999,
        latitude: 47.3,
        longitude: -8.5,
        nature_of_distress: "FLOODING_ENGINE_ROOM".to_string(),
        persons_on_board: 22,
        timestamp_utc: "2026-03-15T08:30:00Z".to_string(),
        dsc_acknowledgement: false,
    };
    let encoded = encode_with_checksum(&alert).expect("encode GMDSS alert");
    let (decoded, consumed): (GmdssAlert, _) =
        decode_with_checksum(&encoded).expect("decode GMDSS alert");
    assert_eq!(decoded, alert);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: Navigation waypoint sequence roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_navigation_waypoint_sequence_roundtrip() {
    let waypoints = vec![
        NavigationWaypoint {
            name: "WP001_DEPARTURE".to_string(),
            latitude: 51.9500,
            longitude: 4.0500,
            planned_speed_knots: 10.0,
            leg_distance_nm: 0.0,
            wheel_over_point: false,
            radius_of_turn_nm: 0.0,
            cross_track_limit_nm: 0.1,
        },
        NavigationWaypoint {
            name: "WP002_MAAS_WEST".to_string(),
            latitude: 51.9800,
            longitude: 3.8000,
            planned_speed_knots: 14.0,
            leg_distance_nm: 12.5,
            wheel_over_point: true,
            radius_of_turn_nm: 0.5,
            cross_track_limit_nm: 0.2,
        },
        NavigationWaypoint {
            name: "WP003_NORTH_HINDER".to_string(),
            latitude: 51.6500,
            longitude: 2.6000,
            planned_speed_knots: 16.0,
            leg_distance_nm: 58.3,
            wheel_over_point: false,
            radius_of_turn_nm: 0.0,
            cross_track_limit_nm: 0.5,
        },
    ];
    let encoded = encode_with_checksum(&waypoints).expect("encode nav waypoints");
    let (decoded, consumed): (Vec<NavigationWaypoint>, _) =
        decode_with_checksum(&encoded).expect("decode nav waypoints");
    assert_eq!(decoded, waypoints);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: VTS zone definition roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vts_zone_roundtrip() {
    let zone = VtsZone {
        zone_id: "VTS_SINGAPORE_EAST".to_string(),
        zone_name: "Singapore Strait Eastern Sector".to_string(),
        authority: "MPA Singapore".to_string(),
        vhf_channel: 14,
        boundary_points: vec![
            (1.25, 103.80),
            (1.25, 104.20),
            (1.10, 104.20),
            (1.10, 103.80),
        ],
        speed_limit_knots: Some(12.0),
        mandatory_reporting: true,
        tss_active: true,
    };
    let encoded = encode_with_checksum(&zone).expect("encode VTS zone");
    let (decoded, consumed): (VtsZone, _) =
        decode_with_checksum(&encoded).expect("decode VTS zone");
    assert_eq!(decoded, zone);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: Sea state measurement roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_sea_state_measurement_roundtrip() {
    let measurement = SeaStateMeasurement {
        timestamp_epoch_ms: 1_710_100_000_000,
        significant_wave_height_m: 4.5,
        max_wave_height_m: 7.2,
        dominant_period_s: 10.5,
        mean_direction_deg: 270,
        sea_surface_temp_celsius: 12.3,
        beaufort_scale: 7,
        douglas_sea_state: 6,
        swell_height_m: 2.8,
        swell_direction_deg: 290,
    };
    let encoded = encode_with_checksum(&measurement).expect("encode sea state");
    let (decoded, consumed): (SeaStateMeasurement, _) =
        decode_with_checksum(&encoded).expect("decode sea state");
    assert_eq!(decoded, measurement);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Crew fatigue management roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_crew_fatigue_management_roundtrip() {
    let entries = vec![
        CrewFatigueEntry {
            crew_id: "OFF-001".to_string(),
            role: "OOW".to_string(),
            rest_hours_last_24h: 10.5,
            rest_hours_last_7d: 77.0,
            watch_start_epoch: 1_710_000_000,
            watch_duration_hours: 4.0,
            fatigue_risk_score: 2,
            compliant_with_stcw: true,
            override_reason: None,
        },
        CrewFatigueEntry {
            crew_id: "ENG-003".to_string(),
            role: "CHIEF_ENGINEER".to_string(),
            rest_hours_last_24h: 7.0,
            rest_hours_last_7d: 62.0,
            watch_start_epoch: 1_710_014_400,
            watch_duration_hours: 6.0,
            fatigue_risk_score: 5,
            compliant_with_stcw: false,
            override_reason: Some("Emergency engine repair".to_string()),
        },
    ];
    let encoded = encode_with_checksum(&entries).expect("encode crew fatigue");
    let (decoded, consumed): (Vec<CrewFatigueEntry>, _) =
        decode_with_checksum(&encoded).expect("decode crew fatigue");
    assert_eq!(decoded, entries);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: COLREG crossing give-way with restricted visibility roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_colreg_crossing_and_restricted_visibility_roundtrip() {
    let decisions = vec![
        ColregDecision {
            own_mmsi: 244_010_123,
            target_mmsi: 538_006_789,
            cpa_nm: 0.15,
            tcpa_minutes: 3.5,
            action: ColregAction::CrossingGiveWay {
                target_mmsi: 538_006_789,
            },
            confidence: 0.88,
        },
        ColregDecision {
            own_mmsi: 244_010_123,
            target_mmsi: 0,
            cpa_nm: 0.0,
            tcpa_minutes: 0.0,
            action: ColregAction::RestrictedVisibility {
                speed_reduction_pct: 40,
            },
            confidence: 0.95,
        },
    ];
    let encoded = encode_with_checksum(&decisions).expect("encode COLREG decisions");
    let (decoded, consumed): (Vec<ColregDecision>, _) =
        decode_with_checksum(&encoded).expect("decode COLREG decisions");
    assert_eq!(decoded, decisions);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: GMDSS SAR coordination roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_gmdss_sar_coordination_roundtrip() {
    let alert = GmdssAlert {
        alert_type: GmdssAlertType::SarCoordination {
            sar_id: "SAR-2026-ATL-0042".to_string(),
        },
        mmsi: 002_275_100,
        latitude: 48.62,
        longitude: -6.15,
        nature_of_distress: "PERSON_OVERBOARD".to_string(),
        persons_on_board: 1,
        timestamp_utc: "2026-03-15T14:22:00Z".to_string(),
        dsc_acknowledgement: true,
    };
    let encoded = encode_with_checksum(&alert).expect("encode SAR alert");
    let (decoded, consumed): (GmdssAlert, _) =
        decode_with_checksum(&encoded).expect("decode SAR alert");
    assert_eq!(decoded, alert);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 18: Engine telemetry with alarm codes roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_engine_telemetry_with_alarms_roundtrip() {
    let telemetry = EngineRoomTelemetry {
        main_engine_rpm: 45,
        main_engine_load_pct: 30,
        exhaust_temp_celsius: vec![420, 415, 450, 440, 418, 422],
        lube_oil_pressure_bar: 2.1,
        coolant_temp_celsius: 92.0,
        fuel_oil_temp_celsius: 140.0,
        turbocharger_rpm: 9_800,
        scavenge_air_pressure_bar: 1.4,
        vibration_mm_s: 8.7,
        alarm_codes: vec![1001, 2005, 3012, 4001],
    };
    let encoded = encode_with_checksum(&telemetry).expect("encode engine alarms");
    let (decoded, consumed): (EngineRoomTelemetry, _) =
        decode_with_checksum(&encoded).expect("decode engine alarms");
    assert_eq!(decoded, telemetry);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 19: Corruption detection -- flipped payload byte in AIS message
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_ais_payload_flip() {
    let msg = AisMessage {
        mmsi: 123_456_789,
        navigation_status: 5,
        rate_of_turn: 0,
        speed_over_ground: 0,
        longitude: 0,
        latitude: 0,
        course_over_ground: 0,
        true_heading: 0,
        timestamp_seconds: 0,
        vessel_name: "TEST_VESSEL".to_string(),
    };
    let mut encoded = encode_with_checksum(&msg).expect("encode AIS for corruption");
    let flip_idx = HEADER_SIZE + 2;
    if flip_idx < encoded.len() {
        encoded[flip_idx] ^= 0xFF;
    }
    let result = decode_with_checksum::<AisMessage>(&encoded);
    assert!(
        result.is_err(),
        "corrupted AIS payload must produce an error"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Corruption detection -- flipped CRC byte in hull stress reading
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_hull_stress_crc_flip() {
    let reading = HullStressReading {
        sensor_location: "FRAME_42_KEEL".to_string(),
        timestamp_epoch_ms: 1_700_000_000_000,
        longitudinal_stress_mpa: 80.0,
        transverse_stress_mpa: 30.0,
        shear_stress_mpa: 15.0,
        bending_moment_kn_m: 2_000_000.0,
        fatigue_damage_index: 0.001,
        exceeded_threshold: false,
    };
    let mut encoded = encode_with_checksum(&reading).expect("encode hull stress for corruption");
    // CRC32 field is at offset 12..16 in the header
    encoded[12] ^= 0x01;
    let result = decode_with_checksum::<HullStressReading>(&encoded);
    assert!(
        result.is_err(),
        "corrupted CRC must produce a checksum mismatch error"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Corruption detection -- truncated cargo loading plan
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_truncated_cargo_plan() {
    let plan = CargoLoadingPlan {
        voyage_number: "VY2026-0099".to_string(),
        port_of_loading: "DEHAM".to_string(),
        total_teu: 5000,
        reefer_count: 150,
        dangerous_goods_count: 20,
        max_stack_weight_tonnes: 110.0,
        gm_metacentric_height_m: 2.0,
        shear_force_limit_pct: 65.0,
        bending_moment_limit_pct: 70.0,
        bay_plans: vec![BayPlan {
            bay_number: 5,
            tier_count: 6,
            row_count: 11,
            container_ids: vec!["EISU1111111".to_string()],
            stack_weights_tonnes: vec![55.0],
        }],
    };
    let encoded = encode_with_checksum(&plan).expect("encode cargo plan for truncation");
    let truncated = &encoded[..encoded.len() / 2];
    let result = decode_with_checksum::<CargoLoadingPlan>(truncated);
    assert!(result.is_err(), "truncated data must produce an error");
}

// ---------------------------------------------------------------------------
// Test 22: VTS zone with no speed limit (Option::None) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vts_zone_no_speed_limit_roundtrip() {
    let zone = VtsZone {
        zone_id: "VTS_DOVER_STRAIT".to_string(),
        zone_name: "Dover Strait TSS".to_string(),
        authority: "HMCG Dover".to_string(),
        vhf_channel: 11,
        boundary_points: vec![(51.10, 1.30), (51.10, 1.80), (50.90, 1.80), (50.90, 1.30)],
        speed_limit_knots: None,
        mandatory_reporting: true,
        tss_active: true,
    };
    let encoded = encode_with_checksum(&zone).expect("encode VTS zone no speed limit");
    let (decoded, consumed): (VtsZone, _) =
        decode_with_checksum(&encoded).expect("decode VTS zone no speed limit");
    assert_eq!(decoded, zone);
    assert_eq!(consumed, encoded.len());
}
