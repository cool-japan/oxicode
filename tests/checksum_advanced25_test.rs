//! Checksum tests for OxiCode — precision agriculture and smart farming domain.
//!
//! Exactly 22 `#[test]` functions covering soil moisture, crop yield predictions,
//! GPS-guided tractor paths, irrigation zones, drone survey metadata, fertilizer
//! application maps, pest detection alerts, weather station data, grain silo
//! conditions, livestock RFID tracking, milk production records, greenhouse
//! environment controls, harvest scheduling, seed lot traceability, and crop
//! rotation plans.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced25_test

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
// Domain types — precision agriculture & smart farming
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SoilMoistureReading {
    sensor_id: u32,
    field_zone: String,
    depth_cm: f32,
    volumetric_water_content: f64,
    temperature_celsius: f32,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CropYieldPrediction {
    field_id: String,
    crop_type: String,
    predicted_yield_kg_per_ha: f64,
    confidence_interval_lower: f64,
    confidence_interval_upper: f64,
    model_version: u32,
    growing_degree_days: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GpsWaypoint {
    latitude: f64,
    longitude: f64,
    altitude_m: f32,
    speed_kmh: f32,
    heading_deg: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TractorPath {
    tractor_id: String,
    field_name: String,
    waypoints: Vec<GpsWaypoint>,
    implement_width_m: f32,
    pass_number: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IrrigationZoneConfig {
    zone_id: u32,
    zone_name: String,
    sprinkler_count: u16,
    flow_rate_lpm: f32,
    target_moisture_pct: f32,
    schedule_start_hour: u8,
    schedule_duration_min: u16,
    enabled: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DroneSurveyMetadata {
    mission_id: String,
    drone_serial: String,
    altitude_agl_m: f32,
    ground_resolution_cm_px: f32,
    image_count: u32,
    coverage_hectares: f64,
    spectral_bands: Vec<String>,
    overlap_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FertilizerApplication {
    application_id: u64,
    field_id: String,
    product_name: String,
    rate_kg_per_ha: f64,
    nitrogen_pct: f32,
    phosphorus_pct: f32,
    potassium_pct: f32,
    zones: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PestSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PestDetectionAlert {
    alert_id: u64,
    field_id: String,
    pest_species: String,
    severity: PestSeverity,
    affected_area_ha: f64,
    detection_method: String,
    gps_lat: f64,
    gps_lon: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherStationData {
    station_id: String,
    temperature_c: f32,
    humidity_pct: f32,
    wind_speed_ms: f32,
    wind_direction_deg: u16,
    precipitation_mm: f32,
    solar_radiation_wm2: f32,
    atmospheric_pressure_hpa: f32,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GrainSiloCondition {
    silo_id: u32,
    grain_type: String,
    fill_level_pct: f32,
    temperature_c: f32,
    moisture_pct: f32,
    co2_ppm: u16,
    last_inspection_epoch: u64,
    aeration_active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LivestockRfidRecord {
    rfid_tag: String,
    animal_type: String,
    breed: String,
    weight_kg: f32,
    last_location_zone: String,
    vaccination_dates: Vec<u64>,
    birth_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MilkProductionRecord {
    cow_rfid: String,
    session_id: u64,
    volume_liters: f64,
    fat_pct: f32,
    protein_pct: f32,
    somatic_cell_count: u32,
    temperature_c: f32,
    milking_duration_sec: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GreenhouseControl {
    greenhouse_id: String,
    target_temp_c: f32,
    target_humidity_pct: f32,
    co2_setpoint_ppm: u16,
    light_hours_per_day: f32,
    vent_position_pct: u8,
    heating_active: bool,
    cooling_active: bool,
    misting_active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HarvestSchedule {
    field_id: String,
    crop: String,
    planned_date_epoch: u64,
    estimated_yield_tonnes: f64,
    equipment_assigned: Vec<String>,
    priority: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeedLotTrace {
    lot_number: String,
    variety: String,
    origin_farm: String,
    harvest_year: u16,
    germination_rate_pct: f32,
    treatment: String,
    certification_code: String,
    quantity_kg: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CropRotationEntry {
    year: u16,
    season: String,
    crop: String,
    field_id: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CropRotationPlan {
    plan_id: String,
    farm_name: String,
    entries: Vec<CropRotationEntry>,
    notes: String,
}

// ---------------------------------------------------------------------------
// Test 1: Soil moisture sensor readings roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_soil_moisture_reading_roundtrip() {
    let reading = SoilMoistureReading {
        sensor_id: 1042,
        field_zone: "North-A3".to_string(),
        depth_cm: 30.0,
        volumetric_water_content: 0.287,
        temperature_celsius: 18.4,
        timestamp_epoch: 1_700_000_000,
    };
    let encoded = encode_with_checksum(&reading).expect("encode soil moisture failed");
    let (decoded, consumed): (SoilMoistureReading, _) =
        decode_with_checksum(&encoded).expect("decode soil moisture failed");
    assert_eq!(decoded, reading);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: Crop yield prediction roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_crop_yield_prediction_roundtrip() {
    let prediction = CropYieldPrediction {
        field_id: "F-2024-007".to_string(),
        crop_type: "Winter Wheat".to_string(),
        predicted_yield_kg_per_ha: 7850.5,
        confidence_interval_lower: 7200.0,
        confidence_interval_upper: 8500.0,
        model_version: 3,
        growing_degree_days: 1823.7,
    };
    let encoded = encode_with_checksum(&prediction).expect("encode yield prediction failed");
    let (decoded, consumed): (CropYieldPrediction, _) =
        decode_with_checksum(&encoded).expect("decode yield prediction failed");
    assert_eq!(decoded, prediction);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: GPS-guided tractor path roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tractor_path_roundtrip() {
    let path = TractorPath {
        tractor_id: "JD-8400R-03".to_string(),
        field_name: "South Meadow".to_string(),
        waypoints: vec![
            GpsWaypoint {
                latitude: 52.520008,
                longitude: 13.404954,
                altitude_m: 34.2,
                speed_kmh: 8.5,
                heading_deg: 172.3,
            },
            GpsWaypoint {
                latitude: 52.520120,
                longitude: 13.405100,
                altitude_m: 34.5,
                speed_kmh: 8.7,
                heading_deg: 173.1,
            },
            GpsWaypoint {
                latitude: 52.520250,
                longitude: 13.405300,
                altitude_m: 34.8,
                speed_kmh: 8.4,
                heading_deg: 171.9,
            },
        ],
        implement_width_m: 12.0,
        pass_number: 7,
    };
    let encoded = encode_with_checksum(&path).expect("encode tractor path failed");
    let (decoded, consumed): (TractorPath, _) =
        decode_with_checksum(&encoded).expect("decode tractor path failed");
    assert_eq!(decoded, path);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 4: Irrigation zone configuration roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_irrigation_zone_config_roundtrip() {
    let config = IrrigationZoneConfig {
        zone_id: 5,
        zone_name: "Vineyard Block C".to_string(),
        sprinkler_count: 48,
        flow_rate_lpm: 22.5,
        target_moisture_pct: 35.0,
        schedule_start_hour: 4,
        schedule_duration_min: 90,
        enabled: true,
    };
    let encoded = encode_with_checksum(&config).expect("encode irrigation config failed");
    let (decoded, consumed): (IrrigationZoneConfig, _) =
        decode_with_checksum(&encoded).expect("decode irrigation config failed");
    assert_eq!(decoded, config);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: Drone survey imagery metadata roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_drone_survey_metadata_roundtrip() {
    let meta = DroneSurveyMetadata {
        mission_id: "DRONE-2024-0312-A".to_string(),
        drone_serial: "DJI-M300-SN4821".to_string(),
        altitude_agl_m: 120.0,
        ground_resolution_cm_px: 2.5,
        image_count: 1847,
        coverage_hectares: 85.3,
        spectral_bands: vec![
            "Red".to_string(),
            "Green".to_string(),
            "Blue".to_string(),
            "NIR".to_string(),
            "RedEdge".to_string(),
        ],
        overlap_pct: 75,
    };
    let encoded = encode_with_checksum(&meta).expect("encode drone survey metadata failed");
    let (decoded, consumed): (DroneSurveyMetadata, _) =
        decode_with_checksum(&encoded).expect("decode drone survey metadata failed");
    assert_eq!(decoded, meta);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 6: Fertilizer application map roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_fertilizer_application_roundtrip() {
    let app = FertilizerApplication {
        application_id: 90001,
        field_id: "F-NW-12".to_string(),
        product_name: "UAN-32".to_string(),
        rate_kg_per_ha: 185.0,
        nitrogen_pct: 32.0,
        phosphorus_pct: 0.0,
        potassium_pct: 0.0,
        zones: vec![1, 3, 5, 7, 9, 11],
    };
    let encoded = encode_with_checksum(&app).expect("encode fertilizer app failed");
    let (decoded, consumed): (FertilizerApplication, _) =
        decode_with_checksum(&encoded).expect("decode fertilizer app failed");
    assert_eq!(decoded, app);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: Pest detection alert roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_pest_detection_alert_roundtrip() {
    let alert = PestDetectionAlert {
        alert_id: 77234,
        field_id: "CORN-EAST-14".to_string(),
        pest_species: "Diabrotica virgifera (Western Corn Rootworm)".to_string(),
        severity: PestSeverity::High,
        affected_area_ha: 3.2,
        detection_method: "Drone NDVI anomaly + scout confirmation".to_string(),
        gps_lat: 41.878_114,
        gps_lon: -93.097_702,
    };
    let encoded = encode_with_checksum(&alert).expect("encode pest alert failed");
    let (decoded, consumed): (PestDetectionAlert, _) =
        decode_with_checksum(&encoded).expect("decode pest alert failed");
    assert_eq!(decoded, alert);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: Weather station data roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_weather_station_data_roundtrip() {
    let data = WeatherStationData {
        station_id: "WX-FARM-CENTRAL-01".to_string(),
        temperature_c: 24.6,
        humidity_pct: 62.3,
        wind_speed_ms: 3.8,
        wind_direction_deg: 225,
        precipitation_mm: 0.0,
        solar_radiation_wm2: 680.0,
        atmospheric_pressure_hpa: 1013.25,
        timestamp_epoch: 1_710_500_000,
    };
    let encoded = encode_with_checksum(&data).expect("encode weather data failed");
    let (decoded, consumed): (WeatherStationData, _) =
        decode_with_checksum(&encoded).expect("decode weather data failed");
    assert_eq!(decoded, data);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: Grain silo conditions roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grain_silo_condition_roundtrip() {
    let silo = GrainSiloCondition {
        silo_id: 3,
        grain_type: "Hard Red Winter Wheat".to_string(),
        fill_level_pct: 87.5,
        temperature_c: 15.2,
        moisture_pct: 12.8,
        co2_ppm: 420,
        last_inspection_epoch: 1_709_000_000,
        aeration_active: true,
    };
    let encoded = encode_with_checksum(&silo).expect("encode silo condition failed");
    let (decoded, consumed): (GrainSiloCondition, _) =
        decode_with_checksum(&encoded).expect("decode silo condition failed");
    assert_eq!(decoded, silo);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: Livestock RFID tracking roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_livestock_rfid_roundtrip() {
    let record = LivestockRfidRecord {
        rfid_tag: "840003210456789".to_string(),
        animal_type: "Cattle".to_string(),
        breed: "Angus".to_string(),
        weight_kg: 542.0,
        last_location_zone: "Pasture-B".to_string(),
        vaccination_dates: vec![1_680_000_000, 1_695_000_000, 1_710_000_000],
        birth_epoch: 1_620_000_000,
    };
    let encoded = encode_with_checksum(&record).expect("encode livestock record failed");
    let (decoded, consumed): (LivestockRfidRecord, _) =
        decode_with_checksum(&encoded).expect("decode livestock record failed");
    assert_eq!(decoded, record);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: Milk production record roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_milk_production_roundtrip() {
    let record = MilkProductionRecord {
        cow_rfid: "840003210009876".to_string(),
        session_id: 420_001,
        volume_liters: 28.7,
        fat_pct: 3.85,
        protein_pct: 3.22,
        somatic_cell_count: 145_000,
        temperature_c: 37.8,
        milking_duration_sec: 480,
    };
    let encoded = encode_with_checksum(&record).expect("encode milk production failed");
    let (decoded, consumed): (MilkProductionRecord, _) =
        decode_with_checksum(&encoded).expect("decode milk production failed");
    assert_eq!(decoded, record);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: Greenhouse environment controls roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_greenhouse_control_roundtrip() {
    let ctrl = GreenhouseControl {
        greenhouse_id: "GH-TOMATO-02".to_string(),
        target_temp_c: 26.0,
        target_humidity_pct: 70.0,
        co2_setpoint_ppm: 800,
        light_hours_per_day: 16.0,
        vent_position_pct: 45,
        heating_active: false,
        cooling_active: true,
        misting_active: true,
    };
    let encoded = encode_with_checksum(&ctrl).expect("encode greenhouse control failed");
    let (decoded, consumed): (GreenhouseControl, _) =
        decode_with_checksum(&encoded).expect("decode greenhouse control failed");
    assert_eq!(decoded, ctrl);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: Harvest scheduling roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_harvest_schedule_roundtrip() {
    let schedule = HarvestSchedule {
        field_id: "SOY-WEST-09".to_string(),
        crop: "Soybeans".to_string(),
        planned_date_epoch: 1_727_000_000,
        estimated_yield_tonnes: 312.5,
        equipment_assigned: vec![
            "Combine-JD-S780".to_string(),
            "GrainCart-Brent-1196".to_string(),
            "Semi-Trailer-01".to_string(),
        ],
        priority: 2,
    };
    let encoded = encode_with_checksum(&schedule).expect("encode harvest schedule failed");
    let (decoded, consumed): (HarvestSchedule, _) =
        decode_with_checksum(&encoded).expect("decode harvest schedule failed");
    assert_eq!(decoded, schedule);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: Seed lot traceability roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_seed_lot_traceability_roundtrip() {
    let lot = SeedLotTrace {
        lot_number: "SL-2024-WW-0042".to_string(),
        variety: "SY Wolverine".to_string(),
        origin_farm: "Great Plains Seed Co.".to_string(),
        harvest_year: 2024,
        germination_rate_pct: 96.5,
        treatment: "Cruiser Maxx Cereals".to_string(),
        certification_code: "AOSCA-KS-2024-1187".to_string(),
        quantity_kg: 25_000.0,
    };
    let encoded = encode_with_checksum(&lot).expect("encode seed lot failed");
    let (decoded, consumed): (SeedLotTrace, _) =
        decode_with_checksum(&encoded).expect("decode seed lot failed");
    assert_eq!(decoded, lot);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Crop rotation plan roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_crop_rotation_plan_roundtrip() {
    let plan = CropRotationPlan {
        plan_id: "ROT-FARM7-2024".to_string(),
        farm_name: "Heartland Acres".to_string(),
        entries: vec![
            CropRotationEntry {
                year: 2024,
                season: "Spring".to_string(),
                crop: "Corn".to_string(),
                field_id: "F-01".to_string(),
            },
            CropRotationEntry {
                year: 2024,
                season: "Spring".to_string(),
                crop: "Soybeans".to_string(),
                field_id: "F-02".to_string(),
            },
            CropRotationEntry {
                year: 2025,
                season: "Spring".to_string(),
                crop: "Soybeans".to_string(),
                field_id: "F-01".to_string(),
            },
            CropRotationEntry {
                year: 2025,
                season: "Fall".to_string(),
                crop: "Winter Wheat".to_string(),
                field_id: "F-02".to_string(),
            },
        ],
        notes: "Cover crop after soybeans; no-till corn on corn avoided".to_string(),
    };
    let encoded = encode_with_checksum(&plan).expect("encode crop rotation plan failed");
    let (decoded, consumed): (CropRotationPlan, _) =
        decode_with_checksum(&encoded).expect("decode crop rotation plan failed");
    assert_eq!(decoded, plan);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: Corruption detection — soil moisture sensor (payload flip)
// ---------------------------------------------------------------------------
#[test]
fn test_soil_moisture_corruption_detected() {
    let reading = SoilMoistureReading {
        sensor_id: 999,
        field_zone: "East-B1".to_string(),
        depth_cm: 15.0,
        volumetric_water_content: 0.312,
        temperature_celsius: 21.0,
        timestamp_epoch: 1_700_100_000,
    };
    let mut encoded = encode_with_checksum(&reading).expect("encode soil moisture failed");
    // Flip a byte in the payload region
    let flip_idx = HEADER_SIZE + 2;
    if flip_idx < encoded.len() {
        encoded[flip_idx] ^= 0xFF;
    }
    let result: Result<(SoilMoistureReading, usize), _> = decode_with_checksum(&encoded);
    assert!(
        result.is_err(),
        "corrupted soil moisture data must be rejected"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Corruption detection — weather station (magic byte flip)
// ---------------------------------------------------------------------------
#[test]
fn test_weather_station_magic_corruption() {
    let data = WeatherStationData {
        station_id: "WX-RIDGE-05".to_string(),
        temperature_c: -5.2,
        humidity_pct: 88.0,
        wind_speed_ms: 12.4,
        wind_direction_deg: 315,
        precipitation_mm: 4.2,
        solar_radiation_wm2: 120.0,
        atmospheric_pressure_hpa: 998.7,
        timestamp_epoch: 1_710_600_000,
    };
    let mut encoded = encode_with_checksum(&data).expect("encode weather data failed");
    // Corrupt magic byte
    encoded[1] ^= 0x01;
    let result: Result<(WeatherStationData, usize), _> = decode_with_checksum(&encoded);
    assert!(result.is_err(), "corrupted magic must be rejected");
}

// ---------------------------------------------------------------------------
// Test 18: Corruption detection — pest alert (last payload byte flip)
// ---------------------------------------------------------------------------
#[test]
fn test_pest_alert_corruption_last_byte() {
    let alert = PestDetectionAlert {
        alert_id: 88001,
        field_id: "WHEAT-SOUTH-03".to_string(),
        pest_species: "Hessian Fly".to_string(),
        severity: PestSeverity::Medium,
        affected_area_ha: 1.5,
        detection_method: "Pheromone trap monitoring".to_string(),
        gps_lat: 38.627_003,
        gps_lon: -90.199_404,
    };
    let mut encoded = encode_with_checksum(&alert).expect("encode pest alert failed");
    let last = encoded.len() - 1;
    encoded[last] ^= 0xAA;
    let result: Result<(PestDetectionAlert, usize), _> = decode_with_checksum(&encoded);
    assert!(result.is_err(), "corrupted last byte must be detected");
}

// ---------------------------------------------------------------------------
// Test 19: Vec of soil moisture readings roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_soil_moisture_readings_roundtrip() {
    let readings: Vec<SoilMoistureReading> = (0..10)
        .map(|i| SoilMoistureReading {
            sensor_id: 2000 + i,
            field_zone: format!("Zone-{}", i),
            depth_cm: 10.0 + (i as f32) * 5.0,
            volumetric_water_content: 0.20 + (i as f64) * 0.02,
            temperature_celsius: 16.0 + (i as f32) * 0.5,
            timestamp_epoch: 1_700_000_000 + u64::from(i) * 300,
        })
        .collect();
    let encoded = encode_with_checksum(&readings).expect("encode vec soil readings failed");
    let (decoded, consumed): (Vec<SoilMoistureReading>, _) =
        decode_with_checksum(&encoded).expect("decode vec soil readings failed");
    assert_eq!(decoded, readings);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 20: Multiple irrigation zones in a single checksummed payload
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_irrigation_zones_roundtrip() {
    let zones: Vec<IrrigationZoneConfig> = vec![
        IrrigationZoneConfig {
            zone_id: 1,
            zone_name: "Orchard-A".to_string(),
            sprinkler_count: 32,
            flow_rate_lpm: 18.0,
            target_moisture_pct: 40.0,
            schedule_start_hour: 5,
            schedule_duration_min: 60,
            enabled: true,
        },
        IrrigationZoneConfig {
            zone_id: 2,
            zone_name: "Orchard-B".to_string(),
            sprinkler_count: 28,
            flow_rate_lpm: 16.5,
            target_moisture_pct: 38.0,
            schedule_start_hour: 6,
            schedule_duration_min: 45,
            enabled: false,
        },
        IrrigationZoneConfig {
            zone_id: 3,
            zone_name: "Vegetable Patch".to_string(),
            sprinkler_count: 12,
            flow_rate_lpm: 10.0,
            target_moisture_pct: 50.0,
            schedule_start_hour: 4,
            schedule_duration_min: 30,
            enabled: true,
        },
    ];
    let encoded = encode_with_checksum(&zones).expect("encode irrigation zones failed");
    let (decoded, consumed): (Vec<IrrigationZoneConfig>, _) =
        decode_with_checksum(&encoded).expect("decode irrigation zones failed");
    assert_eq!(decoded, zones);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 21: Corruption detection — grain silo (flip middle payload byte)
// ---------------------------------------------------------------------------
#[test]
fn test_grain_silo_corruption_middle_byte() {
    let silo = GrainSiloCondition {
        silo_id: 7,
        grain_type: "Yellow Dent Corn".to_string(),
        fill_level_pct: 95.0,
        temperature_c: 22.1,
        moisture_pct: 14.5,
        co2_ppm: 510,
        last_inspection_epoch: 1_711_000_000,
        aeration_active: false,
    };
    let mut encoded = encode_with_checksum(&silo).expect("encode silo failed");
    let mid = HEADER_SIZE + (encoded.len() - HEADER_SIZE) / 2;
    if mid < encoded.len() {
        encoded[mid] ^= 0x42;
    }
    let result: Result<(GrainSiloCondition, usize), _> = decode_with_checksum(&encoded);
    assert!(result.is_err(), "corrupted middle byte must be detected");
}

// ---------------------------------------------------------------------------
// Test 22: Combined farm snapshot — multiple domain types in a tuple
// ---------------------------------------------------------------------------
#[test]
fn test_combined_farm_snapshot_roundtrip() {
    let snapshot = (
        WeatherStationData {
            station_id: "WX-MAIN".to_string(),
            temperature_c: 19.8,
            humidity_pct: 55.0,
            wind_speed_ms: 2.1,
            wind_direction_deg: 180,
            precipitation_mm: 0.0,
            solar_radiation_wm2: 750.0,
            atmospheric_pressure_hpa: 1015.0,
            timestamp_epoch: 1_710_700_000,
        },
        SoilMoistureReading {
            sensor_id: 5001,
            field_zone: "Central-1".to_string(),
            depth_cm: 20.0,
            volumetric_water_content: 0.295,
            temperature_celsius: 17.5,
            timestamp_epoch: 1_710_700_000,
        },
        GreenhouseControl {
            greenhouse_id: "GH-PEPPER-01".to_string(),
            target_temp_c: 28.0,
            target_humidity_pct: 65.0,
            co2_setpoint_ppm: 900,
            light_hours_per_day: 14.0,
            vent_position_pct: 30,
            heating_active: false,
            cooling_active: false,
            misting_active: true,
        },
    );
    let encoded = encode_with_checksum(&snapshot).expect("encode farm snapshot failed");
    let (decoded, consumed): (
        (WeatherStationData, SoilMoistureReading, GreenhouseControl),
        _,
    ) = decode_with_checksum(&encoded).expect("decode farm snapshot failed");
    assert_eq!(decoded, snapshot);
    assert_eq!(consumed, encoded.len());
}
