#![cfg(feature = "std")]

//! Scientific instrument data acquisition tests for OxiCode file I/O.
//!
//! Covers mass spectrometry, electron microscopy, NMR, X-ray diffraction,
//! spectrophotometry, flow cytometry, PCR, HPLC, calorimetry, and more.

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

// ── Mass Spectrometry Types ─────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MassSpecPeak {
    mz_ratio: f64,
    intensity: f64,
    charge_state: i32,
    resolution: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MassSpectrum {
    scan_number: u32,
    retention_time_sec: f64,
    ms_level: u8,
    precursor_mz: Option<f64>,
    collision_energy_ev: Option<f32>,
    peaks: Vec<MassSpecPeak>,
    total_ion_current: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IonizationMode {
    ElectronImpact {
        energy_ev: f64,
    },
    Electrospray {
        polarity_positive: bool,
        voltage_kv: f32,
    },
    Maldi {
        matrix: String,
        laser_frequency_hz: u32,
    },
    ChemicalIonization {
        reagent_gas: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MassSpecExperiment {
    instrument_id: String,
    ionization: IonizationMode,
    mass_range_low: f64,
    mass_range_high: f64,
    scans: Vec<MassSpectrum>,
}

// ── Chromatography Types ────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChromatographyPoint {
    time_min: f64,
    absorbance_au: f64,
    pressure_bar: f32,
    flow_rate_ml_per_min: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChromatographyPeak {
    start_time_min: f64,
    apex_time_min: f64,
    end_time_min: f64,
    area: f64,
    height: f64,
    asymmetry_factor: f32,
    plate_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HplcGradientStep {
    time_min: f64,
    solvent_b_pct: f32,
    flow_rate_ml_per_min: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HplcRun {
    column_id: String,
    column_length_mm: f32,
    particle_size_um: f32,
    temperature_c: f32,
    detection_wavelength_nm: u16,
    gradient: Vec<HplcGradientStep>,
    chromatogram: Vec<ChromatographyPoint>,
    integrated_peaks: Vec<ChromatographyPeak>,
}

// ── NMR Types ───────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NmrPeak {
    chemical_shift_ppm: f64,
    intensity: f64,
    multiplicity: NmrMultiplicity,
    coupling_constants_hz: Vec<f64>,
    assignment: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NmrMultiplicity {
    Singlet,
    Doublet,
    Triplet,
    Quartet,
    Multiplet,
    DoubletOfDoublets,
    DoubletOfTriplets,
    BroadSinglet,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NmrSpectrum {
    nucleus: String,
    frequency_mhz: f64,
    solvent: String,
    temperature_k: f64,
    number_of_scans: u32,
    spectral_width_ppm: f64,
    fid_points: u32,
    peaks: Vec<NmrPeak>,
    reference_compound: String,
}

// ── Electron Microscopy Types ───────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmImagingParams {
    accelerating_voltage_kv: f64,
    beam_current_na: f64,
    working_distance_mm: f64,
    magnification: u32,
    pixel_size_nm: f64,
    dwell_time_us: f64,
    detector: EmDetectorType,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EmDetectorType {
    SecondaryElectron,
    BackscatteredElectron,
    EnergyDispersiveXray {
        elements: Vec<String>,
    },
    BrightField,
    DarkField,
    Haadf {
        inner_angle_mrad: f64,
        outer_angle_mrad: f64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EdxSpectrum {
    energy_kev: Vec<f64>,
    counts: Vec<u32>,
    live_time_sec: f64,
    dead_time_pct: f32,
    quantification: Vec<ElementQuantification>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ElementQuantification {
    element: String,
    weight_pct: f64,
    atomic_pct: f64,
    k_ratio: f64,
}

// ── X-Ray Diffraction Types ────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct XrdDataPoint {
    two_theta_deg: f64,
    intensity_counts: u32,
    d_spacing_angstrom: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct XrdPattern {
    radiation_source: String,
    wavelength_angstrom: f64,
    scan_speed_deg_per_min: f32,
    step_size_deg: f32,
    data_points: Vec<XrdDataPoint>,
    identified_phases: Vec<CrystalPhase>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrystalPhase {
    name: String,
    space_group: String,
    lattice_a_angstrom: f64,
    lattice_b_angstrom: f64,
    lattice_c_angstrom: f64,
    alpha_deg: f64,
    beta_deg: f64,
    gamma_deg: f64,
    weight_fraction: f64,
}

// ── Spectrophotometer Types ────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UvVisSpectrum {
    wavelength_nm: Vec<f64>,
    absorbance: Vec<f64>,
    path_length_cm: f32,
    solvent: String,
    concentration_mol_per_l: f64,
    peaks: Vec<UvVisPeak>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UvVisPeak {
    lambda_max_nm: f64,
    absorbance_max: f64,
    molar_absorptivity: f64,
    bandwidth_nm: f64,
}

// ── Flow Cytometry Types ───────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlowCytometryEvent {
    forward_scatter: f32,
    side_scatter: f32,
    fluorescence_channels: Vec<f32>,
    time_us: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlowCytometryGate {
    name: String,
    gate_type: GateType,
    parent_gate: Option<String>,
    event_count: u64,
    percentage_of_parent: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GateType {
    Polygon {
        vertices_x: Vec<f32>,
        vertices_y: Vec<f32>,
    },
    Rectangle {
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
    },
    Ellipse {
        center_x: f32,
        center_y: f32,
        radius_x: f32,
        radius_y: f32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlowCytometryExperiment {
    sample_id: String,
    laser_wavelengths_nm: Vec<u16>,
    channels: Vec<String>,
    events: Vec<FlowCytometryEvent>,
    gates: Vec<FlowCytometryGate>,
    total_events: u64,
    acquisition_time_sec: f64,
}

// ── PCR / qPCR Types ───────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PcrAmplificationCurve {
    well_id: String,
    sample_name: String,
    target_gene: String,
    cycle_data: Vec<PcrCyclePoint>,
    ct_value: Option<f64>,
    baseline_start: u32,
    baseline_end: u32,
    threshold: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PcrCyclePoint {
    cycle_number: u32,
    fluorescence_rfu: f64,
    temperature_c: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MeltCurvePoint {
    temperature_c: f64,
    neg_derivative_fluorescence: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QpcrExperiment {
    instrument_name: String,
    plate_format: u16,
    amplification_curves: Vec<PcrAmplificationCurve>,
    melt_curves: Vec<Vec<MeltCurvePoint>>,
}

// ── Calorimetry Types ──────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DscDataPoint {
    temperature_c: f64,
    heat_flow_mw: f64,
    time_min: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DscThermalEvent {
    event_type: ThermalEventType,
    onset_temperature_c: f64,
    peak_temperature_c: f64,
    endset_temperature_c: f64,
    enthalpy_j_per_g: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ThermalEventType {
    GlassTransition,
    Melting,
    Crystallization,
    Decomposition,
    Evaporation,
    ColdCrystallization,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DscExperiment {
    sample_mass_mg: f64,
    heating_rate_c_per_min: f64,
    atmosphere: String,
    flow_rate_ml_per_min: f32,
    data_points: Vec<DscDataPoint>,
    thermal_events: Vec<DscThermalEvent>,
}

// ── Infrared Spectroscopy Types ────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FtirSpectrum {
    wavenumber_cm_inv: Vec<f64>,
    transmittance_pct: Vec<f64>,
    resolution_cm_inv: f32,
    number_of_scans: u32,
    sample_technique: FtirTechnique,
    identified_bands: Vec<FtirBand>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FtirTechnique {
    Transmission,
    AttenuatedTotalReflectance { crystal_material: String },
    DiffuseReflectance,
    SpecularReflectance,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FtirBand {
    wavenumber_cm_inv: f64,
    intensity: f64,
    assignment: String,
    functional_group: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

// ── Test 1: Mass spectrum peak roundtrip via encode_to_vec/decode_from_slice
#[test]
fn test_mass_spec_peak_roundtrip() {
    let peak = MassSpecPeak {
        mz_ratio: 445.1202,
        intensity: 1_540_230.5,
        charge_state: 2,
        resolution: 35000.0,
    };
    let bytes = encode_to_vec(&peak).expect("encode mass spec peak");
    let (decoded, _): (MassSpecPeak, _) = decode_from_slice(&bytes).expect("decode mass spec peak");
    assert_eq!(peak, decoded);
}

// ── Test 2: Full mass spectrum with MS/MS via file I/O
#[test]
fn test_mass_spectrum_msms_file_io() {
    let spectrum = MassSpectrum {
        scan_number: 4521,
        retention_time_sec: 1823.45,
        ms_level: 2,
        precursor_mz: Some(578.2894),
        collision_energy_ev: Some(35.0),
        peaks: vec![
            MassSpecPeak {
                mz_ratio: 120.0808,
                intensity: 85420.0,
                charge_state: 1,
                resolution: 30000.0,
            },
            MassSpecPeak {
                mz_ratio: 175.1190,
                intensity: 254310.0,
                charge_state: 1,
                resolution: 30000.0,
            },
            MassSpecPeak {
                mz_ratio: 288.1343,
                intensity: 432100.0,
                charge_state: 1,
                resolution: 30000.0,
            },
            MassSpecPeak {
                mz_ratio: 445.2127,
                intensity: 1243500.0,
                charge_state: 2,
                resolution: 30000.0,
            },
        ],
        total_ion_current: 2_015_330.0,
    };
    let path = temp_dir().join("oxicode_test_instrument_mass_spectrum_msms.bin");
    encode_to_file(&spectrum, &path).expect("encode mass spectrum to file");
    let decoded: MassSpectrum = decode_from_file(&path).expect("decode mass spectrum from file");
    assert_eq!(spectrum, decoded);
    std::fs::remove_file(&path).expect("cleanup mass spectrum file");
}

// ── Test 3: Ionization mode enum variants
#[test]
fn test_ionization_mode_variants() {
    let modes = vec![
        IonizationMode::ElectronImpact { energy_ev: 70.0 },
        IonizationMode::Electrospray {
            polarity_positive: true,
            voltage_kv: 4.5,
        },
        IonizationMode::Maldi {
            matrix: "DHB".to_string(),
            laser_frequency_hz: 1000,
        },
        IonizationMode::ChemicalIonization {
            reagent_gas: "methane".to_string(),
        },
    ];
    let bytes = encode_to_vec(&modes).expect("encode ionization modes");
    let (decoded, _): (Vec<IonizationMode>, _) =
        decode_from_slice(&bytes).expect("decode ionization modes");
    assert_eq!(modes, decoded);
}

// ── Test 4: Complete mass spec experiment via file I/O
#[test]
fn test_mass_spec_experiment_file_io() {
    let experiment = MassSpecExperiment {
        instrument_id: "QExactive-HFX-001".to_string(),
        ionization: IonizationMode::Electrospray {
            polarity_positive: true,
            voltage_kv: 3.8,
        },
        mass_range_low: 100.0,
        mass_range_high: 2000.0,
        scans: vec![
            MassSpectrum {
                scan_number: 1,
                retention_time_sec: 0.5,
                ms_level: 1,
                precursor_mz: None,
                collision_energy_ev: None,
                peaks: vec![
                    MassSpecPeak {
                        mz_ratio: 301.1412,
                        intensity: 6_543_210.0,
                        charge_state: 1,
                        resolution: 60000.0,
                    },
                    MassSpecPeak {
                        mz_ratio: 602.2817,
                        intensity: 982_340.0,
                        charge_state: 2,
                        resolution: 60000.0,
                    },
                ],
                total_ion_current: 7_525_550.0,
            },
            MassSpectrum {
                scan_number: 2,
                retention_time_sec: 1.0,
                ms_level: 2,
                precursor_mz: Some(301.1412),
                collision_energy_ev: Some(25.0),
                peaks: vec![MassSpecPeak {
                    mz_ratio: 153.0546,
                    intensity: 1_234_000.0,
                    charge_state: 1,
                    resolution: 60000.0,
                }],
                total_ion_current: 1_234_000.0,
            },
        ],
    };
    let path = temp_dir().join("oxicode_test_instrument_ms_experiment.bin");
    encode_to_file(&experiment, &path).expect("encode ms experiment to file");
    let decoded: MassSpecExperiment =
        decode_from_file(&path).expect("decode ms experiment from file");
    assert_eq!(experiment, decoded);
    std::fs::remove_file(&path).expect("cleanup ms experiment file");
}

// ── Test 5: HPLC gradient profile roundtrip
#[test]
fn test_hplc_gradient_profile() {
    let gradient = vec![
        HplcGradientStep {
            time_min: 0.0,
            solvent_b_pct: 5.0,
            flow_rate_ml_per_min: 0.3,
        },
        HplcGradientStep {
            time_min: 2.0,
            solvent_b_pct: 5.0,
            flow_rate_ml_per_min: 0.3,
        },
        HplcGradientStep {
            time_min: 25.0,
            solvent_b_pct: 40.0,
            flow_rate_ml_per_min: 0.3,
        },
        HplcGradientStep {
            time_min: 30.0,
            solvent_b_pct: 95.0,
            flow_rate_ml_per_min: 0.3,
        },
        HplcGradientStep {
            time_min: 35.0,
            solvent_b_pct: 95.0,
            flow_rate_ml_per_min: 0.3,
        },
        HplcGradientStep {
            time_min: 36.0,
            solvent_b_pct: 5.0,
            flow_rate_ml_per_min: 0.3,
        },
        HplcGradientStep {
            time_min: 45.0,
            solvent_b_pct: 5.0,
            flow_rate_ml_per_min: 0.3,
        },
    ];
    let bytes = encode_to_vec(&gradient).expect("encode HPLC gradient");
    let (decoded, _): (Vec<HplcGradientStep>, _) =
        decode_from_slice(&bytes).expect("decode HPLC gradient");
    assert_eq!(gradient, decoded);
}

// ── Test 6: Full HPLC run with chromatogram and peak integration via file
#[test]
fn test_hplc_run_file_io() {
    let run = HplcRun {
        column_id: "C18-BEH-130A".to_string(),
        column_length_mm: 150.0,
        particle_size_um: 1.7,
        temperature_c: 40.0,
        detection_wavelength_nm: 254,
        gradient: vec![
            HplcGradientStep {
                time_min: 0.0,
                solvent_b_pct: 10.0,
                flow_rate_ml_per_min: 0.4,
            },
            HplcGradientStep {
                time_min: 20.0,
                solvent_b_pct: 90.0,
                flow_rate_ml_per_min: 0.4,
            },
        ],
        chromatogram: vec![
            ChromatographyPoint {
                time_min: 0.0,
                absorbance_au: 0.001,
                pressure_bar: 420.0,
                flow_rate_ml_per_min: 0.4,
            },
            ChromatographyPoint {
                time_min: 5.5,
                absorbance_au: 0.842,
                pressure_bar: 435.0,
                flow_rate_ml_per_min: 0.4,
            },
            ChromatographyPoint {
                time_min: 12.3,
                absorbance_au: 1.235,
                pressure_bar: 450.0,
                flow_rate_ml_per_min: 0.4,
            },
        ],
        integrated_peaks: vec![
            ChromatographyPeak {
                start_time_min: 5.1,
                apex_time_min: 5.5,
                end_time_min: 6.0,
                area: 12543.2,
                height: 0.842,
                asymmetry_factor: 1.05,
                plate_count: 85000,
            },
            ChromatographyPeak {
                start_time_min: 11.8,
                apex_time_min: 12.3,
                end_time_min: 12.9,
                area: 28940.7,
                height: 1.235,
                asymmetry_factor: 1.12,
                plate_count: 72000,
            },
        ],
    };
    let path = temp_dir().join("oxicode_test_instrument_hplc_run.bin");
    encode_to_file(&run, &path).expect("encode HPLC run to file");
    let decoded: HplcRun = decode_from_file(&path).expect("decode HPLC run from file");
    assert_eq!(run, decoded);
    std::fs::remove_file(&path).expect("cleanup HPLC run file");
}

// ── Test 7: NMR spectrum with coupling constants
#[test]
fn test_nmr_spectrum_roundtrip() {
    let spectrum = NmrSpectrum {
        nucleus: "1H".to_string(),
        frequency_mhz: 600.0,
        solvent: "CDCl3".to_string(),
        temperature_k: 298.15,
        number_of_scans: 64,
        spectral_width_ppm: 14.0,
        fid_points: 65536,
        peaks: vec![
            NmrPeak {
                chemical_shift_ppm: 7.26,
                intensity: 1.0,
                multiplicity: NmrMultiplicity::Singlet,
                coupling_constants_hz: vec![],
                assignment: "CHCl3 (residual)".to_string(),
            },
            NmrPeak {
                chemical_shift_ppm: 3.85,
                intensity: 3.0,
                multiplicity: NmrMultiplicity::Singlet,
                coupling_constants_hz: vec![],
                assignment: "OCH3".to_string(),
            },
            NmrPeak {
                chemical_shift_ppm: 6.92,
                intensity: 2.0,
                multiplicity: NmrMultiplicity::DoubletOfDoublets,
                coupling_constants_hz: vec![8.5, 2.3],
                assignment: "Ar-H".to_string(),
            },
        ],
        reference_compound: "TMS".to_string(),
    };
    let bytes = encode_to_vec(&spectrum).expect("encode NMR spectrum");
    let (decoded, _): (NmrSpectrum, _) = decode_from_slice(&bytes).expect("decode NMR spectrum");
    assert_eq!(spectrum, decoded);
}

// ── Test 8: NMR multiplicity enum exhaustive via file
#[test]
fn test_nmr_multiplicity_all_variants_file_io() {
    let multiplicities = vec![
        NmrMultiplicity::Singlet,
        NmrMultiplicity::Doublet,
        NmrMultiplicity::Triplet,
        NmrMultiplicity::Quartet,
        NmrMultiplicity::Multiplet,
        NmrMultiplicity::DoubletOfDoublets,
        NmrMultiplicity::DoubletOfTriplets,
        NmrMultiplicity::BroadSinglet,
    ];
    let path = temp_dir().join("oxicode_test_instrument_nmr_multiplicities.bin");
    encode_to_file(&multiplicities, &path).expect("encode NMR multiplicities to file");
    let decoded: Vec<NmrMultiplicity> =
        decode_from_file(&path).expect("decode NMR multiplicities from file");
    assert_eq!(multiplicities, decoded);
    std::fs::remove_file(&path).expect("cleanup NMR multiplicities file");
}

// ── Test 9: Electron microscopy imaging parameters
#[test]
fn test_em_imaging_params_roundtrip() {
    let params = EmImagingParams {
        accelerating_voltage_kv: 200.0,
        beam_current_na: 0.5,
        working_distance_mm: 8.2,
        magnification: 500_000,
        pixel_size_nm: 0.05,
        dwell_time_us: 10.0,
        detector: EmDetectorType::Haadf {
            inner_angle_mrad: 50.0,
            outer_angle_mrad: 200.0,
        },
    };
    let bytes = encode_to_vec(&params).expect("encode EM imaging params");
    let (decoded, _): (EmImagingParams, _) =
        decode_from_slice(&bytes).expect("decode EM imaging params");
    assert_eq!(params, decoded);
}

// ── Test 10: EDX spectrum with elemental quantification via file
#[test]
fn test_edx_spectrum_file_io() {
    let edx = EdxSpectrum {
        energy_kev: vec![0.277, 0.525, 1.041, 1.487, 6.404, 7.058],
        counts: vec![12450, 8930, 4520, 15670, 34210, 8760],
        live_time_sec: 120.0,
        dead_time_pct: 15.3,
        quantification: vec![
            ElementQuantification {
                element: "C".to_string(),
                weight_pct: 12.34,
                atomic_pct: 19.82,
                k_ratio: 0.0421,
            },
            ElementQuantification {
                element: "O".to_string(),
                weight_pct: 28.91,
                atomic_pct: 34.87,
                k_ratio: 0.0892,
            },
            ElementQuantification {
                element: "Fe".to_string(),
                weight_pct: 58.75,
                atomic_pct: 20.31,
                k_ratio: 0.5612,
            },
        ],
    };
    let path = temp_dir().join("oxicode_test_instrument_edx_spectrum.bin");
    encode_to_file(&edx, &path).expect("encode EDX spectrum to file");
    let decoded: EdxSpectrum = decode_from_file(&path).expect("decode EDX spectrum from file");
    assert_eq!(edx, decoded);
    std::fs::remove_file(&path).expect("cleanup EDX spectrum file");
}

// ── Test 11: X-ray diffraction pattern with phase identification
#[test]
fn test_xrd_pattern_roundtrip() {
    let pattern = XrdPattern {
        radiation_source: "Cu-Kalpha".to_string(),
        wavelength_angstrom: 1.5406,
        scan_speed_deg_per_min: 2.0,
        step_size_deg: 0.02,
        data_points: vec![
            XrdDataPoint {
                two_theta_deg: 25.3,
                intensity_counts: 1200,
                d_spacing_angstrom: 3.517,
            },
            XrdDataPoint {
                two_theta_deg: 37.8,
                intensity_counts: 850,
                d_spacing_angstrom: 2.379,
            },
            XrdDataPoint {
                two_theta_deg: 48.0,
                intensity_counts: 620,
                d_spacing_angstrom: 1.893,
            },
            XrdDataPoint {
                two_theta_deg: 54.4,
                intensity_counts: 430,
                d_spacing_angstrom: 1.685,
            },
            XrdDataPoint {
                two_theta_deg: 62.7,
                intensity_counts: 380,
                d_spacing_angstrom: 1.480,
            },
        ],
        identified_phases: vec![
            CrystalPhase {
                name: "Anatase TiO2".to_string(),
                space_group: "I4_1/amd".to_string(),
                lattice_a_angstrom: 3.785,
                lattice_b_angstrom: 3.785,
                lattice_c_angstrom: 9.514,
                alpha_deg: 90.0,
                beta_deg: 90.0,
                gamma_deg: 90.0,
                weight_fraction: 0.82,
            },
            CrystalPhase {
                name: "Rutile TiO2".to_string(),
                space_group: "P4_2/mnm".to_string(),
                lattice_a_angstrom: 4.594,
                lattice_b_angstrom: 4.594,
                lattice_c_angstrom: 2.959,
                alpha_deg: 90.0,
                beta_deg: 90.0,
                gamma_deg: 90.0,
                weight_fraction: 0.18,
            },
        ],
    };
    let bytes = encode_to_vec(&pattern).expect("encode XRD pattern");
    let (decoded, _): (XrdPattern, _) = decode_from_slice(&bytes).expect("decode XRD pattern");
    assert_eq!(pattern, decoded);
}

// ── Test 12: UV-Vis spectrophotometer data via file
#[test]
fn test_uvvis_spectrum_file_io() {
    let spectrum = UvVisSpectrum {
        wavelength_nm: vec![200.0, 220.0, 240.0, 260.0, 280.0, 300.0, 320.0, 340.0],
        absorbance: vec![0.12, 0.34, 0.89, 1.45, 0.98, 0.42, 0.15, 0.05],
        path_length_cm: 1.0,
        solvent: "water".to_string(),
        concentration_mol_per_l: 5e-5,
        peaks: vec![
            UvVisPeak {
                lambda_max_nm: 260.0,
                absorbance_max: 1.45,
                molar_absorptivity: 29000.0,
                bandwidth_nm: 30.0,
            },
            UvVisPeak {
                lambda_max_nm: 280.0,
                absorbance_max: 0.98,
                molar_absorptivity: 19600.0,
                bandwidth_nm: 20.0,
            },
        ],
    };
    let path = temp_dir().join("oxicode_test_instrument_uvvis.bin");
    encode_to_file(&spectrum, &path).expect("encode UV-Vis spectrum to file");
    let decoded: UvVisSpectrum = decode_from_file(&path).expect("decode UV-Vis spectrum from file");
    assert_eq!(spectrum, decoded);
    std::fs::remove_file(&path).expect("cleanup UV-Vis spectrum file");
}

// ── Test 13: Flow cytometry events roundtrip
#[test]
fn test_flow_cytometry_events_roundtrip() {
    let events: Vec<FlowCytometryEvent> = (0..100)
        .map(|i| FlowCytometryEvent {
            forward_scatter: 50000.0 + (i as f32) * 100.0,
            side_scatter: 20000.0 + (i as f32) * 50.0,
            fluorescence_channels: vec![
                (i as f32) * 10.0,
                (i as f32) * 5.0 + 100.0,
                (i as f32) * 2.5 + 50.0,
            ],
            time_us: (i as u64) * 1000,
        })
        .collect();
    let bytes = encode_to_vec(&events).expect("encode flow cytometry events");
    let (decoded, _): (Vec<FlowCytometryEvent>, _) =
        decode_from_slice(&bytes).expect("decode flow cytometry events");
    assert_eq!(events, decoded);
}

// ── Test 14: Flow cytometry experiment with gates via file
#[test]
fn test_flow_cytometry_experiment_file_io() {
    let experiment = FlowCytometryExperiment {
        sample_id: "PBMC-donor42".to_string(),
        laser_wavelengths_nm: vec![405, 488, 561, 640],
        channels: vec![
            "BV421".to_string(),
            "FITC".to_string(),
            "PE".to_string(),
            "APC".to_string(),
        ],
        events: vec![
            FlowCytometryEvent {
                forward_scatter: 65000.0,
                side_scatter: 25000.0,
                fluorescence_channels: vec![1200.0, 450.0, 8900.0, 2300.0],
                time_us: 500,
            },
            FlowCytometryEvent {
                forward_scatter: 120000.0,
                side_scatter: 48000.0,
                fluorescence_channels: vec![300.0, 12000.0, 150.0, 9800.0],
                time_us: 1500,
            },
        ],
        gates: vec![
            FlowCytometryGate {
                name: "Lymphocytes".to_string(),
                gate_type: GateType::Polygon {
                    vertices_x: vec![40000.0, 90000.0, 90000.0, 40000.0],
                    vertices_y: vec![10000.0, 10000.0, 50000.0, 50000.0],
                },
                parent_gate: None,
                event_count: 45000,
                percentage_of_parent: 45.0,
            },
            FlowCytometryGate {
                name: "CD4+ T cells".to_string(),
                gate_type: GateType::Rectangle {
                    x_min: 500.0,
                    x_max: 20000.0,
                    y_min: 100.0,
                    y_max: 15000.0,
                },
                parent_gate: Some("Lymphocytes".to_string()),
                event_count: 18000,
                percentage_of_parent: 40.0,
            },
        ],
        total_events: 100_000,
        acquisition_time_sec: 120.5,
    };
    let path = temp_dir().join("oxicode_test_instrument_flow_cytometry.bin");
    encode_to_file(&experiment, &path).expect("encode flow cytometry to file");
    let decoded: FlowCytometryExperiment =
        decode_from_file(&path).expect("decode flow cytometry from file");
    assert_eq!(experiment, decoded);
    std::fs::remove_file(&path).expect("cleanup flow cytometry file");
}

// ── Test 15: PCR amplification curve with Ct value
#[test]
fn test_pcr_amplification_curve_roundtrip() {
    let curve = PcrAmplificationCurve {
        well_id: "A01".to_string(),
        sample_name: "Patient-112".to_string(),
        target_gene: "GAPDH".to_string(),
        cycle_data: (1..=40)
            .map(|c| PcrCyclePoint {
                cycle_number: c,
                fluorescence_rfu: if c < 20 {
                    50.0 + (c as f64) * 2.0
                } else {
                    50.0 + 40.0 + ((c as f64 - 20.0) * 500.0).min(15000.0)
                },
                temperature_c: 60.0,
            })
            .collect(),
        ct_value: Some(22.4),
        baseline_start: 3,
        baseline_end: 15,
        threshold: 200.0,
    };
    let bytes = encode_to_vec(&curve).expect("encode PCR curve");
    let (decoded, _): (PcrAmplificationCurve, _) =
        decode_from_slice(&bytes).expect("decode PCR curve");
    assert_eq!(curve, decoded);
}

// ── Test 16: qPCR experiment with melt curves via file
#[test]
fn test_qpcr_experiment_file_io() {
    let experiment = QpcrExperiment {
        instrument_name: "QuantStudio-7".to_string(),
        plate_format: 384,
        amplification_curves: vec![
            PcrAmplificationCurve {
                well_id: "A01".to_string(),
                sample_name: "NTC".to_string(),
                target_gene: "RNaseP".to_string(),
                cycle_data: (1..=40)
                    .map(|c| PcrCyclePoint {
                        cycle_number: c,
                        fluorescence_rfu: 45.0 + (c as f64) * 0.5,
                        temperature_c: 60.0,
                    })
                    .collect(),
                ct_value: None,
                baseline_start: 3,
                baseline_end: 15,
                threshold: 500.0,
            },
            PcrAmplificationCurve {
                well_id: "B01".to_string(),
                sample_name: "Standard-1".to_string(),
                target_gene: "RNaseP".to_string(),
                cycle_data: (1..=40)
                    .map(|c| PcrCyclePoint {
                        cycle_number: c,
                        fluorescence_rfu: if c < 18 {
                            48.0
                        } else {
                            48.0 + ((c - 18) as f64).powi(3) * 20.0
                        },
                        temperature_c: 60.0,
                    })
                    .collect(),
                ct_value: Some(19.8),
                baseline_start: 3,
                baseline_end: 15,
                threshold: 500.0,
            },
        ],
        melt_curves: vec![(650..=950)
            .map(|t| {
                let temp = t as f64 / 10.0;
                let peak_temp = 82.5;
                let neg_deriv = 5000.0 * (-((temp - peak_temp).powi(2)) / 2.0).exp();
                MeltCurvePoint {
                    temperature_c: temp,
                    neg_derivative_fluorescence: neg_deriv,
                }
            })
            .collect()],
    };
    let path = temp_dir().join("oxicode_test_instrument_qpcr.bin");
    encode_to_file(&experiment, &path).expect("encode qPCR experiment to file");
    let decoded: QpcrExperiment =
        decode_from_file(&path).expect("decode qPCR experiment from file");
    assert_eq!(experiment, decoded);
    std::fs::remove_file(&path).expect("cleanup qPCR experiment file");
}

// ── Test 17: DSC calorimetry experiment
#[test]
fn test_dsc_experiment_roundtrip() {
    let experiment = DscExperiment {
        sample_mass_mg: 8.43,
        heating_rate_c_per_min: 10.0,
        atmosphere: "N2".to_string(),
        flow_rate_ml_per_min: 50.0,
        data_points: (0..200)
            .map(|i| {
                let temp = 25.0 + (i as f64) * 1.5;
                let heat_flow = if (155.0..165.0).contains(&temp) {
                    -15.0 + (temp - 160.0).powi(2) * 0.3
                } else {
                    -0.5 - temp * 0.001
                };
                DscDataPoint {
                    temperature_c: temp,
                    heat_flow_mw: heat_flow,
                    time_min: (i as f64) * 0.15,
                }
            })
            .collect(),
        thermal_events: vec![
            DscThermalEvent {
                event_type: ThermalEventType::GlassTransition,
                onset_temperature_c: 65.0,
                peak_temperature_c: 72.0,
                endset_temperature_c: 78.0,
                enthalpy_j_per_g: 0.0,
            },
            DscThermalEvent {
                event_type: ThermalEventType::Melting,
                onset_temperature_c: 155.0,
                peak_temperature_c: 160.3,
                endset_temperature_c: 165.0,
                enthalpy_j_per_g: 89.2,
            },
            DscThermalEvent {
                event_type: ThermalEventType::Decomposition,
                onset_temperature_c: 280.0,
                peak_temperature_c: 295.0,
                endset_temperature_c: 310.0,
                enthalpy_j_per_g: 420.0,
            },
        ],
    };
    let bytes = encode_to_vec(&experiment).expect("encode DSC experiment");
    let (decoded, _): (DscExperiment, _) =
        decode_from_slice(&bytes).expect("decode DSC experiment");
    assert_eq!(experiment, decoded);
}

// ── Test 18: FTIR spectrum with band assignments via file
#[test]
fn test_ftir_spectrum_file_io() {
    let spectrum = FtirSpectrum {
        wavenumber_cm_inv: vec![
            4000.0, 3500.0, 3000.0, 2500.0, 2000.0, 1750.0, 1500.0, 1250.0, 1000.0, 750.0, 500.0,
        ],
        transmittance_pct: vec![
            95.0, 42.0, 78.0, 92.0, 96.0, 15.0, 65.0, 48.0, 30.0, 72.0, 88.0,
        ],
        resolution_cm_inv: 4.0,
        number_of_scans: 128,
        sample_technique: FtirTechnique::AttenuatedTotalReflectance {
            crystal_material: "Diamond".to_string(),
        },
        identified_bands: vec![
            FtirBand {
                wavenumber_cm_inv: 3400.0,
                intensity: 58.0,
                assignment: "O-H stretch".to_string(),
                functional_group: "Hydroxyl".to_string(),
            },
            FtirBand {
                wavenumber_cm_inv: 1720.0,
                intensity: 85.0,
                assignment: "C=O stretch".to_string(),
                functional_group: "Carbonyl".to_string(),
            },
            FtirBand {
                wavenumber_cm_inv: 1050.0,
                intensity: 70.0,
                assignment: "C-O stretch".to_string(),
                functional_group: "Ether".to_string(),
            },
        ],
    };
    let path = temp_dir().join("oxicode_test_instrument_ftir.bin");
    encode_to_file(&spectrum, &path).expect("encode FTIR spectrum to file");
    let decoded: FtirSpectrum = decode_from_file(&path).expect("decode FTIR spectrum from file");
    assert_eq!(spectrum, decoded);
    std::fs::remove_file(&path).expect("cleanup FTIR spectrum file");
}

// ── Test 19: Thermal event type enum exhaustive
#[test]
fn test_thermal_event_type_all_variants() {
    let events = vec![
        ThermalEventType::GlassTransition,
        ThermalEventType::Melting,
        ThermalEventType::Crystallization,
        ThermalEventType::Decomposition,
        ThermalEventType::Evaporation,
        ThermalEventType::ColdCrystallization,
    ];
    let bytes = encode_to_vec(&events).expect("encode thermal event types");
    let (decoded, _): (Vec<ThermalEventType>, _) =
        decode_from_slice(&bytes).expect("decode thermal event types");
    assert_eq!(events, decoded);
}

// ── Test 20: EM detector type variants with nested data via file
#[test]
fn test_em_detector_types_file_io() {
    let detectors = vec![
        EmDetectorType::SecondaryElectron,
        EmDetectorType::BackscatteredElectron,
        EmDetectorType::EnergyDispersiveXray {
            elements: vec!["Si".to_string(), "O".to_string(), "Al".to_string()],
        },
        EmDetectorType::BrightField,
        EmDetectorType::DarkField,
        EmDetectorType::Haadf {
            inner_angle_mrad: 68.0,
            outer_angle_mrad: 180.0,
        },
    ];
    let path = temp_dir().join("oxicode_test_instrument_em_detectors.bin");
    encode_to_file(&detectors, &path).expect("encode EM detectors to file");
    let decoded: Vec<EmDetectorType> =
        decode_from_file(&path).expect("decode EM detectors from file");
    assert_eq!(detectors, decoded);
    std::fs::remove_file(&path).expect("cleanup EM detectors file");
}

// ── Test 21: Multi-instrument dataset combining mass spec + chromatography
#[test]
fn test_multi_instrument_combined_roundtrip() {
    // Simulate an LC-MS dataset: chromatogram points paired with mass spectra
    let chromatogram_points = vec![
        ChromatographyPoint {
            time_min: 0.0,
            absorbance_au: 0.002,
            pressure_bar: 400.0,
            flow_rate_ml_per_min: 0.3,
        },
        ChromatographyPoint {
            time_min: 8.5,
            absorbance_au: 0.534,
            pressure_bar: 415.0,
            flow_rate_ml_per_min: 0.3,
        },
        ChromatographyPoint {
            time_min: 14.2,
            absorbance_au: 1.820,
            pressure_bar: 430.0,
            flow_rate_ml_per_min: 0.3,
        },
        ChromatographyPoint {
            time_min: 22.1,
            absorbance_au: 0.720,
            pressure_bar: 445.0,
            flow_rate_ml_per_min: 0.3,
        },
    ];
    let mass_spectra = vec![
        MassSpectrum {
            scan_number: 510,
            retention_time_sec: 510.0,
            ms_level: 1,
            precursor_mz: None,
            collision_energy_ev: None,
            peaks: vec![
                MassSpecPeak {
                    mz_ratio: 311.1390,
                    intensity: 3_421_000.0,
                    charge_state: 1,
                    resolution: 45000.0,
                },
                MassSpecPeak {
                    mz_ratio: 622.2710,
                    intensity: 812_000.0,
                    charge_state: 2,
                    resolution: 45000.0,
                },
            ],
            total_ion_current: 4_233_000.0,
        },
        MassSpectrum {
            scan_number: 852,
            retention_time_sec: 852.0,
            ms_level: 1,
            precursor_mz: None,
            collision_energy_ev: None,
            peaks: vec![MassSpecPeak {
                mz_ratio: 489.2541,
                intensity: 8_920_000.0,
                charge_state: 1,
                resolution: 45000.0,
            }],
            total_ion_current: 8_920_000.0,
        },
    ];
    let dataset: (Vec<ChromatographyPoint>, Vec<MassSpectrum>) =
        (chromatogram_points.clone(), mass_spectra.clone());
    let bytes = encode_to_vec(&dataset).expect("encode LC-MS dataset");
    let (decoded, _): ((Vec<ChromatographyPoint>, Vec<MassSpectrum>), _) =
        decode_from_slice(&bytes).expect("decode LC-MS dataset");
    assert_eq!(decoded.0, chromatogram_points);
    assert_eq!(decoded.1, mass_spectra);
}

// ── Test 22: Crystal phase data with triclinic/monoclinic lattice via file
#[test]
fn test_crystal_phases_triclinic_monoclinic_file_io() {
    let phases = vec![
        CrystalPhase {
            name: "Albite NaAlSi3O8".to_string(),
            space_group: "C-1".to_string(),
            lattice_a_angstrom: 8.144,
            lattice_b_angstrom: 12.787,
            lattice_c_angstrom: 7.160,
            alpha_deg: 94.26,
            beta_deg: 116.59,
            gamma_deg: 87.68,
            weight_fraction: 0.55,
        },
        CrystalPhase {
            name: "Gypsum CaSO4·2H2O".to_string(),
            space_group: "A2/a".to_string(),
            lattice_a_angstrom: 5.679,
            lattice_b_angstrom: 15.202,
            lattice_c_angstrom: 6.523,
            alpha_deg: 90.0,
            beta_deg: 118.43,
            gamma_deg: 90.0,
            weight_fraction: 0.30,
        },
        CrystalPhase {
            name: "Quartz SiO2".to_string(),
            space_group: "P3_221".to_string(),
            lattice_a_angstrom: 4.913,
            lattice_b_angstrom: 4.913,
            lattice_c_angstrom: 5.405,
            alpha_deg: 90.0,
            beta_deg: 90.0,
            gamma_deg: 120.0,
            weight_fraction: 0.15,
        },
    ];
    let path = temp_dir().join("oxicode_test_instrument_crystal_phases.bin");
    encode_to_file(&phases, &path).expect("encode crystal phases to file");
    let decoded: Vec<CrystalPhase> =
        decode_from_file(&path).expect("decode crystal phases from file");
    assert_eq!(phases, decoded);
    std::fs::remove_file(&path).expect("cleanup crystal phases file");
}
