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

// ---------------------------------------------------------------------------
// Domain types — Oceanography & Marine Science
// ---------------------------------------------------------------------------

/// CTD profile sample (Conductivity, Temperature, Depth)
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CtdSample {
    depth_m: f64,
    temperature_c: f64,
    conductivity_sm: f64,
    pressure_dbar: f64,
}

/// Full CTD cast containing multiple samples
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CtdCast {
    station_id: u32,
    latitude: f64,
    longitude: f64,
    samples: Vec<CtdSample>,
}

/// ADCP (Acoustic Doppler Current Profiler) bin reading
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AdcpBin {
    bin_depth_m: f64,
    east_velocity_ms: f64,
    north_velocity_ms: f64,
    vertical_velocity_ms: f64,
    echo_intensity_db: u8,
}

/// ADCP ensemble (one ping cycle across all depth bins)
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AdcpEnsemble {
    ensemble_number: u32,
    timestamp_epoch_s: u64,
    heading_deg: f64,
    pitch_deg: f64,
    roll_deg: f64,
    bins: Vec<AdcpBin>,
}

/// Sea-surface temperature observation
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeaSurfaceTemp {
    lat: f64,
    lon: f64,
    sst_kelvin: f64,
    quality_flag: u8,
    satellite_id: u16,
}

/// Wave spectrum record
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaveSpectrum {
    significant_height_m: f64,
    peak_period_s: f64,
    mean_direction_deg: f64,
    spectral_width: f64,
    energy_densities: Vec<f64>,
}

/// Salinity profile measurement
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SalinityProfile {
    station_id: u32,
    depth_m: f64,
    practical_salinity_psu: f64,
    absolute_salinity_gkg: f64,
}

/// Dissolved oxygen observation
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DissolvedOxygen {
    depth_m: f64,
    concentration_umol_kg: f64,
    saturation_pct: f64,
    sensor_serial: u32,
}

/// Chlorophyll fluorescence reading
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChlorophyllFluorescence {
    depth_m: f64,
    fluorescence_rfu: f64,
    chlorophyll_a_ug_l: f64,
    turbidity_ntu: f64,
}

/// Bathymetric survey point
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BathymetryPoint {
    easting_m: f64,
    northing_m: f64,
    depth_m: f64,
    backscatter_db: f64,
    beam_angle_deg: f64,
}

/// Tsunami warning buoy payload
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TsunamiBuoyData {
    buoy_id: u16,
    water_column_height_m: f64,
    pressure_psia: f64,
    temperature_c: f64,
    alert_level: u8,
    transmission_seq: u32,
}

/// Coral reef health index record
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CoralReefHealth {
    reef_id: u32,
    live_coral_cover_pct: f64,
    bleaching_pct: f64,
    algae_cover_pct: f64,
    species_richness: u16,
    water_temp_c: f64,
}

/// Ocean pH / acidification measurement
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OceanPh {
    depth_m: f64,
    ph_total_scale: f64,
    pco2_uatm: f64,
    total_alkalinity_umol_kg: f64,
    dic_umol_kg: f64,
}

/// Marine mammal acoustic detection
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MarineMammalCall {
    Whistle {
        freq_min_hz: f64,
        freq_max_hz: f64,
        duration_s: f64,
    },
    Click {
        peak_freq_hz: f64,
        ici_ms: f64,
    },
    Song {
        fundamental_hz: f64,
        harmonic_count: u8,
        duration_s: f64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AcousticDetection {
    hydrophone_id: u16,
    timestamp_epoch_ms: u64,
    snr_db: f64,
    bearing_deg: f64,
    call: MarineMammalCall,
}

/// Subsea cable monitoring telemetry
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SubseaCableTelemetry {
    segment_id: u32,
    voltage_kv: f64,
    current_a: f64,
    insulation_resistance_mohm: f64,
    fiber_attenuation_db_km: f64,
    water_ingress_alarm: bool,
}

/// Tidal gauge reading
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TidalGaugeReading {
    gauge_id: u16,
    water_level_m: f64,
    predicted_level_m: f64,
    residual_m: f64,
    air_pressure_hpa: f64,
}

// ---------------------------------------------------------------------------
// prop_compose! strategies
// ---------------------------------------------------------------------------

prop_compose! {
    fn arb_ctd_sample()(
        depth_m in 0.0f64..6000.0,
        temperature_c in -2.0f64..35.0,
        conductivity_sm in 0.0f64..7.0,
        pressure_dbar in 0.0f64..6500.0,
    ) -> CtdSample {
        CtdSample { depth_m, temperature_c, conductivity_sm, pressure_dbar }
    }
}

prop_compose! {
    fn arb_ctd_cast()(
        station_id in any::<u32>(),
        latitude in -90.0f64..90.0,
        longitude in -180.0f64..180.0,
        samples in prop::collection::vec(arb_ctd_sample(), 1..8),
    ) -> CtdCast {
        CtdCast { station_id, latitude, longitude, samples }
    }
}

prop_compose! {
    fn arb_adcp_bin()(
        bin_depth_m in 0.0f64..1000.0,
        east_velocity_ms in -5.0f64..5.0,
        north_velocity_ms in -5.0f64..5.0,
        vertical_velocity_ms in -2.0f64..2.0,
        echo_intensity_db in any::<u8>(),
    ) -> AdcpBin {
        AdcpBin { bin_depth_m, east_velocity_ms, north_velocity_ms, vertical_velocity_ms, echo_intensity_db }
    }
}

prop_compose! {
    fn arb_adcp_ensemble()(
        ensemble_number in any::<u32>(),
        timestamp_epoch_s in any::<u64>(),
        heading_deg in 0.0f64..360.0,
        pitch_deg in -30.0f64..30.0,
        roll_deg in -30.0f64..30.0,
        bins in prop::collection::vec(arb_adcp_bin(), 1..10),
    ) -> AdcpEnsemble {
        AdcpEnsemble { ensemble_number, timestamp_epoch_s, heading_deg, pitch_deg, roll_deg, bins }
    }
}

prop_compose! {
    fn arb_sst()(
        lat in -90.0f64..90.0,
        lon in -180.0f64..180.0,
        sst_kelvin in 271.0f64..310.0,
        quality_flag in 0u8..5,
        satellite_id in any::<u16>(),
    ) -> SeaSurfaceTemp {
        SeaSurfaceTemp { lat, lon, sst_kelvin, quality_flag, satellite_id }
    }
}

prop_compose! {
    fn arb_wave_spectrum()(
        significant_height_m in 0.0f64..20.0,
        peak_period_s in 1.0f64..25.0,
        mean_direction_deg in 0.0f64..360.0,
        spectral_width in 0.0f64..1.0,
        energy_densities in prop::collection::vec(0.0f64..100.0, 1..16),
    ) -> WaveSpectrum {
        WaveSpectrum { significant_height_m, peak_period_s, mean_direction_deg, spectral_width, energy_densities }
    }
}

prop_compose! {
    fn arb_salinity_profile()(
        station_id in any::<u32>(),
        depth_m in 0.0f64..6000.0,
        practical_salinity_psu in 0.0f64..42.0,
        absolute_salinity_gkg in 0.0f64..42.5,
    ) -> SalinityProfile {
        SalinityProfile { station_id, depth_m, practical_salinity_psu, absolute_salinity_gkg }
    }
}

prop_compose! {
    fn arb_dissolved_oxygen()(
        depth_m in 0.0f64..6000.0,
        concentration_umol_kg in 0.0f64..400.0,
        saturation_pct in 0.0f64..120.0,
        sensor_serial in any::<u32>(),
    ) -> DissolvedOxygen {
        DissolvedOxygen { depth_m, concentration_umol_kg, saturation_pct, sensor_serial }
    }
}

prop_compose! {
    fn arb_chlorophyll()(
        depth_m in 0.0f64..200.0,
        fluorescence_rfu in 0.0f64..500.0,
        chlorophyll_a_ug_l in 0.0f64..50.0,
        turbidity_ntu in 0.0f64..1000.0,
    ) -> ChlorophyllFluorescence {
        ChlorophyllFluorescence { depth_m, fluorescence_rfu, chlorophyll_a_ug_l, turbidity_ntu }
    }
}

prop_compose! {
    fn arb_bathymetry()(
        easting_m in 0.0f64..1_000_000.0,
        northing_m in 0.0f64..10_000_000.0,
        depth_m in 0.0f64..11_000.0,
        backscatter_db in -60.0f64..0.0,
        beam_angle_deg in 0.0f64..75.0,
    ) -> BathymetryPoint {
        BathymetryPoint { easting_m, northing_m, depth_m, backscatter_db, beam_angle_deg }
    }
}

prop_compose! {
    fn arb_tsunami_buoy()(
        buoy_id in any::<u16>(),
        water_column_height_m in 4000.0f64..6500.0,
        pressure_psia in 5800.0f64..9500.0,
        temperature_c in 1.0f64..4.0,
        alert_level in 0u8..4,
        transmission_seq in any::<u32>(),
    ) -> TsunamiBuoyData {
        TsunamiBuoyData { buoy_id, water_column_height_m, pressure_psia, temperature_c, alert_level, transmission_seq }
    }
}

prop_compose! {
    fn arb_coral_reef()(
        reef_id in any::<u32>(),
        live_coral_cover_pct in 0.0f64..100.0,
        bleaching_pct in 0.0f64..100.0,
        algae_cover_pct in 0.0f64..100.0,
        species_richness in 0u16..500,
        water_temp_c in 18.0f64..34.0,
    ) -> CoralReefHealth {
        CoralReefHealth { reef_id, live_coral_cover_pct, bleaching_pct, algae_cover_pct, species_richness, water_temp_c }
    }
}

prop_compose! {
    fn arb_ocean_ph()(
        depth_m in 0.0f64..6000.0,
        ph_total_scale in 7.5f64..8.5,
        pco2_uatm in 150.0f64..1200.0,
        total_alkalinity_umol_kg in 2100.0f64..2500.0,
        dic_umol_kg in 1900.0f64..2300.0,
    ) -> OceanPh {
        OceanPh { depth_m, ph_total_scale, pco2_uatm, total_alkalinity_umol_kg, dic_umol_kg }
    }
}

fn arb_marine_mammal_call() -> impl Strategy<Value = MarineMammalCall> {
    prop_oneof![
        (1000.0f64..30000.0, 2000.0f64..50000.0, 0.01f64..5.0).prop_map(
            |(freq_min_hz, freq_max_hz, duration_s)| {
                MarineMammalCall::Whistle {
                    freq_min_hz,
                    freq_max_hz,
                    duration_s,
                }
            }
        ),
        (5000.0f64..150000.0, 0.1f64..200.0).prop_map(|(peak_freq_hz, ici_ms)| {
            MarineMammalCall::Click {
                peak_freq_hz,
                ici_ms,
            }
        }),
        (20.0f64..4000.0, 1u8..12, 0.5f64..30.0).prop_map(
            |(fundamental_hz, harmonic_count, duration_s)| {
                MarineMammalCall::Song {
                    fundamental_hz,
                    harmonic_count,
                    duration_s,
                }
            }
        ),
    ]
}

prop_compose! {
    fn arb_acoustic_detection()(
        hydrophone_id in any::<u16>(),
        timestamp_epoch_ms in any::<u64>(),
        snr_db in 0.0f64..60.0,
        bearing_deg in 0.0f64..360.0,
        call in arb_marine_mammal_call(),
    ) -> AcousticDetection {
        AcousticDetection { hydrophone_id, timestamp_epoch_ms, snr_db, bearing_deg, call }
    }
}

prop_compose! {
    fn arb_subsea_cable()(
        segment_id in any::<u32>(),
        voltage_kv in 100.0f64..500.0,
        current_a in 0.0f64..2000.0,
        insulation_resistance_mohm in 1.0f64..10000.0,
        fiber_attenuation_db_km in 0.15f64..0.5,
        water_ingress_alarm in any::<bool>(),
    ) -> SubseaCableTelemetry {
        SubseaCableTelemetry { segment_id, voltage_kv, current_a, insulation_resistance_mohm, fiber_attenuation_db_km, water_ingress_alarm }
    }
}

prop_compose! {
    fn arb_tidal_gauge()(
        gauge_id in any::<u16>(),
        water_level_m in -5.0f64..15.0,
        predicted_level_m in -5.0f64..15.0,
        residual_m in -2.0f64..2.0,
        air_pressure_hpa in 950.0f64..1060.0,
    ) -> TidalGaugeReading {
        TidalGaugeReading { gauge_id, water_level_m, predicted_level_m, residual_m, air_pressure_hpa }
    }
}

// ---------------------------------------------------------------------------
// Tests (22 total)
// ---------------------------------------------------------------------------

#[test]
fn test_ctd_sample_roundtrip() {
    proptest!(|(sample in arb_ctd_sample())| {
        let encoded = encode_to_vec(&sample).expect("encode CtdSample failed");
        let (decoded, _) = decode_from_slice::<CtdSample>(&encoded).expect("decode CtdSample failed");
        prop_assert_eq!(sample, decoded);
    });
}

#[test]
fn test_ctd_cast_roundtrip() {
    proptest!(|(cast in arb_ctd_cast())| {
        let encoded = encode_to_vec(&cast).expect("encode CtdCast failed");
        let (decoded, _) = decode_from_slice::<CtdCast>(&encoded).expect("decode CtdCast failed");
        prop_assert_eq!(cast, decoded);
    });
}

#[test]
fn test_adcp_bin_roundtrip() {
    proptest!(|(bin in arb_adcp_bin())| {
        let encoded = encode_to_vec(&bin).expect("encode AdcpBin failed");
        let (decoded, _) = decode_from_slice::<AdcpBin>(&encoded).expect("decode AdcpBin failed");
        prop_assert_eq!(bin, decoded);
    });
}

#[test]
fn test_adcp_ensemble_roundtrip() {
    proptest!(|(ens in arb_adcp_ensemble())| {
        let encoded = encode_to_vec(&ens).expect("encode AdcpEnsemble failed");
        let (decoded, _) = decode_from_slice::<AdcpEnsemble>(&encoded).expect("decode AdcpEnsemble failed");
        prop_assert_eq!(ens, decoded);
    });
}

#[test]
fn test_sea_surface_temp_roundtrip() {
    proptest!(|(sst in arb_sst())| {
        let encoded = encode_to_vec(&sst).expect("encode SeaSurfaceTemp failed");
        let (decoded, _) = decode_from_slice::<SeaSurfaceTemp>(&encoded).expect("decode SeaSurfaceTemp failed");
        prop_assert_eq!(sst, decoded);
    });
}

#[test]
fn test_wave_spectrum_roundtrip() {
    proptest!(|(ws in arb_wave_spectrum())| {
        let encoded = encode_to_vec(&ws).expect("encode WaveSpectrum failed");
        let (decoded, _) = decode_from_slice::<WaveSpectrum>(&encoded).expect("decode WaveSpectrum failed");
        prop_assert_eq!(ws, decoded);
    });
}

#[test]
fn test_salinity_profile_roundtrip() {
    proptest!(|(sp in arb_salinity_profile())| {
        let encoded = encode_to_vec(&sp).expect("encode SalinityProfile failed");
        let (decoded, _) = decode_from_slice::<SalinityProfile>(&encoded).expect("decode SalinityProfile failed");
        prop_assert_eq!(sp, decoded);
    });
}

#[test]
fn test_dissolved_oxygen_roundtrip() {
    proptest!(|(dox in arb_dissolved_oxygen())| {
        let encoded = encode_to_vec(&dox).expect("encode DissolvedOxygen failed");
        let (decoded, _) = decode_from_slice::<DissolvedOxygen>(&encoded).expect("decode DissolvedOxygen failed");
        prop_assert_eq!(dox, decoded);
    });
}

#[test]
fn test_chlorophyll_fluorescence_roundtrip() {
    proptest!(|(chl in arb_chlorophyll())| {
        let encoded = encode_to_vec(&chl).expect("encode ChlorophyllFluorescence failed");
        let (decoded, _) = decode_from_slice::<ChlorophyllFluorescence>(&encoded).expect("decode ChlorophyllFluorescence failed");
        prop_assert_eq!(chl, decoded);
    });
}

#[test]
fn test_bathymetry_point_roundtrip() {
    proptest!(|(bp in arb_bathymetry())| {
        let encoded = encode_to_vec(&bp).expect("encode BathymetryPoint failed");
        let (decoded, _) = decode_from_slice::<BathymetryPoint>(&encoded).expect("decode BathymetryPoint failed");
        prop_assert_eq!(bp, decoded);
    });
}

#[test]
fn test_tsunami_buoy_roundtrip() {
    proptest!(|(tb in arb_tsunami_buoy())| {
        let encoded = encode_to_vec(&tb).expect("encode TsunamiBuoyData failed");
        let (decoded, _) = decode_from_slice::<TsunamiBuoyData>(&encoded).expect("decode TsunamiBuoyData failed");
        prop_assert_eq!(tb, decoded);
    });
}

#[test]
fn test_coral_reef_health_roundtrip() {
    proptest!(|(cr in arb_coral_reef())| {
        let encoded = encode_to_vec(&cr).expect("encode CoralReefHealth failed");
        let (decoded, _) = decode_from_slice::<CoralReefHealth>(&encoded).expect("decode CoralReefHealth failed");
        prop_assert_eq!(cr, decoded);
    });
}

#[test]
fn test_ocean_ph_roundtrip() {
    proptest!(|(ph in arb_ocean_ph())| {
        let encoded = encode_to_vec(&ph).expect("encode OceanPh failed");
        let (decoded, _) = decode_from_slice::<OceanPh>(&encoded).expect("decode OceanPh failed");
        prop_assert_eq!(ph, decoded);
    });
}

#[test]
fn test_marine_mammal_call_roundtrip() {
    proptest!(|(call in arb_marine_mammal_call())| {
        let encoded = encode_to_vec(&call).expect("encode MarineMammalCall failed");
        let (decoded, _) = decode_from_slice::<MarineMammalCall>(&encoded).expect("decode MarineMammalCall failed");
        prop_assert_eq!(call, decoded);
    });
}

#[test]
fn test_acoustic_detection_roundtrip() {
    proptest!(|(det in arb_acoustic_detection())| {
        let encoded = encode_to_vec(&det).expect("encode AcousticDetection failed");
        let (decoded, _) = decode_from_slice::<AcousticDetection>(&encoded).expect("decode AcousticDetection failed");
        prop_assert_eq!(det, decoded);
    });
}

#[test]
fn test_subsea_cable_telemetry_roundtrip() {
    proptest!(|(cable in arb_subsea_cable())| {
        let encoded = encode_to_vec(&cable).expect("encode SubseaCableTelemetry failed");
        let (decoded, _) = decode_from_slice::<SubseaCableTelemetry>(&encoded).expect("decode SubseaCableTelemetry failed");
        prop_assert_eq!(cable, decoded);
    });
}

#[test]
fn test_tidal_gauge_roundtrip() {
    proptest!(|(tg in arb_tidal_gauge())| {
        let encoded = encode_to_vec(&tg).expect("encode TidalGaugeReading failed");
        let (decoded, _) = decode_from_slice::<TidalGaugeReading>(&encoded).expect("decode TidalGaugeReading failed");
        prop_assert_eq!(tg, decoded);
    });
}

#[test]
fn test_vec_sst_observations_roundtrip() {
    proptest!(|(obs in prop::collection::vec(arb_sst(), 0..12))| {
        let encoded = encode_to_vec(&obs).expect("encode Vec<SeaSurfaceTemp> failed");
        let (decoded, _) = decode_from_slice::<Vec<SeaSurfaceTemp>>(&encoded)
            .expect("decode Vec<SeaSurfaceTemp> failed");
        prop_assert_eq!(obs, decoded);
    });
}

#[test]
fn test_salinity_profile_series_roundtrip() {
    proptest!(|(profiles in prop::collection::vec(arb_salinity_profile(), 1..10))| {
        let encoded = encode_to_vec(&profiles).expect("encode salinity series failed");
        let (decoded, _) = decode_from_slice::<Vec<SalinityProfile>>(&encoded)
            .expect("decode salinity series failed");
        prop_assert_eq!(profiles, decoded);
    });
}

#[test]
fn test_dissolved_oxygen_with_chlorophyll_tuple_roundtrip() {
    proptest!(|(pair in (arb_dissolved_oxygen(), arb_chlorophyll()))| {
        let encoded = encode_to_vec(&pair).expect("encode DO-Chl tuple failed");
        let (decoded, _) = decode_from_slice::<(DissolvedOxygen, ChlorophyllFluorescence)>(&encoded)
            .expect("decode DO-Chl tuple failed");
        prop_assert_eq!(pair, decoded);
    });
}

#[test]
fn test_optional_tsunami_alert_roundtrip() {
    proptest!(|(maybe_buoy in prop::option::of(arb_tsunami_buoy()))| {
        let encoded = encode_to_vec(&maybe_buoy).expect("encode Option<TsunamiBuoyData> failed");
        let (decoded, _) = decode_from_slice::<Option<TsunamiBuoyData>>(&encoded)
            .expect("decode Option<TsunamiBuoyData> failed");
        prop_assert_eq!(maybe_buoy, decoded);
    });
}

#[test]
fn test_bathymetry_survey_line_roundtrip() {
    proptest!(|(line in prop::collection::vec(arb_bathymetry(), 2..20))| {
        let encoded = encode_to_vec(&line).expect("encode bathymetry survey line failed");
        let (decoded, _) = decode_from_slice::<Vec<BathymetryPoint>>(&encoded)
            .expect("decode bathymetry survey line failed");
        prop_assert_eq!(line, decoded);
    });
}
