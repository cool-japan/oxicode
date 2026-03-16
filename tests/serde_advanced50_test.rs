#![cfg(feature = "serde")]
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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

// --- Domain types: Scientific Instruments & Laboratory Equipment ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MassSpecPeak {
    mz_ratio: f64,
    intensity: f64,
    charge_state: u8,
    isotope_pattern: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MassSpecReading {
    scan_id: u64,
    scan_time_sec: f64,
    ionization_mode: String,
    peaks: Vec<MassSpecPeak>,
    total_ion_current: f64,
    base_peak_mz: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ChromatographyPeak {
    retention_time_min: f64,
    area: f64,
    height: f64,
    width_at_half_max: f64,
    asymmetry_factor: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ChromatographyRun {
    run_id: u64,
    method_name: String,
    instrument_type: String,
    column_id: String,
    mobile_phase: String,
    flow_rate_ml_min: f64,
    injection_volume_ul: f64,
    peaks: Vec<ChromatographyPeak>,
    total_run_time_min: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct NmrPeak {
    chemical_shift_ppm: f64,
    multiplicity: String,
    coupling_constant_hz: Vec<f64>,
    integration: f64,
    assignment: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct NmrSpectrum {
    spectrum_id: u64,
    nucleus: String,
    frequency_mhz: f64,
    solvent: String,
    temperature_k: f64,
    peaks: Vec<NmrPeak>,
    reference_standard: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PcrStep {
    step_name: String,
    temperature_c: f64,
    duration_sec: u32,
    ramp_rate_c_per_sec: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PcrProtocol {
    protocol_id: u64,
    protocol_name: String,
    enzyme: String,
    cycle_count: u32,
    steps: Vec<PcrStep>,
    lid_temperature_c: f64,
    final_hold_c: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ElectronMicroscopeImage {
    image_id: u64,
    microscope_type: String,
    accelerating_voltage_kv: f64,
    magnification: u32,
    working_distance_mm: f64,
    detector: String,
    pixel_width: u32,
    pixel_height: u32,
    scale_bar_nm: f64,
    sample_tilt_deg: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FlowCytometryGate {
    gate_name: String,
    parameter_x: String,
    parameter_y: String,
    event_count: u64,
    parent_gate: Option<String>,
    percentage_of_parent: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FlowCytometryExperiment {
    experiment_id: u64,
    panel_name: String,
    total_events: u64,
    fluorochromes: Vec<String>,
    gates: Vec<FlowCytometryGate>,
    compensation_applied: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AbsorbancePoint {
    wavelength_nm: f64,
    absorbance: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SpectrophotometerReading {
    reading_id: u64,
    sample_name: String,
    path_length_cm: f64,
    blank_subtracted: bool,
    curve: Vec<AbsorbancePoint>,
    peak_wavelength_nm: f64,
    peak_absorbance: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PhCalibrationPoint {
    buffer_ph: f64,
    measured_mv: f64,
    temperature_c: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PhCalibrationRecord {
    record_id: u64,
    electrode_serial: String,
    calibration_date: String,
    points: Vec<PhCalibrationPoint>,
    slope_mv_per_ph: f64,
    offset_mv: f64,
    r_squared: f64,
    passed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CentrifugeRunParams {
    run_id: u64,
    rotor_id: String,
    rotor_type: String,
    speed_rpm: u32,
    rcf_g: f64,
    duration_min: u32,
    temperature_c: f64,
    acceleration_profile: u8,
    deceleration_profile: u8,
    sample_count: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AutoclaveCyclePhase {
    phase_name: String,
    target_temperature_c: f64,
    target_pressure_kpa: f64,
    duration_min: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AutoclaveCycle {
    cycle_id: u64,
    cycle_type: String,
    load_description: String,
    phases: Vec<AutoclaveCyclePhase>,
    biological_indicator_passed: bool,
    chemical_indicator_passed: bool,
    operator: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PipetteCalibrationTest {
    volume_ul: f64,
    measured_weights_mg: Vec<f64>,
    mean_mg: f64,
    std_dev_mg: f64,
    accuracy_pct: f64,
    precision_cv_pct: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PipetteCalibrationLog {
    log_id: u64,
    pipette_serial: String,
    pipette_model: String,
    nominal_volume_ul: f64,
    calibration_date: String,
    tests: Vec<PipetteCalibrationTest>,
    overall_pass: bool,
    technician: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CustodyEvent {
    timestamp: String,
    action: String,
    person: String,
    location: String,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SampleChainOfCustody {
    sample_id: String,
    sample_type: String,
    collection_date: String,
    storage_condition: String,
    events: Vec<CustodyEvent>,
    is_compromised: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReagentBottle {
    catalog_number: String,
    reagent_name: String,
    manufacturer: String,
    lot_number: String,
    grade: String,
    quantity_ml: f64,
    concentration_m: Option<f64>,
    expiration_date: String,
    storage_temp_c: f64,
    cas_number: String,
    hazard_codes: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReagentInventory {
    inventory_id: u64,
    lab_name: String,
    last_audit_date: String,
    bottles: Vec<ReagentBottle>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MaintenanceTask {
    task_id: u64,
    equipment_id: String,
    equipment_name: String,
    task_description: String,
    frequency_days: u32,
    last_performed: String,
    next_due: String,
    performed_by: String,
    cost_usd: f64,
    is_overdue: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MaintenanceSchedule {
    schedule_id: u64,
    department: String,
    tasks: Vec<MaintenanceTask>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LimsTestResult {
    analyte: String,
    method: String,
    value: f64,
    unit: String,
    lower_limit: f64,
    upper_limit: f64,
    within_spec: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LimsRecord {
    record_id: u64,
    sample_id: String,
    project_code: String,
    submission_date: String,
    results: Vec<LimsTestResult>,
    approved_by: Option<String>,
    status: String,
}

// --- Tests ---

#[test]
fn test_mass_spec_reading_roundtrip() {
    let reading = MassSpecReading {
        scan_id: 48201,
        scan_time_sec: 124.567,
        ionization_mode: "ESI+".to_string(),
        peaks: vec![
            MassSpecPeak {
                mz_ratio: 256.1892,
                intensity: 1_540_000.0,
                charge_state: 1,
                isotope_pattern: vec![100.0, 27.3, 4.1],
            },
            MassSpecPeak {
                mz_ratio: 128.5983,
                intensity: 890_000.0,
                charge_state: 2,
                isotope_pattern: vec![100.0, 13.6, 1.0],
            },
        ],
        total_ion_current: 3_200_000.0,
        base_peak_mz: 256.1892,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&reading, cfg).expect("encode mass spec reading");
    let (decoded, _): (MassSpecReading, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode mass spec reading");
    assert_eq!(reading, decoded);
}

#[test]
fn test_hplc_chromatography_run_roundtrip() {
    let run = ChromatographyRun {
        run_id: 99301,
        method_name: "USP_Aspirin_Assay_v3".to_string(),
        instrument_type: "HPLC".to_string(),
        column_id: "C18-250x4.6-5um-SN8842".to_string(),
        mobile_phase: "ACN:H2O 60:40 + 0.1% TFA".to_string(),
        flow_rate_ml_min: 1.0,
        injection_volume_ul: 10.0,
        peaks: vec![
            ChromatographyPeak {
                retention_time_min: 3.42,
                area: 2_456_789.0,
                height: 345_000.0,
                width_at_half_max: 0.12,
                asymmetry_factor: 1.05,
            },
            ChromatographyPeak {
                retention_time_min: 7.89,
                area: 1_234_567.0,
                height: 198_000.0,
                width_at_half_max: 0.15,
                asymmetry_factor: 1.12,
            },
        ],
        total_run_time_min: 20.0,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&run, cfg).expect("encode HPLC run");
    let (decoded, _): (ChromatographyRun, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode HPLC run");
    assert_eq!(run, decoded);
}

#[test]
fn test_gc_chromatography_single_peak() {
    let run = ChromatographyRun {
        run_id: 55001,
        method_name: "EPA_8260B_VOC".to_string(),
        instrument_type: "GC-MS".to_string(),
        column_id: "DB-5ms-30mx0.25mm-SN4411".to_string(),
        mobile_phase: "He carrier gas".to_string(),
        flow_rate_ml_min: 1.2,
        injection_volume_ul: 1.0,
        peaks: vec![ChromatographyPeak {
            retention_time_min: 5.67,
            area: 789_012.0,
            height: 112_000.0,
            width_at_half_max: 0.08,
            asymmetry_factor: 0.98,
        }],
        total_run_time_min: 35.0,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&run, cfg).expect("encode GC run");
    let (decoded, _): (ChromatographyRun, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode GC run");
    assert_eq!(run, decoded);
}

#[test]
fn test_nmr_proton_spectrum_roundtrip() {
    let spectrum = NmrSpectrum {
        spectrum_id: 71020,
        nucleus: "1H".to_string(),
        frequency_mhz: 400.13,
        solvent: "CDCl3".to_string(),
        temperature_k: 298.0,
        peaks: vec![
            NmrPeak {
                chemical_shift_ppm: 7.26,
                multiplicity: "s".to_string(),
                coupling_constant_hz: vec![],
                integration: 1.0,
                assignment: "CHCl3 residual".to_string(),
            },
            NmrPeak {
                chemical_shift_ppm: 3.85,
                multiplicity: "q".to_string(),
                coupling_constant_hz: vec![7.1],
                integration: 2.0,
                assignment: "OCH2".to_string(),
            },
            NmrPeak {
                chemical_shift_ppm: 1.22,
                multiplicity: "t".to_string(),
                coupling_constant_hz: vec![7.1],
                integration: 3.0,
                assignment: "CH3".to_string(),
            },
        ],
        reference_standard: "TMS".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&spectrum, cfg).expect("encode NMR spectrum");
    let (decoded, _): (NmrSpectrum, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode NMR spectrum");
    assert_eq!(spectrum, decoded);
}

#[test]
fn test_nmr_carbon13_spectrum() {
    let spectrum = NmrSpectrum {
        spectrum_id: 71021,
        nucleus: "13C".to_string(),
        frequency_mhz: 100.61,
        solvent: "DMSO-d6".to_string(),
        temperature_k: 300.0,
        peaks: vec![
            NmrPeak {
                chemical_shift_ppm: 170.5,
                multiplicity: "s".to_string(),
                coupling_constant_hz: vec![],
                integration: 1.0,
                assignment: "C=O".to_string(),
            },
            NmrPeak {
                chemical_shift_ppm: 39.5,
                multiplicity: "s".to_string(),
                coupling_constant_hz: vec![],
                integration: 1.0,
                assignment: "DMSO-d6".to_string(),
            },
        ],
        reference_standard: "TMS".to_string(),
    };
    let encoded = encode_to_vec(&spectrum, config::standard()).expect("encode 13C NMR");
    let (decoded, _): (NmrSpectrum, usize) =
        decode_owned_from_slice(&encoded, config::standard()).expect("decode 13C NMR");
    assert_eq!(spectrum, decoded);
}

#[test]
fn test_pcr_thermocycler_protocol_roundtrip() {
    let protocol = PcrProtocol {
        protocol_id: 20450,
        protocol_name: "Standard_Taq_PCR_v2".to_string(),
        enzyme: "Taq DNA Polymerase".to_string(),
        cycle_count: 35,
        steps: vec![
            PcrStep {
                step_name: "Initial Denaturation".to_string(),
                temperature_c: 95.0,
                duration_sec: 300,
                ramp_rate_c_per_sec: 3.0,
            },
            PcrStep {
                step_name: "Denaturation".to_string(),
                temperature_c: 95.0,
                duration_sec: 30,
                ramp_rate_c_per_sec: 3.0,
            },
            PcrStep {
                step_name: "Annealing".to_string(),
                temperature_c: 58.0,
                duration_sec: 30,
                ramp_rate_c_per_sec: 2.5,
            },
            PcrStep {
                step_name: "Extension".to_string(),
                temperature_c: 72.0,
                duration_sec: 60,
                ramp_rate_c_per_sec: 2.5,
            },
            PcrStep {
                step_name: "Final Extension".to_string(),
                temperature_c: 72.0,
                duration_sec: 600,
                ramp_rate_c_per_sec: 2.5,
            },
        ],
        lid_temperature_c: 105.0,
        final_hold_c: 4.0,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&protocol, cfg).expect("encode PCR protocol");
    let (decoded, _): (PcrProtocol, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode PCR protocol");
    assert_eq!(protocol, decoded);
}

#[test]
fn test_sem_image_metadata_roundtrip() {
    let image = ElectronMicroscopeImage {
        image_id: 880012,
        microscope_type: "SEM".to_string(),
        accelerating_voltage_kv: 15.0,
        magnification: 50000,
        working_distance_mm: 8.5,
        detector: "SE2".to_string(),
        pixel_width: 2048,
        pixel_height: 1536,
        scale_bar_nm: 500.0,
        sample_tilt_deg: 0.0,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&image, cfg).expect("encode SEM image");
    let (decoded, _): (ElectronMicroscopeImage, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode SEM image");
    assert_eq!(image, decoded);
}

#[test]
fn test_tem_image_metadata_roundtrip() {
    let image = ElectronMicroscopeImage {
        image_id: 880099,
        microscope_type: "TEM".to_string(),
        accelerating_voltage_kv: 200.0,
        magnification: 500_000,
        working_distance_mm: 0.0,
        detector: "BF-STEM".to_string(),
        pixel_width: 4096,
        pixel_height: 4096,
        scale_bar_nm: 10.0,
        sample_tilt_deg: 15.0,
    };
    let encoded = encode_to_vec(&image, config::standard()).expect("encode TEM image");
    let (decoded, _): (ElectronMicroscopeImage, usize) =
        decode_owned_from_slice(&encoded, config::standard()).expect("decode TEM image");
    assert_eq!(image, decoded);
}

#[test]
fn test_flow_cytometry_experiment_roundtrip() {
    let experiment = FlowCytometryExperiment {
        experiment_id: 33210,
        panel_name: "T-Cell Immunophenotyping".to_string(),
        total_events: 100_000,
        fluorochromes: vec![
            "FITC".to_string(),
            "PE".to_string(),
            "PerCP-Cy5.5".to_string(),
            "APC".to_string(),
        ],
        gates: vec![
            FlowCytometryGate {
                gate_name: "Lymphocytes".to_string(),
                parameter_x: "FSC-A".to_string(),
                parameter_y: "SSC-A".to_string(),
                event_count: 45_000,
                parent_gate: None,
                percentage_of_parent: 45.0,
            },
            FlowCytometryGate {
                gate_name: "CD3+".to_string(),
                parameter_x: "CD3-FITC".to_string(),
                parameter_y: "SSC-A".to_string(),
                event_count: 32_400,
                parent_gate: Some("Lymphocytes".to_string()),
                percentage_of_parent: 72.0,
            },
            FlowCytometryGate {
                gate_name: "CD4+".to_string(),
                parameter_x: "CD4-PE".to_string(),
                parameter_y: "CD8-APC".to_string(),
                event_count: 20_736,
                parent_gate: Some("CD3+".to_string()),
                percentage_of_parent: 64.0,
            },
        ],
        compensation_applied: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&experiment, cfg).expect("encode flow cytometry");
    let (decoded, _): (FlowCytometryExperiment, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode flow cytometry");
    assert_eq!(experiment, decoded);
}

#[test]
fn test_spectrophotometer_absorbance_curve() {
    let reading = SpectrophotometerReading {
        reading_id: 12500,
        sample_name: "BSA_standard_1mgml".to_string(),
        path_length_cm: 1.0,
        blank_subtracted: true,
        curve: vec![
            AbsorbancePoint {
                wavelength_nm: 260.0,
                absorbance: 0.312,
            },
            AbsorbancePoint {
                wavelength_nm: 270.0,
                absorbance: 0.287,
            },
            AbsorbancePoint {
                wavelength_nm: 280.0,
                absorbance: 0.665,
            },
            AbsorbancePoint {
                wavelength_nm: 290.0,
                absorbance: 0.401,
            },
            AbsorbancePoint {
                wavelength_nm: 300.0,
                absorbance: 0.095,
            },
        ],
        peak_wavelength_nm: 280.0,
        peak_absorbance: 0.665,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&reading, cfg).expect("encode spectrophotometer");
    let (decoded, _): (SpectrophotometerReading, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode spectrophotometer");
    assert_eq!(reading, decoded);
}

#[test]
fn test_ph_calibration_record_roundtrip() {
    let record = PhCalibrationRecord {
        record_id: 4501,
        electrode_serial: "ORION-9102BNWP-SN44821".to_string(),
        calibration_date: "2026-03-15".to_string(),
        points: vec![
            PhCalibrationPoint {
                buffer_ph: 4.01,
                measured_mv: 178.2,
                temperature_c: 25.0,
            },
            PhCalibrationPoint {
                buffer_ph: 7.00,
                measured_mv: 1.3,
                temperature_c: 25.0,
            },
            PhCalibrationPoint {
                buffer_ph: 10.01,
                measured_mv: -176.8,
                temperature_c: 25.0,
            },
        ],
        slope_mv_per_ph: -59.16,
        offset_mv: 1.3,
        r_squared: 0.9998,
        passed: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&record, cfg).expect("encode pH calibration");
    let (decoded, _): (PhCalibrationRecord, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode pH calibration");
    assert_eq!(record, decoded);
}

#[test]
fn test_centrifuge_run_parameters_roundtrip() {
    let run = CentrifugeRunParams {
        run_id: 67890,
        rotor_id: "FA-45-30-11-SN2203".to_string(),
        rotor_type: "Fixed Angle".to_string(),
        speed_rpm: 14_000,
        rcf_g: 16_873.0,
        duration_min: 10,
        temperature_c: 4.0,
        acceleration_profile: 9,
        deceleration_profile: 7,
        sample_count: 24,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&run, cfg).expect("encode centrifuge run");
    let (decoded, _): (CentrifugeRunParams, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode centrifuge run");
    assert_eq!(run, decoded);
}

#[test]
fn test_autoclave_sterilization_cycle_roundtrip() {
    let cycle = AutoclaveCycle {
        cycle_id: 11200,
        cycle_type: "Gravity".to_string(),
        load_description: "Wrapped surgical instruments, mixed metal".to_string(),
        phases: vec![
            AutoclaveCyclePhase {
                phase_name: "Conditioning".to_string(),
                target_temperature_c: 100.0,
                target_pressure_kpa: 101.3,
                duration_min: 5,
            },
            AutoclaveCyclePhase {
                phase_name: "Sterilization".to_string(),
                target_temperature_c: 121.0,
                target_pressure_kpa: 205.0,
                duration_min: 30,
            },
            AutoclaveCyclePhase {
                phase_name: "Exhaust".to_string(),
                target_temperature_c: 100.0,
                target_pressure_kpa: 101.3,
                duration_min: 15,
            },
            AutoclaveCyclePhase {
                phase_name: "Drying".to_string(),
                target_temperature_c: 80.0,
                target_pressure_kpa: 80.0,
                duration_min: 20,
            },
        ],
        biological_indicator_passed: true,
        chemical_indicator_passed: true,
        operator: "J. Tanaka".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&cycle, cfg).expect("encode autoclave cycle");
    let (decoded, _): (AutoclaveCycle, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode autoclave cycle");
    assert_eq!(cycle, decoded);
}

#[test]
fn test_pipette_calibration_log_roundtrip() {
    let log = PipetteCalibrationLog {
        log_id: 9010,
        pipette_serial: "EJ-P200-SN12847".to_string(),
        pipette_model: "Eppendorf Research Plus 20-200uL".to_string(),
        nominal_volume_ul: 200.0,
        calibration_date: "2026-03-10".to_string(),
        tests: vec![
            PipetteCalibrationTest {
                volume_ul: 200.0,
                measured_weights_mg: vec![
                    199.8, 200.1, 199.9, 200.0, 199.7, 200.2, 199.9, 200.0, 199.8, 200.1,
                ],
                mean_mg: 199.95,
                std_dev_mg: 0.16,
                accuracy_pct: 99.975,
                precision_cv_pct: 0.08,
            },
            PipetteCalibrationTest {
                volume_ul: 100.0,
                measured_weights_mg: vec![
                    99.9, 100.1, 100.0, 99.8, 100.2, 100.0, 99.9, 100.1, 100.0, 99.9,
                ],
                mean_mg: 99.99,
                std_dev_mg: 0.12,
                accuracy_pct: 99.99,
                precision_cv_pct: 0.12,
            },
            PipetteCalibrationTest {
                volume_ul: 20.0,
                measured_weights_mg: vec![
                    19.8, 20.1, 20.0, 19.9, 20.2, 20.0, 19.9, 20.1, 19.8, 20.0,
                ],
                mean_mg: 19.98,
                std_dev_mg: 0.13,
                accuracy_pct: 99.9,
                precision_cv_pct: 0.65,
            },
        ],
        overall_pass: true,
        technician: "M. Nakamura".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&log, cfg).expect("encode pipette calibration");
    let (decoded, _): (PipetteCalibrationLog, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode pipette calibration");
    assert_eq!(log, decoded);
}

#[test]
fn test_sample_chain_of_custody_roundtrip() {
    let chain = SampleChainOfCustody {
        sample_id: "ENV-2026-W-00412".to_string(),
        sample_type: "Groundwater".to_string(),
        collection_date: "2026-03-12T08:30:00Z".to_string(),
        storage_condition: "4C refrigerated, amber glass".to_string(),
        events: vec![
            CustodyEvent {
                timestamp: "2026-03-12T08:30:00Z".to_string(),
                action: "Collected".to_string(),
                person: "Field Tech A. Suzuki".to_string(),
                location: "Site MW-7, 15m depth".to_string(),
                notes: "Purged 3 well volumes, low turbidity".to_string(),
            },
            CustodyEvent {
                timestamp: "2026-03-12T10:15:00Z".to_string(),
                action: "Transported".to_string(),
                person: "Courier K. Yamamoto".to_string(),
                location: "In transit to Central Lab".to_string(),
                notes: "Cooler temp verified 2-6C".to_string(),
            },
            CustodyEvent {
                timestamp: "2026-03-12T14:00:00Z".to_string(),
                action: "Received".to_string(),
                person: "Lab Tech R. Ito".to_string(),
                location: "Central Analytical Lab, Room 204".to_string(),
                notes: "Seal intact, temp 3.8C on arrival".to_string(),
            },
        ],
        is_compromised: false,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&chain, cfg).expect("encode chain of custody");
    let (decoded, _): (SampleChainOfCustody, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode chain of custody");
    assert_eq!(chain, decoded);
}

#[test]
fn test_reagent_inventory_roundtrip() {
    let inventory = ReagentInventory {
        inventory_id: 3300,
        lab_name: "Organic Chemistry Lab B-210".to_string(),
        last_audit_date: "2026-02-28".to_string(),
        bottles: vec![
            ReagentBottle {
                catalog_number: "A1001-500ML".to_string(),
                reagent_name: "Acetonitrile, HPLC Grade".to_string(),
                manufacturer: "Fisher Scientific".to_string(),
                lot_number: "LOT-2025-11-4821".to_string(),
                grade: "HPLC".to_string(),
                quantity_ml: 500.0,
                concentration_m: None,
                expiration_date: "2027-11-30".to_string(),
                storage_temp_c: 22.0,
                cas_number: "75-05-8".to_string(),
                hazard_codes: vec![
                    "H225".to_string(),
                    "H302".to_string(),
                    "H312".to_string(),
                    "H332".to_string(),
                ],
            },
            ReagentBottle {
                catalog_number: "HCL-1M-1L".to_string(),
                reagent_name: "Hydrochloric Acid, 1M".to_string(),
                manufacturer: "Sigma-Aldrich".to_string(),
                lot_number: "LOT-2025-09-1193".to_string(),
                grade: "ACS Reagent".to_string(),
                quantity_ml: 1000.0,
                concentration_m: Some(1.0),
                expiration_date: "2028-09-15".to_string(),
                storage_temp_c: 22.0,
                cas_number: "7647-01-0".to_string(),
                hazard_codes: vec!["H290".to_string(), "H314".to_string(), "H335".to_string()],
            },
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&inventory, cfg).expect("encode reagent inventory");
    let (decoded, _): (ReagentInventory, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode reagent inventory");
    assert_eq!(inventory, decoded);
}

#[test]
fn test_equipment_maintenance_schedule_roundtrip() {
    let schedule = MaintenanceSchedule {
        schedule_id: 5500,
        department: "Analytical Chemistry".to_string(),
        tasks: vec![
            MaintenanceTask {
                task_id: 5501,
                equipment_id: "HPLC-AG1260-SN22019".to_string(),
                equipment_name: "Agilent 1260 Infinity II HPLC".to_string(),
                task_description: "Replace pump seals and check valves".to_string(),
                frequency_days: 365,
                last_performed: "2025-06-15".to_string(),
                next_due: "2026-06-15".to_string(),
                performed_by: "Service Engineer T. Watanabe".to_string(),
                cost_usd: 1250.0,
                is_overdue: false,
            },
            MaintenanceTask {
                task_id: 5502,
                equipment_id: "GCMS-SH2020-SN89001".to_string(),
                equipment_name: "Shimadzu GCMS-QP2020 NX".to_string(),
                task_description: "Source cleaning and tuning".to_string(),
                frequency_days: 180,
                last_performed: "2025-10-01".to_string(),
                next_due: "2026-03-30".to_string(),
                performed_by: "Instrument Specialist H. Kimura".to_string(),
                cost_usd: 450.0,
                is_overdue: false,
            },
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&schedule, cfg).expect("encode maintenance schedule");
    let (decoded, _): (MaintenanceSchedule, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode maintenance schedule");
    assert_eq!(schedule, decoded);
}

#[test]
fn test_lims_record_with_results_roundtrip() {
    let record = LimsRecord {
        record_id: 200100,
        sample_id: "PHARMA-QC-2026-0315-A".to_string(),
        project_code: "PRJ-TABLET-DISSOLUTION".to_string(),
        submission_date: "2026-03-15".to_string(),
        results: vec![
            LimsTestResult {
                analyte: "Active Ingredient A".to_string(),
                method: "USP <711> Dissolution".to_string(),
                value: 98.7,
                unit: "%".to_string(),
                lower_limit: 80.0,
                upper_limit: 110.0,
                within_spec: true,
            },
            LimsTestResult {
                analyte: "Impurity B".to_string(),
                method: "HPLC-UV Related Substances".to_string(),
                value: 0.12,
                unit: "%".to_string(),
                lower_limit: 0.0,
                upper_limit: 0.5,
                within_spec: true,
            },
            LimsTestResult {
                analyte: "Water Content".to_string(),
                method: "Karl Fischer Titration".to_string(),
                value: 2.3,
                unit: "%w/w".to_string(),
                lower_limit: 0.0,
                upper_limit: 5.0,
                within_spec: true,
            },
        ],
        approved_by: Some("QC Manager S. Kobayashi".to_string()),
        status: "Approved".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&record, cfg).expect("encode LIMS record");
    let (decoded, _): (LimsRecord, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode LIMS record");
    assert_eq!(record, decoded);
}

#[test]
fn test_lims_record_pending_approval() {
    let record = LimsRecord {
        record_id: 200101,
        sample_id: "ENV-SOIL-2026-0314-C".to_string(),
        project_code: "PRJ-SITE-REMEDIATION".to_string(),
        submission_date: "2026-03-14".to_string(),
        results: vec![LimsTestResult {
            analyte: "Lead (Pb)".to_string(),
            method: "EPA 6010D ICP-OES".to_string(),
            value: 245.0,
            unit: "mg/kg".to_string(),
            lower_limit: 0.0,
            upper_limit: 400.0,
            within_spec: true,
        }],
        approved_by: None,
        status: "Pending Review".to_string(),
    };
    let encoded = encode_to_vec(&record, config::standard()).expect("encode pending LIMS");
    let (decoded, _): (LimsRecord, usize) =
        decode_owned_from_slice(&encoded, config::standard()).expect("decode pending LIMS");
    assert_eq!(record, decoded);
}

#[test]
fn test_mass_spec_empty_peaks() {
    let reading = MassSpecReading {
        scan_id: 48300,
        scan_time_sec: 0.5,
        ionization_mode: "APCI-".to_string(),
        peaks: vec![],
        total_ion_current: 0.0,
        base_peak_mz: 0.0,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&reading, cfg).expect("encode empty mass spec");
    let (decoded, _): (MassSpecReading, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode empty mass spec");
    assert_eq!(reading, decoded);
}

#[test]
fn test_ph_calibration_failed_record() {
    let record = PhCalibrationRecord {
        record_id: 4502,
        electrode_serial: "ORION-9102BNWP-SN44900".to_string(),
        calibration_date: "2026-03-14".to_string(),
        points: vec![
            PhCalibrationPoint {
                buffer_ph: 4.01,
                measured_mv: 165.0,
                temperature_c: 25.0,
            },
            PhCalibrationPoint {
                buffer_ph: 7.00,
                measured_mv: 12.5,
                temperature_c: 25.0,
            },
        ],
        slope_mv_per_ph: -50.92,
        offset_mv: 12.5,
        r_squared: 0.9812,
        passed: false,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&record, cfg).expect("encode failed pH cal");
    let (decoded, _): (PhCalibrationRecord, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode failed pH cal");
    assert_eq!(record, decoded);
}

#[test]
fn test_autoclave_prevac_cycle() {
    let cycle = AutoclaveCycle {
        cycle_id: 11250,
        cycle_type: "Pre-Vacuum".to_string(),
        load_description: "Porous loads, wrapped textiles and gowns".to_string(),
        phases: vec![
            AutoclaveCyclePhase {
                phase_name: "Pre-Vacuum Pulse 1".to_string(),
                target_temperature_c: 80.0,
                target_pressure_kpa: 30.0,
                duration_min: 3,
            },
            AutoclaveCyclePhase {
                phase_name: "Pre-Vacuum Pulse 2".to_string(),
                target_temperature_c: 80.0,
                target_pressure_kpa: 30.0,
                duration_min: 3,
            },
            AutoclaveCyclePhase {
                phase_name: "Pre-Vacuum Pulse 3".to_string(),
                target_temperature_c: 80.0,
                target_pressure_kpa: 30.0,
                duration_min: 3,
            },
            AutoclaveCyclePhase {
                phase_name: "Sterilization".to_string(),
                target_temperature_c: 134.0,
                target_pressure_kpa: 304.0,
                duration_min: 4,
            },
            AutoclaveCyclePhase {
                phase_name: "Post-Vacuum Dry".to_string(),
                target_temperature_c: 90.0,
                target_pressure_kpa: 10.0,
                duration_min: 30,
            },
        ],
        biological_indicator_passed: true,
        chemical_indicator_passed: true,
        operator: "N. Sato".to_string(),
    };
    let encoded = encode_to_vec(&cycle, config::standard()).expect("encode prevac autoclave");
    let (decoded, _): (AutoclaveCycle, usize) =
        decode_owned_from_slice(&encoded, config::standard()).expect("decode prevac autoclave");
    assert_eq!(cycle, decoded);
}
