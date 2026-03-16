#![cfg(all(feature = "compression-lz4", feature = "compression-zstd"))]

//! Advanced mixed-compression tests for OxiCode using smart city infrastructure domain.
//!
//! Covers traffic sensors, air quality monitors, noise sensors, waste management,
//! street lighting, public Wi-Fi, crowd density, energy consumption, and water meters.
//! All tests exercise both LZ4 and Zstd compression together.

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

// ──────────────────────────────────────────────────────────────────────────────
// Domain types
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TrafficCondition {
    FreeFlow,
    Moderate,
    HeavyCongestion,
    StandstillJam,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrafficSensorReading {
    sensor_id: u32,
    timestamp_unix: u64,
    vehicle_count: u32,
    average_speed_kmh: f32,
    occupancy_percent: f32,
    condition: TrafficCondition,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AirQualityIndex {
    Good,
    Moderate,
    UnhealthyForSensitiveGroups,
    Unhealthy,
    VeryUnhealthy,
    Hazardous,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AirQualityReading {
    station_id: u32,
    timestamp_unix: u64,
    pm2_5_ug_m3: f32,
    pm10_ug_m3: f32,
    no2_ppb: f32,
    o3_ppb: f32,
    aqi: AirQualityIndex,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NoiseLevelCategory {
    Quiet,
    Moderate,
    Loud,
    VeryLoud,
    Harmful,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NoiseSensorRecord {
    sensor_id: u32,
    location_lat: f64,
    location_lon: f64,
    decibels: f32,
    category: NoiseLevelCategory,
    peak_event_count: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WasteBinStatus {
    Empty,
    PartiallyFull,
    NearlyFull,
    Overflowing,
    SensorFault,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WasteBinTelemetry {
    bin_id: u32,
    zone_code: String,
    fill_level_percent: u8,
    weight_kg: f32,
    status: WasteBinStatus,
    last_emptied_unix: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LightingMode {
    Off,
    Dim(u8),
    FullBrightness,
    AdaptiveMotion,
    ScheduledOverride,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StreetLightRecord {
    pole_id: u32,
    district: String,
    mode: LightingMode,
    power_watts: f32,
    cumulative_kwh: f64,
    fault_code: Option<u16>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WifiHotspotStatus {
    Active,
    Congested,
    Maintenance,
    Offline,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PublicWifiSnapshot {
    hotspot_id: u32,
    ssid: String,
    connected_clients: u16,
    bandwidth_mbps_down: f32,
    bandwidth_mbps_up: f32,
    status: WifiHotspotStatus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrowdDensityMeasurement {
    zone_id: u32,
    timestamp_unix: u64,
    estimated_people: u32,
    density_per_sqm: f32,
    alert_triggered: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnergyConsumptionRecord {
    meter_id: u32,
    building_type: String,
    kwh_consumed: f64,
    peak_kw_demand: f32,
    hour_of_day: u8,
    day_of_week: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaterMeterReading {
    meter_id: u32,
    district: String,
    cubic_meters: f64,
    flow_rate_lps: f32,
    pressure_bar: f32,
    anomaly_detected: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmartCityBundle {
    traffic: Vec<TrafficSensorReading>,
    air_quality: Vec<AirQualityReading>,
    noise: Vec<NoiseSensorRecord>,
    energy: Vec<EnergyConsumptionRecord>,
}

// ──────────────────────────────────────────────────────────────────────────────
// Helpers
// ──────────────────────────────────────────────────────────────────────────────

fn make_traffic_reading(id: u32) -> TrafficSensorReading {
    TrafficSensorReading {
        sensor_id: id,
        timestamp_unix: 1_700_000_000 + id as u64 * 60,
        vehicle_count: 42 + id % 200,
        average_speed_kmh: 50.0 + (id % 30) as f32,
        occupancy_percent: 30.0 + (id % 50) as f32,
        condition: match id % 4 {
            0 => TrafficCondition::FreeFlow,
            1 => TrafficCondition::Moderate,
            2 => TrafficCondition::HeavyCongestion,
            _ => TrafficCondition::StandstillJam,
        },
    }
}

fn make_air_quality(id: u32) -> AirQualityReading {
    AirQualityReading {
        station_id: id,
        timestamp_unix: 1_700_000_000 + id as u64 * 300,
        pm2_5_ug_m3: 12.5,
        pm10_ug_m3: 25.0,
        no2_ppb: 18.3,
        o3_ppb: 42.7,
        aqi: AirQualityIndex::Good,
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests — 22 total
// ──────────────────────────────────────────────────────────────────────────────

// 1. LZ4 roundtrip: TrafficCondition enum variants
#[test]
fn test_lz4_traffic_condition_variants_roundtrip() {
    let variants = vec![
        TrafficCondition::FreeFlow,
        TrafficCondition::Moderate,
        TrafficCondition::HeavyCongestion,
        TrafficCondition::StandstillJam,
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode TrafficCondition");
        let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
        let decompressed = decompress(&compressed).expect("lz4 decompress");
        let (decoded, _): (TrafficCondition, usize) =
            decode_from_slice(&decompressed).expect("decode TrafficCondition");
        assert_eq!(variant, &decoded);
    }
}

// 2. Zstd roundtrip: AirQualityIndex enum variants
#[test]
fn test_zstd_air_quality_index_variants_roundtrip() {
    let variants = vec![
        AirQualityIndex::Good,
        AirQualityIndex::Moderate,
        AirQualityIndex::UnhealthyForSensitiveGroups,
        AirQualityIndex::Unhealthy,
        AirQualityIndex::VeryUnhealthy,
        AirQualityIndex::Hazardous,
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode AirQualityIndex");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");
        let decompressed = decompress(&compressed).expect("zstd decompress");
        let (decoded, _): (AirQualityIndex, usize) =
            decode_from_slice(&decompressed).expect("decode AirQualityIndex");
        assert_eq!(variant, &decoded);
    }
}

// 3. LZ4 roundtrip: NoiseLevelCategory enum variants
#[test]
fn test_lz4_noise_level_category_variants_roundtrip() {
    let variants = vec![
        NoiseLevelCategory::Quiet,
        NoiseLevelCategory::Moderate,
        NoiseLevelCategory::Loud,
        NoiseLevelCategory::VeryLoud,
        NoiseLevelCategory::Harmful,
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode NoiseLevelCategory");
        let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
        let decompressed = decompress(&compressed).expect("lz4 decompress");
        let (decoded, _): (NoiseLevelCategory, usize) =
            decode_from_slice(&decompressed).expect("decode NoiseLevelCategory");
        assert_eq!(variant, &decoded);
    }
}

// 4. Zstd roundtrip: WasteBinStatus enum variants
#[test]
fn test_zstd_waste_bin_status_variants_roundtrip() {
    let variants = vec![
        WasteBinStatus::Empty,
        WasteBinStatus::PartiallyFull,
        WasteBinStatus::NearlyFull,
        WasteBinStatus::Overflowing,
        WasteBinStatus::SensorFault,
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode WasteBinStatus");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");
        let decompressed = decompress(&compressed).expect("zstd decompress");
        let (decoded, _): (WasteBinStatus, usize) =
            decode_from_slice(&decompressed).expect("decode WasteBinStatus");
        assert_eq!(variant, &decoded);
    }
}

// 5. LZ4 roundtrip: LightingMode enum including tuple variant
#[test]
fn test_lz4_lighting_mode_variants_roundtrip() {
    let variants = vec![
        LightingMode::Off,
        LightingMode::Dim(25),
        LightingMode::Dim(128),
        LightingMode::FullBrightness,
        LightingMode::AdaptiveMotion,
        LightingMode::ScheduledOverride,
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode LightingMode");
        let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
        let decompressed = decompress(&compressed).expect("lz4 decompress");
        let (decoded, _): (LightingMode, usize) =
            decode_from_slice(&decompressed).expect("decode LightingMode");
        assert_eq!(variant, &decoded);
    }
}

// 6. Zstd roundtrip: WifiHotspotStatus enum variants
#[test]
fn test_zstd_wifi_hotspot_status_variants_roundtrip() {
    let variants = vec![
        WifiHotspotStatus::Active,
        WifiHotspotStatus::Congested,
        WifiHotspotStatus::Maintenance,
        WifiHotspotStatus::Offline,
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode WifiHotspotStatus");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");
        let decompressed = decompress(&compressed).expect("zstd decompress");
        let (decoded, _): (WifiHotspotStatus, usize) =
            decode_from_slice(&decompressed).expect("decode WifiHotspotStatus");
        assert_eq!(variant, &decoded);
    }
}

// 7. Zstd roundtrip: nested TrafficSensorReading struct
#[test]
fn test_zstd_traffic_sensor_reading_nested_roundtrip() {
    let reading = TrafficSensorReading {
        sensor_id: 1001,
        timestamp_unix: 1_700_100_000,
        vehicle_count: 87,
        average_speed_kmh: 62.4,
        occupancy_percent: 41.7,
        condition: TrafficCondition::Moderate,
    };
    let encoded = encode_to_vec(&reading).expect("encode TrafficSensorReading");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");
    let decompressed = decompress(&compressed).expect("zstd decompress");
    let (decoded, _): (TrafficSensorReading, usize) =
        decode_from_slice(&decompressed).expect("decode TrafficSensorReading");
    assert_eq!(reading, decoded);
}

// 8. Zstd roundtrip: nested AirQualityReading struct
#[test]
fn test_zstd_air_quality_reading_nested_roundtrip() {
    let reading = AirQualityReading {
        station_id: 502,
        timestamp_unix: 1_700_200_000,
        pm2_5_ug_m3: 8.3,
        pm10_ug_m3: 19.7,
        no2_ppb: 14.2,
        o3_ppb: 38.9,
        aqi: AirQualityIndex::Moderate,
    };
    let encoded = encode_to_vec(&reading).expect("encode AirQualityReading");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");
    let decompressed = decompress(&compressed).expect("zstd decompress");
    let (decoded, _): (AirQualityReading, usize) =
        decode_from_slice(&decompressed).expect("decode AirQualityReading");
    assert_eq!(reading, decoded);
}

// 9. Zstd roundtrip: WasteBinTelemetry with string field
#[test]
fn test_zstd_waste_bin_telemetry_nested_roundtrip() {
    let telemetry = WasteBinTelemetry {
        bin_id: 9901,
        zone_code: "ZONE-B7".to_string(),
        fill_level_percent: 78,
        weight_kg: 43.2,
        status: WasteBinStatus::NearlyFull,
        last_emptied_unix: 1_699_900_000,
    };
    let encoded = encode_to_vec(&telemetry).expect("encode WasteBinTelemetry");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");
    let decompressed = decompress(&compressed).expect("zstd decompress");
    let (decoded, _): (WasteBinTelemetry, usize) =
        decode_from_slice(&decompressed).expect("decode WasteBinTelemetry");
    assert_eq!(telemetry, decoded);
}

// 10. Zstd roundtrip: StreetLightRecord with Option field
#[test]
fn test_zstd_street_light_record_with_option_roundtrip() {
    let record_no_fault = StreetLightRecord {
        pole_id: 3321,
        district: "Downtown".to_string(),
        mode: LightingMode::AdaptiveMotion,
        power_watts: 150.0,
        cumulative_kwh: 12043.7,
        fault_code: None,
    };
    let record_with_fault = StreetLightRecord {
        pole_id: 3322,
        district: "Eastside".to_string(),
        mode: LightingMode::Dim(40),
        power_watts: 60.0,
        cumulative_kwh: 8001.2,
        fault_code: Some(0xE001),
    };
    for record in &[record_no_fault, record_with_fault] {
        let encoded = encode_to_vec(record).expect("encode StreetLightRecord");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");
        let decompressed = decompress(&compressed).expect("zstd decompress");
        let (decoded, _): (StreetLightRecord, usize) =
            decode_from_slice(&decompressed).expect("decode StreetLightRecord");
        assert_eq!(record, &decoded);
    }
}

// 11. Both algorithms on same TrafficSensorReading: verify decode equivalence
#[test]
fn test_both_algorithms_traffic_reading_decode_equivalence() {
    let reading = TrafficSensorReading {
        sensor_id: 777,
        timestamp_unix: 1_700_500_000,
        vehicle_count: 120,
        average_speed_kmh: 45.0,
        occupancy_percent: 55.0,
        condition: TrafficCondition::HeavyCongestion,
    };
    let encoded = encode_to_vec(&reading).expect("encode");

    let compressed_lz4 = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let compressed_zstd = compress(&encoded, Compression::Zstd).expect("zstd compress");

    // Byte streams differ between algorithms
    assert_ne!(
        compressed_lz4, compressed_zstd,
        "LZ4 and Zstd byte streams should differ"
    );

    let decompressed_lz4 = decompress(&compressed_lz4).expect("lz4 decompress");
    let decompressed_zstd = decompress(&compressed_zstd).expect("zstd decompress");

    let (decoded_lz4, _): (TrafficSensorReading, usize) =
        decode_from_slice(&decompressed_lz4).expect("decode lz4");
    let (decoded_zstd, _): (TrafficSensorReading, usize) =
        decode_from_slice(&decompressed_zstd).expect("decode zstd");

    assert_eq!(
        decoded_lz4, decoded_zstd,
        "Both algorithms must decode to same value"
    );
}

// 12. Both algorithms on same AirQualityReading: cross-algorithm byte difference
#[test]
fn test_both_algorithms_air_quality_byte_difference_and_equivalence() {
    let reading = AirQualityReading {
        station_id: 200,
        timestamp_unix: 1_700_600_000,
        pm2_5_ug_m3: 35.6,
        pm10_ug_m3: 70.2,
        no2_ppb: 55.1,
        o3_ppb: 88.4,
        aqi: AirQualityIndex::Unhealthy,
    };
    let encoded = encode_to_vec(&reading).expect("encode");

    let compressed_lz4 = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let compressed_zstd = compress(&encoded, Compression::Zstd).expect("zstd compress");

    assert_ne!(
        compressed_lz4, compressed_zstd,
        "Compressed bytes must differ by algorithm"
    );

    let (decoded_lz4, _): (AirQualityReading, usize) =
        decode_from_slice(&decompress(&compressed_lz4).expect("lz4 decompress"))
            .expect("decode lz4");
    let (decoded_zstd, _): (AirQualityReading, usize) =
        decode_from_slice(&decompress(&compressed_zstd).expect("zstd decompress"))
            .expect("decode zstd");

    assert_eq!(
        decoded_lz4, decoded_zstd,
        "Both algorithms decode to same AirQualityReading"
    );
}

// 13. LZ4 compression ratio: 1000+ repetitive traffic readings
#[test]
fn test_lz4_compression_ratio_large_repetitive_traffic_dataset() {
    let readings: Vec<TrafficSensorReading> = (0..1200)
        .map(|_| TrafficSensorReading {
            sensor_id: 42,
            timestamp_unix: 1_700_000_000,
            vehicle_count: 100,
            average_speed_kmh: 60.0,
            occupancy_percent: 40.0,
            condition: TrafficCondition::FreeFlow,
        })
        .collect();

    let encoded = encode_to_vec(&readings).expect("encode large traffic dataset");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");

    assert!(
        compressed.len() < encoded.len(),
        "LZ4 must compress 1200 identical traffic readings: compressed={} < encoded={}",
        compressed.len(),
        encoded.len()
    );
}

// 14. Zstd compression ratio: 1000+ repetitive air quality readings
#[test]
fn test_zstd_compression_ratio_large_repetitive_air_quality_dataset() {
    let readings: Vec<AirQualityReading> = (0..1500)
        .map(|_| AirQualityReading {
            station_id: 99,
            timestamp_unix: 1_700_000_000,
            pm2_5_ug_m3: 12.0,
            pm10_ug_m3: 24.0,
            no2_ppb: 18.0,
            o3_ppb: 40.0,
            aqi: AirQualityIndex::Good,
        })
        .collect();

    let encoded = encode_to_vec(&readings).expect("encode large air quality dataset");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd must compress 1500 identical air quality readings: compressed={} < encoded={}",
        compressed.len(),
        encoded.len()
    );
}

// 15. LZ4 compression ratio: 1000+ repetitive energy consumption records
#[test]
fn test_lz4_compression_ratio_large_energy_consumption_dataset() {
    let records: Vec<EnergyConsumptionRecord> = (0..1000)
        .map(|_| EnergyConsumptionRecord {
            meter_id: 7,
            building_type: "Residential".to_string(),
            kwh_consumed: 3.75,
            peak_kw_demand: 1.2,
            hour_of_day: 14,
            day_of_week: 2,
        })
        .collect();

    let encoded = encode_to_vec(&records).expect("encode large energy dataset");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");

    assert!(
        compressed.len() < encoded.len(),
        "LZ4 must compress 1000 identical energy records: compressed={} < encoded={}",
        compressed.len(),
        encoded.len()
    );
}

// 16. Zstd compression ratio: 1000+ repetitive crowd density measurements
#[test]
fn test_zstd_compression_ratio_large_crowd_density_dataset() {
    let measurements: Vec<CrowdDensityMeasurement> = (0..1100)
        .map(|_| CrowdDensityMeasurement {
            zone_id: 5,
            timestamp_unix: 1_700_000_000,
            estimated_people: 250,
            density_per_sqm: 0.85,
            alert_triggered: false,
        })
        .collect();

    let encoded = encode_to_vec(&measurements).expect("encode large crowd dataset");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd must compress 1100 identical crowd density records: compressed={} < encoded={}",
        compressed.len(),
        encoded.len()
    );
}

// 17. Vec roundtrip: Vec<WaterMeterReading> with LZ4
#[test]
fn test_lz4_vec_water_meter_readings_roundtrip() {
    let readings: Vec<WaterMeterReading> = (0..50)
        .map(|i| WaterMeterReading {
            meter_id: i,
            district: format!("District-{}", i % 5),
            cubic_meters: 1000.0 + i as f64 * 0.5,
            flow_rate_lps: 0.3 + i as f32 * 0.01,
            pressure_bar: 4.2,
            anomaly_detected: i % 10 == 0,
        })
        .collect();

    let encoded = encode_to_vec(&readings).expect("encode Vec<WaterMeterReading>");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let decompressed = decompress(&compressed).expect("lz4 decompress");
    let (decoded, _): (Vec<WaterMeterReading>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<WaterMeterReading>");
    assert_eq!(readings, decoded);
}

// 18. Vec roundtrip: Vec<PublicWifiSnapshot> with Zstd
#[test]
fn test_zstd_vec_public_wifi_snapshots_roundtrip() {
    let snapshots: Vec<PublicWifiSnapshot> = (0..30)
        .map(|i| PublicWifiSnapshot {
            hotspot_id: i,
            ssid: format!("CityWifi-{:03}", i),
            connected_clients: 10 + i as u16,
            bandwidth_mbps_down: 50.0 + i as f32,
            bandwidth_mbps_up: 20.0 + i as f32 * 0.5,
            status: if i % 4 == 3 {
                WifiHotspotStatus::Congested
            } else {
                WifiHotspotStatus::Active
            },
        })
        .collect();

    let encoded = encode_to_vec(&snapshots).expect("encode Vec<PublicWifiSnapshot>");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");
    let decompressed = decompress(&compressed).expect("zstd decompress");
    let (decoded, _): (Vec<PublicWifiSnapshot>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<PublicWifiSnapshot>");
    assert_eq!(snapshots, decoded);
}

// 19. Empty data: empty Vec<NoiseSensorRecord> with LZ4
#[test]
fn test_lz4_empty_vec_noise_sensor_records_roundtrip() {
    let records: Vec<NoiseSensorRecord> = vec![];
    let encoded = encode_to_vec(&records).expect("encode empty Vec<NoiseSensorRecord>");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress empty");
    let decompressed = decompress(&compressed).expect("lz4 decompress empty");
    let (decoded, _): (Vec<NoiseSensorRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode empty Vec<NoiseSensorRecord>");
    assert_eq!(records, decoded);
}

// 20. Empty data: empty Vec<WasteBinTelemetry> with Zstd
#[test]
fn test_zstd_empty_vec_waste_bin_telemetry_roundtrip() {
    let records: Vec<WasteBinTelemetry> = vec![];
    let encoded = encode_to_vec(&records).expect("encode empty Vec<WasteBinTelemetry>");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress empty");
    let decompressed = decompress(&compressed).expect("zstd decompress empty");
    let (decoded, _): (Vec<WasteBinTelemetry>, usize) =
        decode_from_slice(&decompressed).expect("decode empty Vec<WasteBinTelemetry>");
    assert_eq!(records, decoded);
}

// 21. Error detection: truncated LZ4 compressed smart city data
#[test]
fn test_lz4_truncated_data_returns_error() {
    let readings: Vec<TrafficSensorReading> = (0..20).map(make_traffic_reading).collect();
    let encoded = encode_to_vec(&readings).expect("encode for truncation test");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");

    // Truncate to roughly half the compressed output
    let truncated = &compressed[..compressed.len() / 2];
    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "decompress must fail on truncated LZ4 compressed data"
    );
}

// 22. Error detection: corrupted Zstd compressed smart city bundle
#[test]
fn test_zstd_corrupted_data_returns_error() {
    let bundle = SmartCityBundle {
        traffic: (0..10).map(make_traffic_reading).collect(),
        air_quality: (0..5).map(make_air_quality).collect(),
        noise: vec![NoiseSensorRecord {
            sensor_id: 1,
            location_lat: 59.4370,
            location_lon: 24.7536,
            decibels: 68.5,
            category: NoiseLevelCategory::Loud,
            peak_event_count: 3,
        }],
        energy: vec![EnergyConsumptionRecord {
            meter_id: 101,
            building_type: "Commercial".to_string(),
            kwh_consumed: 88.4,
            peak_kw_demand: 22.1,
            hour_of_day: 10,
            day_of_week: 1,
        }],
    };
    let encoded = encode_to_vec(&bundle).expect("encode SmartCityBundle");
    let mut compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");

    // Overwrite a large swath of bytes in the payload region to corrupt it.
    // Skip the first 8 bytes (oxicode compression header) and zero-fill the rest
    // so Zstd has no valid frame to decode.
    let header_len = 8.min(compressed.len());
    for byte in compressed[header_len..].iter_mut() {
        *byte = 0xAA;
    }

    let result = decompress(&compressed);
    assert!(
        result.is_err(),
        "decompress must fail on corrupted Zstd compressed data"
    );
}
