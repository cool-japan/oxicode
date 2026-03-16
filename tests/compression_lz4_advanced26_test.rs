#![cfg(feature = "compression-lz4")]
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

fn compress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Lz4).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

// ---------------------------------------------------------------------------
// Domain types for climate science and atmospheric modeling
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GreenhouseGasConcentration {
    year: u16,
    co2_ppm: f64,
    ch4_ppb: f64,
    n2o_ppb: f64,
    station_id: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GlobalCirculationModelOutput {
    model_name: String,
    grid_lat: Vec<f64>,
    grid_lon: Vec<f64>,
    temperature_field_k: Vec<f64>,
    pressure_field_hpa: Vec<f64>,
    wind_u_ms: Vec<f64>,
    wind_v_ms: Vec<f64>,
    timestep_hours: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IceCoreProxyRecord {
    core_id: String,
    depth_m: Vec<f64>,
    age_years_bp: Vec<u64>,
    delta_o18: Vec<f64>,
    delta_d: Vec<f64>,
    dust_concentration_mg_per_kg: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeaLevelRiseProjection {
    scenario_name: String,
    base_year: u16,
    projection_years: Vec<u16>,
    thermal_expansion_mm: Vec<f64>,
    glacier_contribution_mm: Vec<f64>,
    ice_sheet_contribution_mm: Vec<f64>,
    total_rise_mm: Vec<f64>,
    confidence_interval_lower: Vec<f64>,
    confidence_interval_upper: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AlbedoMeasurement {
    latitude: f64,
    longitude: f64,
    surface_type: String,
    broadband_albedo: f64,
    nir_albedo: f64,
    visible_albedo: f64,
    observation_day_of_year: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AerosolOpticalDepth {
    station_name: String,
    wavelength_nm: Vec<u16>,
    aod_values: Vec<f64>,
    angstrom_exponent: f64,
    single_scattering_albedo: f64,
    measurement_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiativeForcingEstimate {
    agent_name: String,
    forcing_wm2: f64,
    uncertainty_low: f64,
    uncertainty_high: f64,
    confidence_level: String,
    reference_year: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PaleoclimateReconstruction {
    proxy_type: String,
    region: String,
    time_series_years_bp: Vec<u64>,
    temperature_anomaly_c: Vec<f64>,
    precipitation_anomaly_mm: Vec<f64>,
    reconstruction_method: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExtremeWeatherEvent {
    event_type: String,
    category: u8,
    start_year: u16,
    duration_days: u32,
    affected_area_km2: f64,
    max_intensity: f64,
    attribution_score: f64,
    location: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CarbonCycleFlux {
    flux_name: String,
    source_reservoir: String,
    sink_reservoir: String,
    flux_gt_c_per_year: f64,
    uncertainty_gt_c: f64,
    measurement_method: String,
    year: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OceanHeatContent {
    basin_name: String,
    depth_range_m: (u32, u32),
    heat_content_zj: Vec<f64>,
    years: Vec<u16>,
    trend_zj_per_decade: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PermafrostThawMonitoring {
    site_id: String,
    latitude: f64,
    longitude: f64,
    active_layer_thickness_m: Vec<f64>,
    ground_temperature_c: Vec<f64>,
    measurement_years: Vec<u16>,
    soil_carbon_kg_per_m2: f64,
    thaw_rate_cm_per_year: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClimateSensitivityParameter {
    model_name: String,
    ecs_k: f64,
    tcr_k: f64,
    feedback_parameter_wm2k: f64,
    cloud_feedback: f64,
    water_vapor_feedback: f64,
    lapse_rate_feedback: f64,
    albedo_feedback: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmissionScenarioPathway {
    scenario_id: String,
    framework: String,
    years: Vec<u16>,
    co2_emissions_gt: Vec<f64>,
    ch4_emissions_mt: Vec<f64>,
    cumulative_co2_gt: Vec<f64>,
    peak_warming_c: f64,
    net_zero_year: Option<u16>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DroughtSeverityIndex {
    region: String,
    index_type: String,
    months: Vec<String>,
    index_values: Vec<f64>,
    soil_moisture_anomaly: Vec<f64>,
    affected_population: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExtremeWeatherCatalog {
    catalog_year: u16,
    events: Vec<ExtremeWeatherEvent>,
    total_economic_loss_billion_usd: f64,
    total_fatalities: u64,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_greenhouse_gas_concentration_lz4_roundtrip() {
    let val = GreenhouseGasConcentration {
        year: 2024,
        co2_ppm: 421.37,
        ch4_ppb: 1923.5,
        n2o_ppb: 336.7,
        station_id: "MLO-Mauna-Loa".to_string(),
    };
    let enc = encode_to_vec(&val).expect("encode greenhouse gas");
    let compressed = compress_lz4(&enc).expect("compress greenhouse gas");
    let decompressed = decompress_lz4(&compressed).expect("decompress greenhouse gas");
    let (decoded, _): (GreenhouseGasConcentration, usize) =
        decode_from_slice(&decompressed).expect("decode greenhouse gas");
    assert_eq!(val, decoded);
}

#[test]
fn test_global_circulation_model_output_lz4_roundtrip() {
    let val = GlobalCirculationModelOutput {
        model_name: "CESM2-WACCM".to_string(),
        grid_lat: (0..18).map(|i| -85.0 + (i as f64) * 10.0).collect(),
        grid_lon: (0..36).map(|i| (i as f64) * 10.0).collect(),
        temperature_field_k: (0..648).map(|i| 200.0 + (i as f64) * 0.15).collect(),
        pressure_field_hpa: (0..648).map(|i| 1013.25 - (i as f64) * 0.5).collect(),
        wind_u_ms: (0..648).map(|i| ((i as f64) * 0.1).sin() * 15.0).collect(),
        wind_v_ms: (0..648).map(|i| ((i as f64) * 0.07).cos() * 10.0).collect(),
        timestep_hours: 6,
    };
    let enc = encode_to_vec(&val).expect("encode gcm output");
    let compressed = compress_lz4(&enc).expect("compress gcm output");
    let decompressed = decompress_lz4(&compressed).expect("decompress gcm output");
    let (decoded, _): (GlobalCirculationModelOutput, usize) =
        decode_from_slice(&decompressed).expect("decode gcm output");
    assert_eq!(val, decoded);
}

#[test]
fn test_gcm_output_compression_ratio() {
    let val = GlobalCirculationModelOutput {
        model_name: "GFDL-ESM4".to_string(),
        grid_lat: (0..36).map(|i| -87.5 + (i as f64) * 5.0).collect(),
        grid_lon: (0..72).map(|i| (i as f64) * 5.0).collect(),
        temperature_field_k: vec![288.15; 2592],
        pressure_field_hpa: vec![1013.25; 2592],
        wind_u_ms: vec![0.0; 2592],
        wind_v_ms: vec![0.0; 2592],
        timestep_hours: 3,
    };
    let enc = encode_to_vec(&val).expect("encode gcm for ratio");
    let compressed = compress_lz4(&enc).expect("compress gcm for ratio");
    assert!(
        compressed.len() < enc.len(),
        "repetitive GCM data should compress: {} < {}",
        compressed.len(),
        enc.len()
    );
}

#[test]
fn test_ice_core_proxy_record_lz4_roundtrip() {
    let depths: Vec<f64> = (0..200).map(|i| (i as f64) * 0.5).collect();
    let ages: Vec<u64> = (0..200).map(|i| (i as u64) * 400).collect();
    let val = IceCoreProxyRecord {
        core_id: "EPICA-Dome-C".to_string(),
        depth_m: depths,
        age_years_bp: ages,
        delta_o18: (0..200).map(|i| -55.0 + (i as f64) * 0.05).collect(),
        delta_d: (0..200).map(|i| -430.0 + (i as f64) * 0.4).collect(),
        dust_concentration_mg_per_kg: (0..200).map(|i| 0.02 + (i as f64) * 0.001).collect(),
    };
    let enc = encode_to_vec(&val).expect("encode ice core");
    let compressed = compress_lz4(&enc).expect("compress ice core");
    let decompressed = decompress_lz4(&compressed).expect("decompress ice core");
    let (decoded, _): (IceCoreProxyRecord, usize) =
        decode_from_slice(&decompressed).expect("decode ice core");
    assert_eq!(val, decoded);
}

#[test]
fn test_sea_level_rise_projection_lz4_roundtrip() {
    let years: Vec<u16> = (2020..=2100).step_by(10).collect();
    let n = years.len();
    let val = SeaLevelRiseProjection {
        scenario_name: "SSP5-8.5 High-End".to_string(),
        base_year: 2020,
        projection_years: years,
        thermal_expansion_mm: (0..n).map(|i| (i as f64) * 25.0).collect(),
        glacier_contribution_mm: (0..n).map(|i| (i as f64) * 12.0).collect(),
        ice_sheet_contribution_mm: (0..n).map(|i| (i as f64) * 18.0).collect(),
        total_rise_mm: (0..n).map(|i| (i as f64) * 55.0).collect(),
        confidence_interval_lower: (0..n).map(|i| (i as f64) * 30.0).collect(),
        confidence_interval_upper: (0..n).map(|i| (i as f64) * 80.0).collect(),
    };
    let enc = encode_to_vec(&val).expect("encode sea level");
    let compressed = compress_lz4(&enc).expect("compress sea level");
    let decompressed = decompress_lz4(&compressed).expect("decompress sea level");
    let (decoded, _): (SeaLevelRiseProjection, usize) =
        decode_from_slice(&decompressed).expect("decode sea level");
    assert_eq!(val, decoded);
}

#[test]
fn test_albedo_measurement_lz4_roundtrip() {
    let val = AlbedoMeasurement {
        latitude: 71.32,
        longitude: -156.61,
        surface_type: "sea-ice-first-year".to_string(),
        broadband_albedo: 0.72,
        nir_albedo: 0.58,
        visible_albedo: 0.87,
        observation_day_of_year: 91,
    };
    let enc = encode_to_vec(&val).expect("encode albedo");
    let compressed = compress_lz4(&enc).expect("compress albedo");
    let decompressed = decompress_lz4(&compressed).expect("decompress albedo");
    let (decoded, _): (AlbedoMeasurement, usize) =
        decode_from_slice(&decompressed).expect("decode albedo");
    assert_eq!(val, decoded);
}

#[test]
fn test_aerosol_optical_depth_lz4_roundtrip() {
    let val = AerosolOpticalDepth {
        station_name: "AERONET-Goddard-GSFC".to_string(),
        wavelength_nm: vec![340, 380, 440, 500, 675, 870, 1020, 1640],
        aod_values: vec![0.45, 0.38, 0.30, 0.24, 0.14, 0.09, 0.06, 0.03],
        angstrom_exponent: 1.62,
        single_scattering_albedo: 0.93,
        measurement_timestamp: 1711000000,
    };
    let enc = encode_to_vec(&val).expect("encode aod");
    let compressed = compress_lz4(&enc).expect("compress aod");
    let decompressed = decompress_lz4(&compressed).expect("decompress aod");
    let (decoded, _): (AerosolOpticalDepth, usize) =
        decode_from_slice(&decompressed).expect("decode aod");
    assert_eq!(val, decoded);
}

#[test]
fn test_radiative_forcing_estimates_vec_lz4_roundtrip() {
    let val: Vec<RadiativeForcingEstimate> = vec![
        RadiativeForcingEstimate {
            agent_name: "CO2".to_string(),
            forcing_wm2: 2.16,
            uncertainty_low: 1.90,
            uncertainty_high: 2.41,
            confidence_level: "very high".to_string(),
            reference_year: 2023,
        },
        RadiativeForcingEstimate {
            agent_name: "CH4".to_string(),
            forcing_wm2: 0.54,
            uncertainty_low: 0.43,
            uncertainty_high: 0.65,
            confidence_level: "high".to_string(),
            reference_year: 2023,
        },
        RadiativeForcingEstimate {
            agent_name: "N2O".to_string(),
            forcing_wm2: 0.21,
            uncertainty_low: 0.18,
            uncertainty_high: 0.24,
            confidence_level: "high".to_string(),
            reference_year: 2023,
        },
        RadiativeForcingEstimate {
            agent_name: "Tropospheric ozone".to_string(),
            forcing_wm2: 0.47,
            uncertainty_low: 0.24,
            uncertainty_high: 0.70,
            confidence_level: "medium".to_string(),
            reference_year: 2023,
        },
        RadiativeForcingEstimate {
            agent_name: "Sulphate aerosol (direct)".to_string(),
            forcing_wm2: -0.50,
            uncertainty_low: -0.80,
            uncertainty_high: -0.20,
            confidence_level: "medium".to_string(),
            reference_year: 2023,
        },
    ];
    let enc = encode_to_vec(&val).expect("encode forcing vec");
    let compressed = compress_lz4(&enc).expect("compress forcing vec");
    let decompressed = decompress_lz4(&compressed).expect("decompress forcing vec");
    let (decoded, _): (Vec<RadiativeForcingEstimate>, usize) =
        decode_from_slice(&decompressed).expect("decode forcing vec");
    assert_eq!(val, decoded);
}

#[test]
fn test_paleoclimate_reconstruction_lz4_roundtrip() {
    let val = PaleoclimateReconstruction {
        proxy_type: "tree-ring-width".to_string(),
        region: "Northern Hemisphere extratropical".to_string(),
        time_series_years_bp: (0..500).map(|i| (i as u64) * 2).collect(),
        temperature_anomaly_c: (0..500)
            .map(|i| ((i as f64) * 0.0126).sin() * 0.8)
            .collect(),
        precipitation_anomaly_mm: (0..500)
            .map(|i| ((i as f64) * 0.031).cos() * 50.0)
            .collect(),
        reconstruction_method: "Composite-Plus-Scale (CPS)".to_string(),
    };
    let enc = encode_to_vec(&val).expect("encode paleoclimate");
    let compressed = compress_lz4(&enc).expect("compress paleoclimate");
    let decompressed = decompress_lz4(&compressed).expect("decompress paleoclimate");
    let (decoded, _): (PaleoclimateReconstruction, usize) =
        decode_from_slice(&decompressed).expect("decode paleoclimate");
    assert_eq!(val, decoded);
}

#[test]
fn test_extreme_weather_event_lz4_roundtrip() {
    let val = ExtremeWeatherEvent {
        event_type: "Tropical Cyclone".to_string(),
        category: 5,
        start_year: 2023,
        duration_days: 14,
        affected_area_km2: 850_000.0,
        max_intensity: 295.0,
        attribution_score: 0.34,
        location: "Western Pacific Basin".to_string(),
    };
    let enc = encode_to_vec(&val).expect("encode extreme weather");
    let compressed = compress_lz4(&enc).expect("compress extreme weather");
    let decompressed = decompress_lz4(&compressed).expect("decompress extreme weather");
    let (decoded, _): (ExtremeWeatherEvent, usize) =
        decode_from_slice(&decompressed).expect("decode extreme weather");
    assert_eq!(val, decoded);
}

#[test]
fn test_carbon_cycle_flux_lz4_roundtrip() {
    let val = CarbonCycleFlux {
        flux_name: "Terrestrial net primary production".to_string(),
        source_reservoir: "Atmosphere".to_string(),
        sink_reservoir: "Land biosphere".to_string(),
        flux_gt_c_per_year: 123.0,
        uncertainty_gt_c: 8.0,
        measurement_method: "FLUXNET eddy covariance + upscaling".to_string(),
        year: 2022,
    };
    let enc = encode_to_vec(&val).expect("encode carbon flux");
    let compressed = compress_lz4(&enc).expect("compress carbon flux");
    let decompressed = decompress_lz4(&compressed).expect("decompress carbon flux");
    let (decoded, _): (CarbonCycleFlux, usize) =
        decode_from_slice(&decompressed).expect("decode carbon flux");
    assert_eq!(val, decoded);
}

#[test]
fn test_ocean_heat_content_lz4_roundtrip() {
    let val = OceanHeatContent {
        basin_name: "Global Ocean 0-2000m".to_string(),
        depth_range_m: (0, 2000),
        heat_content_zj: (1960..=2023)
            .map(|y| ((y - 1960) as f64) * 5.8 + 10.0)
            .collect(),
        years: (1960..=2023).map(|y| y as u16).collect(),
        trend_zj_per_decade: 9.1,
    };
    let enc = encode_to_vec(&val).expect("encode ohc");
    let compressed = compress_lz4(&enc).expect("compress ohc");
    let decompressed = decompress_lz4(&compressed).expect("decompress ohc");
    let (decoded, _): (OceanHeatContent, usize) =
        decode_from_slice(&decompressed).expect("decode ohc");
    assert_eq!(val, decoded);
}

#[test]
fn test_permafrost_thaw_monitoring_lz4_roundtrip() {
    let val = PermafrostThawMonitoring {
        site_id: "CALM-RU-07-Yakutsk".to_string(),
        latitude: 62.03,
        longitude: 129.68,
        active_layer_thickness_m: (0..30).map(|i| 1.2 + (i as f64) * 0.03).collect(),
        ground_temperature_c: (0..30).map(|i| -3.5 + (i as f64) * 0.08).collect(),
        measurement_years: (1995..=2024).map(|y| y as u16).collect(),
        soil_carbon_kg_per_m2: 48.7,
        thaw_rate_cm_per_year: 0.9,
    };
    let enc = encode_to_vec(&val).expect("encode permafrost");
    let compressed = compress_lz4(&enc).expect("compress permafrost");
    let decompressed = decompress_lz4(&compressed).expect("decompress permafrost");
    let (decoded, _): (PermafrostThawMonitoring, usize) =
        decode_from_slice(&decompressed).expect("decode permafrost");
    assert_eq!(val, decoded);
}

#[test]
fn test_climate_sensitivity_parameter_lz4_roundtrip() {
    let val = ClimateSensitivityParameter {
        model_name: "UKESM1-0-LL".to_string(),
        ecs_k: 5.34,
        tcr_k: 2.79,
        feedback_parameter_wm2k: -0.69,
        cloud_feedback: 0.45,
        water_vapor_feedback: 1.80,
        lapse_rate_feedback: -0.60,
        albedo_feedback: 0.35,
    };
    let enc = encode_to_vec(&val).expect("encode sensitivity");
    let compressed = compress_lz4(&enc).expect("compress sensitivity");
    let decompressed = decompress_lz4(&compressed).expect("decompress sensitivity");
    let (decoded, _): (ClimateSensitivityParameter, usize) =
        decode_from_slice(&decompressed).expect("decode sensitivity");
    assert_eq!(val, decoded);
}

#[test]
fn test_emission_scenario_pathway_ssp_lz4_roundtrip() {
    let years: Vec<u16> = (2020..=2100).step_by(5).collect();
    let n = years.len();
    let val = EmissionScenarioPathway {
        scenario_id: "SSP2-4.5".to_string(),
        framework: "Shared Socioeconomic Pathways".to_string(),
        years,
        co2_emissions_gt: (0..n)
            .map(|i| 40.0 - (i as f64) * 2.0 + ((i as f64) * 0.3).sin() * 1.5)
            .collect(),
        ch4_emissions_mt: (0..n).map(|i| 370.0 - (i as f64) * 8.0).collect(),
        cumulative_co2_gt: (0..n)
            .map(|i| 40.0 * (i as f64) - (i as f64).powi(2))
            .collect(),
        peak_warming_c: 2.7,
        net_zero_year: Some(2075),
    };
    let enc = encode_to_vec(&val).expect("encode ssp pathway");
    let compressed = compress_lz4(&enc).expect("compress ssp pathway");
    let decompressed = decompress_lz4(&compressed).expect("decompress ssp pathway");
    let (decoded, _): (EmissionScenarioPathway, usize) =
        decode_from_slice(&decompressed).expect("decode ssp pathway");
    assert_eq!(val, decoded);
}

#[test]
fn test_emission_scenario_rcp_no_net_zero_lz4_roundtrip() {
    let years: Vec<u16> = (2020..=2100).step_by(10).collect();
    let n = years.len();
    let val = EmissionScenarioPathway {
        scenario_id: "RCP8.5".to_string(),
        framework: "Representative Concentration Pathways".to_string(),
        years,
        co2_emissions_gt: (0..n).map(|i| 40.0 + (i as f64) * 5.0).collect(),
        ch4_emissions_mt: (0..n).map(|i| 370.0 + (i as f64) * 15.0).collect(),
        cumulative_co2_gt: (0..n)
            .map(|i| 40.0 * (i as f64) + (i as f64).powi(2) * 2.5)
            .collect(),
        peak_warming_c: 4.8,
        net_zero_year: None,
    };
    let enc = encode_to_vec(&val).expect("encode rcp pathway");
    let compressed = compress_lz4(&enc).expect("compress rcp pathway");
    let decompressed = decompress_lz4(&compressed).expect("decompress rcp pathway");
    let (decoded, _): (EmissionScenarioPathway, usize) =
        decode_from_slice(&decompressed).expect("decode rcp pathway");
    assert_eq!(val, decoded);
}

#[test]
fn test_drought_severity_index_lz4_roundtrip() {
    let val = DroughtSeverityIndex {
        region: "US Southern Great Plains".to_string(),
        index_type: "Palmer Drought Severity Index (PDSI)".to_string(),
        months: vec![
            "2023-01".to_string(),
            "2023-02".to_string(),
            "2023-03".to_string(),
            "2023-04".to_string(),
            "2023-05".to_string(),
            "2023-06".to_string(),
            "2023-07".to_string(),
            "2023-08".to_string(),
            "2023-09".to_string(),
            "2023-10".to_string(),
            "2023-11".to_string(),
            "2023-12".to_string(),
        ],
        index_values: vec![
            -3.2, -3.5, -3.8, -4.1, -4.4, -4.0, -3.6, -3.1, -2.8, -2.4, -2.0, -1.7,
        ],
        soil_moisture_anomaly: vec![
            -25.0, -28.0, -32.0, -36.0, -40.0, -38.0, -34.0, -29.0, -24.0, -20.0, -16.0, -12.0,
        ],
        affected_population: 12_500_000,
    };
    let enc = encode_to_vec(&val).expect("encode drought");
    let compressed = compress_lz4(&enc).expect("compress drought");
    let decompressed = decompress_lz4(&compressed).expect("decompress drought");
    let (decoded, _): (DroughtSeverityIndex, usize) =
        decode_from_slice(&decompressed).expect("decode drought");
    assert_eq!(val, decoded);
}

#[test]
fn test_extreme_weather_catalog_lz4_roundtrip() {
    let events = vec![
        ExtremeWeatherEvent {
            event_type: "Heat wave".to_string(),
            category: 3,
            start_year: 2023,
            duration_days: 21,
            affected_area_km2: 2_500_000.0,
            max_intensity: 49.5,
            attribution_score: 0.87,
            location: "Southern Europe".to_string(),
        },
        ExtremeWeatherEvent {
            event_type: "Atmospheric river".to_string(),
            category: 4,
            start_year: 2023,
            duration_days: 5,
            affected_area_km2: 180_000.0,
            max_intensity: 210.0,
            attribution_score: 0.42,
            location: "California, USA".to_string(),
        },
        ExtremeWeatherEvent {
            event_type: "Drought".to_string(),
            category: 4,
            start_year: 2022,
            duration_days: 365,
            affected_area_km2: 3_200_000.0,
            max_intensity: -4.5,
            attribution_score: 0.65,
            location: "Horn of Africa".to_string(),
        },
    ];
    let val = ExtremeWeatherCatalog {
        catalog_year: 2023,
        events,
        total_economic_loss_billion_usd: 380.0,
        total_fatalities: 74_000,
    };
    let enc = encode_to_vec(&val).expect("encode catalog");
    let compressed = compress_lz4(&enc).expect("compress catalog");
    let decompressed = decompress_lz4(&compressed).expect("decompress catalog");
    let (decoded, _): (ExtremeWeatherCatalog, usize) =
        decode_from_slice(&decompressed).expect("decode catalog");
    assert_eq!(val, decoded);
}

#[test]
fn test_greenhouse_gas_time_series_compression_ratio() {
    let val: Vec<GreenhouseGasConcentration> = (1958..=2024)
        .map(|y| GreenhouseGasConcentration {
            year: y as u16,
            co2_ppm: 315.0 + ((y - 1958) as f64) * 1.5 + ((y as f64) * 0.1).sin() * 3.0,
            ch4_ppb: 1200.0 + ((y - 1958) as f64) * 10.0,
            n2o_ppb: 290.0 + ((y - 1958) as f64) * 0.7,
            station_id: "MLO".to_string(),
        })
        .collect();
    let enc = encode_to_vec(&val).expect("encode ghg series");
    let compressed = compress_lz4(&enc).expect("compress ghg series");
    assert!(
        compressed.len() < enc.len(),
        "repeated station_id in GHG series should help compression: compressed={} vs original={}",
        compressed.len(),
        enc.len()
    );
}

#[test]
fn test_multiple_albedo_measurements_lz4_roundtrip() {
    let val: Vec<AlbedoMeasurement> = vec![
        AlbedoMeasurement {
            latitude: -89.98,
            longitude: 0.0,
            surface_type: "ice-sheet".to_string(),
            broadband_albedo: 0.84,
            nir_albedo: 0.73,
            visible_albedo: 0.96,
            observation_day_of_year: 1,
        },
        AlbedoMeasurement {
            latitude: 0.0,
            longitude: -60.0,
            surface_type: "tropical-forest".to_string(),
            broadband_albedo: 0.13,
            nir_albedo: 0.22,
            visible_albedo: 0.04,
            observation_day_of_year: 180,
        },
        AlbedoMeasurement {
            latitude: 35.0,
            longitude: 25.0,
            surface_type: "desert-sand".to_string(),
            broadband_albedo: 0.40,
            nir_albedo: 0.45,
            visible_albedo: 0.35,
            observation_day_of_year: 200,
        },
        AlbedoMeasurement {
            latitude: 60.0,
            longitude: 10.0,
            surface_type: "ocean-open-water".to_string(),
            broadband_albedo: 0.06,
            nir_albedo: 0.04,
            visible_albedo: 0.08,
            observation_day_of_year: 172,
        },
    ];
    let enc = encode_to_vec(&val).expect("encode albedo vec");
    let compressed = compress_lz4(&enc).expect("compress albedo vec");
    let decompressed = decompress_lz4(&compressed).expect("decompress albedo vec");
    let (decoded, _): (Vec<AlbedoMeasurement>, usize) =
        decode_from_slice(&decompressed).expect("decode albedo vec");
    assert_eq!(val, decoded);
}

#[test]
fn test_climate_sensitivity_multi_model_ensemble_lz4_roundtrip() {
    let val: Vec<ClimateSensitivityParameter> = vec![
        ClimateSensitivityParameter {
            model_name: "CESM2".to_string(),
            ecs_k: 5.16,
            tcr_k: 2.06,
            feedback_parameter_wm2k: -0.72,
            cloud_feedback: 0.58,
            water_vapor_feedback: 1.77,
            lapse_rate_feedback: -0.55,
            albedo_feedback: 0.33,
        },
        ClimateSensitivityParameter {
            model_name: "GFDL-CM4".to_string(),
            ecs_k: 3.89,
            tcr_k: 2.05,
            feedback_parameter_wm2k: -0.95,
            cloud_feedback: 0.24,
            water_vapor_feedback: 1.82,
            lapse_rate_feedback: -0.62,
            albedo_feedback: 0.31,
        },
        ClimateSensitivityParameter {
            model_name: "MPI-ESM1-2-LR".to_string(),
            ecs_k: 2.98,
            tcr_k: 1.84,
            feedback_parameter_wm2k: -1.24,
            cloud_feedback: 0.04,
            water_vapor_feedback: 1.75,
            lapse_rate_feedback: -0.58,
            albedo_feedback: 0.30,
        },
    ];
    let enc = encode_to_vec(&val).expect("encode ensemble");
    let compressed = compress_lz4(&enc).expect("compress ensemble");
    let decompressed = decompress_lz4(&compressed).expect("decompress ensemble");
    let (decoded, _): (Vec<ClimateSensitivityParameter>, usize) =
        decode_from_slice(&decompressed).expect("decode ensemble");
    assert_eq!(val, decoded);
}

#[test]
fn test_carbon_cycle_flux_batch_compression_size() {
    let val: Vec<CarbonCycleFlux> = vec![
        CarbonCycleFlux {
            flux_name: "Fossil fuel emissions".to_string(),
            source_reservoir: "Lithosphere".to_string(),
            sink_reservoir: "Atmosphere".to_string(),
            flux_gt_c_per_year: 9.9,
            uncertainty_gt_c: 0.5,
            measurement_method: "IEA energy statistics".to_string(),
            year: 2022,
        },
        CarbonCycleFlux {
            flux_name: "Land use change".to_string(),
            source_reservoir: "Land biosphere".to_string(),
            sink_reservoir: "Atmosphere".to_string(),
            flux_gt_c_per_year: 1.2,
            uncertainty_gt_c: 0.7,
            measurement_method: "Bookkeeping models + satellite".to_string(),
            year: 2022,
        },
        CarbonCycleFlux {
            flux_name: "Ocean sink".to_string(),
            source_reservoir: "Atmosphere".to_string(),
            sink_reservoir: "Ocean".to_string(),
            flux_gt_c_per_year: 2.8,
            uncertainty_gt_c: 0.4,
            measurement_method: "pCO2-based flux products".to_string(),
            year: 2022,
        },
        CarbonCycleFlux {
            flux_name: "Land sink".to_string(),
            source_reservoir: "Atmosphere".to_string(),
            sink_reservoir: "Land biosphere".to_string(),
            flux_gt_c_per_year: 3.1,
            uncertainty_gt_c: 0.9,
            measurement_method: "Residual of global carbon budget".to_string(),
            year: 2022,
        },
    ];
    let enc = encode_to_vec(&val).expect("encode flux batch");
    let compressed = compress_lz4(&enc).expect("compress flux batch");
    let decompressed = decompress_lz4(&compressed).expect("decompress flux batch");
    let (decoded, _): (Vec<CarbonCycleFlux>, usize) =
        decode_from_slice(&decompressed).expect("decode flux batch");
    assert_eq!(val, decoded);
    // Verify compressed form is different from original
    assert_ne!(
        enc, compressed,
        "compressed bytes should differ from original encoding"
    );
}
