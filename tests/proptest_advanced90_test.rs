//! Advanced property-based tests (set 90) — Weather Forecasting & Meteorological
//! Observation Systems domain.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers METAR surface observations, upper-air soundings, radar reflectivity,
//! satellite cloud-top temperatures, NWP model grid points, severe weather warnings,
//! tropical cyclone advisories, precipitation accumulation, visibility/ceiling,
//! wind shear alerts, air quality indices, and frost/freeze predictions.

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

/// METAR surface weather observation record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MetarObservation {
    /// ICAO station identifier (4-character code stored as u32).
    station_icao: u32,
    /// Observation timestamp (Unix seconds).
    obs_time_s: u64,
    /// Wind direction in degrees (0-360).
    wind_dir_deg: u16,
    /// Wind speed in knots.
    wind_speed_kt: u16,
    /// Wind gust speed in knots (0 if no gusts).
    wind_gust_kt: u16,
    /// Visibility in statute miles.
    visibility_sm: f32,
    /// Temperature in degrees Celsius.
    temperature_c: f32,
    /// Dewpoint in degrees Celsius.
    dewpoint_c: f32,
    /// Altimeter setting in inches of mercury.
    altimeter_inhg: f32,
    /// Whether the station is automated.
    is_automated: bool,
}

/// Upper-air sounding level (radiosonde data at a single pressure level).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SoundingLevel {
    /// Pressure in hPa.
    pressure_hpa: f32,
    /// Height in geopotential metres.
    height_gpm: f32,
    /// Temperature in degrees Celsius.
    temperature_c: f32,
    /// Dewpoint depression in degrees Celsius.
    dewpoint_depression_c: f32,
    /// Wind direction in degrees.
    wind_dir_deg: u16,
    /// Wind speed in knots.
    wind_speed_kt: u16,
}

/// Complete radiosonde sounding profile.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiosondeProfile {
    /// Launch site identifier.
    site_id: u32,
    /// Launch time (Unix seconds).
    launch_time_s: u64,
    /// Sounding levels from surface upward.
    levels: Vec<SoundingLevel>,
}

/// Single radar reflectivity cell in a plan-position indicator scan.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadarReflectivityCell {
    /// Azimuth angle in degrees (0-360).
    azimuth_deg: f32,
    /// Range from radar in kilometres.
    range_km: f32,
    /// Reflectivity in dBZ.
    reflectivity_dbz: f32,
    /// Elevation tilt angle in degrees.
    elevation_deg: f32,
    /// Differential reflectivity (dual-pol).
    zdr_db: f32,
}

/// Satellite infrared cloud-top temperature measurement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CloudTopTemperature {
    /// Latitude in degrees (-90 to 90).
    latitude_deg: f32,
    /// Longitude in degrees (-180 to 180).
    longitude_deg: f32,
    /// Brightness temperature in Kelvin.
    brightness_temp_k: f32,
    /// Satellite channel identifier.
    channel_id: u8,
    /// Scan line number.
    scan_line: u32,
    /// Pixel index within scan line.
    pixel_index: u32,
}

/// NWP (Numerical Weather Prediction) model grid point forecast.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NwpGridPoint {
    /// Grid point latitude.
    latitude_deg: f32,
    /// Grid point longitude.
    longitude_deg: f32,
    /// Forecast hour (offset from model init time).
    forecast_hour: u16,
    /// 2-metre temperature in Kelvin.
    temp_2m_k: f32,
    /// Mean sea level pressure in Pa.
    mslp_pa: f32,
    /// 10-metre U wind component in m/s.
    u_wind_10m: f32,
    /// 10-metre V wind component in m/s.
    v_wind_10m: f32,
    /// Total precipitation in mm.
    total_precip_mm: f32,
    /// Relative humidity (0-1).
    rh_fraction: f32,
}

/// Severe weather warning bulletin.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SevereWeatherWarning {
    /// Tornado warning with estimated EF scale.
    Tornado {
        ef_scale: u8,
        latitude_deg: f32,
        longitude_deg: f32,
        valid_until_s: u64,
    },
    /// Severe thunderstorm warning.
    SevereThunderstorm {
        max_hail_cm: f32,
        max_wind_kt: u16,
        valid_until_s: u64,
    },
    /// Flash flood warning.
    FlashFlood {
        rainfall_mm: f32,
        area_sq_km: f32,
        valid_until_s: u64,
    },
    /// Winter storm warning.
    WinterStorm {
        snow_cm: f32,
        ice_accum_mm: f32,
        wind_chill_c: f32,
        valid_until_s: u64,
    },
}

/// Tropical cyclone advisory.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TropicalCycloneAdvisory {
    /// Storm name tag (encoded as a small number for simplicity).
    storm_id: u32,
    /// Centre latitude.
    center_lat_deg: f32,
    /// Centre longitude.
    center_lon_deg: f32,
    /// Maximum sustained wind in knots.
    max_wind_kt: u16,
    /// Central pressure in hPa.
    central_pressure_hpa: f32,
    /// Radius of maximum winds in nautical miles.
    rmw_nm: f32,
    /// Movement direction in degrees.
    movement_dir_deg: u16,
    /// Movement speed in knots.
    movement_speed_kt: u16,
    /// Saffir-Simpson category (0-5, 0 for tropical storm).
    category: u8,
}

/// Precipitation accumulation record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrecipitationAccumulation {
    /// Gauge station identifier.
    gauge_id: u32,
    /// Accumulation period start (Unix seconds).
    period_start_s: u64,
    /// Accumulation period end (Unix seconds).
    period_end_s: u64,
    /// Total liquid equivalent in mm.
    liquid_equiv_mm: f32,
    /// Snow depth in cm (0 if rain only).
    snow_depth_cm: f32,
    /// Gauge type code.
    gauge_type: u8,
}

/// Visibility and ceiling measurement.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VisibilityCeiling {
    /// Station identifier.
    station_id: u32,
    /// Prevailing visibility in metres.
    visibility_m: f32,
    /// Cloud ceiling height in feet AGL.
    ceiling_ft: u32,
    /// Whether ceiling is variable.
    ceiling_variable: bool,
    /// Number of cloud layers reported.
    cloud_layers: u8,
    /// Observation time (Unix seconds).
    obs_time_s: u64,
}

/// Low-level wind shear alert.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WindShearAlert {
    /// Runway identifier code.
    runway_id: u16,
    /// Airport ICAO as numeric code.
    airport_icao: u32,
    /// Loss/gain in knots (negative = loss).
    delta_speed_kt: i16,
    /// Altitude in feet AGL where shear detected.
    altitude_ft: u32,
    /// Detection timestamp (Unix seconds).
    detect_time_s: u64,
    /// Alert severity (1-3).
    severity: u8,
}

/// Air quality index observation.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AirQualityIndex {
    /// Monitoring station identifier.
    station_id: u32,
    /// PM2.5 concentration in micrograms per cubic metre.
    pm25_ug_m3: f32,
    /// PM10 concentration in micrograms per cubic metre.
    pm10_ug_m3: f32,
    /// Ozone concentration in ppb.
    ozone_ppb: f32,
    /// Nitrogen dioxide in ppb.
    no2_ppb: f32,
    /// Overall AQI value (0-500).
    aqi_value: u16,
    /// Observation timestamp (Unix seconds).
    obs_time_s: u64,
}

/// Frost/freeze prediction for an agricultural zone.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FrostFreezePrediction {
    /// Zone identifier.
    zone_id: u32,
    /// Predicted minimum temperature in degrees Celsius.
    min_temp_c: f32,
    /// Predicted ground temperature in degrees Celsius.
    ground_temp_c: f32,
    /// Wind speed at prediction time in m/s.
    wind_speed_ms: f32,
    /// Cloud cover fraction (0-1).
    cloud_cover: f32,
    /// Probability of frost (0-1).
    frost_probability: f32,
    /// Whether freeze warning is issued.
    freeze_warning: bool,
}

/// Lightning strike detection record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LightningStrike {
    /// Latitude in degrees.
    latitude_deg: f32,
    /// Longitude in degrees.
    longitude_deg: f32,
    /// Peak current in kiloamperes (negative for cloud-to-ground).
    peak_current_ka: f32,
    /// Detection timestamp (Unix seconds).
    time_s: u64,
    /// Number of sensors that detected the stroke.
    sensor_count: u8,
    /// Whether cloud-to-ground or intra-cloud.
    cloud_to_ground: bool,
}

/// Atmospheric turbulence (pilot report / PIREP).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TurbulenceReport {
    /// No turbulence.
    Smooth,
    /// Light turbulence.
    Light {
        altitude_ft: u32,
        latitude_deg: f32,
        longitude_deg: f32,
    },
    /// Moderate turbulence.
    Moderate {
        altitude_ft: u32,
        latitude_deg: f32,
        longitude_deg: f32,
        eddy_dissipation_rate: f32,
    },
    /// Severe turbulence.
    Severe {
        altitude_ft: u32,
        latitude_deg: f32,
        longitude_deg: f32,
        eddy_dissipation_rate: f32,
        duration_s: u16,
    },
}

/// Sea surface temperature observation.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeaSurfaceTemp {
    /// Buoy or ship station identifier.
    station_id: u32,
    /// Latitude in degrees.
    latitude_deg: f32,
    /// Longitude in degrees.
    longitude_deg: f32,
    /// Sea surface temperature in degrees Celsius.
    sst_c: f32,
    /// Wave height in metres.
    wave_height_m: f32,
    /// Wave period in seconds.
    wave_period_s: f32,
}

// ── Tests 1–22 ────────────────────────────────────────────────────────────────

// ── 1. MetarObservation roundtrip ─────────────────────────────────────────────

#[test]
fn test_metar_observation_roundtrip() {
    proptest!(|(
        station_icao: u32,
        obs_time_s: u64,
        wind_dir_deg in 0u16..=360u16,
        wind_speed_kt in 0u16..150u16,
        wind_gust_kt in 0u16..200u16,
        visibility_sm in 0.0f32..15.0f32,
        temperature_c in (-60.0f32)..60.0f32,
        dewpoint_c in (-80.0f32)..40.0f32,
        altimeter_inhg in 27.0f32..32.0f32,
        is_automated: bool,
    )| {
        let val = MetarObservation {
            station_icao, obs_time_s, wind_dir_deg, wind_speed_kt,
            wind_gust_kt, visibility_sm, temperature_c, dewpoint_c,
            altimeter_inhg, is_automated,
        };
        let enc = encode_to_vec(&val).expect("encode MetarObservation failed");
        let (dec, consumed): (MetarObservation, usize) =
            decode_from_slice(&enc).expect("decode MetarObservation failed");
        prop_assert_eq!(&val, &dec, "MetarObservation roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 2. MetarObservation deterministic encoding ────────────────────────────────

#[test]
fn test_metar_observation_determinism() {
    proptest!(|(
        station_icao: u32,
        obs_time_s: u64,
        wind_dir_deg in 0u16..=360u16,
        wind_speed_kt in 0u16..150u16,
        wind_gust_kt in 0u16..200u16,
        visibility_sm in 0.0f32..15.0f32,
        temperature_c in (-60.0f32)..60.0f32,
        dewpoint_c in (-80.0f32)..40.0f32,
        altimeter_inhg in 27.0f32..32.0f32,
        is_automated: bool,
    )| {
        let val = MetarObservation {
            station_icao, obs_time_s, wind_dir_deg, wind_speed_kt,
            wind_gust_kt, visibility_sm, temperature_c, dewpoint_c,
            altimeter_inhg, is_automated,
        };
        let enc1 = encode_to_vec(&val).expect("first encode MetarObservation failed");
        let enc2 = encode_to_vec(&val).expect("second encode MetarObservation failed");
        prop_assert_eq!(enc1, enc2, "encoding must be deterministic");
    });
}

// ── 3. SoundingLevel roundtrip ────────────────────────────────────────────────

#[test]
fn test_sounding_level_roundtrip() {
    proptest!(|(
        pressure_hpa in 1.0f32..1050.0f32,
        height_gpm in 0.0f32..40000.0f32,
        temperature_c in (-90.0f32)..50.0f32,
        dewpoint_depression_c in 0.0f32..50.0f32,
        wind_dir_deg in 0u16..=360u16,
        wind_speed_kt in 0u16..300u16,
    )| {
        let val = SoundingLevel {
            pressure_hpa, height_gpm, temperature_c,
            dewpoint_depression_c, wind_dir_deg, wind_speed_kt,
        };
        let enc = encode_to_vec(&val).expect("encode SoundingLevel failed");
        let (dec, consumed): (SoundingLevel, usize) =
            decode_from_slice(&enc).expect("decode SoundingLevel failed");
        prop_assert_eq!(&val, &dec, "SoundingLevel roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 4. RadiosondeProfile with Vec<SoundingLevel> roundtrip ────────────────────

#[test]
fn test_radiosonde_profile_roundtrip() {
    proptest!(|(
        site_id: u32,
        launch_time_s: u64,
        levels in prop::collection::vec(
            (
                1.0f32..1050.0f32,
                0.0f32..40000.0f32,
                (-90.0f32)..50.0f32,
                0.0f32..50.0f32,
                0u16..=360u16,
                0u16..300u16,
            ).prop_map(|(pressure_hpa, height_gpm, temperature_c, dewpoint_depression_c, wind_dir_deg, wind_speed_kt)| {
                SoundingLevel { pressure_hpa, height_gpm, temperature_c, dewpoint_depression_c, wind_dir_deg, wind_speed_kt }
            }),
            0..12usize,
        ),
    )| {
        let val = RadiosondeProfile { site_id, launch_time_s, levels };
        let enc = encode_to_vec(&val).expect("encode RadiosondeProfile failed");
        let (dec, consumed): (RadiosondeProfile, usize) =
            decode_from_slice(&enc).expect("decode RadiosondeProfile failed");
        prop_assert_eq!(&val, &dec, "RadiosondeProfile roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 5. RadarReflectivityCell roundtrip ────────────────────────────────────────

#[test]
fn test_radar_reflectivity_cell_roundtrip() {
    proptest!(|(
        azimuth_deg in 0.0f32..360.0f32,
        range_km in 0.0f32..400.0f32,
        reflectivity_dbz in (-30.0f32)..75.0f32,
        elevation_deg in 0.0f32..20.0f32,
        zdr_db in (-4.0f32)..8.0f32,
    )| {
        let val = RadarReflectivityCell {
            azimuth_deg, range_km, reflectivity_dbz, elevation_deg, zdr_db,
        };
        let enc = encode_to_vec(&val).expect("encode RadarReflectivityCell failed");
        let (dec, consumed): (RadarReflectivityCell, usize) =
            decode_from_slice(&enc).expect("decode RadarReflectivityCell failed");
        prop_assert_eq!(&val, &dec, "RadarReflectivityCell roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 6. Vec<RadarReflectivityCell> roundtrip ───────────────────────────────────

#[test]
fn test_vec_radar_cells_roundtrip() {
    proptest!(|(
        cells in prop::collection::vec(
            (
                0.0f32..360.0f32,
                0.0f32..400.0f32,
                (-30.0f32)..75.0f32,
                0.0f32..20.0f32,
                (-4.0f32)..8.0f32,
            ).prop_map(|(azimuth_deg, range_km, reflectivity_dbz, elevation_deg, zdr_db)| {
                RadarReflectivityCell { azimuth_deg, range_km, reflectivity_dbz, elevation_deg, zdr_db }
            }),
            0..16usize,
        ),
    )| {
        let enc = encode_to_vec(&cells).expect("encode Vec<RadarReflectivityCell> failed");
        let (dec, consumed): (Vec<RadarReflectivityCell>, usize) =
            decode_from_slice(&enc).expect("decode Vec<RadarReflectivityCell> failed");
        prop_assert_eq!(&cells, &dec, "Vec<RadarReflectivityCell> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 7. CloudTopTemperature roundtrip ──────────────────────────────────────────

#[test]
fn test_cloud_top_temperature_roundtrip() {
    proptest!(|(
        latitude_deg in (-90.0f32)..90.0f32,
        longitude_deg in (-180.0f32)..180.0f32,
        brightness_temp_k in 180.0f32..320.0f32,
        channel_id in 1u8..16u8,
        scan_line: u32,
        pixel_index: u32,
    )| {
        let val = CloudTopTemperature {
            latitude_deg, longitude_deg, brightness_temp_k,
            channel_id, scan_line, pixel_index,
        };
        let enc = encode_to_vec(&val).expect("encode CloudTopTemperature failed");
        let (dec, consumed): (CloudTopTemperature, usize) =
            decode_from_slice(&enc).expect("decode CloudTopTemperature failed");
        prop_assert_eq!(&val, &dec, "CloudTopTemperature roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 8. NwpGridPoint roundtrip ─────────────────────────────────────────────────

#[test]
fn test_nwp_grid_point_roundtrip() {
    proptest!(|(
        latitude_deg in (-90.0f32)..90.0f32,
        longitude_deg in (-180.0f32)..180.0f32,
        forecast_hour in 0u16..384u16,
        temp_2m_k in 200.0f32..330.0f32,
        mslp_pa in 87000.0f32..108000.0f32,
        u_wind_10m in (-50.0f32)..50.0f32,
        v_wind_10m in (-50.0f32)..50.0f32,
        total_precip_mm in 0.0f32..500.0f32,
        rh_fraction in 0.0f32..1.0f32,
    )| {
        let val = NwpGridPoint {
            latitude_deg, longitude_deg, forecast_hour, temp_2m_k,
            mslp_pa, u_wind_10m, v_wind_10m, total_precip_mm, rh_fraction,
        };
        let enc = encode_to_vec(&val).expect("encode NwpGridPoint failed");
        let (dec, consumed): (NwpGridPoint, usize) =
            decode_from_slice(&enc).expect("decode NwpGridPoint failed");
        prop_assert_eq!(&val, &dec, "NwpGridPoint roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 9. NwpGridPoint re-encode idempotency ─────────────────────────────────────

#[test]
fn test_nwp_grid_point_reencode_idempotent() {
    proptest!(|(
        latitude_deg in (-90.0f32)..90.0f32,
        longitude_deg in (-180.0f32)..180.0f32,
        forecast_hour in 0u16..384u16,
        temp_2m_k in 200.0f32..330.0f32,
        mslp_pa in 87000.0f32..108000.0f32,
        u_wind_10m in (-50.0f32)..50.0f32,
        v_wind_10m in (-50.0f32)..50.0f32,
        total_precip_mm in 0.0f32..500.0f32,
        rh_fraction in 0.0f32..1.0f32,
    )| {
        let val = NwpGridPoint {
            latitude_deg, longitude_deg, forecast_hour, temp_2m_k,
            mslp_pa, u_wind_10m, v_wind_10m, total_precip_mm, rh_fraction,
        };
        let enc1 = encode_to_vec(&val).expect("first encode NwpGridPoint failed");
        let (decoded, _): (NwpGridPoint, usize) =
            decode_from_slice(&enc1).expect("decode NwpGridPoint failed");
        let enc2 = encode_to_vec(&decoded).expect("re-encode NwpGridPoint failed");
        prop_assert_eq!(enc1, enc2, "re-encoding must produce identical bytes");
    });
}

// ── 10. SevereWeatherWarning::Tornado roundtrip ───────────────────────────────

#[test]
fn test_severe_weather_tornado_roundtrip() {
    proptest!(|(
        ef_scale in 0u8..5u8,
        latitude_deg in (-90.0f32)..90.0f32,
        longitude_deg in (-180.0f32)..180.0f32,
        valid_until_s: u64,
    )| {
        let val = SevereWeatherWarning::Tornado {
            ef_scale, latitude_deg, longitude_deg, valid_until_s,
        };
        let enc = encode_to_vec(&val).expect("encode Tornado warning failed");
        let (dec, consumed): (SevereWeatherWarning, usize) =
            decode_from_slice(&enc).expect("decode Tornado warning failed");
        prop_assert_eq!(&val, &dec, "Tornado warning roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 11. SevereWeatherWarning::WinterStorm roundtrip ───────────────────────────

#[test]
fn test_severe_weather_winter_storm_roundtrip() {
    proptest!(|(
        snow_cm in 0.0f32..200.0f32,
        ice_accum_mm in 0.0f32..50.0f32,
        wind_chill_c in (-60.0f32)..0.0f32,
        valid_until_s: u64,
    )| {
        let val = SevereWeatherWarning::WinterStorm {
            snow_cm, ice_accum_mm, wind_chill_c, valid_until_s,
        };
        let enc = encode_to_vec(&val).expect("encode WinterStorm warning failed");
        let (dec, consumed): (SevereWeatherWarning, usize) =
            decode_from_slice(&enc).expect("decode WinterStorm warning failed");
        prop_assert_eq!(&val, &dec, "WinterStorm warning roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 12. SevereWeatherWarning all variants roundtrip ───────────────────────────

#[test]
fn test_severe_weather_all_variants_roundtrip() {
    let tornado_strat = (
        0u8..5u8,
        (-90.0f32)..90.0f32,
        (-180.0f32)..180.0f32,
        any::<u64>(),
    )
        .prop_map(|(ef_scale, latitude_deg, longitude_deg, valid_until_s)| {
            SevereWeatherWarning::Tornado {
                ef_scale,
                latitude_deg,
                longitude_deg,
                valid_until_s,
            }
        });
    let tstorm_strat = (0.0f32..12.0f32, 0u16..150u16, any::<u64>()).prop_map(
        |(max_hail_cm, max_wind_kt, valid_until_s)| SevereWeatherWarning::SevereThunderstorm {
            max_hail_cm,
            max_wind_kt,
            valid_until_s,
        },
    );
    let flood_strat = (0.0f32..500.0f32, 1.0f32..10000.0f32, any::<u64>()).prop_map(
        |(rainfall_mm, area_sq_km, valid_until_s)| SevereWeatherWarning::FlashFlood {
            rainfall_mm,
            area_sq_km,
            valid_until_s,
        },
    );
    let winter_strat = (
        0.0f32..200.0f32,
        0.0f32..50.0f32,
        (-60.0f32)..0.0f32,
        any::<u64>(),
    )
        .prop_map(|(snow_cm, ice_accum_mm, wind_chill_c, valid_until_s)| {
            SevereWeatherWarning::WinterStorm {
                snow_cm,
                ice_accum_mm,
                wind_chill_c,
                valid_until_s,
            }
        });

    let warning_strat = prop_oneof![tornado_strat, tstorm_strat, flood_strat, winter_strat];

    proptest!(|(val in warning_strat)| {
        let enc = encode_to_vec(&val).expect("encode SevereWeatherWarning failed");
        let (dec, consumed): (SevereWeatherWarning, usize) =
            decode_from_slice(&enc).expect("decode SevereWeatherWarning failed");
        prop_assert_eq!(&val, &dec, "SevereWeatherWarning roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 13. TropicalCycloneAdvisory roundtrip ─────────────────────────────────────

#[test]
fn test_tropical_cyclone_advisory_roundtrip() {
    proptest!(|(
        storm_id: u32,
        center_lat_deg in (-40.0f32)..40.0f32,
        center_lon_deg in (-180.0f32)..180.0f32,
        max_wind_kt in 30u16..200u16,
        central_pressure_hpa in 870.0f32..1020.0f32,
        rmw_nm in 5.0f32..200.0f32,
        movement_dir_deg in 0u16..=360u16,
        movement_speed_kt in 0u16..40u16,
        category in 0u8..5u8,
    )| {
        let val = TropicalCycloneAdvisory {
            storm_id, center_lat_deg, center_lon_deg, max_wind_kt,
            central_pressure_hpa, rmw_nm, movement_dir_deg, movement_speed_kt, category,
        };
        let enc = encode_to_vec(&val).expect("encode TropicalCycloneAdvisory failed");
        let (dec, consumed): (TropicalCycloneAdvisory, usize) =
            decode_from_slice(&enc).expect("decode TropicalCycloneAdvisory failed");
        prop_assert_eq!(&val, &dec, "TropicalCycloneAdvisory roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 14. Vec<TropicalCycloneAdvisory> roundtrip ────────────────────────────────

#[test]
fn test_vec_tropical_cyclone_advisory_roundtrip() {
    proptest!(|(
        items in prop::collection::vec(
            (
                any::<u32>(),
                (-40.0f32)..40.0f32,
                (-180.0f32)..180.0f32,
                30u16..200u16,
                870.0f32..1020.0f32,
                5.0f32..200.0f32,
                0u16..=360u16,
                0u16..40u16,
                0u8..5u8,
            ).prop_map(|(storm_id, center_lat_deg, center_lon_deg, max_wind_kt, central_pressure_hpa, rmw_nm, movement_dir_deg, movement_speed_kt, category)| {
                TropicalCycloneAdvisory {
                    storm_id, center_lat_deg, center_lon_deg, max_wind_kt,
                    central_pressure_hpa, rmw_nm, movement_dir_deg, movement_speed_kt, category,
                }
            }),
            0..6usize,
        ),
    )| {
        let enc = encode_to_vec(&items).expect("encode Vec<TropicalCycloneAdvisory> failed");
        let (dec, consumed): (Vec<TropicalCycloneAdvisory>, usize) =
            decode_from_slice(&enc).expect("decode Vec<TropicalCycloneAdvisory> failed");
        prop_assert_eq!(&items, &dec, "Vec<TropicalCycloneAdvisory> roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 15. PrecipitationAccumulation roundtrip ───────────────────────────────────

#[test]
fn test_precipitation_accumulation_roundtrip() {
    proptest!(|(
        gauge_id: u32,
        period_start_s: u64,
        period_end_s: u64,
        liquid_equiv_mm in 0.0f32..1000.0f32,
        snow_depth_cm in 0.0f32..500.0f32,
        gauge_type in 0u8..10u8,
    )| {
        let val = PrecipitationAccumulation {
            gauge_id, period_start_s, period_end_s,
            liquid_equiv_mm, snow_depth_cm, gauge_type,
        };
        let enc = encode_to_vec(&val).expect("encode PrecipitationAccumulation failed");
        let (dec, consumed): (PrecipitationAccumulation, usize) =
            decode_from_slice(&enc).expect("decode PrecipitationAccumulation failed");
        prop_assert_eq!(&val, &dec, "PrecipitationAccumulation roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 16. VisibilityCeiling roundtrip ───────────────────────────────────────────

#[test]
fn test_visibility_ceiling_roundtrip() {
    proptest!(|(
        station_id: u32,
        visibility_m in 0.0f32..50000.0f32,
        ceiling_ft in 0u32..60000u32,
        ceiling_variable: bool,
        cloud_layers in 0u8..8u8,
        obs_time_s: u64,
    )| {
        let val = VisibilityCeiling {
            station_id, visibility_m, ceiling_ft,
            ceiling_variable, cloud_layers, obs_time_s,
        };
        let enc = encode_to_vec(&val).expect("encode VisibilityCeiling failed");
        let (dec, consumed): (VisibilityCeiling, usize) =
            decode_from_slice(&enc).expect("decode VisibilityCeiling failed");
        prop_assert_eq!(&val, &dec, "VisibilityCeiling roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 17. WindShearAlert roundtrip ──────────────────────────────────────────────

#[test]
fn test_wind_shear_alert_roundtrip() {
    proptest!(|(
        runway_id in 1u16..36u16,
        airport_icao: u32,
        delta_speed_kt in (-50i16)..50i16,
        altitude_ft in 0u32..3000u32,
        detect_time_s: u64,
        severity in 1u8..=3u8,
    )| {
        let val = WindShearAlert {
            runway_id, airport_icao, delta_speed_kt,
            altitude_ft, detect_time_s, severity,
        };
        let enc = encode_to_vec(&val).expect("encode WindShearAlert failed");
        let (dec, consumed): (WindShearAlert, usize) =
            decode_from_slice(&enc).expect("decode WindShearAlert failed");
        prop_assert_eq!(&val, &dec, "WindShearAlert roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 18. AirQualityIndex roundtrip ─────────────────────────────────────────────

#[test]
fn test_air_quality_index_roundtrip() {
    proptest!(|(
        station_id: u32,
        pm25_ug_m3 in 0.0f32..500.0f32,
        pm10_ug_m3 in 0.0f32..600.0f32,
        ozone_ppb in 0.0f32..300.0f32,
        no2_ppb in 0.0f32..200.0f32,
        aqi_value in 0u16..500u16,
        obs_time_s: u64,
    )| {
        let val = AirQualityIndex {
            station_id, pm25_ug_m3, pm10_ug_m3,
            ozone_ppb, no2_ppb, aqi_value, obs_time_s,
        };
        let enc = encode_to_vec(&val).expect("encode AirQualityIndex failed");
        let (dec, consumed): (AirQualityIndex, usize) =
            decode_from_slice(&enc).expect("decode AirQualityIndex failed");
        prop_assert_eq!(&val, &dec, "AirQualityIndex roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 19. FrostFreezePrediction roundtrip ───────────────────────────────────────

#[test]
fn test_frost_freeze_prediction_roundtrip() {
    proptest!(|(
        zone_id: u32,
        min_temp_c in (-30.0f32)..10.0f32,
        ground_temp_c in (-35.0f32)..15.0f32,
        wind_speed_ms in 0.0f32..30.0f32,
        cloud_cover in 0.0f32..1.0f32,
        frost_probability in 0.0f32..1.0f32,
        freeze_warning: bool,
    )| {
        let val = FrostFreezePrediction {
            zone_id, min_temp_c, ground_temp_c,
            wind_speed_ms, cloud_cover, frost_probability, freeze_warning,
        };
        let enc = encode_to_vec(&val).expect("encode FrostFreezePrediction failed");
        let (dec, consumed): (FrostFreezePrediction, usize) =
            decode_from_slice(&enc).expect("decode FrostFreezePrediction failed");
        prop_assert_eq!(&val, &dec, "FrostFreezePrediction roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 20. LightningStrike roundtrip ─────────────────────────────────────────────

#[test]
fn test_lightning_strike_roundtrip() {
    proptest!(|(
        latitude_deg in (-90.0f32)..90.0f32,
        longitude_deg in (-180.0f32)..180.0f32,
        peak_current_ka in (-300.0f32)..300.0f32,
        time_s: u64,
        sensor_count in 3u8..32u8,
        cloud_to_ground: bool,
    )| {
        let val = LightningStrike {
            latitude_deg, longitude_deg, peak_current_ka,
            time_s, sensor_count, cloud_to_ground,
        };
        let enc = encode_to_vec(&val).expect("encode LightningStrike failed");
        let (dec, consumed): (LightningStrike, usize) =
            decode_from_slice(&enc).expect("decode LightningStrike failed");
        prop_assert_eq!(&val, &dec, "LightningStrike roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 21. TurbulenceReport all variants roundtrip ───────────────────────────────

#[test]
fn test_turbulence_report_all_variants_roundtrip() {
    let smooth_strat = Just(TurbulenceReport::Smooth);
    let light_strat = (0u32..45000u32, (-90.0f32)..90.0f32, (-180.0f32)..180.0f32).prop_map(
        |(altitude_ft, latitude_deg, longitude_deg)| TurbulenceReport::Light {
            altitude_ft,
            latitude_deg,
            longitude_deg,
        },
    );
    let moderate_strat = (
        0u32..45000u32,
        (-90.0f32)..90.0f32,
        (-180.0f32)..180.0f32,
        0.0f32..0.5f32,
    )
        .prop_map(
            |(altitude_ft, latitude_deg, longitude_deg, eddy_dissipation_rate)| {
                TurbulenceReport::Moderate {
                    altitude_ft,
                    latitude_deg,
                    longitude_deg,
                    eddy_dissipation_rate,
                }
            },
        );
    let severe_strat = (
        0u32..45000u32,
        (-90.0f32)..90.0f32,
        (-180.0f32)..180.0f32,
        0.0f32..1.0f32,
        0u16..600u16,
    )
        .prop_map(
            |(altitude_ft, latitude_deg, longitude_deg, eddy_dissipation_rate, duration_s)| {
                TurbulenceReport::Severe {
                    altitude_ft,
                    latitude_deg,
                    longitude_deg,
                    eddy_dissipation_rate,
                    duration_s,
                }
            },
        );

    let turb_strat = prop_oneof![smooth_strat, light_strat, moderate_strat, severe_strat];

    proptest!(|(val in turb_strat)| {
        let enc = encode_to_vec(&val).expect("encode TurbulenceReport failed");
        let (dec, consumed): (TurbulenceReport, usize) =
            decode_from_slice(&enc).expect("decode TurbulenceReport failed");
        prop_assert_eq!(&val, &dec, "TurbulenceReport roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}

// ── 22. SeaSurfaceTemp roundtrip ──────────────────────────────────────────────

#[test]
fn test_sea_surface_temp_roundtrip() {
    proptest!(|(
        station_id: u32,
        latitude_deg in (-90.0f32)..90.0f32,
        longitude_deg in (-180.0f32)..180.0f32,
        sst_c in (-2.0f32)..35.0f32,
        wave_height_m in 0.0f32..20.0f32,
        wave_period_s in 1.0f32..25.0f32,
    )| {
        let val = SeaSurfaceTemp {
            station_id, latitude_deg, longitude_deg,
            sst_c, wave_height_m, wave_period_s,
        };
        let enc = encode_to_vec(&val).expect("encode SeaSurfaceTemp failed");
        let (dec, consumed): (SeaSurfaceTemp, usize) =
            decode_from_slice(&enc).expect("decode SeaSurfaceTemp failed");
        prop_assert_eq!(&val, &dec, "SeaSurfaceTemp roundtrip mismatch");
        prop_assert_eq!(consumed, enc.len(), "consumed bytes must equal enc.len()");
    });
}
