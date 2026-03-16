//! Advanced tests for oceanographic research and marine science instrumentation.
//! 22 test functions covering CTD casts, ADCP, ROV telemetry, wave buoys, and more.

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

// --- CTD Cast Data ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum CtdCastPhase {
    Downcast,
    Upcast,
    Soak,
    OnDeck,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum WaterMassType {
    NorthAtlanticDeepWater,
    AntarcticBottomWater,
    MediterraneanOverflow,
    ArcticIntermediate,
    PacificDeepWater,
    IndianCentralWater,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CtdSample {
    depth_cm: u32,
    temperature_milli_c: i32,
    conductivity_us_cm: u32,
    salinity_psu_x1000: u32,
    dissolved_oxygen_umol_kg: u32,
    phase: CtdCastPhase,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CtdProfile {
    station_id: u32,
    cast_number: u16,
    latitude_micro_deg: i64,
    longitude_micro_deg: i64,
    timestamp_epoch_ms: u64,
    water_mass: WaterMassType,
    samples: Vec<CtdSample>,
}

// --- Ocean Current Profiler ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum AdcpMode {
    BottomTrack,
    WaterProfile,
    HighResolution,
    WavesBurst,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdcpBin {
    bin_index: u16,
    velocity_east_mm_s: i32,
    velocity_north_mm_s: i32,
    velocity_up_mm_s: i32,
    echo_intensity_db_x10: u16,
    correlation_pct: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdcpEnsemble {
    ensemble_number: u32,
    mode: AdcpMode,
    heading_deci_deg: u16,
    pitch_deci_deg: i16,
    roll_deci_deg: i16,
    bins: Vec<AdcpBin>,
}

// --- Seafloor Sediment Classification ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum SedimentGrainSize {
    Clay,
    Silt,
    FineSand,
    MediumSand,
    CoarseSand,
    Gravel,
    Cobble,
    Boulder,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SedimentOrigin {
    Terrigenous,
    Biogenic,
    Hydrogenous,
    Cosmogenous,
    Volcanic,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SedimentCore {
    core_id: u32,
    depth_below_seafloor_cm: u32,
    grain_size: SedimentGrainSize,
    origin: SedimentOrigin,
    organic_carbon_pct_x100: u16,
    calcium_carbonate_pct_x100: u16,
    porosity_pct_x100: u16,
}

// --- Wave Buoy Spectral Data ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum SeaState {
    Calm,
    Slight,
    Moderate,
    Rough,
    VeryRough,
    High,
    Phenomenal,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SpectralBand {
    frequency_milli_hz: u32,
    energy_density_cm2_hz_x1000: u64,
    direction_deci_deg: u16,
    spread_deci_deg: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WaveBuoyRecord {
    buoy_id: u32,
    timestamp_epoch_s: u64,
    significant_wave_height_mm: u32,
    peak_period_ms: u32,
    sea_state: SeaState,
    spectral_bands: Vec<SpectralBand>,
}

// --- Plankton Classification ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum PlanktonDomain {
    Phytoplankton(PhytoType),
    Zooplankton(ZooType),
    Bacterioplankton,
    Virioplankton,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PhytoType {
    Diatom,
    Dinoflagellate,
    Coccolithophore,
    Cyanobacteria,
    GreenAlga,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ZooType {
    Copepod,
    Krill,
    Pteropod,
    Larvacean,
    Foraminifera,
    Radiolarian,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PlanktonSample {
    sample_id: u32,
    depth_m: u16,
    domain: PlanktonDomain,
    count_per_liter: u64,
    size_range_min_um: u32,
    size_range_max_um: u32,
}

// --- Deep Sea Vent Chemistry ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum VentType {
    BlackSmoker,
    WhiteSmoker,
    DiffuseFlow,
    Flange,
    Beehive,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VentFluidChemistry {
    vent_id: u32,
    vent_type: VentType,
    temperature_milli_c: u32,
    ph_x100: u16,
    hydrogen_sulfide_umol_l: u32,
    methane_nmol_l: u32,
    iron_umol_l: u32,
    manganese_umol_l: u32,
}

// --- ROV Telemetry ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum RovState {
    OnDeck,
    Descending,
    OnBottom,
    Sampling,
    Ascending,
    Emergency,
    Maintenance,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum RovSubsystem {
    Propulsion {
        thruster_rpm: Vec<i16>,
    },
    Manipulator {
        joint_angles_deci_deg: Vec<i16>,
    },
    Camera {
        pan_deci_deg: i16,
        tilt_deci_deg: i16,
        zoom_pct: u8,
    },
    Lighting {
        lumens: Vec<u32>,
    },
    Sonar {
        range_cm: u32,
        bearing_deci_deg: u16,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RovTelemetry {
    dive_number: u32,
    state: RovState,
    depth_cm: u32,
    altitude_cm: u32,
    heading_deci_deg: u16,
    battery_mv: u32,
    subsystems: Vec<RovSubsystem>,
}

// --- Mooring Configuration ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum MooringInstrument {
    CtdSensor { sample_interval_s: u16 },
    CurrentMeter { frequency_khz: u16 },
    SedimentTrap { collection_days: u16 },
    AcousticRelease { code: u32 },
    Fluorometer { gain_setting: u8 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MooringElement {
    element_id: u16,
    depth_target_m: u16,
    instrument: MooringInstrument,
    serial_number: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MooringDeployment {
    mooring_id: u32,
    deploy_epoch_s: u64,
    recovery_epoch_s: u64,
    latitude_micro_deg: i64,
    longitude_micro_deg: i64,
    water_depth_m: u32,
    elements: Vec<MooringElement>,
}

// --- Tide Gauge ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum TidalConstituent {
    M2,
    S2,
    N2,
    K1,
    O1,
    P1,
    K2,
    Q1,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TidalHarmonic {
    constituent: TidalConstituent,
    amplitude_mm: u32,
    phase_deci_deg: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TideGaugeRecord {
    station_id: u32,
    station_name: String,
    datum_offset_mm: i32,
    harmonics: Vec<TidalHarmonic>,
    water_level_readings_mm: Vec<i32>,
}

// --- Ocean Color Satellite ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum OceanColorBand {
    Coastal(u16),
    Blue(u16),
    Green(u16),
    Red(u16),
    NearInfrared(u16),
    Panchromatic(u16),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OceanColorPixel {
    row: u32,
    col: u32,
    bands: Vec<OceanColorBand>,
    chlorophyll_a_ug_l_x100: u32,
    turbidity_ntu_x100: u32,
    cloud_flag: bool,
}

// --- Marine Protected Area ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum MpaZoneType {
    NoTake,
    LimitedUse { allowed_gear_types: Vec<String> },
    GeneralUse,
    Buffer,
    ResearchOnly { permit_required: bool },
    TransitCorridor,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MpaZone {
    zone_id: u32,
    name: String,
    zone_type: MpaZoneType,
    area_hectares_x10: u32,
    max_depth_m: u16,
    min_depth_m: u16,
}

// ======================================================================
// TESTS
// ======================================================================

#[test]
fn test_ctd_downcast_profile() {
    let profile = CtdProfile {
        station_id: 1001,
        cast_number: 3,
        latitude_micro_deg: 36_867_000,
        longitude_micro_deg: -122_045_000,
        timestamp_epoch_ms: 1_700_000_000_000,
        water_mass: WaterMassType::PacificDeepWater,
        samples: vec![
            CtdSample {
                depth_cm: 1000,
                temperature_milli_c: 14_350,
                conductivity_us_cm: 42_100,
                salinity_psu_x1000: 33_850,
                dissolved_oxygen_umol_kg: 245,
                phase: CtdCastPhase::Downcast,
            },
            CtdSample {
                depth_cm: 50_000,
                temperature_milli_c: 2_100,
                conductivity_us_cm: 33_400,
                salinity_psu_x1000: 34_680,
                dissolved_oxygen_umol_kg: 165,
                phase: CtdCastPhase::Downcast,
            },
        ],
    };
    let bytes = encode_to_vec(&profile).expect("encode CTD downcast profile");
    let (decoded, _) =
        decode_from_slice::<CtdProfile>(&bytes).expect("decode CTD downcast profile");
    assert_eq!(profile, decoded);
}

#[test]
fn test_ctd_upcast_with_soak() {
    let profile = CtdProfile {
        station_id: 2050,
        cast_number: 1,
        latitude_micro_deg: -60_500_000,
        longitude_micro_deg: -55_200_000,
        timestamp_epoch_ms: 1_710_000_000_000,
        water_mass: WaterMassType::AntarcticBottomWater,
        samples: vec![
            CtdSample {
                depth_cm: 400_000,
                temperature_milli_c: -800,
                conductivity_us_cm: 32_800,
                salinity_psu_x1000: 34_660,
                dissolved_oxygen_umol_kg: 220,
                phase: CtdCastPhase::Soak,
            },
            CtdSample {
                depth_cm: 200_000,
                temperature_milli_c: 500,
                conductivity_us_cm: 34_000,
                salinity_psu_x1000: 34_700,
                dissolved_oxygen_umol_kg: 200,
                phase: CtdCastPhase::Upcast,
            },
        ],
    };
    let bytes = encode_to_vec(&profile).expect("encode CTD upcast with soak");
    let (decoded, _) =
        decode_from_slice::<CtdProfile>(&bytes).expect("decode CTD upcast with soak");
    assert_eq!(profile, decoded);
}

#[test]
fn test_adcp_bottom_track_ensemble() {
    let ensemble = AdcpEnsemble {
        ensemble_number: 44100,
        mode: AdcpMode::BottomTrack,
        heading_deci_deg: 1823,
        pitch_deci_deg: -15,
        roll_deci_deg: 8,
        bins: vec![
            AdcpBin {
                bin_index: 0,
                velocity_east_mm_s: 120,
                velocity_north_mm_s: -85,
                velocity_up_mm_s: 3,
                echo_intensity_db_x10: 750,
                correlation_pct: 98,
            },
            AdcpBin {
                bin_index: 1,
                velocity_east_mm_s: 115,
                velocity_north_mm_s: -90,
                velocity_up_mm_s: -2,
                echo_intensity_db_x10: 680,
                correlation_pct: 95,
            },
            AdcpBin {
                bin_index: 2,
                velocity_east_mm_s: 108,
                velocity_north_mm_s: -78,
                velocity_up_mm_s: 5,
                echo_intensity_db_x10: 610,
                correlation_pct: 91,
            },
        ],
    };
    let bytes = encode_to_vec(&ensemble).expect("encode ADCP bottom track");
    let (decoded, _) = decode_from_slice::<AdcpEnsemble>(&bytes).expect("decode ADCP bottom track");
    assert_eq!(ensemble, decoded);
}

#[test]
fn test_adcp_high_resolution_mode() {
    let ensemble = AdcpEnsemble {
        ensemble_number: 99_001,
        mode: AdcpMode::HighResolution,
        heading_deci_deg: 900,
        pitch_deci_deg: 0,
        roll_deci_deg: -3,
        bins: vec![AdcpBin {
            bin_index: 0,
            velocity_east_mm_s: -450,
            velocity_north_mm_s: 320,
            velocity_up_mm_s: -12,
            echo_intensity_db_x10: 820,
            correlation_pct: 99,
        }],
    };
    let bytes = encode_to_vec(&ensemble).expect("encode ADCP high res");
    let (decoded, _) = decode_from_slice::<AdcpEnsemble>(&bytes).expect("decode ADCP high res");
    assert_eq!(ensemble, decoded);
}

#[test]
fn test_sediment_core_biogenic_clay() {
    let core = SedimentCore {
        core_id: 7800,
        depth_below_seafloor_cm: 350,
        grain_size: SedimentGrainSize::Clay,
        origin: SedimentOrigin::Biogenic,
        organic_carbon_pct_x100: 220,
        calcium_carbonate_pct_x100: 6500,
        porosity_pct_x100: 7800,
    };
    let bytes = encode_to_vec(&core).expect("encode biogenic clay core");
    let (decoded, _) =
        decode_from_slice::<SedimentCore>(&bytes).expect("decode biogenic clay core");
    assert_eq!(core, decoded);
}

#[test]
fn test_sediment_core_volcanic_gravel() {
    let core = SedimentCore {
        core_id: 12_300,
        depth_below_seafloor_cm: 15,
        grain_size: SedimentGrainSize::Gravel,
        origin: SedimentOrigin::Volcanic,
        organic_carbon_pct_x100: 10,
        calcium_carbonate_pct_x100: 80,
        porosity_pct_x100: 4200,
    };
    let bytes = encode_to_vec(&core).expect("encode volcanic gravel core");
    let (decoded, _) =
        decode_from_slice::<SedimentCore>(&bytes).expect("decode volcanic gravel core");
    assert_eq!(core, decoded);
}

#[test]
fn test_wave_buoy_rough_sea() {
    let record = WaveBuoyRecord {
        buoy_id: 46042,
        timestamp_epoch_s: 1_700_100_000,
        significant_wave_height_mm: 4_500,
        peak_period_ms: 12_000,
        sea_state: SeaState::Rough,
        spectral_bands: vec![
            SpectralBand {
                frequency_milli_hz: 50,
                energy_density_cm2_hz_x1000: 12_400,
                direction_deci_deg: 2700,
                spread_deci_deg: 350,
            },
            SpectralBand {
                frequency_milli_hz: 83,
                energy_density_cm2_hz_x1000: 45_600,
                direction_deci_deg: 2850,
                spread_deci_deg: 280,
            },
            SpectralBand {
                frequency_milli_hz: 125,
                energy_density_cm2_hz_x1000: 8_300,
                direction_deci_deg: 2650,
                spread_deci_deg: 420,
            },
        ],
    };
    let bytes = encode_to_vec(&record).expect("encode rough sea wave buoy");
    let (decoded, _) =
        decode_from_slice::<WaveBuoyRecord>(&bytes).expect("decode rough sea wave buoy");
    assert_eq!(record, decoded);
}

#[test]
fn test_wave_buoy_calm_sea_empty_spectrum() {
    let record = WaveBuoyRecord {
        buoy_id: 51001,
        timestamp_epoch_s: 1_700_200_000,
        significant_wave_height_mm: 200,
        peak_period_ms: 6_000,
        sea_state: SeaState::Calm,
        spectral_bands: vec![],
    };
    let bytes = encode_to_vec(&record).expect("encode calm sea wave buoy");
    let (decoded, _) =
        decode_from_slice::<WaveBuoyRecord>(&bytes).expect("decode calm sea wave buoy");
    assert_eq!(record, decoded);
}

#[test]
fn test_plankton_diatom_phytoplankton() {
    let sample = PlanktonSample {
        sample_id: 300,
        depth_m: 25,
        domain: PlanktonDomain::Phytoplankton(PhytoType::Diatom),
        count_per_liter: 1_500_000,
        size_range_min_um: 20,
        size_range_max_um: 200,
    };
    let bytes = encode_to_vec(&sample).expect("encode diatom plankton");
    let (decoded, _) = decode_from_slice::<PlanktonSample>(&bytes).expect("decode diatom plankton");
    assert_eq!(sample, decoded);
}

#[test]
fn test_plankton_krill_zooplankton() {
    let sample = PlanktonSample {
        sample_id: 301,
        depth_m: 80,
        domain: PlanktonDomain::Zooplankton(ZooType::Krill),
        count_per_liter: 42,
        size_range_min_um: 10_000,
        size_range_max_um: 60_000,
    };
    let bytes = encode_to_vec(&sample).expect("encode krill zooplankton");
    let (decoded, _) =
        decode_from_slice::<PlanktonSample>(&bytes).expect("decode krill zooplankton");
    assert_eq!(sample, decoded);
}

#[test]
fn test_plankton_bacterioplankton() {
    let sample = PlanktonSample {
        sample_id: 302,
        depth_m: 5,
        domain: PlanktonDomain::Bacterioplankton,
        count_per_liter: 500_000_000,
        size_range_min_um: 1,
        size_range_max_um: 5,
    };
    let bytes = encode_to_vec(&sample).expect("encode bacterioplankton");
    let (decoded, _) =
        decode_from_slice::<PlanktonSample>(&bytes).expect("decode bacterioplankton");
    assert_eq!(sample, decoded);
}

#[test]
fn test_deep_sea_vent_black_smoker() {
    let vent = VentFluidChemistry {
        vent_id: 5001,
        vent_type: VentType::BlackSmoker,
        temperature_milli_c: 350_000,
        ph_x100: 280,
        hydrogen_sulfide_umol_l: 8_500,
        methane_nmol_l: 120_000,
        iron_umol_l: 24_000,
        manganese_umol_l: 1_100,
    };
    let bytes = encode_to_vec(&vent).expect("encode black smoker");
    let (decoded, _) =
        decode_from_slice::<VentFluidChemistry>(&bytes).expect("decode black smoker");
    assert_eq!(vent, decoded);
}

#[test]
fn test_deep_sea_vent_diffuse_flow() {
    let vent = VentFluidChemistry {
        vent_id: 5050,
        vent_type: VentType::DiffuseFlow,
        temperature_milli_c: 15_000,
        ph_x100: 550,
        hydrogen_sulfide_umol_l: 200,
        methane_nmol_l: 5_000,
        iron_umol_l: 800,
        manganese_umol_l: 150,
    };
    let bytes = encode_to_vec(&vent).expect("encode diffuse flow vent");
    let (decoded, _) =
        decode_from_slice::<VentFluidChemistry>(&bytes).expect("decode diffuse flow vent");
    assert_eq!(vent, decoded);
}

#[test]
fn test_rov_sampling_with_subsystems() {
    let telemetry = RovTelemetry {
        dive_number: 4872,
        state: RovState::Sampling,
        depth_cm: 250_000,
        altitude_cm: 150,
        heading_deci_deg: 450,
        battery_mv: 48_200,
        subsystems: vec![
            RovSubsystem::Propulsion {
                thruster_rpm: vec![0, 0, 0, 0, 0, 0],
            },
            RovSubsystem::Manipulator {
                joint_angles_deci_deg: vec![450, -120, 900, 0, 300],
            },
            RovSubsystem::Camera {
                pan_deci_deg: -150,
                tilt_deci_deg: -450,
                zoom_pct: 80,
            },
            RovSubsystem::Lighting {
                lumens: vec![15_000, 15_000, 8_000],
            },
        ],
    };
    let bytes = encode_to_vec(&telemetry).expect("encode ROV sampling telemetry");
    let (decoded, _) =
        decode_from_slice::<RovTelemetry>(&bytes).expect("decode ROV sampling telemetry");
    assert_eq!(telemetry, decoded);
}

#[test]
fn test_rov_emergency_ascent() {
    let telemetry = RovTelemetry {
        dive_number: 4873,
        state: RovState::Emergency,
        depth_cm: 180_000,
        altitude_cm: 0,
        heading_deci_deg: 0,
        battery_mv: 38_500,
        subsystems: vec![
            RovSubsystem::Propulsion {
                thruster_rpm: vec![1500, 1500, 1500, 1500, 2000, 2000],
            },
            RovSubsystem::Sonar {
                range_cm: 500_000,
                bearing_deci_deg: 0,
            },
        ],
    };
    let bytes = encode_to_vec(&telemetry).expect("encode ROV emergency");
    let (decoded, _) = decode_from_slice::<RovTelemetry>(&bytes).expect("decode ROV emergency");
    assert_eq!(telemetry, decoded);
}

#[test]
fn test_mooring_full_deployment() {
    let deployment = MooringDeployment {
        mooring_id: 9001,
        deploy_epoch_s: 1_680_000_000,
        recovery_epoch_s: 1_711_000_000,
        latitude_micro_deg: 47_600_000,
        longitude_micro_deg: -127_800_000,
        water_depth_m: 2800,
        elements: vec![
            MooringElement {
                element_id: 1,
                depth_target_m: 100,
                instrument: MooringInstrument::CurrentMeter { frequency_khz: 300 },
                serial_number: "WHS300-17892".to_string(),
            },
            MooringElement {
                element_id: 2,
                depth_target_m: 500,
                instrument: MooringInstrument::CtdSensor {
                    sample_interval_s: 600,
                },
                serial_number: "SBE37-20145".to_string(),
            },
            MooringElement {
                element_id: 3,
                depth_target_m: 2700,
                instrument: MooringInstrument::SedimentTrap {
                    collection_days: 14,
                },
                serial_number: "MCL-12-0045".to_string(),
            },
            MooringElement {
                element_id: 4,
                depth_target_m: 2790,
                instrument: MooringInstrument::AcousticRelease { code: 0xABCD_1234 },
                serial_number: "IXSEA-AR25-0091".to_string(),
            },
        ],
    };
    let bytes = encode_to_vec(&deployment).expect("encode mooring deployment");
    let (decoded, _) =
        decode_from_slice::<MooringDeployment>(&bytes).expect("decode mooring deployment");
    assert_eq!(deployment, decoded);
}

#[test]
fn test_mooring_fluorometer_element() {
    let element = MooringElement {
        element_id: 10,
        depth_target_m: 30,
        instrument: MooringInstrument::Fluorometer { gain_setting: 3 },
        serial_number: "ECO-FL-2288".to_string(),
    };
    let bytes = encode_to_vec(&element).expect("encode fluorometer element");
    let (decoded, _) =
        decode_from_slice::<MooringElement>(&bytes).expect("decode fluorometer element");
    assert_eq!(element, decoded);
}

#[test]
fn test_tide_gauge_with_harmonics() {
    let record = TideGaugeRecord {
        station_id: 9414290,
        station_name: "San Francisco, CA".to_string(),
        datum_offset_mm: -1_830,
        harmonics: vec![
            TidalHarmonic {
                constituent: TidalConstituent::M2,
                amplitude_mm: 580,
                phase_deci_deg: 1920,
            },
            TidalHarmonic {
                constituent: TidalConstituent::K1,
                amplitude_mm: 370,
                phase_deci_deg: 2280,
            },
            TidalHarmonic {
                constituent: TidalConstituent::O1,
                amplitude_mm: 230,
                phase_deci_deg: 2050,
            },
            TidalHarmonic {
                constituent: TidalConstituent::S2,
                amplitude_mm: 130,
                phase_deci_deg: 2100,
            },
        ],
        water_level_readings_mm: vec![1250, 1310, 1400, 1520, 1650, 1780, 1870, 1900, 1860, 1770],
    };
    let bytes = encode_to_vec(&record).expect("encode tide gauge record");
    let (decoded, _) =
        decode_from_slice::<TideGaugeRecord>(&bytes).expect("decode tide gauge record");
    assert_eq!(record, decoded);
}

#[test]
fn test_ocean_color_pixel_with_bands() {
    let pixel = OceanColorPixel {
        row: 1024,
        col: 2048,
        bands: vec![
            OceanColorBand::Coastal(412),
            OceanColorBand::Blue(443),
            OceanColorBand::Green(555),
            OceanColorBand::Red(670),
            OceanColorBand::NearInfrared(865),
        ],
        chlorophyll_a_ug_l_x100: 325,
        turbidity_ntu_x100: 150,
        cloud_flag: false,
    };
    let bytes = encode_to_vec(&pixel).expect("encode ocean color pixel");
    let (decoded, _) =
        decode_from_slice::<OceanColorPixel>(&bytes).expect("decode ocean color pixel");
    assert_eq!(pixel, decoded);
}

#[test]
fn test_ocean_color_pixel_cloudy() {
    let pixel = OceanColorPixel {
        row: 500,
        col: 900,
        bands: vec![OceanColorBand::Panchromatic(550)],
        chlorophyll_a_ug_l_x100: 0,
        turbidity_ntu_x100: 0,
        cloud_flag: true,
    };
    let bytes = encode_to_vec(&pixel).expect("encode cloudy pixel");
    let (decoded, _) = decode_from_slice::<OceanColorPixel>(&bytes).expect("decode cloudy pixel");
    assert_eq!(pixel, decoded);
}

#[test]
fn test_mpa_no_take_zone() {
    let zone = MpaZone {
        zone_id: 100,
        name: "Papah\u{101}naumoku\u{101}kea Core".to_string(),
        zone_type: MpaZoneType::NoTake,
        area_hectares_x10: 3_600_000,
        max_depth_m: 5800,
        min_depth_m: 0,
    };
    let bytes = encode_to_vec(&zone).expect("encode no-take MPA zone");
    let (decoded, _) = decode_from_slice::<MpaZone>(&bytes).expect("decode no-take MPA zone");
    assert_eq!(zone, decoded);
}

#[test]
fn test_mpa_limited_use_zone() {
    let zone = MpaZone {
        zone_id: 201,
        name: "Channel Islands Buffer".to_string(),
        zone_type: MpaZoneType::LimitedUse {
            allowed_gear_types: vec!["hook_and_line".to_string(), "trap".to_string()],
        },
        area_hectares_x10: 45_000,
        max_depth_m: 300,
        min_depth_m: 10,
    };
    let bytes = encode_to_vec(&zone).expect("encode limited-use MPA zone");
    let (decoded, _) = decode_from_slice::<MpaZone>(&bytes).expect("decode limited-use MPA zone");
    assert_eq!(zone, decoded);
}
