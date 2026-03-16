#![cfg(feature = "std")]
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
use std::env::temp_dir;

// ── Smart Cities & Urban Infrastructure Domain Types ────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SignalPhase {
    Red,
    Yellow,
    Green,
    FlashingYellow,
    FlashingRed,
    LeftArrow,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrafficSignalTimingPlan {
    intersection_id: u32,
    plan_name: String,
    phases: Vec<SignalPhase>,
    phase_durations_ms: Vec<u32>,
    cycle_length_ms: u32,
    offset_ms: u32,
    adaptive_enabled: bool,
    min_green_ms: u32,
    max_green_ms: u32,
    all_red_clearance_ms: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ParkingSpotStatus {
    Vacant,
    Occupied,
    Reserved,
    OutOfService,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ParkingOccupancySensor {
    sensor_id: u64,
    lot_name: String,
    spot_number: u16,
    status: ParkingSpotStatus,
    vehicle_detected_timestamp: u64,
    battery_level_pct: u8,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RouteType {
    Bus,
    Subway,
    Rail,
    Ferry,
    CableCar,
    Trolleybus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GtfsStopTime {
    stop_id: String,
    stop_name: String,
    arrival_secs: u32,
    departure_secs: u32,
    stop_sequence: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GtfsTransitSchedule {
    route_id: String,
    route_short_name: String,
    route_long_name: String,
    route_type: RouteType,
    trip_id: String,
    service_id: String,
    direction_id: u8,
    stop_times: Vec<GtfsStopTime>,
    wheelchair_accessible: bool,
    bikes_allowed: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AirQualityReading {
    station_id: u32,
    station_name: String,
    pm25_ugm3: f64,
    pm10_ugm3: f64,
    o3_ppb: f64,
    no2_ppb: f64,
    so2_ppb: f64,
    co_ppm: f64,
    aqi_index: u16,
    timestamp_epoch: u64,
    temperature_c: f32,
    humidity_pct: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LightingMode {
    Off,
    Dim,
    Medium,
    Bright,
    Emergency,
    Adaptive,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartStreetLightConfig {
    light_id: u64,
    pole_number: String,
    mode: LightingMode,
    brightness_pct: u8,
    color_temperature_kelvin: u16,
    motion_sensor_active: bool,
    ambient_light_threshold_lux: f32,
    schedule_on_hour: u8,
    schedule_off_hour: u8,
    power_consumption_watts: f32,
    firmware_version: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaterPressureSensor {
    sensor_id: u32,
    pipe_segment_id: String,
    pressure_kpa: f64,
    flow_rate_lpm: f64,
    temperature_c: f32,
    chlorine_level_ppm: f32,
    turbidity_ntu: f32,
    leak_detected: bool,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WasteCollectionRoute {
    route_id: u32,
    route_name: String,
    vehicle_id: String,
    bin_ids: Vec<u64>,
    bin_fill_levels_pct: Vec<u8>,
    estimated_total_kg: f64,
    start_time_epoch: u64,
    end_time_epoch: u64,
    distance_km: f64,
    num_stops: u16,
    recycling_route: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EmergencyType {
    Fire,
    Medical,
    Police,
    HazMat,
    Rescue,
    TrafficAccident,
    NaturalDisaster,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DispatchPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmergencyDispatchEvent {
    event_id: u64,
    call_timestamp_epoch: u64,
    emergency_type: EmergencyType,
    priority: DispatchPriority,
    caller_phone_hash: u64,
    latitude: f64,
    longitude: f64,
    description: String,
    units_dispatched: Vec<String>,
    response_time_secs: u32,
    resolved: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HvacMode {
    Heating,
    Cooling,
    Auto,
    FanOnly,
    Off,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BuildingEnergyManagement {
    building_id: u32,
    building_name: String,
    floor_count: u8,
    hvac_mode: HvacMode,
    target_temperature_c: f32,
    current_temperature_c: f32,
    total_power_kw: f64,
    solar_generation_kw: f64,
    battery_storage_kwh: f64,
    occupancy_count: u16,
    co2_level_ppm: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NoisePollutionLevel {
    sensor_id: u32,
    location_name: String,
    decibel_avg: f64,
    decibel_peak: f64,
    decibel_min: f64,
    measurement_duration_secs: u32,
    timestamp_epoch: u64,
    frequency_dominant_hz: f32,
    exceeds_limit: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChargerType {
    Level1Ac,
    Level2Ac,
    DcFastChademo,
    DcFastCcs,
    Tesla,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChargerStatus {
    Available,
    InUse,
    Faulted,
    Offline,
    Reserved,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EvChargingStation {
    station_id: u64,
    station_name: String,
    charger_type: ChargerType,
    status: ChargerStatus,
    max_power_kw: f32,
    current_power_kw: f32,
    energy_delivered_kwh: f64,
    session_duration_secs: u32,
    price_per_kwh_cents: u16,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FlowDirection {
    Northbound,
    Southbound,
    Eastbound,
    Westbound,
    Mixed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PedestrianFlowCounter {
    counter_id: u32,
    location_name: String,
    direction: FlowDirection,
    count_15min: u32,
    count_hourly: u32,
    count_daily: u32,
    avg_speed_mps: f32,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartGridLoadBalance {
    grid_zone_id: u32,
    zone_name: String,
    total_demand_mw: f64,
    total_supply_mw: f64,
    renewable_supply_mw: f64,
    fossil_supply_mw: f64,
    battery_discharge_mw: f64,
    frequency_hz: f64,
    voltage_kv: f64,
    load_factor_pct: f32,
    curtailment_mw: f64,
    price_per_mwh_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FloodRiskLevel {
    None,
    Watch,
    Warning,
    Critical,
    Evacuation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UrbanFloodSensor {
    sensor_id: u32,
    location_name: String,
    water_level_cm: f64,
    flow_velocity_mps: f64,
    rainfall_rate_mmh: f64,
    risk_level: FloodRiskLevel,
    drain_capacity_pct: u8,
    upstream_sensor_id: Option<u32>,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PublicWifiAccessPoint {
    ap_id: u64,
    ssid: String,
    location_name: String,
    connected_clients: u16,
    bandwidth_usage_mbps: f64,
    max_bandwidth_mbps: f64,
    uptime_secs: u64,
    signal_strength_dbm: i8,
    channel: u8,
    latitude: f64,
    longitude: f64,
    firmware_version: String,
}

// ── Test 1: Traffic signal timing plan via file I/O ─────────────────────────

#[test]
fn test_traffic_signal_timing_plan_file_io() {
    let path = temp_dir().join(format!("oxicode_fio38_t01_{}.bin", std::process::id()));
    let plan = TrafficSignalTimingPlan {
        intersection_id: 4201,
        plan_name: "Downtown_Main_5th".into(),
        phases: vec![
            SignalPhase::Green,
            SignalPhase::Yellow,
            SignalPhase::Red,
            SignalPhase::LeftArrow,
            SignalPhase::Green,
        ],
        phase_durations_ms: vec![30_000, 4_000, 2_000, 15_000, 25_000],
        cycle_length_ms: 120_000,
        offset_ms: 8_000,
        adaptive_enabled: true,
        min_green_ms: 10_000,
        max_green_ms: 60_000,
        all_red_clearance_ms: 2_500,
    };
    encode_to_file(&plan, &path).expect("encode traffic signal plan to file");
    let decoded: TrafficSignalTimingPlan =
        decode_from_file(&path).expect("decode traffic signal plan from file");
    assert_eq!(plan, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 2: Parking occupancy sensor via slice ──────────────────────────────

#[test]
fn test_parking_occupancy_sensor_slice() {
    let sensor = ParkingOccupancySensor {
        sensor_id: 900_100,
        lot_name: "City_Hall_Garage_B2".into(),
        spot_number: 147,
        status: ParkingSpotStatus::Occupied,
        vehicle_detected_timestamp: 1_700_000_000,
        battery_level_pct: 83,
        latitude: 40.712_776,
        longitude: -74.005_974,
    };
    let bytes = encode_to_vec(&sensor).expect("encode parking sensor to vec");
    let (decoded, _): (ParkingOccupancySensor, _) =
        decode_from_slice(&bytes).expect("decode parking sensor from slice");
    assert_eq!(sensor, decoded);
}

// ── Test 3: GTFS transit schedule via file I/O ──────────────────────────────

#[test]
fn test_gtfs_transit_schedule_file_io() {
    let path = temp_dir().join(format!("oxicode_fio38_t03_{}.bin", std::process::id()));
    let schedule = GtfsTransitSchedule {
        route_id: "B42".into(),
        route_short_name: "42".into(),
        route_long_name: "Downtown Express via Main Street".into(),
        route_type: RouteType::Bus,
        trip_id: "T-42-0800-WD".into(),
        service_id: "WEEKDAY".into(),
        direction_id: 0,
        stop_times: vec![
            GtfsStopTime {
                stop_id: "S100".into(),
                stop_name: "Central Station".into(),
                arrival_secs: 28_800,
                departure_secs: 28_860,
                stop_sequence: 1,
            },
            GtfsStopTime {
                stop_id: "S105".into(),
                stop_name: "Market Square".into(),
                arrival_secs: 29_400,
                departure_secs: 29_430,
                stop_sequence: 2,
            },
            GtfsStopTime {
                stop_id: "S112".into(),
                stop_name: "University Campus".into(),
                arrival_secs: 30_000,
                departure_secs: 30_060,
                stop_sequence: 3,
            },
        ],
        wheelchair_accessible: true,
        bikes_allowed: false,
    };
    encode_to_file(&schedule, &path).expect("encode GTFS schedule to file");
    let decoded: GtfsTransitSchedule =
        decode_from_file(&path).expect("decode GTFS schedule from file");
    assert_eq!(schedule, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 4: Air quality monitoring via slice ────────────────────────────────

#[test]
fn test_air_quality_reading_slice() {
    let reading = AirQualityReading {
        station_id: 7700,
        station_name: "Industrial_District_AQ3".into(),
        pm25_ugm3: 35.4,
        pm10_ugm3: 72.1,
        o3_ppb: 48.3,
        no2_ppb: 22.7,
        so2_ppb: 5.1,
        co_ppm: 0.8,
        aqi_index: 101,
        timestamp_epoch: 1_710_000_000,
        temperature_c: 28.5,
        humidity_pct: 65.0,
    };
    let bytes = encode_to_vec(&reading).expect("encode air quality to vec");
    let (decoded, _): (AirQualityReading, _) =
        decode_from_slice(&bytes).expect("decode air quality from slice");
    assert_eq!(reading, decoded);
}

// ── Test 5: Smart street lighting config via file I/O ───────────────────────

#[test]
fn test_smart_street_light_config_file_io() {
    let path = temp_dir().join(format!("oxicode_fio38_t05_{}.bin", std::process::id()));
    let config = SmartStreetLightConfig {
        light_id: 550_001,
        pole_number: "P-ELM-2247".into(),
        mode: LightingMode::Adaptive,
        brightness_pct: 70,
        color_temperature_kelvin: 4000,
        motion_sensor_active: true,
        ambient_light_threshold_lux: 15.0,
        schedule_on_hour: 18,
        schedule_off_hour: 6,
        power_consumption_watts: 42.5,
        firmware_version: "3.2.1-rc4".into(),
    };
    encode_to_file(&config, &path).expect("encode street light config to file");
    let decoded: SmartStreetLightConfig =
        decode_from_file(&path).expect("decode street light config from file");
    assert_eq!(config, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 6: Water distribution pressure sensor via slice ────────────────────

#[test]
fn test_water_pressure_sensor_slice() {
    let sensor = WaterPressureSensor {
        sensor_id: 3300,
        pipe_segment_id: "WM-ZONE4-SEG-88".into(),
        pressure_kpa: 345.67,
        flow_rate_lpm: 1200.5,
        temperature_c: 12.3,
        chlorine_level_ppm: 0.45,
        turbidity_ntu: 1.2,
        leak_detected: false,
        timestamp_epoch: 1_700_100_000,
    };
    let bytes = encode_to_vec(&sensor).expect("encode water pressure to vec");
    let (decoded, _): (WaterPressureSensor, _) =
        decode_from_slice(&bytes).expect("decode water pressure from slice");
    assert_eq!(sensor, decoded);
}

// ── Test 7: Waste collection route via file I/O ─────────────────────────────

#[test]
fn test_waste_collection_route_file_io() {
    let path = temp_dir().join(format!("oxicode_fio38_t07_{}.bin", std::process::id()));
    let route = WasteCollectionRoute {
        route_id: 210,
        route_name: "NorthEast_Residential_A".into(),
        vehicle_id: "WC-TRUCK-044".into(),
        bin_ids: vec![10001, 10002, 10003, 10004, 10005, 10006],
        bin_fill_levels_pct: vec![85, 92, 45, 78, 60, 100],
        estimated_total_kg: 2850.0,
        start_time_epoch: 1_700_020_800,
        end_time_epoch: 1_700_038_800,
        distance_km: 34.7,
        num_stops: 48,
        recycling_route: false,
    };
    encode_to_file(&route, &path).expect("encode waste collection route to file");
    let decoded: WasteCollectionRoute =
        decode_from_file(&path).expect("decode waste collection route from file");
    assert_eq!(route, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 8: Emergency dispatch event via slice ──────────────────────────────

#[test]
fn test_emergency_dispatch_event_slice() {
    let event = EmergencyDispatchEvent {
        event_id: 20260315_001,
        call_timestamp_epoch: 1_710_500_000,
        emergency_type: EmergencyType::TrafficAccident,
        priority: DispatchPriority::High,
        caller_phone_hash: 0xDEAD_BEEF_CAFE_1234,
        latitude: 51.507_351,
        longitude: -0.127_758,
        description: "Multi-vehicle collision at intersection of Oak and 3rd".into(),
        units_dispatched: vec!["Engine-7".into(), "Medic-3".into(), "PD-Unit-14".into()],
        response_time_secs: 287,
        resolved: false,
    };
    let bytes = encode_to_vec(&event).expect("encode emergency dispatch to vec");
    let (decoded, _): (EmergencyDispatchEvent, _) =
        decode_from_slice(&bytes).expect("decode emergency dispatch from slice");
    assert_eq!(event, decoded);
}

// ── Test 9: Building energy management via file I/O ─────────────────────────

#[test]
fn test_building_energy_management_file_io() {
    let path = temp_dir().join(format!("oxicode_fio38_t09_{}.bin", std::process::id()));
    let bems = BuildingEnergyManagement {
        building_id: 501,
        building_name: "Civic_Center_Tower_A".into(),
        floor_count: 22,
        hvac_mode: HvacMode::Cooling,
        target_temperature_c: 22.0,
        current_temperature_c: 23.8,
        total_power_kw: 485.3,
        solar_generation_kw: 120.5,
        battery_storage_kwh: 850.0,
        occupancy_count: 1230,
        co2_level_ppm: 620,
    };
    encode_to_file(&bems, &path).expect("encode building energy to file");
    let decoded: BuildingEnergyManagement =
        decode_from_file(&path).expect("decode building energy from file");
    assert_eq!(bems, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 10: Noise pollution levels via slice ───────────────────────────────

#[test]
fn test_noise_pollution_level_slice() {
    let noise = NoisePollutionLevel {
        sensor_id: 8801,
        location_name: "Airport_Approach_Corridor_NW".into(),
        decibel_avg: 72.4,
        decibel_peak: 94.1,
        decibel_min: 48.6,
        measurement_duration_secs: 3600,
        timestamp_epoch: 1_710_600_000,
        frequency_dominant_hz: 250.0,
        exceeds_limit: true,
    };
    let bytes = encode_to_vec(&noise).expect("encode noise pollution to vec");
    let (decoded, _): (NoisePollutionLevel, _) =
        decode_from_slice(&bytes).expect("decode noise pollution from slice");
    assert_eq!(noise, decoded);
}

// ── Test 11: EV charging station via file I/O ───────────────────────────────

#[test]
fn test_ev_charging_station_file_io() {
    let path = temp_dir().join(format!("oxicode_fio38_t11_{}.bin", std::process::id()));
    let station = EvChargingStation {
        station_id: 660_042,
        station_name: "GreenPark_DC_Fast_Hub".into(),
        charger_type: ChargerType::DcFastCcs,
        status: ChargerStatus::InUse,
        max_power_kw: 150.0,
        current_power_kw: 132.7,
        energy_delivered_kwh: 42.3,
        session_duration_secs: 1140,
        price_per_kwh_cents: 35,
        latitude: 37.774_929,
        longitude: -122.419_416,
    };
    encode_to_file(&station, &path).expect("encode EV charging station to file");
    let decoded: EvChargingStation =
        decode_from_file(&path).expect("decode EV charging station from file");
    assert_eq!(station, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 12: Pedestrian flow counter via slice ──────────────────────────────

#[test]
fn test_pedestrian_flow_counter_slice() {
    let counter = PedestrianFlowCounter {
        counter_id: 1200,
        location_name: "High_Street_Bridge_Entrance".into(),
        direction: FlowDirection::Northbound,
        count_15min: 347,
        count_hourly: 1423,
        count_daily: 18_500,
        avg_speed_mps: 1.35,
        timestamp_epoch: 1_710_700_000,
    };
    let bytes = encode_to_vec(&counter).expect("encode pedestrian flow to vec");
    let (decoded, _): (PedestrianFlowCounter, _) =
        decode_from_slice(&bytes).expect("decode pedestrian flow from slice");
    assert_eq!(counter, decoded);
}

// ── Test 13: Smart grid load balancing via file I/O ─────────────────────────

#[test]
fn test_smart_grid_load_balance_file_io() {
    let path = temp_dir().join(format!("oxicode_fio38_t13_{}.bin", std::process::id()));
    let grid = SmartGridLoadBalance {
        grid_zone_id: 15,
        zone_name: "Metropolitan_East_Substation_G".into(),
        total_demand_mw: 2450.8,
        total_supply_mw: 2510.3,
        renewable_supply_mw: 870.2,
        fossil_supply_mw: 1340.1,
        battery_discharge_mw: 300.0,
        frequency_hz: 59.998,
        voltage_kv: 138.5,
        load_factor_pct: 87.3,
        curtailment_mw: 59.5,
        price_per_mwh_cents: 4520,
    };
    encode_to_file(&grid, &path).expect("encode smart grid to file");
    let decoded: SmartGridLoadBalance =
        decode_from_file(&path).expect("decode smart grid from file");
    assert_eq!(grid, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 14: Urban flood sensor via slice ───────────────────────────────────

#[test]
fn test_urban_flood_sensor_slice() {
    let flood = UrbanFloodSensor {
        sensor_id: 4401,
        location_name: "Riverside_Underpass_Drain_7".into(),
        water_level_cm: 45.8,
        flow_velocity_mps: 2.1,
        rainfall_rate_mmh: 38.4,
        risk_level: FloodRiskLevel::Warning,
        drain_capacity_pct: 82,
        upstream_sensor_id: Some(4399),
        timestamp_epoch: 1_710_800_000,
    };
    let bytes = encode_to_vec(&flood).expect("encode flood sensor to vec");
    let (decoded, _): (UrbanFloodSensor, _) =
        decode_from_slice(&bytes).expect("decode flood sensor from slice");
    assert_eq!(flood, decoded);
}

// ── Test 15: Public WiFi access point via file I/O ──────────────────────────

#[test]
fn test_public_wifi_access_point_file_io() {
    let path = temp_dir().join(format!("oxicode_fio38_t15_{}.bin", std::process::id()));
    let ap = PublicWifiAccessPoint {
        ap_id: 77_000_100,
        ssid: "CityFreeWiFi".into(),
        location_name: "Central_Library_2F_Reading_Room".into(),
        connected_clients: 87,
        bandwidth_usage_mbps: 245.3,
        max_bandwidth_mbps: 500.0,
        uptime_secs: 2_592_000,
        signal_strength_dbm: -42,
        channel: 36,
        latitude: 48.856_614,
        longitude: 2.352_222,
        firmware_version: "AP-OS-5.1.2".into(),
    };
    encode_to_file(&ap, &path).expect("encode WiFi AP to file");
    let decoded: PublicWifiAccessPoint = decode_from_file(&path).expect("decode WiFi AP from file");
    assert_eq!(ap, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 16: Multiple parking sensors batch via file I/O ────────────────────

#[test]
fn test_parking_sensor_batch_file_io() {
    let path = temp_dir().join(format!("oxicode_fio38_t16_{}.bin", std::process::id()));
    let sensors: Vec<ParkingOccupancySensor> = (0..50)
        .map(|i| ParkingOccupancySensor {
            sensor_id: 200_000 + i as u64,
            lot_name: format!("Metro_Garage_Level_{}", i / 10 + 1),
            spot_number: i as u16 + 1,
            status: if i % 3 == 0 {
                ParkingSpotStatus::Vacant
            } else if i % 3 == 1 {
                ParkingSpotStatus::Occupied
            } else {
                ParkingSpotStatus::Reserved
            },
            vehicle_detected_timestamp: 1_700_000_000 + i as u64 * 60,
            battery_level_pct: (90 - i % 40) as u8,
            latitude: 35.689_487 + i as f64 * 0.0001,
            longitude: 139.691_706 + i as f64 * 0.0001,
        })
        .collect();
    encode_to_file(&sensors, &path).expect("encode parking sensor batch to file");
    let decoded: Vec<ParkingOccupancySensor> =
        decode_from_file(&path).expect("decode parking sensor batch from file");
    assert_eq!(sensors, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 17: Emergency dispatch events vector via slice ─────────────────────

#[test]
fn test_emergency_dispatch_batch_slice() {
    let events = vec![
        EmergencyDispatchEvent {
            event_id: 100_001,
            call_timestamp_epoch: 1_710_500_100,
            emergency_type: EmergencyType::Fire,
            priority: DispatchPriority::Critical,
            caller_phone_hash: 0xAAAA_BBBB_CCCC_0001,
            latitude: 40.758_896,
            longitude: -73.985_130,
            description: "Structure fire reported at warehouse complex".into(),
            units_dispatched: vec![
                "Engine-1".into(),
                "Engine-5".into(),
                "Ladder-2".into(),
                "BC-1".into(),
            ],
            response_time_secs: 195,
            resolved: false,
        },
        EmergencyDispatchEvent {
            event_id: 100_002,
            call_timestamp_epoch: 1_710_500_400,
            emergency_type: EmergencyType::Medical,
            priority: DispatchPriority::High,
            caller_phone_hash: 0xAAAA_BBBB_CCCC_0002,
            latitude: 40.730_610,
            longitude: -73.935_242,
            description: "Cardiac arrest, elderly patient, bystander CPR in progress".into(),
            units_dispatched: vec!["Medic-8".into(), "Engine-12".into()],
            response_time_secs: 312,
            resolved: true,
        },
        EmergencyDispatchEvent {
            event_id: 100_003,
            call_timestamp_epoch: 1_710_500_700,
            emergency_type: EmergencyType::HazMat,
            priority: DispatchPriority::Critical,
            caller_phone_hash: 0xAAAA_BBBB_CCCC_0003,
            latitude: 40.741_895,
            longitude: -73.989_308,
            description: "Chemical spill on industrial loading dock".into(),
            units_dispatched: vec!["HazMat-1".into(), "Engine-3".into(), "PD-Unit-22".into()],
            response_time_secs: 420,
            resolved: false,
        },
    ];
    let bytes = encode_to_vec(&events).expect("encode emergency batch to vec");
    let (decoded, _): (Vec<EmergencyDispatchEvent>, _) =
        decode_from_slice(&bytes).expect("decode emergency batch from slice");
    assert_eq!(events, decoded);
}

// ── Test 18: Flood sensor with upstream chain via file I/O ──────────────────

#[test]
fn test_flood_sensor_chain_file_io() {
    let path = temp_dir().join(format!("oxicode_fio38_t18_{}.bin", std::process::id()));
    let chain: Vec<UrbanFloodSensor> = vec![
        UrbanFloodSensor {
            sensor_id: 5000,
            location_name: "Hilltop_Creek_Origin".into(),
            water_level_cm: 12.3,
            flow_velocity_mps: 0.5,
            rainfall_rate_mmh: 55.0,
            risk_level: FloodRiskLevel::Watch,
            drain_capacity_pct: 30,
            upstream_sensor_id: None,
            timestamp_epoch: 1_710_900_000,
        },
        UrbanFloodSensor {
            sensor_id: 5001,
            location_name: "Mid_Valley_Culvert".into(),
            water_level_cm: 67.9,
            flow_velocity_mps: 3.8,
            rainfall_rate_mmh: 55.0,
            risk_level: FloodRiskLevel::Warning,
            drain_capacity_pct: 75,
            upstream_sensor_id: Some(5000),
            timestamp_epoch: 1_710_900_000,
        },
        UrbanFloodSensor {
            sensor_id: 5002,
            location_name: "Downtown_Storm_Basin".into(),
            water_level_cm: 110.5,
            flow_velocity_mps: 5.2,
            rainfall_rate_mmh: 55.0,
            risk_level: FloodRiskLevel::Critical,
            drain_capacity_pct: 95,
            upstream_sensor_id: Some(5001),
            timestamp_epoch: 1_710_900_000,
        },
    ];
    encode_to_file(&chain, &path).expect("encode flood sensor chain to file");
    let decoded: Vec<UrbanFloodSensor> =
        decode_from_file(&path).expect("decode flood sensor chain from file");
    assert_eq!(chain, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 19: Building energy management across multiple buildings via slice ──

#[test]
fn test_building_energy_portfolio_slice() {
    let portfolio = vec![
        BuildingEnergyManagement {
            building_id: 601,
            building_name: "Innovation_Hub_West".into(),
            floor_count: 8,
            hvac_mode: HvacMode::Auto,
            target_temperature_c: 21.5,
            current_temperature_c: 21.3,
            total_power_kw: 210.4,
            solar_generation_kw: 85.0,
            battery_storage_kwh: 400.0,
            occupancy_count: 450,
            co2_level_ppm: 520,
        },
        BuildingEnergyManagement {
            building_id: 602,
            building_name: "Convention_Center_Main".into(),
            floor_count: 3,
            hvac_mode: HvacMode::Heating,
            target_temperature_c: 20.0,
            current_temperature_c: 18.7,
            total_power_kw: 780.0,
            solar_generation_kw: 200.0,
            battery_storage_kwh: 1200.0,
            occupancy_count: 3200,
            co2_level_ppm: 890,
        },
    ];
    let bytes = encode_to_vec(&portfolio).expect("encode building portfolio to vec");
    let (decoded, _): (Vec<BuildingEnergyManagement>, _) =
        decode_from_slice(&bytes).expect("decode building portfolio from slice");
    assert_eq!(portfolio, decoded);
}

// ── Test 20: Mixed EV charger fleet via file I/O ────────────────────────────

#[test]
fn test_ev_charger_fleet_file_io() {
    let path = temp_dir().join(format!("oxicode_fio38_t20_{}.bin", std::process::id()));
    let fleet = vec![
        EvChargingStation {
            station_id: 770_001,
            station_name: "Airport_Terminal_L2_Bay1".into(),
            charger_type: ChargerType::Level2Ac,
            status: ChargerStatus::Available,
            max_power_kw: 19.2,
            current_power_kw: 0.0,
            energy_delivered_kwh: 0.0,
            session_duration_secs: 0,
            price_per_kwh_cents: 22,
            latitude: 33.942_536,
            longitude: -118.408_075,
        },
        EvChargingStation {
            station_id: 770_002,
            station_name: "Airport_Terminal_DC_Fast_1".into(),
            charger_type: ChargerType::DcFastChademo,
            status: ChargerStatus::InUse,
            max_power_kw: 62.5,
            current_power_kw: 58.2,
            energy_delivered_kwh: 28.1,
            session_duration_secs: 1800,
            price_per_kwh_cents: 40,
            latitude: 33.942_600,
            longitude: -118.408_100,
        },
        EvChargingStation {
            station_id: 770_003,
            station_name: "Airport_Employee_Tesla".into(),
            charger_type: ChargerType::Tesla,
            status: ChargerStatus::Faulted,
            max_power_kw: 250.0,
            current_power_kw: 0.0,
            energy_delivered_kwh: 0.0,
            session_duration_secs: 0,
            price_per_kwh_cents: 30,
            latitude: 33.942_700,
            longitude: -118.408_200,
        },
    ];
    encode_to_file(&fleet, &path).expect("encode EV fleet to file");
    let decoded: Vec<EvChargingStation> =
        decode_from_file(&path).expect("decode EV fleet from file");
    assert_eq!(fleet, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 21: Smart grid with curtailment scenario via slice ─────────────────

#[test]
fn test_smart_grid_curtailment_scenario_slice() {
    let zones = vec![
        SmartGridLoadBalance {
            grid_zone_id: 30,
            zone_name: "Solar_Farm_Corridor_A".into(),
            total_demand_mw: 800.0,
            total_supply_mw: 1200.0,
            renewable_supply_mw: 1050.0,
            fossil_supply_mw: 0.0,
            battery_discharge_mw: 150.0,
            frequency_hz: 60.002,
            voltage_kv: 69.0,
            load_factor_pct: 66.7,
            curtailment_mw: 400.0,
            price_per_mwh_cents: 1200,
        },
        SmartGridLoadBalance {
            grid_zone_id: 31,
            zone_name: "Wind_Farm_Offshore_B".into(),
            total_demand_mw: 1500.0,
            total_supply_mw: 1480.0,
            renewable_supply_mw: 980.0,
            fossil_supply_mw: 500.0,
            battery_discharge_mw: 0.0,
            frequency_hz: 59.995,
            voltage_kv: 230.0,
            load_factor_pct: 98.7,
            curtailment_mw: 0.0,
            price_per_mwh_cents: 6800,
        },
    ];
    let bytes = encode_to_vec(&zones).expect("encode grid zones to vec");
    let (decoded, _): (Vec<SmartGridLoadBalance>, _) =
        decode_from_slice(&bytes).expect("decode grid zones from slice");
    assert_eq!(zones, decoded);
}

// ── Test 22: Full city dashboard snapshot via file I/O ──────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CityDashboardSnapshot {
    snapshot_epoch: u64,
    city_name: String,
    traffic_plans: Vec<TrafficSignalTimingPlan>,
    air_quality: Vec<AirQualityReading>,
    street_lights: Vec<SmartStreetLightConfig>,
    water_sensors: Vec<WaterPressureSensor>,
    noise_levels: Vec<NoisePollutionLevel>,
    pedestrian_counters: Vec<PedestrianFlowCounter>,
    flood_sensors: Vec<UrbanFloodSensor>,
    wifi_access_points: Vec<PublicWifiAccessPoint>,
}

#[test]
fn test_city_dashboard_snapshot_file_io() {
    let path = temp_dir().join(format!("oxicode_fio38_t22_{}.bin", std::process::id()));
    let snapshot = CityDashboardSnapshot {
        snapshot_epoch: 1_711_000_000,
        city_name: "MetroCity_Central_Dashboard".into(),
        traffic_plans: vec![TrafficSignalTimingPlan {
            intersection_id: 9001,
            plan_name: "Broadway_7th_Peak".into(),
            phases: vec![SignalPhase::Green, SignalPhase::Yellow, SignalPhase::Red],
            phase_durations_ms: vec![35_000, 5_000, 40_000],
            cycle_length_ms: 80_000,
            offset_ms: 0,
            adaptive_enabled: true,
            min_green_ms: 15_000,
            max_green_ms: 50_000,
            all_red_clearance_ms: 2_000,
        }],
        air_quality: vec![AirQualityReading {
            station_id: 9100,
            station_name: "City_Center_AQ".into(),
            pm25_ugm3: 18.2,
            pm10_ugm3: 34.5,
            o3_ppb: 32.0,
            no2_ppb: 15.4,
            so2_ppb: 2.8,
            co_ppm: 0.3,
            aqi_index: 63,
            timestamp_epoch: 1_711_000_000,
            temperature_c: 19.5,
            humidity_pct: 55.0,
        }],
        street_lights: vec![SmartStreetLightConfig {
            light_id: 9200,
            pole_number: "P-BWAY-001".into(),
            mode: LightingMode::Bright,
            brightness_pct: 100,
            color_temperature_kelvin: 5000,
            motion_sensor_active: false,
            ambient_light_threshold_lux: 5.0,
            schedule_on_hour: 17,
            schedule_off_hour: 7,
            power_consumption_watts: 65.0,
            firmware_version: "4.0.0".into(),
        }],
        water_sensors: vec![WaterPressureSensor {
            sensor_id: 9300,
            pipe_segment_id: "WM-MAIN-001".into(),
            pressure_kpa: 400.0,
            flow_rate_lpm: 5000.0,
            temperature_c: 10.0,
            chlorine_level_ppm: 0.5,
            turbidity_ntu: 0.8,
            leak_detected: false,
            timestamp_epoch: 1_711_000_000,
        }],
        noise_levels: vec![NoisePollutionLevel {
            sensor_id: 9400,
            location_name: "Times_Square_North".into(),
            decibel_avg: 78.3,
            decibel_peak: 102.0,
            decibel_min: 62.1,
            measurement_duration_secs: 900,
            timestamp_epoch: 1_711_000_000,
            frequency_dominant_hz: 500.0,
            exceeds_limit: true,
        }],
        pedestrian_counters: vec![PedestrianFlowCounter {
            counter_id: 9500,
            location_name: "Central_Park_South_Gate".into(),
            direction: FlowDirection::Mixed,
            count_15min: 580,
            count_hourly: 2300,
            count_daily: 28_000,
            avg_speed_mps: 1.2,
            timestamp_epoch: 1_711_000_000,
        }],
        flood_sensors: vec![UrbanFloodSensor {
            sensor_id: 9600,
            location_name: "Subway_Vent_42nd_St".into(),
            water_level_cm: 2.1,
            flow_velocity_mps: 0.0,
            rainfall_rate_mmh: 0.0,
            risk_level: FloodRiskLevel::None,
            drain_capacity_pct: 5,
            upstream_sensor_id: None,
            timestamp_epoch: 1_711_000_000,
        }],
        wifi_access_points: vec![PublicWifiAccessPoint {
            ap_id: 9700,
            ssid: "MetroCity_Free".into(),
            location_name: "Grand_Central_Main_Hall".into(),
            connected_clients: 312,
            bandwidth_usage_mbps: 890.5,
            max_bandwidth_mbps: 1000.0,
            uptime_secs: 5_184_000,
            signal_strength_dbm: -35,
            channel: 149,
            latitude: 40.752_726,
            longitude: -73.977_229,
            firmware_version: "AP-OS-6.0.1".into(),
        }],
    };
    encode_to_file(&snapshot, &path).expect("encode city dashboard to file");
    let decoded: CityDashboardSnapshot =
        decode_from_file(&path).expect("decode city dashboard from file");
    assert_eq!(snapshot, decoded);
    std::fs::remove_file(&path).ok();
}
