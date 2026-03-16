//! Advanced LZ4 compression tests for the oil & gas upstream operations domain.
//!
//! Covers seismic survey traces, well log data (gamma ray, resistivity, porosity),
//! drilling parameters (ROP, WOB, RPM, torque), mud weight/rheology, BHA configurations,
//! formation evaluation, reservoir simulation cells, production allocation, separator
//! readings, pipeline SCADA data, artificial lift systems (ESP, rod pump, gas lift),
//! well test results, decline curve analysis, enhanced oil recovery parameters,
//! and HSE incident tracking.

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeismicTrace {
    line_id: u32,
    trace_number: u32,
    sample_interval_us: u16,
    amplitudes: Vec<f32>,
    source_x: f64,
    source_y: f64,
    receiver_x: f64,
    receiver_y: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeismicSurvey {
    survey_name: String,
    acquisition_date: String,
    traces: Vec<SeismicTrace>,
    bin_size_m: f64,
    fold_coverage: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WellLogData {
    well_id: String,
    depth_m: Vec<f64>,
    gamma_ray_api: Vec<f32>,
    resistivity_ohm_m: Vec<f32>,
    porosity_fraction: Vec<f32>,
    density_gcc: Vec<f32>,
    neutron_porosity: Vec<f32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DrillingParameters {
    timestamp_epoch: u64,
    well_id: String,
    measured_depth_m: f64,
    rate_of_penetration_mh: f32,
    weight_on_bit_klb: f32,
    rotary_rpm: u16,
    torque_kft_lb: f32,
    standpipe_pressure_psi: f32,
    flow_rate_gpm: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MudRheology {
    mud_weight_ppg: f32,
    plastic_viscosity_cp: f32,
    yield_point_lbf: f32,
    gel_strength_10s: f32,
    gel_strength_10m: f32,
    ph: f32,
    chlorides_ppm: u32,
    funnel_viscosity_s: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MudReport {
    well_id: String,
    report_number: u32,
    depth_m: f64,
    rheology: MudRheology,
    mud_type: String,
    solids_percent: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BhaComponentType {
    DrillBit { size_in: f32, bit_type: String },
    StabilizerNearBit { od_in: f32 },
    DrillCollar { od_in: f32, id_in: f32 },
    Mwd { gamma_ray: bool, resistivity: bool },
    Lwd { density: bool, neutron: bool },
    MotorSteerable { bend_angle_deg: f32 },
    Jar,
    HeavyWeightDrillPipe { od_in: f32 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BhaComponent {
    position: u8,
    component_type: BhaComponentType,
    length_m: f32,
    weight_kg: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BottomHoleAssembly {
    bha_number: u16,
    well_id: String,
    components: Vec<BhaComponent>,
    total_length_m: f32,
    total_weight_kg: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FormationEvaluation {
    zone_name: String,
    top_depth_m: f64,
    base_depth_m: f64,
    net_pay_m: f64,
    avg_porosity: f32,
    avg_water_saturation: f32,
    avg_permeability_md: f32,
    hydrocarbon_type: String,
    formation_pressure_psi: f32,
    temperature_f: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReservoirCell {
    i: u32,
    j: u32,
    k: u32,
    pressure_psi: f32,
    oil_saturation: f32,
    water_saturation: f32,
    gas_saturation: f32,
    permeability_md: f32,
    porosity: f32,
    depth_m: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReservoirSimGrid {
    model_name: String,
    nx: u32,
    ny: u32,
    nz: u32,
    cells: Vec<ReservoirCell>,
    timestep_days: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductionAllocation {
    field_name: String,
    well_id: String,
    date: String,
    oil_bbl: f64,
    water_bbl: f64,
    gas_mscf: f64,
    ngl_bbl: f64,
    hours_on: f32,
    choke_size_64ths: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SeparatorReading {
    separator_id: String,
    timestamp_epoch: u64,
    inlet_pressure_psi: f32,
    outlet_pressure_psi: f32,
    temperature_f: f32,
    oil_rate_bpd: f32,
    water_rate_bpd: f32,
    gas_rate_mscfd: f32,
    level_percent: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScadaDataPoint {
    tag_name: String,
    timestamp_epoch: u64,
    value: f64,
    quality: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PipelineScada {
    pipeline_id: String,
    segment_name: String,
    data_points: Vec<ScadaDataPoint>,
    flow_rate_bpd: f64,
    inlet_pressure_psi: f32,
    outlet_pressure_psi: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ArtificialLiftType {
    Esp {
        motor_hp: f32,
        frequency_hz: f32,
        intake_pressure_psi: f32,
        discharge_pressure_psi: f32,
        motor_temp_f: f32,
        vibration_ips: f32,
    },
    RodPump {
        stroke_length_in: f32,
        strokes_per_min: f32,
        pump_fillage_percent: f32,
        polished_rod_load_lb: f32,
        counterbalance_lb: f32,
    },
    GasLift {
        injection_rate_mscfd: f32,
        injection_pressure_psi: f32,
        valve_depths_m: Vec<f32>,
        operating_valve_idx: u8,
        glr_scf_bbl: f32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ArtificialLiftSystem {
    well_id: String,
    lift_type: ArtificialLiftType,
    install_date: String,
    last_service_date: String,
    runtime_hours: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WellTestResult {
    well_id: String,
    test_date: String,
    choke_size_64ths: u8,
    oil_rate_bpd: f64,
    water_rate_bpd: f64,
    gas_rate_mscfd: f64,
    gor_scf_bbl: f64,
    water_cut_percent: f32,
    flowing_tubing_pressure_psi: f32,
    flowing_casing_pressure_psi: f32,
    bsw_percent: f32,
    api_gravity: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DeclineModel {
    Exponential { di_per_year: f64 },
    Harmonic { di_per_year: f64 },
    Hyperbolic { di_per_year: f64, b_factor: f64 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DeclineCurveAnalysis {
    well_id: String,
    analysis_date: String,
    model: DeclineModel,
    qi_bpd: f64,
    economic_limit_bpd: f64,
    estimated_eur_bbl: f64,
    remaining_reserves_bbl: f64,
    production_history_months: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EorMethod {
    WaterFlood {
        injection_rate_bpd: f64,
        voidage_replacement_ratio: f64,
    },
    Co2Flood {
        co2_injection_mscfd: f64,
        minimum_miscibility_pressure_psi: f32,
        wag_ratio: f32,
    },
    PolymerFlood {
        polymer_concentration_ppm: f32,
        slug_size_pv: f32,
        viscosity_target_cp: f32,
    },
    SteamFlood {
        steam_quality_percent: f32,
        injection_rate_bpd_cwe: f64,
        oil_steam_ratio: f32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EorProject {
    project_name: String,
    field_name: String,
    method: EorMethod,
    injector_wells: Vec<String>,
    producer_wells: Vec<String>,
    start_date: String,
    incremental_oil_bpd: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IncidentSeverity {
    NearMiss,
    FirstAid,
    MedicalTreatment,
    RestrictedWork,
    LostTime,
    Fatality,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IncidentCategory {
    Slip,
    DroppedObject,
    H2sExposure,
    Spill { volume_bbl: f32 },
    Fire,
    Pressure { overpressure_psi: f32 },
    VehicleIncident,
    EnvironmentalRelease { material: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HseIncident {
    incident_id: u64,
    date: String,
    location: String,
    severity: IncidentSeverity,
    category: IncidentCategory,
    description: String,
    corrective_actions: Vec<String>,
    days_away: u16,
}

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

fn lz4_compress(data: &[u8]) -> Vec<u8> {
    compress(data, Compression::Lz4).expect("lz4 compress failed")
}

fn lz4_decompress(data: &[u8]) -> Vec<u8> {
    decompress(data).expect("lz4 decompress failed")
}

fn roundtrip<T: Encode + Decode<()> + std::fmt::Debug + PartialEq>(val: &T) {
    let encoded = encode_to_vec(val).expect("encode failed");
    let compressed = lz4_compress(&encoded);
    let decompressed = lz4_decompress(&compressed);
    let (decoded, _): (T, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(*val, decoded);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_seismic_trace_lz4_roundtrip() {
    let trace = SeismicTrace {
        line_id: 1001,
        trace_number: 5500,
        sample_interval_us: 2000,
        amplitudes: (0..256).map(|i| (i as f32 * 0.01).sin() * 1000.0).collect(),
        source_x: 456789.12,
        source_y: 3456789.45,
        receiver_x: 456839.12,
        receiver_y: 3456789.45,
    };
    roundtrip(&trace);
}

#[test]
fn test_seismic_survey_lz4_compression_ratio() {
    let survey = SeismicSurvey {
        survey_name: "Block-42 3D Survey".to_string(),
        acquisition_date: "2025-06-15".to_string(),
        traces: (0..50)
            .map(|t| SeismicTrace {
                line_id: 100,
                trace_number: t,
                sample_interval_us: 2000,
                amplitudes: (0..512)
                    .map(|i| ((i + t) as f32 * 0.005).sin() * 500.0)
                    .collect(),
                source_x: 400000.0 + t as f64 * 25.0,
                source_y: 3000000.0,
                receiver_x: 400000.0 + t as f64 * 25.0 + 50.0,
                receiver_y: 3000000.0,
            })
            .collect(),
        bin_size_m: 12.5,
        fold_coverage: 60,
    };
    let encoded = encode_to_vec(&survey).expect("encode seismic survey");
    let compressed = lz4_compress(&encoded);
    let decompressed = lz4_decompress(&compressed);
    let (decoded, _): (SeismicSurvey, usize) =
        decode_from_slice(&decompressed).expect("decode seismic survey");
    assert_eq!(survey, decoded);
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
}

#[test]
fn test_well_log_data_lz4_roundtrip() {
    let n = 200;
    let log = WellLogData {
        well_id: "WELL-A-007".to_string(),
        depth_m: (0..n).map(|i| 1500.0 + i as f64 * 0.15).collect(),
        gamma_ray_api: (0..n)
            .map(|i| 30.0 + (i as f32 * 0.1).sin() * 80.0)
            .collect(),
        resistivity_ohm_m: (0..n)
            .map(|i| 1.0 + (i as f32 * 0.05).cos() * 50.0)
            .collect(),
        porosity_fraction: (0..n)
            .map(|i| 0.05 + (i as f32 * 0.02).sin().abs() * 0.25)
            .collect(),
        density_gcc: (0..n)
            .map(|i| 2.2 + (i as f32 * 0.03).sin() * 0.4)
            .collect(),
        neutron_porosity: (0..n)
            .map(|i| 0.08 + (i as f32 * 0.015).sin().abs() * 0.22)
            .collect(),
    };
    roundtrip(&log);
}

#[test]
fn test_well_log_compression_saves_space() {
    let n = 500;
    let log = WellLogData {
        well_id: "WELL-B-003".to_string(),
        depth_m: (0..n).map(|i| 2000.0 + i as f64 * 0.15).collect(),
        gamma_ray_api: vec![45.0; n],
        resistivity_ohm_m: vec![10.0; n],
        porosity_fraction: vec![0.18; n],
        density_gcc: vec![2.35; n],
        neutron_porosity: vec![0.15; n],
    };
    let encoded = encode_to_vec(&log).expect("encode well log");
    let compressed = lz4_compress(&encoded);
    assert!(
        compressed.len() < encoded.len(),
        "repetitive well log data should compress: {} vs {}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = lz4_decompress(&compressed);
    let (decoded, _): (WellLogData, usize) =
        decode_from_slice(&decompressed).expect("decode well log");
    assert_eq!(log, decoded);
}

#[test]
fn test_drilling_parameters_lz4_roundtrip() {
    let params = DrillingParameters {
        timestamp_epoch: 1700000000,
        well_id: "DRILL-RIG-17".to_string(),
        measured_depth_m: 3250.5,
        rate_of_penetration_mh: 12.4,
        weight_on_bit_klb: 35.0,
        rotary_rpm: 120,
        torque_kft_lb: 18.5,
        standpipe_pressure_psi: 3200.0,
        flow_rate_gpm: 650.0,
    };
    roundtrip(&params);
}

#[test]
fn test_drilling_parameter_series_lz4_roundtrip() {
    let series: Vec<DrillingParameters> = (0..100)
        .map(|i| DrillingParameters {
            timestamp_epoch: 1700000000 + i * 30,
            well_id: "RIG-22-WELL-C".to_string(),
            measured_depth_m: 2800.0 + i as f64 * 0.3,
            rate_of_penetration_mh: 8.0 + (i as f32 * 0.1).sin() * 5.0,
            weight_on_bit_klb: 30.0 + (i as f32 % 10.0) * 0.5,
            rotary_rpm: 100 + (i % 40) as u16,
            torque_kft_lb: 15.0 + (i as f32 * 0.07).cos() * 5.0,
            standpipe_pressure_psi: 3000.0 + (i as f32 % 20.0) * 10.0,
            flow_rate_gpm: 600.0 + (i as f32 % 5.0) * 10.0,
        })
        .collect();
    roundtrip(&series);
}

#[test]
fn test_mud_report_lz4_roundtrip() {
    let report = MudReport {
        well_id: "WELL-D-012".to_string(),
        report_number: 45,
        depth_m: 4100.0,
        rheology: MudRheology {
            mud_weight_ppg: 12.5,
            plastic_viscosity_cp: 22.0,
            yield_point_lbf: 14.0,
            gel_strength_10s: 6.0,
            gel_strength_10m: 12.0,
            ph: 10.2,
            chlorides_ppm: 85000,
            funnel_viscosity_s: 52.0,
        },
        mud_type: "Oil-Based Mud (OBM)".to_string(),
        solids_percent: 18.5,
    };
    roundtrip(&report);
}

#[test]
fn test_bha_configuration_lz4_roundtrip() {
    let bha = BottomHoleAssembly {
        bha_number: 3,
        well_id: "LATERAL-01".to_string(),
        components: vec![
            BhaComponent {
                position: 1,
                component_type: BhaComponentType::DrillBit {
                    size_in: 8.5,
                    bit_type: "PDC 5-blade".to_string(),
                },
                length_m: 0.35,
                weight_kg: 45.0,
            },
            BhaComponent {
                position: 2,
                component_type: BhaComponentType::MotorSteerable {
                    bend_angle_deg: 1.5,
                },
                length_m: 9.5,
                weight_kg: 1200.0,
            },
            BhaComponent {
                position: 3,
                component_type: BhaComponentType::Mwd {
                    gamma_ray: true,
                    resistivity: true,
                },
                length_m: 9.0,
                weight_kg: 850.0,
            },
            BhaComponent {
                position: 4,
                component_type: BhaComponentType::Lwd {
                    density: true,
                    neutron: true,
                },
                length_m: 7.5,
                weight_kg: 750.0,
            },
            BhaComponent {
                position: 5,
                component_type: BhaComponentType::StabilizerNearBit { od_in: 8.25 },
                length_m: 1.8,
                weight_kg: 320.0,
            },
            BhaComponent {
                position: 6,
                component_type: BhaComponentType::DrillCollar {
                    od_in: 6.75,
                    id_in: 2.8125,
                },
                length_m: 9.1,
                weight_kg: 1500.0,
            },
            BhaComponent {
                position: 7,
                component_type: BhaComponentType::Jar,
                length_m: 9.5,
                weight_kg: 900.0,
            },
            BhaComponent {
                position: 8,
                component_type: BhaComponentType::HeavyWeightDrillPipe { od_in: 5.0 },
                length_m: 27.4,
                weight_kg: 2100.0,
            },
        ],
        total_length_m: 74.15,
        total_weight_kg: 7665.0,
    };
    roundtrip(&bha);
}

#[test]
fn test_formation_evaluation_lz4_roundtrip() {
    let zones: Vec<FormationEvaluation> = vec![
        FormationEvaluation {
            zone_name: "Upper Sandstone A".to_string(),
            top_depth_m: 2850.0,
            base_depth_m: 2880.0,
            net_pay_m: 22.5,
            avg_porosity: 0.21,
            avg_water_saturation: 0.25,
            avg_permeability_md: 150.0,
            hydrocarbon_type: "Light Oil (38 API)".to_string(),
            formation_pressure_psi: 4200.0,
            temperature_f: 195.0,
        },
        FormationEvaluation {
            zone_name: "Lower Limestone B".to_string(),
            top_depth_m: 3120.0,
            base_depth_m: 3155.0,
            net_pay_m: 18.0,
            avg_porosity: 0.14,
            avg_water_saturation: 0.35,
            avg_permeability_md: 45.0,
            hydrocarbon_type: "Gas Condensate".to_string(),
            formation_pressure_psi: 4800.0,
            temperature_f: 215.0,
        },
    ];
    roundtrip(&zones);
}

#[test]
fn test_reservoir_simulation_grid_lz4_roundtrip() {
    let cells: Vec<ReservoirCell> = (0..8)
        .flat_map(|i| {
            (0..8).flat_map(move |j| {
                (0..4).map(move |k| ReservoirCell {
                    i,
                    j,
                    k,
                    pressure_psi: 3500.0 - k as f32 * 100.0,
                    oil_saturation: 0.6 - k as f32 * 0.05,
                    water_saturation: 0.25 + k as f32 * 0.04,
                    gas_saturation: 0.15 + k as f32 * 0.01,
                    permeability_md: 100.0 + (i * 10 + j * 5) as f32,
                    porosity: 0.18 + (i as f32 * 0.002),
                    depth_m: 3000.0 + k as f64 * 5.0 + i as f64 * 0.5,
                })
            })
        })
        .collect();
    let grid = ReservoirSimGrid {
        model_name: "Field-X Sector Model".to_string(),
        nx: 8,
        ny: 8,
        nz: 4,
        cells,
        timestep_days: 30.0,
    };
    let encoded = encode_to_vec(&grid).expect("encode reservoir grid");
    let compressed = lz4_compress(&encoded);
    let decompressed = lz4_decompress(&compressed);
    let (decoded, _): (ReservoirSimGrid, usize) =
        decode_from_slice(&decompressed).expect("decode reservoir grid");
    assert_eq!(grid, decoded);
    assert!(
        compressed.len() < encoded.len(),
        "sim grid should compress well: {} vs {}",
        compressed.len(),
        encoded.len()
    );
}

#[test]
fn test_production_allocation_lz4_roundtrip() {
    let allocations: Vec<ProductionAllocation> = (0..30)
        .map(|d| ProductionAllocation {
            field_name: "Caspian Block-7".to_string(),
            well_id: format!("CB7-W{:03}", d % 5 + 1),
            date: format!("2025-11-{:02}", d % 28 + 1),
            oil_bbl: 450.0 + d as f64 * 3.5,
            water_bbl: 120.0 + d as f64 * 8.0,
            gas_mscf: 850.0 + d as f64 * 5.0,
            ngl_bbl: 15.0 + d as f64 * 0.3,
            hours_on: 24.0 - (d % 3) as f32 * 0.5,
            choke_size_64ths: 24 + (d % 8) as u8,
        })
        .collect();
    roundtrip(&allocations);
}

#[test]
fn test_separator_readings_lz4_roundtrip() {
    let readings: Vec<SeparatorReading> = (0..60)
        .map(|i| SeparatorReading {
            separator_id: "SEP-HP-001".to_string(),
            timestamp_epoch: 1700000000 + i * 60,
            inlet_pressure_psi: 450.0 + (i as f32 * 0.1).sin() * 20.0,
            outlet_pressure_psi: 120.0 + (i as f32 * 0.1).cos() * 5.0,
            temperature_f: 145.0 + (i as f32 * 0.05).sin() * 10.0,
            oil_rate_bpd: 2500.0 + (i as f32 * 0.08).sin() * 200.0,
            water_rate_bpd: 800.0 + (i as f32 * 0.06).cos() * 100.0,
            gas_rate_mscfd: 4500.0 + (i as f32 * 0.04).sin() * 500.0,
            level_percent: 55.0 + (i as f32 * 0.12).sin() * 15.0,
        })
        .collect();
    roundtrip(&readings);
}

#[test]
fn test_pipeline_scada_lz4_roundtrip() {
    let scada = PipelineScada {
        pipeline_id: "PL-MAIN-24IN".to_string(),
        segment_name: "Pump Station 3 to Terminal".to_string(),
        data_points: vec![
            ScadaDataPoint {
                tag_name: "PI-3401".to_string(),
                timestamp_epoch: 1700000000,
                value: 850.5,
                quality: 192,
            },
            ScadaDataPoint {
                tag_name: "TI-3402".to_string(),
                timestamp_epoch: 1700000000,
                value: 135.2,
                quality: 192,
            },
            ScadaDataPoint {
                tag_name: "FI-3403".to_string(),
                timestamp_epoch: 1700000000,
                value: 45000.0,
                quality: 192,
            },
            ScadaDataPoint {
                tag_name: "LI-3404".to_string(),
                timestamp_epoch: 1700000000,
                value: 72.8,
                quality: 192,
            },
            ScadaDataPoint {
                tag_name: "VI-3405".to_string(),
                timestamp_epoch: 1700000000,
                value: 0.12,
                quality: 0,
            },
        ],
        flow_rate_bpd: 45000.0,
        inlet_pressure_psi: 850.5,
        outlet_pressure_psi: 320.0,
    };
    roundtrip(&scada);
}

#[test]
fn test_esp_artificial_lift_lz4_roundtrip() {
    let esp = ArtificialLiftSystem {
        well_id: "WELL-ESP-009".to_string(),
        lift_type: ArtificialLiftType::Esp {
            motor_hp: 250.0,
            frequency_hz: 55.0,
            intake_pressure_psi: 1800.0,
            discharge_pressure_psi: 4200.0,
            motor_temp_f: 285.0,
            vibration_ips: 0.35,
        },
        install_date: "2024-03-15".to_string(),
        last_service_date: "2025-09-20".to_string(),
        runtime_hours: 13200,
    };
    roundtrip(&esp);
}

#[test]
fn test_rod_pump_artificial_lift_lz4_roundtrip() {
    let rod_pump = ArtificialLiftSystem {
        well_id: "WELL-RP-042".to_string(),
        lift_type: ArtificialLiftType::RodPump {
            stroke_length_in: 144.0,
            strokes_per_min: 8.5,
            pump_fillage_percent: 82.0,
            polished_rod_load_lb: 22000.0,
            counterbalance_lb: 18000.0,
        },
        install_date: "2023-07-01".to_string(),
        last_service_date: "2025-11-10".to_string(),
        runtime_hours: 18500,
    };
    roundtrip(&rod_pump);
}

#[test]
fn test_gas_lift_system_lz4_roundtrip() {
    let gas_lift = ArtificialLiftSystem {
        well_id: "WELL-GL-018".to_string(),
        lift_type: ArtificialLiftType::GasLift {
            injection_rate_mscfd: 1200.0,
            injection_pressure_psi: 2800.0,
            valve_depths_m: vec![800.0, 1200.0, 1600.0, 2000.0, 2400.0],
            operating_valve_idx: 4,
            glr_scf_bbl: 3500.0,
        },
        install_date: "2024-01-20".to_string(),
        last_service_date: "2025-08-05".to_string(),
        runtime_hours: 15800,
    };
    roundtrip(&gas_lift);
}

#[test]
fn test_well_test_results_lz4_roundtrip() {
    let tests: Vec<WellTestResult> = vec![
        WellTestResult {
            well_id: "APPRAISAL-2".to_string(),
            test_date: "2025-10-01".to_string(),
            choke_size_64ths: 32,
            oil_rate_bpd: 2800.0,
            water_rate_bpd: 150.0,
            gas_rate_mscfd: 5600.0,
            gor_scf_bbl: 2000.0,
            water_cut_percent: 5.1,
            flowing_tubing_pressure_psi: 1850.0,
            flowing_casing_pressure_psi: 2100.0,
            bsw_percent: 5.3,
            api_gravity: 38.5,
        },
        WellTestResult {
            well_id: "APPRAISAL-2".to_string(),
            test_date: "2025-10-01".to_string(),
            choke_size_64ths: 48,
            oil_rate_bpd: 3600.0,
            water_rate_bpd: 210.0,
            gas_rate_mscfd: 7200.0,
            gor_scf_bbl: 2000.0,
            water_cut_percent: 5.5,
            flowing_tubing_pressure_psi: 1620.0,
            flowing_casing_pressure_psi: 1900.0,
            bsw_percent: 5.8,
            api_gravity: 38.5,
        },
    ];
    roundtrip(&tests);
}

#[test]
fn test_decline_curve_analysis_lz4_roundtrip() {
    let analyses = vec![
        DeclineCurveAnalysis {
            well_id: "PROD-W-101".to_string(),
            analysis_date: "2025-12-01".to_string(),
            model: DeclineModel::Hyperbolic {
                di_per_year: 0.65,
                b_factor: 0.8,
            },
            qi_bpd: 1500.0,
            economic_limit_bpd: 10.0,
            estimated_eur_bbl: 1_250_000.0,
            remaining_reserves_bbl: 450_000.0,
            production_history_months: 48,
        },
        DeclineCurveAnalysis {
            well_id: "PROD-W-102".to_string(),
            analysis_date: "2025-12-01".to_string(),
            model: DeclineModel::Exponential { di_per_year: 0.35 },
            qi_bpd: 800.0,
            economic_limit_bpd: 5.0,
            estimated_eur_bbl: 650_000.0,
            remaining_reserves_bbl: 180_000.0,
            production_history_months: 72,
        },
        DeclineCurveAnalysis {
            well_id: "PROD-W-103".to_string(),
            analysis_date: "2025-12-01".to_string(),
            model: DeclineModel::Harmonic { di_per_year: 0.50 },
            qi_bpd: 2200.0,
            economic_limit_bpd: 15.0,
            estimated_eur_bbl: 2_800_000.0,
            remaining_reserves_bbl: 1_100_000.0,
            production_history_months: 36,
        },
    ];
    roundtrip(&analyses);
}

#[test]
fn test_eor_co2_flood_lz4_roundtrip() {
    let project = EorProject {
        project_name: "CO2-EOR Phase II".to_string(),
        field_name: "Permian Basin Unit 14".to_string(),
        method: EorMethod::Co2Flood {
            co2_injection_mscfd: 25000.0,
            minimum_miscibility_pressure_psi: 1850.0,
            wag_ratio: 1.5,
        },
        injector_wells: vec![
            "INJ-01".to_string(),
            "INJ-02".to_string(),
            "INJ-03".to_string(),
            "INJ-04".to_string(),
        ],
        producer_wells: vec![
            "PRD-01".to_string(),
            "PRD-02".to_string(),
            "PRD-03".to_string(),
            "PRD-04".to_string(),
            "PRD-05".to_string(),
        ],
        start_date: "2024-06-01".to_string(),
        incremental_oil_bpd: 1200.0,
    };
    roundtrip(&project);
}

#[test]
fn test_eor_steam_polymer_waterflood_lz4_roundtrip() {
    let projects = vec![
        EorProject {
            project_name: "Thermal Recovery Pilot".to_string(),
            field_name: "Heavy Oil Sands Block A".to_string(),
            method: EorMethod::SteamFlood {
                steam_quality_percent: 80.0,
                injection_rate_bpd_cwe: 5000.0,
                oil_steam_ratio: 0.25,
            },
            injector_wells: vec!["SI-1".to_string(), "SI-2".to_string()],
            producer_wells: vec!["SP-1".to_string(), "SP-2".to_string(), "SP-3".to_string()],
            start_date: "2025-01-15".to_string(),
            incremental_oil_bpd: 600.0,
        },
        EorProject {
            project_name: "Polymer Conformance".to_string(),
            field_name: "Offshore Field Y".to_string(),
            method: EorMethod::PolymerFlood {
                polymer_concentration_ppm: 1500.0,
                slug_size_pv: 0.3,
                viscosity_target_cp: 12.0,
            },
            injector_wells: vec!["PI-A1".to_string()],
            producer_wells: vec!["PP-A1".to_string(), "PP-A2".to_string()],
            start_date: "2025-04-01".to_string(),
            incremental_oil_bpd: 350.0,
        },
        EorProject {
            project_name: "Secondary Waterflood".to_string(),
            field_name: "Onshore Block 9".to_string(),
            method: EorMethod::WaterFlood {
                injection_rate_bpd: 8000.0,
                voidage_replacement_ratio: 1.1,
            },
            injector_wells: vec![
                "WI-01".to_string(),
                "WI-02".to_string(),
                "WI-03".to_string(),
            ],
            producer_wells: vec![
                "WP-01".to_string(),
                "WP-02".to_string(),
                "WP-03".to_string(),
                "WP-04".to_string(),
            ],
            start_date: "2023-09-01".to_string(),
            incremental_oil_bpd: 1800.0,
        },
    ];
    roundtrip(&projects);
}

#[test]
fn test_hse_incident_tracking_lz4_roundtrip() {
    let incidents = vec![
        HseIncident {
            incident_id: 20251101,
            date: "2025-11-01".to_string(),
            location: "Drilling Rig Floor".to_string(),
            severity: IncidentSeverity::FirstAid,
            category: IncidentCategory::Slip,
            description: "Worker slipped on wet rig floor during pipe tripping operations"
                .to_string(),
            corrective_actions: vec![
                "Installed additional non-slip matting".to_string(),
                "Revised rig floor housekeeping checklist".to_string(),
            ],
            days_away: 0,
        },
        HseIncident {
            incident_id: 20251115,
            date: "2025-11-15".to_string(),
            location: "Tank Battery Area".to_string(),
            severity: IncidentSeverity::NearMiss,
            category: IncidentCategory::H2sExposure,
            description: "H2S alarm triggered at 15 ppm during tank gauging; personnel evacuated"
                .to_string(),
            corrective_actions: vec![
                "Installed continuous H2S monitors at tank hatches".to_string(),
                "Updated emergency response procedure".to_string(),
                "Conducted H2S awareness refresher training".to_string(),
            ],
            days_away: 0,
        },
        HseIncident {
            incident_id: 20251120,
            date: "2025-11-20".to_string(),
            location: "Flowline Right-of-Way".to_string(),
            severity: IncidentSeverity::RestrictedWork,
            category: IncidentCategory::Spill { volume_bbl: 2.5 },
            description: "Small oil spill from corroded fitting on 4-inch flowline".to_string(),
            corrective_actions: vec![
                "Replaced corroded fitting and 50 ft of line".to_string(),
                "Deployed absorbent pads and remediated soil".to_string(),
                "Scheduled integrity inspection for entire flowline".to_string(),
            ],
            days_away: 5,
        },
    ];
    roundtrip(&incidents);
}

#[test]
fn test_full_upstream_dataset_lz4_compression_ratio() {
    // Build a combined dataset with multiple domain objects
    let allocations: Vec<ProductionAllocation> = (0..20)
        .map(|d| ProductionAllocation {
            field_name: "Deepwater Field Zeta".to_string(),
            well_id: format!("DWZ-{:02}", d % 4 + 1),
            date: format!("2025-12-{:02}", d % 28 + 1),
            oil_bbl: 3200.0 + d as f64 * 15.0,
            water_bbl: 500.0 + d as f64 * 25.0,
            gas_mscf: 6400.0 + d as f64 * 30.0,
            ngl_bbl: 80.0 + d as f64 * 1.2,
            hours_on: 24.0,
            choke_size_64ths: 32,
        })
        .collect();

    let encoded = encode_to_vec(&allocations).expect("encode allocations");
    let compressed = lz4_compress(&encoded);
    let decompressed = lz4_decompress(&compressed);
    let (decoded, _): (Vec<ProductionAllocation>, usize) =
        decode_from_slice(&decompressed).expect("decode allocations");
    assert_eq!(allocations, decoded);

    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 1.0,
        "compression ratio should be less than 1.0, got {:.3}",
        ratio
    );
}
