//! Advanced property-based tests (set 85) — Precision Agriculture & Smart Farming domain.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers soil moisture sensors, crop yield predictions, drone flight paths,
//! fertilizer application rates, irrigation zone schedules, pest detection,
//! harvest maturity indices, livestock health metrics, greenhouse climate controls,
//! GPS-guided tractor waypoints, satellite vegetation indices, weather station data,
//! and grain silo monitoring.

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

// ── Domain types ──────────────────────────────────────────────────────────────

/// Soil moisture sensor reading from a field probe.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SoilMoistureReading {
    /// Probe identifier within the field grid.
    probe_id: u32,
    /// Volumetric water content (0.0 – 1.0).
    vwc_fraction: f32,
    /// Soil temperature at probe depth in degrees C.
    soil_temp_c: f32,
    /// Probe depth in centimetres.
    depth_cm: u16,
    /// Electrical conductivity in dS/m.
    ec_ds_per_m: f32,
    /// Measurement epoch (Unix seconds).
    timestamp_s: u64,
}

/// Crop yield prediction for a single parcel.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CropYieldPrediction {
    /// Parcel identifier.
    parcel_id: u64,
    /// Predicted yield in tonnes per hectare.
    yield_t_per_ha: f32,
    /// Confidence interval lower bound.
    ci_lower: f32,
    /// Confidence interval upper bound.
    ci_upper: f32,
    /// Crop type code.
    crop_code: u16,
    /// Growth stage day count since planting.
    days_since_planting: u16,
}

/// Drone flight path waypoint for crop surveillance.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DroneWaypoint {
    /// Waypoint sequence number.
    seq: u32,
    /// Latitude in degrees.
    lat_deg: f64,
    /// Longitude in degrees.
    lon_deg: f64,
    /// Altitude above ground level in metres.
    alt_m: f32,
    /// Ground speed in m/s.
    speed_m_per_s: f32,
    /// Heading in degrees (0 = North).
    heading_deg: f32,
}

/// Fertilizer application rate command for variable-rate technology.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FertilizerApplication {
    /// Zone identifier within the field.
    zone_id: u32,
    /// Nitrogen rate in kg/ha.
    nitrogen_kg_per_ha: f32,
    /// Phosphorus rate in kg/ha.
    phosphorus_kg_per_ha: f32,
    /// Potassium rate in kg/ha.
    potassium_kg_per_ha: f32,
    /// Application timestamp (Unix seconds).
    timestamp_s: u64,
}

/// Irrigation zone schedule entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IrrigationSchedule {
    /// Zone identifier.
    zone_id: u16,
    /// Start time offset from midnight in minutes.
    start_min: u16,
    /// Duration in minutes.
    duration_min: u16,
    /// Flow rate in litres per minute.
    flow_lpm: f32,
    /// Soil moisture threshold to trigger irrigation (0.0 – 1.0).
    moisture_threshold: f32,
    /// Whether schedule is currently active.
    active: bool,
}

/// Pest detection classification from a camera trap or scout.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PestDetection {
    /// No pest detected.
    Clear { confidence: f32 },
    /// Insect pest detected.
    Insect {
        species_code: u16,
        severity: u8,
        confidence: f32,
    },
    /// Fungal infection detected.
    Fungal {
        pathogen_code: u16,
        affected_area_pct: f32,
        confidence: f32,
    },
    /// Weed presence detected.
    Weed {
        weed_code: u16,
        density_per_m2: f32,
        confidence: f32,
    },
}

/// Harvest maturity index for a fruit/grain lot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HarvestMaturityIndex {
    /// Lot identifier.
    lot_id: u64,
    /// Brix (sugar content) measurement.
    brix: f32,
    /// Firmness in Newtons.
    firmness_n: f32,
    /// Moisture content fraction (0.0 – 1.0).
    moisture_fraction: f32,
    /// Days until optimal harvest.
    days_to_harvest: u16,
    /// Ready for harvest flag.
    harvest_ready: bool,
}

/// Livestock health metric snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LivestockHealthMetric {
    /// Animal RFID tag identifier.
    tag_id: u64,
    /// Body temperature in degrees C.
    body_temp_c: f32,
    /// Heart rate in bpm.
    heart_rate_bpm: u16,
    /// Rumination minutes in last 24 hours.
    rumination_min: u16,
    /// Daily step count.
    step_count: u32,
    /// Activity level (0 = resting, 255 = high activity).
    activity_level: u8,
}

/// Greenhouse climate control setpoint.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GreenhouseClimate {
    /// Greenhouse section identifier.
    section_id: u16,
    /// Target temperature in degrees C.
    target_temp_c: f32,
    /// Target relative humidity (0.0 – 1.0).
    target_rh_fraction: f32,
    /// CO2 concentration setpoint in ppm.
    co2_ppm: u16,
    /// Ventilation fan duty cycle (0.0 – 1.0).
    vent_duty: f32,
    /// Supplemental lighting on.
    lighting_on: bool,
}

/// GPS-guided tractor waypoint for autonomous field operations.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TractorWaypoint {
    /// Waypoint index in the planned path.
    index: u32,
    /// Latitude in degrees.
    lat_deg: f64,
    /// Longitude in degrees.
    lon_deg: f64,
    /// Target speed in km/h.
    speed_kmh: f32,
    /// Implement engagement flag.
    implement_engaged: bool,
    /// Cross-track error tolerance in metres.
    xte_tolerance_m: f32,
}

/// Satellite-derived vegetation index measurement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VegetationIndex {
    /// Pixel grid row.
    row: u32,
    /// Pixel grid column.
    col: u32,
    /// NDVI value (-1.0 – 1.0).
    ndvi: f32,
    /// Enhanced Vegetation Index.
    evi: f32,
    /// Leaf Area Index estimate.
    lai: f32,
    /// Cloud cover fraction for this pixel (0.0 – 1.0).
    cloud_fraction: f32,
}

/// On-farm weather station data record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeatherStationData {
    /// Station identifier.
    station_id: u32,
    /// Air temperature in degrees C.
    air_temp_c: f32,
    /// Relative humidity (0.0 – 1.0).
    rh_fraction: f32,
    /// Wind speed in m/s.
    wind_speed_m_per_s: f32,
    /// Wind direction in degrees.
    wind_dir_deg: f32,
    /// Precipitation in mm for the last hour.
    precip_mm: f32,
    /// Barometric pressure in hPa.
    pressure_hpa: f32,
}

/// Grain silo monitoring record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GrainSiloMonitor {
    /// Silo identifier.
    silo_id: u16,
    /// Fill level fraction (0.0 – 1.0).
    fill_fraction: f32,
    /// Grain temperature in degrees C.
    grain_temp_c: f32,
    /// Grain moisture content fraction.
    grain_moisture: f32,
    /// CO2 concentration in ppm (spoilage indicator).
    co2_ppm: u16,
    /// Aeration fan running.
    fan_running: bool,
}

// ── Tests 1–22 ────────────────────────────────────────────────────────────────

// ── 1. SoilMoistureReading roundtrip ─────────────────────────────────────────

#[test]
fn test_soil_moisture_reading_roundtrip() {
    proptest!(|(
        probe_id: u32,
        vwc_fraction in 0.0f32..1.0f32,
        soil_temp_c in (-10.0f32)..60.0f32,
        depth_cm in 5u16..200u16,
        ec_ds_per_m in 0.0f32..10.0f32,
        timestamp_s: u64,
    )| {
        let val = SoilMoistureReading {
            probe_id, vwc_fraction, soil_temp_c, depth_cm, ec_ds_per_m, timestamp_s,
        };
        let enc = encode_to_vec(&val).expect("encode SoilMoistureReading failed");
        let (dec, consumed): (SoilMoistureReading, usize) =
            decode_from_slice(&enc).expect("decode SoilMoistureReading failed");
        prop_assert_eq!(&val, &dec, "SoilMoistureReading roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 2. SoilMoistureReading re-encode determinism ─────────────────────────────

#[test]
fn test_soil_moisture_determinism() {
    proptest!(|(
        probe_id: u32,
        vwc_fraction in 0.0f32..1.0f32,
        soil_temp_c in (-10.0f32)..60.0f32,
        depth_cm in 5u16..200u16,
        ec_ds_per_m in 0.0f32..10.0f32,
        timestamp_s: u64,
    )| {
        let val = SoilMoistureReading {
            probe_id, vwc_fraction, soil_temp_c, depth_cm, ec_ds_per_m, timestamp_s,
        };
        let enc1 = encode_to_vec(&val).expect("first encode SoilMoistureReading failed");
        let enc2 = encode_to_vec(&val).expect("second encode SoilMoistureReading failed");
        prop_assert_eq!(enc1, enc2, "encoding must be deterministic");
    });
}

// ── 3. CropYieldPrediction roundtrip ─────────────────────────────────────────

#[test]
fn test_crop_yield_prediction_roundtrip() {
    proptest!(|(
        parcel_id: u64,
        yield_t_per_ha in 0.0f32..30.0f32,
        ci_lower in 0.0f32..15.0f32,
        ci_upper in 15.0f32..30.0f32,
        crop_code in 1u16..500u16,
        days_since_planting in 0u16..365u16,
    )| {
        let val = CropYieldPrediction {
            parcel_id, yield_t_per_ha, ci_lower, ci_upper, crop_code, days_since_planting,
        };
        let enc = encode_to_vec(&val).expect("encode CropYieldPrediction failed");
        let (dec, consumed): (CropYieldPrediction, usize) =
            decode_from_slice(&enc).expect("decode CropYieldPrediction failed");
        prop_assert_eq!(&val, &dec, "CropYieldPrediction roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 4. Vec<CropYieldPrediction> roundtrip ────────────────────────────────────

#[test]
fn test_vec_crop_yield_prediction_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(
            (
                any::<u64>(),
                0.0f32..30.0f32,
                0.0f32..15.0f32,
                15.0f32..30.0f32,
                1u16..500u16,
                0u16..365u16,
            ).prop_map(|(parcel_id, yield_t_per_ha, ci_lower, ci_upper, crop_code, days_since_planting)| {
                CropYieldPrediction {
                    parcel_id, yield_t_per_ha, ci_lower, ci_upper, crop_code, days_since_planting,
                }
            }),
            0..6usize,
        ),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<CropYieldPrediction> failed");
        let (dec, consumed): (Vec<CropYieldPrediction>, usize) =
            decode_from_slice(&enc).expect("decode Vec<CropYieldPrediction> failed");
        prop_assert_eq!(&items, &dec, "Vec<CropYieldPrediction> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 5. DroneWaypoint roundtrip ───────────────────────────────────────────────

#[test]
fn test_drone_waypoint_roundtrip() {
    proptest!(|(
        seq: u32,
        lat_deg in (-90.0f64)..90.0f64,
        lon_deg in (-180.0f64)..180.0f64,
        alt_m in 1.0f32..150.0f32,
        speed_m_per_s in 0.0f32..25.0f32,
        heading_deg in 0.0f32..360.0f32,
    )| {
        let val = DroneWaypoint { seq, lat_deg, lon_deg, alt_m, speed_m_per_s, heading_deg };
        let enc = encode_to_vec(&val).expect("encode DroneWaypoint failed");
        let (dec, consumed): (DroneWaypoint, usize) =
            decode_from_slice(&enc).expect("decode DroneWaypoint failed");
        prop_assert_eq!(&val, &dec, "DroneWaypoint roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 6. FertilizerApplication roundtrip ───────────────────────────────────────

#[test]
fn test_fertilizer_application_roundtrip() {
    proptest!(|(
        zone_id: u32,
        nitrogen_kg_per_ha in 0.0f32..300.0f32,
        phosphorus_kg_per_ha in 0.0f32..150.0f32,
        potassium_kg_per_ha in 0.0f32..200.0f32,
        timestamp_s: u64,
    )| {
        let val = FertilizerApplication {
            zone_id, nitrogen_kg_per_ha, phosphorus_kg_per_ha, potassium_kg_per_ha, timestamp_s,
        };
        let enc = encode_to_vec(&val).expect("encode FertilizerApplication failed");
        let (dec, consumed): (FertilizerApplication, usize) =
            decode_from_slice(&enc).expect("decode FertilizerApplication failed");
        prop_assert_eq!(&val, &dec, "FertilizerApplication roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 7. FertilizerApplication re-encode idempotency ───────────────────────────

#[test]
fn test_fertilizer_reencode_idempotent() {
    proptest!(|(
        zone_id: u32,
        nitrogen_kg_per_ha in 0.0f32..300.0f32,
        phosphorus_kg_per_ha in 0.0f32..150.0f32,
        potassium_kg_per_ha in 0.0f32..200.0f32,
        timestamp_s: u64,
    )| {
        let val = FertilizerApplication {
            zone_id, nitrogen_kg_per_ha, phosphorus_kg_per_ha, potassium_kg_per_ha, timestamp_s,
        };
        let enc1 = encode_to_vec(&val).expect("first encode FertilizerApplication failed");
        let (decoded, _): (FertilizerApplication, usize) =
            decode_from_slice(&enc1).expect("decode FertilizerApplication failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode FertilizerApplication failed");
        prop_assert_eq!(enc1, enc2, "re-encoding must produce identical bytes");
    });
}

// ── 8. IrrigationSchedule roundtrip ──────────────────────────────────────────

#[test]
fn test_irrigation_schedule_roundtrip() {
    proptest!(|(
        zone_id in 1u16..64u16,
        start_min in 0u16..1440u16,
        duration_min in 1u16..180u16,
        flow_lpm in 0.5f32..500.0f32,
        moisture_threshold in 0.0f32..1.0f32,
        active: bool,
    )| {
        let val = IrrigationSchedule {
            zone_id, start_min, duration_min, flow_lpm, moisture_threshold, active,
        };
        let enc = encode_to_vec(&val).expect("encode IrrigationSchedule failed");
        let (dec, consumed): (IrrigationSchedule, usize) =
            decode_from_slice(&enc).expect("decode IrrigationSchedule failed");
        prop_assert_eq!(&val, &dec, "IrrigationSchedule roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 9. PestDetection::Clear roundtrip ────────────────────────────────────────

#[test]
fn test_pest_detection_clear_roundtrip() {
    proptest!(|(confidence in 0.0f32..1.0f32)| {
        let val = PestDetection::Clear { confidence };
        let enc = encode_to_vec(&val).expect("encode PestDetection::Clear failed");
        let (dec, consumed): (PestDetection, usize) =
            decode_from_slice(&enc).expect("decode PestDetection::Clear failed");
        prop_assert_eq!(&val, &dec, "PestDetection::Clear roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 10. PestDetection::Insect roundtrip ──────────────────────────────────────

#[test]
fn test_pest_detection_insect_roundtrip() {
    proptest!(|(
        species_code in 1u16..1000u16,
        severity in 1u8..5u8,
        confidence in 0.0f32..1.0f32,
    )| {
        let val = PestDetection::Insect { species_code, severity, confidence };
        let enc = encode_to_vec(&val).expect("encode PestDetection::Insect failed");
        let (dec, consumed): (PestDetection, usize) =
            decode_from_slice(&enc).expect("decode PestDetection::Insect failed");
        prop_assert_eq!(&val, &dec, "PestDetection::Insect roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 11. PestDetection::Fungal roundtrip ──────────────────────────────────────

#[test]
fn test_pest_detection_fungal_roundtrip() {
    proptest!(|(
        pathogen_code in 1u16..500u16,
        affected_area_pct in 0.0f32..100.0f32,
        confidence in 0.0f32..1.0f32,
    )| {
        let val = PestDetection::Fungal { pathogen_code, affected_area_pct, confidence };
        let enc = encode_to_vec(&val).expect("encode PestDetection::Fungal failed");
        let (dec, consumed): (PestDetection, usize) =
            decode_from_slice(&enc).expect("decode PestDetection::Fungal failed");
        prop_assert_eq!(&val, &dec, "PestDetection::Fungal roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 12. PestDetection::Weed roundtrip ────────────────────────────────────────

#[test]
fn test_pest_detection_weed_roundtrip() {
    proptest!(|(
        weed_code in 1u16..300u16,
        density_per_m2 in 0.0f32..200.0f32,
        confidence in 0.0f32..1.0f32,
    )| {
        let val = PestDetection::Weed { weed_code, density_per_m2, confidence };
        let enc = encode_to_vec(&val).expect("encode PestDetection::Weed failed");
        let (dec, consumed): (PestDetection, usize) =
            decode_from_slice(&enc).expect("decode PestDetection::Weed failed");
        prop_assert_eq!(&val, &dec, "PestDetection::Weed roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 13. HarvestMaturityIndex roundtrip ───────────────────────────────────────

#[test]
fn test_harvest_maturity_index_roundtrip() {
    proptest!(|(
        lot_id: u64,
        brix in 0.0f32..35.0f32,
        firmness_n in 0.0f32..100.0f32,
        moisture_fraction in 0.0f32..1.0f32,
        days_to_harvest in 0u16..120u16,
        harvest_ready: bool,
    )| {
        let val = HarvestMaturityIndex {
            lot_id, brix, firmness_n, moisture_fraction, days_to_harvest, harvest_ready,
        };
        let enc = encode_to_vec(&val).expect("encode HarvestMaturityIndex failed");
        let (dec, consumed): (HarvestMaturityIndex, usize) =
            decode_from_slice(&enc).expect("decode HarvestMaturityIndex failed");
        prop_assert_eq!(&val, &dec, "HarvestMaturityIndex roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 14. LivestockHealthMetric roundtrip ──────────────────────────────────────

#[test]
fn test_livestock_health_metric_roundtrip() {
    proptest!(|(
        tag_id: u64,
        body_temp_c in 35.0f32..42.0f32,
        heart_rate_bpm in 30u16..120u16,
        rumination_min in 0u16..720u16,
        step_count in 0u32..50_000u32,
        activity_level: u8,
    )| {
        let val = LivestockHealthMetric {
            tag_id, body_temp_c, heart_rate_bpm, rumination_min, step_count, activity_level,
        };
        let enc = encode_to_vec(&val).expect("encode LivestockHealthMetric failed");
        let (dec, consumed): (LivestockHealthMetric, usize) =
            decode_from_slice(&enc).expect("decode LivestockHealthMetric failed");
        prop_assert_eq!(&val, &dec, "LivestockHealthMetric roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 15. Vec<LivestockHealthMetric> roundtrip ─────────────────────────────────

#[test]
fn test_vec_livestock_health_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(
            (
                any::<u64>(),
                35.0f32..42.0f32,
                30u16..120u16,
                0u16..720u16,
                0u32..50_000u32,
                any::<u8>(),
            ).prop_map(|(tag_id, body_temp_c, heart_rate_bpm, rumination_min, step_count, activity_level)| {
                LivestockHealthMetric {
                    tag_id, body_temp_c, heart_rate_bpm, rumination_min, step_count, activity_level,
                }
            }),
            0..8usize,
        ),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<LivestockHealthMetric> failed");
        let (dec, consumed): (Vec<LivestockHealthMetric>, usize) =
            decode_from_slice(&enc).expect("decode Vec<LivestockHealthMetric> failed");
        prop_assert_eq!(&items, &dec, "Vec<LivestockHealthMetric> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 16. GreenhouseClimate roundtrip ──────────────────────────────────────────

#[test]
fn test_greenhouse_climate_roundtrip() {
    proptest!(|(
        section_id in 1u16..64u16,
        target_temp_c in 10.0f32..40.0f32,
        target_rh_fraction in 0.3f32..0.95f32,
        co2_ppm in 300u16..2000u16,
        vent_duty in 0.0f32..1.0f32,
        lighting_on: bool,
    )| {
        let val = GreenhouseClimate {
            section_id, target_temp_c, target_rh_fraction, co2_ppm, vent_duty, lighting_on,
        };
        let enc = encode_to_vec(&val).expect("encode GreenhouseClimate failed");
        let (dec, consumed): (GreenhouseClimate, usize) =
            decode_from_slice(&enc).expect("decode GreenhouseClimate failed");
        prop_assert_eq!(&val, &dec, "GreenhouseClimate roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 17. TractorWaypoint roundtrip ────────────────────────────────────────────

#[test]
fn test_tractor_waypoint_roundtrip() {
    proptest!(|(
        index: u32,
        lat_deg in (-90.0f64)..90.0f64,
        lon_deg in (-180.0f64)..180.0f64,
        speed_kmh in 0.5f32..20.0f32,
        implement_engaged: bool,
        xte_tolerance_m in 0.01f32..1.0f32,
    )| {
        let val = TractorWaypoint {
            index, lat_deg, lon_deg, speed_kmh, implement_engaged, xte_tolerance_m,
        };
        let enc = encode_to_vec(&val).expect("encode TractorWaypoint failed");
        let (dec, consumed): (TractorWaypoint, usize) =
            decode_from_slice(&enc).expect("decode TractorWaypoint failed");
        prop_assert_eq!(&val, &dec, "TractorWaypoint roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 18. VegetationIndex roundtrip ────────────────────────────────────────────

#[test]
fn test_vegetation_index_roundtrip() {
    proptest!(|(
        row in 0u32..4096u32,
        col in 0u32..4096u32,
        ndvi in (-1.0f32)..1.0f32,
        evi in (-1.0f32)..1.0f32,
        lai in 0.0f32..10.0f32,
        cloud_fraction in 0.0f32..1.0f32,
    )| {
        let val = VegetationIndex { row, col, ndvi, evi, lai, cloud_fraction };
        let enc = encode_to_vec(&val).expect("encode VegetationIndex failed");
        let (dec, consumed): (VegetationIndex, usize) =
            decode_from_slice(&enc).expect("decode VegetationIndex failed");
        prop_assert_eq!(&val, &dec, "VegetationIndex roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 19. WeatherStationData roundtrip ─────────────────────────────────────────

#[test]
fn test_weather_station_data_roundtrip() {
    proptest!(|(
        station_id: u32,
        air_temp_c in (-40.0f32)..55.0f32,
        rh_fraction in 0.0f32..1.0f32,
        wind_speed_m_per_s in 0.0f32..60.0f32,
        wind_dir_deg in 0.0f32..360.0f32,
        precip_mm in 0.0f32..100.0f32,
        pressure_hpa in 900.0f32..1100.0f32,
    )| {
        let val = WeatherStationData {
            station_id, air_temp_c, rh_fraction, wind_speed_m_per_s,
            wind_dir_deg, precip_mm, pressure_hpa,
        };
        let enc = encode_to_vec(&val).expect("encode WeatherStationData failed");
        let (dec, consumed): (WeatherStationData, usize) =
            decode_from_slice(&enc).expect("decode WeatherStationData failed");
        prop_assert_eq!(&val, &dec, "WeatherStationData roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 20. WeatherStationData re-encode idempotency ─────────────────────────────

#[test]
fn test_weather_station_reencode_idempotent() {
    proptest!(|(
        station_id: u32,
        air_temp_c in (-40.0f32)..55.0f32,
        rh_fraction in 0.0f32..1.0f32,
        wind_speed_m_per_s in 0.0f32..60.0f32,
        wind_dir_deg in 0.0f32..360.0f32,
        precip_mm in 0.0f32..100.0f32,
        pressure_hpa in 900.0f32..1100.0f32,
    )| {
        let val = WeatherStationData {
            station_id, air_temp_c, rh_fraction, wind_speed_m_per_s,
            wind_dir_deg, precip_mm, pressure_hpa,
        };
        let enc1 = encode_to_vec(&val).expect("first encode WeatherStationData failed");
        let (decoded, _): (WeatherStationData, usize) =
            decode_from_slice(&enc1).expect("decode WeatherStationData failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode WeatherStationData failed");
        prop_assert_eq!(enc1, enc2, "re-encoding must produce identical bytes");
    });
}

// ── 21. GrainSiloMonitor roundtrip ───────────────────────────────────────────

#[test]
fn test_grain_silo_monitor_roundtrip() {
    proptest!(|(
        silo_id in 1u16..100u16,
        fill_fraction in 0.0f32..1.0f32,
        grain_temp_c in (-20.0f32)..60.0f32,
        grain_moisture in 0.05f32..0.40f32,
        co2_ppm in 300u16..5000u16,
        fan_running: bool,
    )| {
        let val = GrainSiloMonitor {
            silo_id, fill_fraction, grain_temp_c, grain_moisture, co2_ppm, fan_running,
        };
        let enc = encode_to_vec(&val).expect("encode GrainSiloMonitor failed");
        let (dec, consumed): (GrainSiloMonitor, usize) =
            decode_from_slice(&enc).expect("decode GrainSiloMonitor failed");
        prop_assert_eq!(&val, &dec, "GrainSiloMonitor roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 22. Vec<GrainSiloMonitor> roundtrip ──────────────────────────────────────

#[test]
fn test_vec_grain_silo_monitor_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(
            (
                1u16..100u16,
                0.0f32..1.0f32,
                (-20.0f32)..60.0f32,
                0.05f32..0.40f32,
                300u16..5000u16,
                any::<bool>(),
            ).prop_map(|(silo_id, fill_fraction, grain_temp_c, grain_moisture, co2_ppm, fan_running)| {
                GrainSiloMonitor {
                    silo_id, fill_fraction, grain_temp_c, grain_moisture, co2_ppm, fan_running,
                }
            }),
            0..10usize,
        ),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<GrainSiloMonitor> failed");
        let (dec, consumed): (Vec<GrainSiloMonitor>, usize) =
            decode_from_slice(&enc).expect("decode Vec<GrainSiloMonitor> failed");
        prop_assert_eq!(&items, &dec, "Vec<GrainSiloMonitor> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}
