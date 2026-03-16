//! Advanced nested struct encoding tests for OxiCode (set 10)
//! Theme: Neuroscience and Brain-Computer Interfaces

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

// ── Level-1 primitives ──────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Coordinate3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Impedance {
    magnitude_kohm: f64,
    phase_deg: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TimeWindow {
    start_ms: f64,
    end_ms: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FrequencyBand {
    name: String,
    low_hz: f64,
    high_hz: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DoseLevel {
    compound: String,
    concentration_um: f64,
    volume_ml: f64,
}

// ── Level-2 building blocks ─────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EegElectrode {
    label: String,
    position: Coordinate3D,
    impedance: Impedance,
    reference_channel: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EegMontage {
    name: String,
    electrodes: Vec<EegElectrode>,
    sampling_rate_hz: u32,
    band_filters: Vec<FrequencyBand>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpikeWaveform {
    template_id: u32,
    samples: Vec<f64>,
    peak_amplitude_uv: f64,
    trough_to_peak_ms: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpikeCluster {
    cluster_id: u32,
    neuron_type: String,
    waveform: SpikeWaveform,
    mean_firing_rate_hz: f64,
    isolation_distance: f64,
    spike_times_ms: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpikeSortingResult {
    session_id: String,
    electrode_label: String,
    clusters: Vec<SpikeCluster>,
    noise_floor_uv: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Voxel {
    position: Coordinate3D,
    bold_signal: f64,
    t_statistic: f64,
    p_value: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BrodmannArea {
    area_number: u16,
    name: String,
    hemisphere: String,
    centroid: Coordinate3D,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FmriBoldMap {
    subject_id: String,
    contrast: String,
    active_voxels: Vec<Voxel>,
    regions: Vec<BrodmannArea>,
    tr_ms: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MegSensor {
    sensor_id: u32,
    sensor_type: String,
    position: Coordinate3D,
    orientation: Coordinate3D,
    noise_floor_ft: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MegSensorArray {
    system_name: String,
    sensors: Vec<MegSensor>,
    sampling_rate_hz: u32,
    head_position: Coordinate3D,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StimCoilPosition {
    coil_type: String,
    center: Coordinate3D,
    normal_vector: Coordinate3D,
    angle_deg: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TranscranialStimParams {
    modality: String,
    coil: StimCoilPosition,
    intensity_ma: f64,
    frequency_hz: Option<f64>,
    duration_s: f64,
    target_region: BrodmannArea,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct KalmanFilterState {
    state_dim: u32,
    state_estimate: Vec<f64>,
    covariance_diag: Vec<f64>,
    process_noise: Vec<f64>,
    measurement_noise: Vec<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NeuralNetLayer {
    layer_name: String,
    input_dim: u32,
    output_dim: u32,
    weights_flat: Vec<f64>,
    bias: Vec<f64>,
    activation: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BciDecoderModel {
    model_id: String,
    decoder_type: String,
    kalman_state: Option<KalmanFilterState>,
    nn_layers: Vec<NeuralNetLayer>,
    input_channels: Vec<String>,
    output_dimensions: u32,
    decode_interval_ms: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProstheticDof {
    joint_name: String,
    min_angle_deg: f64,
    max_angle_deg: f64,
    current_angle_deg: f64,
    velocity_deg_s: f64,
    torque_nm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NeuroprostheticControl {
    device_id: String,
    degrees_of_freedom: Vec<ProstheticDof>,
    decoder: BciDecoderModel,
    latency_ms: f64,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MicroelectrodeShank {
    shank_id: u32,
    contact_positions: Vec<Coordinate3D>,
    impedances: Vec<Impedance>,
    contact_area_um2: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IntracorticalArray {
    array_name: String,
    manufacturer: String,
    shanks: Vec<MicroelectrodeShank>,
    insertion_depth_mm: f64,
    target_region: BrodmannArea,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OptogeneticsOpsin {
    opsin_name: String,
    excitation_wavelength_nm: u16,
    tau_on_ms: f64,
    tau_off_ms: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OptogeneticsStimulation {
    experiment_id: String,
    opsin: OptogeneticsOpsin,
    fiber_position: Coordinate3D,
    power_mw: f64,
    pulse_width_ms: f64,
    frequency_hz: f64,
    target_region: BrodmannArea,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConnectomeEdge {
    source_region: u16,
    target_region: u16,
    weight: f64,
    tract_name: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConnectomeMatrix {
    atlas_name: String,
    regions: Vec<BrodmannArea>,
    edges: Vec<ConnectomeEdge>,
    total_streamlines: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SleepStage {
    stage_label: String,
    start_epoch: u32,
    end_epoch: u32,
    dominant_frequency: FrequencyBand,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PolysomnographySession {
    subject_id: String,
    montage: EegMontage,
    stages: Vec<SleepStage>,
    total_epochs: u32,
    epoch_duration_s: f64,
    apnea_events: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CognitiveEvent {
    event_code: u16,
    label: String,
    onset_ms: f64,
    duration_ms: f64,
    response_correct: bool,
    reaction_time_ms: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CognitiveTaskDesign {
    task_name: String,
    block_id: u32,
    events: Vec<CognitiveEvent>,
    montage: EegMontage,
    erp_windows: Vec<TimeWindow>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DoseResponseCurve {
    compound_name: String,
    doses: Vec<DoseLevel>,
    responses: Vec<f64>,
    ec50_um: f64,
    hill_coefficient: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NeuropharmStudy {
    study_id: String,
    target_receptor: String,
    curves: Vec<DoseResponseCurve>,
    brain_region: BrodmannArea,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TelemetryPacket {
    timestamp_us: u64,
    battery_pct: f64,
    temperature_c: f64,
    signal_quality: f64,
    error_flags: Vec<u16>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImplantTelemetry {
    implant_id: String,
    array: IntracorticalArray,
    packets: Vec<TelemetryPacket>,
    uptime_hours: f64,
    firmware_version: String,
}

// ── Helpers ─────────────────────────────────────────────────────────

fn coord(x: f64, y: f64, z: f64) -> Coordinate3D {
    Coordinate3D { x, y, z }
}

fn impedance(mag: f64, phase: f64) -> Impedance {
    Impedance {
        magnitude_kohm: mag,
        phase_deg: phase,
    }
}

fn brodmann(num: u16, name: &str, hemi: &str, cx: f64, cy: f64, cz: f64) -> BrodmannArea {
    BrodmannArea {
        area_number: num,
        name: name.to_string(),
        hemisphere: hemi.to_string(),
        centroid: coord(cx, cy, cz),
    }
}

fn freq_band(name: &str, lo: f64, hi: f64) -> FrequencyBand {
    FrequencyBand {
        name: name.to_string(),
        low_hz: lo,
        high_hz: hi,
    }
}

fn make_electrode(label: &str, x: f64, y: f64, z: f64, imp_k: f64) -> EegElectrode {
    EegElectrode {
        label: label.to_string(),
        position: coord(x, y, z),
        impedance: impedance(imp_k, -15.0),
        reference_channel: None,
    }
}

fn make_montage() -> EegMontage {
    EegMontage {
        name: "10-20".to_string(),
        electrodes: vec![
            make_electrode("Fp1", -3.0, 10.5, 0.0, 5.2),
            make_electrode("Fp2", 3.0, 10.5, 0.0, 4.8),
            make_electrode("Cz", 0.0, 0.0, 12.0, 3.1),
            make_electrode("O1", -3.0, -10.0, 0.0, 6.0),
        ],
        sampling_rate_hz: 256,
        band_filters: vec![
            freq_band("delta", 0.5, 4.0),
            freq_band("theta", 4.0, 8.0),
            freq_band("alpha", 8.0, 13.0),
            freq_band("beta", 13.0, 30.0),
        ],
    }
}

// ── Tests ───────────────────────────────────────────────────────────

// Test 1: EEG electrode montage roundtrip
#[test]
fn test_eeg_montage_roundtrip() {
    let montage = make_montage();
    let bytes = encode_to_vec(&montage).expect("encode EEG montage");
    let (dec, _): (EegMontage, usize) = decode_from_slice(&bytes).expect("decode EEG montage");
    assert_eq!(montage, dec);
}

// Test 2: Spike sorting result with multiple clusters
#[test]
fn test_spike_sorting_result_roundtrip() {
    let result = SpikeSortingResult {
        session_id: "sess_20260315_001".to_string(),
        electrode_label: "tetrode_A1".to_string(),
        clusters: vec![
            SpikeCluster {
                cluster_id: 1,
                neuron_type: "pyramidal".to_string(),
                waveform: SpikeWaveform {
                    template_id: 101,
                    samples: vec![-20.0, -80.5, -120.3, -60.0, 15.0, 40.2, 20.0, 5.0],
                    peak_amplitude_uv: 120.3,
                    trough_to_peak_ms: 0.45,
                },
                mean_firing_rate_hz: 8.3,
                isolation_distance: 22.5,
                spike_times_ms: vec![100.5, 230.1, 445.9, 678.2],
            },
            SpikeCluster {
                cluster_id: 2,
                neuron_type: "interneuron".to_string(),
                waveform: SpikeWaveform {
                    template_id: 102,
                    samples: vec![-15.0, -55.0, -90.0, -40.0, 10.0, 25.0, 12.0, 3.0],
                    peak_amplitude_uv: 90.0,
                    trough_to_peak_ms: 0.25,
                },
                mean_firing_rate_hz: 35.7,
                isolation_distance: 18.1,
                spike_times_ms: vec![50.2, 78.5, 110.3, 145.0, 180.9],
            },
        ],
        noise_floor_uv: 12.5,
    };
    let bytes = encode_to_vec(&result).expect("encode spike sorting");
    let (dec, _): (SpikeSortingResult, usize) =
        decode_from_slice(&bytes).expect("decode spike sorting");
    assert_eq!(result, dec);
}

// Test 3: fMRI BOLD map with voxels and Brodmann areas
#[test]
fn test_fmri_bold_map_roundtrip() {
    let bold = FmriBoldMap {
        subject_id: "sub-01".to_string(),
        contrast: "faces_vs_houses".to_string(),
        active_voxels: vec![
            Voxel {
                position: coord(45.0, -62.0, 10.0),
                bold_signal: 2.8,
                t_statistic: 5.2,
                p_value: 0.00001,
            },
            Voxel {
                position: coord(46.0, -60.0, 12.0),
                bold_signal: 3.1,
                t_statistic: 6.0,
                p_value: 0.000005,
            },
            Voxel {
                position: coord(-44.0, -63.0, 9.0),
                bold_signal: 2.5,
                t_statistic: 4.8,
                p_value: 0.00003,
            },
        ],
        regions: vec![
            brodmann(37, "fusiform gyrus", "right", 45.0, -61.0, 11.0),
            brodmann(37, "fusiform gyrus", "left", -44.0, -61.0, 10.0),
        ],
        tr_ms: 2000.0,
    };
    let bytes = encode_to_vec(&bold).expect("encode fMRI BOLD map");
    let (dec, _): (FmriBoldMap, usize) = decode_from_slice(&bytes).expect("decode fMRI BOLD map");
    assert_eq!(bold, dec);
}

// Test 4: MEG sensor array
#[test]
fn test_meg_sensor_array_roundtrip() {
    let meg = MegSensorArray {
        system_name: "Elekta Triux".to_string(),
        sensors: vec![
            MegSensor {
                sensor_id: 1,
                sensor_type: "magnetometer".to_string(),
                position: coord(5.0, 8.0, 12.0),
                orientation: coord(0.0, 0.0, 1.0),
                noise_floor_ft: 3.5,
            },
            MegSensor {
                sensor_id: 2,
                sensor_type: "gradiometer".to_string(),
                position: coord(5.5, 8.2, 11.8),
                orientation: coord(0.1, 0.0, 0.99),
                noise_floor_ft: 2.8,
            },
        ],
        sampling_rate_hz: 1000,
        head_position: coord(0.0, 0.0, 5.0),
    };
    let bytes = encode_to_vec(&meg).expect("encode MEG array");
    let (dec, _): (MegSensorArray, usize) = decode_from_slice(&bytes).expect("decode MEG array");
    assert_eq!(meg, dec);
}

// Test 5: Transcranial stimulation parameters (TMS)
#[test]
fn test_transcranial_stim_tms_roundtrip() {
    let stim = TranscranialStimParams {
        modality: "TMS".to_string(),
        coil: StimCoilPosition {
            coil_type: "figure-of-eight".to_string(),
            center: coord(-4.0, 6.0, 14.0),
            normal_vector: coord(0.0, -0.3, 0.95),
            angle_deg: 45.0,
        },
        intensity_ma: 1.8,
        frequency_hz: Some(10.0),
        duration_s: 600.0,
        target_region: brodmann(46, "DLPFC", "left", -4.0, 5.5, 13.0),
    };
    let bytes = encode_to_vec(&stim).expect("encode TMS params");
    let (dec, _): (TranscranialStimParams, usize) =
        decode_from_slice(&bytes).expect("decode TMS params");
    assert_eq!(stim, dec);
}

// Test 6: Transcranial stimulation (tDCS) with no frequency
#[test]
fn test_transcranial_stim_tdcs_roundtrip() {
    let stim = TranscranialStimParams {
        modality: "tDCS".to_string(),
        coil: StimCoilPosition {
            coil_type: "sponge_5x7cm".to_string(),
            center: coord(4.0, 6.0, 14.0),
            normal_vector: coord(0.0, 0.0, 1.0),
            angle_deg: 0.0,
        },
        intensity_ma: 2.0,
        frequency_hz: None,
        duration_s: 1200.0,
        target_region: brodmann(9, "DLPFC", "right", 4.0, 5.5, 13.5),
    };
    let bytes = encode_to_vec(&stim).expect("encode tDCS params");
    let (dec, _): (TranscranialStimParams, usize) =
        decode_from_slice(&bytes).expect("decode tDCS params");
    assert_eq!(stim, dec);
}

// Test 7: BCI decoder with Kalman filter only
#[test]
fn test_bci_decoder_kalman_roundtrip() {
    let decoder = BciDecoderModel {
        model_id: "kalman_v3".to_string(),
        decoder_type: "kalman".to_string(),
        kalman_state: Some(KalmanFilterState {
            state_dim: 4,
            state_estimate: vec![0.1, -0.05, 0.3, 0.0],
            covariance_diag: vec![0.01, 0.01, 0.02, 0.005],
            process_noise: vec![0.001, 0.001, 0.002, 0.001],
            measurement_noise: vec![0.05, 0.05, 0.05, 0.05],
        }),
        nn_layers: vec![],
        input_channels: vec![
            "M1_ch1".to_string(),
            "M1_ch2".to_string(),
            "M1_ch3".to_string(),
            "M1_ch4".to_string(),
        ],
        output_dimensions: 2,
        decode_interval_ms: 50.0,
    };
    let bytes = encode_to_vec(&decoder).expect("encode Kalman BCI");
    let (dec, _): (BciDecoderModel, usize) = decode_from_slice(&bytes).expect("decode Kalman BCI");
    assert_eq!(decoder, dec);
}

// Test 8: BCI decoder with neural network layers
#[test]
fn test_bci_decoder_nn_roundtrip() {
    let decoder = BciDecoderModel {
        model_id: "nn_v2".to_string(),
        decoder_type: "feedforward_nn".to_string(),
        kalman_state: None,
        nn_layers: vec![
            NeuralNetLayer {
                layer_name: "hidden1".to_string(),
                input_dim: 96,
                output_dim: 64,
                weights_flat: vec![0.01; 16],
                bias: vec![0.0; 4],
                activation: "relu".to_string(),
            },
            NeuralNetLayer {
                layer_name: "hidden2".to_string(),
                input_dim: 64,
                output_dim: 32,
                weights_flat: vec![-0.02; 8],
                bias: vec![0.1; 4],
                activation: "relu".to_string(),
            },
            NeuralNetLayer {
                layer_name: "output".to_string(),
                input_dim: 32,
                output_dim: 3,
                weights_flat: vec![0.05; 6],
                bias: vec![0.0; 3],
                activation: "linear".to_string(),
            },
        ],
        input_channels: vec!["PMd_01".to_string(), "PMd_02".to_string()],
        output_dimensions: 3,
        decode_interval_ms: 20.0,
    };
    let bytes = encode_to_vec(&decoder).expect("encode NN BCI");
    let (dec, _): (BciDecoderModel, usize) = decode_from_slice(&bytes).expect("decode NN BCI");
    assert_eq!(decoder, dec);
}

// Test 9: Neuroprosthetic control with full decoder
#[test]
fn test_neuroprosthetic_control_roundtrip() {
    let ctrl = NeuroprostheticControl {
        device_id: "hand_prosthesis_v4".to_string(),
        degrees_of_freedom: vec![
            ProstheticDof {
                joint_name: "wrist_flexion".to_string(),
                min_angle_deg: -60.0,
                max_angle_deg: 60.0,
                current_angle_deg: 15.0,
                velocity_deg_s: 5.2,
                torque_nm: 0.8,
            },
            ProstheticDof {
                joint_name: "index_mcp".to_string(),
                min_angle_deg: 0.0,
                max_angle_deg: 90.0,
                current_angle_deg: 45.0,
                velocity_deg_s: -10.0,
                torque_nm: 0.3,
            },
            ProstheticDof {
                joint_name: "thumb_opposition".to_string(),
                min_angle_deg: 0.0,
                max_angle_deg: 80.0,
                current_angle_deg: 30.0,
                velocity_deg_s: 2.1,
                torque_nm: 0.5,
            },
        ],
        decoder: BciDecoderModel {
            model_id: "prosthetic_kalman_v1".to_string(),
            decoder_type: "kalman".to_string(),
            kalman_state: Some(KalmanFilterState {
                state_dim: 6,
                state_estimate: vec![15.0, 45.0, 30.0, 5.2, -10.0, 2.1],
                covariance_diag: vec![1.0; 6],
                process_noise: vec![0.1; 6],
                measurement_noise: vec![0.5; 6],
            }),
            nn_layers: vec![],
            input_channels: vec!["M1_array".to_string()],
            output_dimensions: 3,
            decode_interval_ms: 30.0,
        },
        latency_ms: 12.5,
        active: true,
    };
    let bytes = encode_to_vec(&ctrl).expect("encode neuroprosthetic");
    let (dec, _): (NeuroprostheticControl, usize) =
        decode_from_slice(&bytes).expect("decode neuroprosthetic");
    assert_eq!(ctrl, dec);
}

// Test 10: Intracortical microelectrode array
#[test]
fn test_intracortical_array_roundtrip() {
    let array = IntracorticalArray {
        array_name: "Utah_96ch".to_string(),
        manufacturer: "Blackrock".to_string(),
        shanks: vec![
            MicroelectrodeShank {
                shank_id: 1,
                contact_positions: vec![
                    coord(0.0, 0.0, 0.0),
                    coord(0.0, 0.0, -0.4),
                    coord(0.0, 0.0, -0.8),
                    coord(0.0, 0.0, -1.2),
                ],
                impedances: vec![
                    impedance(150.0, -12.0),
                    impedance(180.0, -14.0),
                    impedance(200.0, -10.0),
                    impedance(160.0, -13.0),
                ],
                contact_area_um2: 1250.0,
            },
            MicroelectrodeShank {
                shank_id: 2,
                contact_positions: vec![coord(0.4, 0.0, 0.0), coord(0.4, 0.0, -0.4)],
                impedances: vec![impedance(170.0, -11.0), impedance(190.0, -15.0)],
                contact_area_um2: 1250.0,
            },
        ],
        insertion_depth_mm: 1.5,
        target_region: brodmann(4, "primary motor cortex", "left", -35.0, -20.0, 60.0),
    };
    let bytes = encode_to_vec(&array).expect("encode intracortical array");
    let (dec, _): (IntracorticalArray, usize) =
        decode_from_slice(&bytes).expect("decode intracortical array");
    assert_eq!(array, dec);
}

// Test 11: Optogenetics stimulation parameters
#[test]
fn test_optogenetics_stimulation_roundtrip() {
    let opto = OptogeneticsStimulation {
        experiment_id: "opto_2026_exp42".to_string(),
        opsin: OptogeneticsOpsin {
            opsin_name: "ChR2-H134R".to_string(),
            excitation_wavelength_nm: 470,
            tau_on_ms: 1.2,
            tau_off_ms: 10.5,
        },
        fiber_position: coord(-2.5, 1.8, -3.0),
        power_mw: 5.0,
        pulse_width_ms: 10.0,
        frequency_hz: 20.0,
        target_region: brodmann(24, "anterior cingulate", "left", -2.0, 2.0, -2.5),
    };
    let bytes = encode_to_vec(&opto).expect("encode optogenetics");
    let (dec, _): (OptogeneticsStimulation, usize) =
        decode_from_slice(&bytes).expect("decode optogenetics");
    assert_eq!(opto, dec);
}

// Test 12: Connectome adjacency matrix
#[test]
fn test_connectome_matrix_roundtrip() {
    let connectome = ConnectomeMatrix {
        atlas_name: "Desikan-Killiany".to_string(),
        regions: vec![
            brodmann(4, "precentral", "left", -35.0, -20.0, 60.0),
            brodmann(6, "premotor", "left", -30.0, -5.0, 55.0),
            brodmann(17, "V1", "left", -10.0, -90.0, 5.0),
        ],
        edges: vec![
            ConnectomeEdge {
                source_region: 4,
                target_region: 6,
                weight: 0.85,
                tract_name: "SLF_I".to_string(),
            },
            ConnectomeEdge {
                source_region: 6,
                target_region: 4,
                weight: 0.82,
                tract_name: "SLF_I".to_string(),
            },
            ConnectomeEdge {
                source_region: 17,
                target_region: 4,
                weight: 0.15,
                tract_name: "IFOF".to_string(),
            },
        ],
        total_streamlines: 1_250_000,
    };
    let bytes = encode_to_vec(&connectome).expect("encode connectome");
    let (dec, _): (ConnectomeMatrix, usize) = decode_from_slice(&bytes).expect("decode connectome");
    assert_eq!(connectome, dec);
}

// Test 13: Sleep scoring polysomnography
#[test]
fn test_polysomnography_session_roundtrip() {
    let psg = PolysomnographySession {
        subject_id: "sleep_sub_07".to_string(),
        montage: EegMontage {
            name: "sleep_montage".to_string(),
            electrodes: vec![
                make_electrode("F3", -4.0, 7.0, 8.0, 4.5),
                make_electrode("F4", 4.0, 7.0, 8.0, 5.0),
                make_electrode("C3", -6.0, 0.0, 12.0, 3.8),
                make_electrode("C4", 6.0, 0.0, 12.0, 4.1),
                make_electrode("O1", -3.0, -10.0, 0.0, 5.5),
                make_electrode("O2", 3.0, -10.0, 0.0, 5.2),
            ],
            sampling_rate_hz: 512,
            band_filters: vec![
                freq_band("delta", 0.5, 4.0),
                freq_band("theta", 4.0, 8.0),
                freq_band("sigma", 11.0, 16.0),
            ],
        },
        stages: vec![
            SleepStage {
                stage_label: "W".to_string(),
                start_epoch: 0,
                end_epoch: 15,
                dominant_frequency: freq_band("alpha", 8.0, 13.0),
            },
            SleepStage {
                stage_label: "N1".to_string(),
                start_epoch: 16,
                end_epoch: 30,
                dominant_frequency: freq_band("theta", 4.0, 8.0),
            },
            SleepStage {
                stage_label: "N2".to_string(),
                start_epoch: 31,
                end_epoch: 120,
                dominant_frequency: freq_band("sigma", 11.0, 16.0),
            },
            SleepStage {
                stage_label: "N3".to_string(),
                start_epoch: 121,
                end_epoch: 200,
                dominant_frequency: freq_band("delta", 0.5, 4.0),
            },
            SleepStage {
                stage_label: "REM".to_string(),
                start_epoch: 201,
                end_epoch: 260,
                dominant_frequency: freq_band("theta", 4.0, 8.0),
            },
        ],
        total_epochs: 960,
        epoch_duration_s: 30.0,
        apnea_events: 3,
    };
    let bytes = encode_to_vec(&psg).expect("encode polysomnography");
    let (dec, _): (PolysomnographySession, usize) =
        decode_from_slice(&bytes).expect("decode polysomnography");
    assert_eq!(psg, dec);
}

// Test 14: Cognitive task event markers
#[test]
fn test_cognitive_task_design_roundtrip() {
    let task = CognitiveTaskDesign {
        task_name: "Stroop_Color_Word".to_string(),
        block_id: 3,
        events: vec![
            CognitiveEvent {
                event_code: 10,
                label: "congruent".to_string(),
                onset_ms: 0.0,
                duration_ms: 500.0,
                response_correct: true,
                reaction_time_ms: Some(420.5),
            },
            CognitiveEvent {
                event_code: 20,
                label: "incongruent".to_string(),
                onset_ms: 1500.0,
                duration_ms: 500.0,
                response_correct: false,
                reaction_time_ms: Some(780.2),
            },
            CognitiveEvent {
                event_code: 30,
                label: "neutral".to_string(),
                onset_ms: 3000.0,
                duration_ms: 500.0,
                response_correct: true,
                reaction_time_ms: None,
            },
        ],
        montage: make_montage(),
        erp_windows: vec![
            TimeWindow {
                start_ms: -200.0,
                end_ms: 0.0,
            },
            TimeWindow {
                start_ms: 250.0,
                end_ms: 500.0,
            },
            TimeWindow {
                start_ms: 300.0,
                end_ms: 600.0,
            },
        ],
    };
    let bytes = encode_to_vec(&task).expect("encode cognitive task");
    let (dec, _): (CognitiveTaskDesign, usize) =
        decode_from_slice(&bytes).expect("decode cognitive task");
    assert_eq!(task, dec);
}

// Test 15: Neuropharmacology dose-response study
#[test]
fn test_neuropharm_study_roundtrip() {
    let study = NeuropharmStudy {
        study_id: "pharma_2026_gaba".to_string(),
        target_receptor: "GABA_A".to_string(),
        curves: vec![
            DoseResponseCurve {
                compound_name: "diazepam".to_string(),
                doses: vec![
                    DoseLevel {
                        compound: "diazepam".to_string(),
                        concentration_um: 0.01,
                        volume_ml: 0.1,
                    },
                    DoseLevel {
                        compound: "diazepam".to_string(),
                        concentration_um: 0.1,
                        volume_ml: 0.1,
                    },
                    DoseLevel {
                        compound: "diazepam".to_string(),
                        concentration_um: 1.0,
                        volume_ml: 0.1,
                    },
                    DoseLevel {
                        compound: "diazepam".to_string(),
                        concentration_um: 10.0,
                        volume_ml: 0.1,
                    },
                ],
                responses: vec![5.0, 25.0, 72.0, 95.0],
                ec50_um: 0.35,
                hill_coefficient: 1.2,
            },
            DoseResponseCurve {
                compound_name: "muscimol".to_string(),
                doses: vec![
                    DoseLevel {
                        compound: "muscimol".to_string(),
                        concentration_um: 0.001,
                        volume_ml: 0.1,
                    },
                    DoseLevel {
                        compound: "muscimol".to_string(),
                        concentration_um: 0.01,
                        volume_ml: 0.1,
                    },
                    DoseLevel {
                        compound: "muscimol".to_string(),
                        concentration_um: 0.1,
                        volume_ml: 0.1,
                    },
                ],
                responses: vec![10.0, 55.0, 98.0],
                ec50_um: 0.008,
                hill_coefficient: 1.8,
            },
        ],
        brain_region: brodmann(10, "anterior prefrontal", "right", 8.0, 65.0, 5.0),
    };
    let bytes = encode_to_vec(&study).expect("encode neuropharm study");
    let (dec, _): (NeuropharmStudy, usize) =
        decode_from_slice(&bytes).expect("decode neuropharm study");
    assert_eq!(study, dec);
}

// Test 16: Neural implant telemetry
#[test]
fn test_implant_telemetry_roundtrip() {
    let telemetry = ImplantTelemetry {
        implant_id: "NI_2026_patient12".to_string(),
        array: IntracorticalArray {
            array_name: "Neuropixels_2.0".to_string(),
            manufacturer: "IMEC".to_string(),
            shanks: vec![MicroelectrodeShank {
                shank_id: 1,
                contact_positions: vec![
                    coord(0.0, 0.0, 0.0),
                    coord(0.0, 0.0, -0.02),
                    coord(0.0, 0.0, -0.04),
                ],
                impedances: vec![
                    impedance(50.0, -8.0),
                    impedance(55.0, -9.0),
                    impedance(48.0, -7.5),
                ],
                contact_area_um2: 144.0,
            }],
            insertion_depth_mm: 4.0,
            target_region: brodmann(4, "M1 hand knob", "left", -37.0, -22.0, 58.0),
        },
        packets: vec![
            TelemetryPacket {
                timestamp_us: 1_000_000,
                battery_pct: 85.0,
                temperature_c: 37.1,
                signal_quality: 0.95,
                error_flags: vec![],
            },
            TelemetryPacket {
                timestamp_us: 2_000_000,
                battery_pct: 84.8,
                temperature_c: 37.2,
                signal_quality: 0.93,
                error_flags: vec![0x0010],
            },
        ],
        uptime_hours: 720.5,
        firmware_version: "3.2.1".to_string(),
    };
    let bytes = encode_to_vec(&telemetry).expect("encode implant telemetry");
    let (dec, _): (ImplantTelemetry, usize) =
        decode_from_slice(&bytes).expect("decode implant telemetry");
    assert_eq!(telemetry, dec);
}

// Test 17: Empty spike sorting (no clusters found)
#[test]
fn test_spike_sorting_empty_clusters_roundtrip() {
    let result = SpikeSortingResult {
        session_id: "sess_empty".to_string(),
        electrode_label: "ch_64".to_string(),
        clusters: vec![],
        noise_floor_uv: 25.0,
    };
    let bytes = encode_to_vec(&result).expect("encode empty spike sorting");
    let (dec, _): (SpikeSortingResult, usize) =
        decode_from_slice(&bytes).expect("decode empty spike sorting");
    assert_eq!(result, dec);
}

// Test 18: Electrode with reference channel
#[test]
fn test_eeg_electrode_with_reference_roundtrip() {
    let montage = EegMontage {
        name: "bipolar_temporal".to_string(),
        electrodes: vec![
            EegElectrode {
                label: "T3".to_string(),
                position: coord(-7.0, 0.0, 3.0),
                impedance: impedance(4.0, -12.0),
                reference_channel: Some("T5".to_string()),
            },
            EegElectrode {
                label: "T4".to_string(),
                position: coord(7.0, 0.0, 3.0),
                impedance: impedance(3.8, -11.0),
                reference_channel: Some("T6".to_string()),
            },
        ],
        sampling_rate_hz: 1024,
        band_filters: vec![freq_band("gamma", 30.0, 100.0)],
    };
    let bytes = encode_to_vec(&montage).expect("encode bipolar montage");
    let (dec, _): (EegMontage, usize) = decode_from_slice(&bytes).expect("decode bipolar montage");
    assert_eq!(montage, dec);
}

// Test 19: Connectome with empty edges
#[test]
fn test_connectome_empty_edges_roundtrip() {
    let connectome = ConnectomeMatrix {
        atlas_name: "empty_test".to_string(),
        regions: vec![brodmann(1, "somatosensory", "left", -40.0, -30.0, 55.0)],
        edges: vec![],
        total_streamlines: 0,
    };
    let bytes = encode_to_vec(&connectome).expect("encode empty connectome");
    let (dec, _): (ConnectomeMatrix, usize) =
        decode_from_slice(&bytes).expect("decode empty connectome");
    assert_eq!(connectome, dec);
}

// Test 20: Neuroprosthetic control (inactive)
#[test]
fn test_neuroprosthetic_inactive_roundtrip() {
    let ctrl = NeuroprostheticControl {
        device_id: "arm_exo_v1".to_string(),
        degrees_of_freedom: vec![ProstheticDof {
            joint_name: "elbow_flexion".to_string(),
            min_angle_deg: 0.0,
            max_angle_deg: 150.0,
            current_angle_deg: 0.0,
            velocity_deg_s: 0.0,
            torque_nm: 0.0,
        }],
        decoder: BciDecoderModel {
            model_id: "standby".to_string(),
            decoder_type: "none".to_string(),
            kalman_state: None,
            nn_layers: vec![],
            input_channels: vec![],
            output_dimensions: 0,
            decode_interval_ms: 0.0,
        },
        latency_ms: 0.0,
        active: false,
    };
    let bytes = encode_to_vec(&ctrl).expect("encode inactive prosthetic");
    let (dec, _): (NeuroprostheticControl, usize) =
        decode_from_slice(&bytes).expect("decode inactive prosthetic");
    assert_eq!(ctrl, dec);
}

// Test 21: Optogenetics with inhibitory opsin
#[test]
fn test_optogenetics_inhibitory_roundtrip() {
    let opto = OptogeneticsStimulation {
        experiment_id: "opto_inhibit_01".to_string(),
        opsin: OptogeneticsOpsin {
            opsin_name: "eNpHR3.0".to_string(),
            excitation_wavelength_nm: 590,
            tau_on_ms: 4.2,
            tau_off_ms: 45.0,
        },
        fiber_position: coord(1.5, -3.2, -5.0),
        power_mw: 8.0,
        pulse_width_ms: 5000.0,
        frequency_hz: 0.0,
        target_region: brodmann(11, "orbitofrontal cortex", "right", 2.0, -3.0, -4.5),
    };
    let bytes = encode_to_vec(&opto).expect("encode inhibitory optogenetics");
    let (dec, _): (OptogeneticsStimulation, usize) =
        decode_from_slice(&bytes).expect("decode inhibitory optogenetics");
    assert_eq!(opto, dec);
}

// Test 22: Full BCI pipeline (array + sorting + decoder + prosthetic)
#[test]
fn test_full_bci_pipeline_roundtrip() {
    // This test validates the deepest nesting by combining multiple subsystems.
    let array = IntracorticalArray {
        array_name: "Utah_Combo".to_string(),
        manufacturer: "Blackrock".to_string(),
        shanks: vec![MicroelectrodeShank {
            shank_id: 1,
            contact_positions: vec![coord(0.0, 0.0, -1.0)],
            impedances: vec![impedance(200.0, -10.0)],
            contact_area_um2: 1250.0,
        }],
        insertion_depth_mm: 1.5,
        target_region: brodmann(4, "M1", "left", -35.0, -20.0, 60.0),
    };

    let sorting = SpikeSortingResult {
        session_id: "pipeline_sess".to_string(),
        electrode_label: "sh1_c1".to_string(),
        clusters: vec![SpikeCluster {
            cluster_id: 1,
            neuron_type: "pyramidal".to_string(),
            waveform: SpikeWaveform {
                template_id: 1,
                samples: vec![-10.0, -50.0, -100.0, -30.0, 20.0, 10.0],
                peak_amplitude_uv: 100.0,
                trough_to_peak_ms: 0.4,
            },
            mean_firing_rate_hz: 12.0,
            isolation_distance: 25.0,
            spike_times_ms: vec![10.0, 95.0, 180.0],
        }],
        noise_floor_uv: 10.0,
    };

    let telemetry = ImplantTelemetry {
        implant_id: "pipeline_implant".to_string(),
        array,
        packets: vec![TelemetryPacket {
            timestamp_us: 500_000,
            battery_pct: 92.0,
            temperature_c: 36.8,
            signal_quality: 0.98,
            error_flags: vec![],
        }],
        uptime_hours: 48.0,
        firmware_version: "4.0.0".to_string(),
    };

    let prosthetic = NeuroprostheticControl {
        device_id: "pipeline_hand".to_string(),
        degrees_of_freedom: vec![ProstheticDof {
            joint_name: "grip".to_string(),
            min_angle_deg: 0.0,
            max_angle_deg: 100.0,
            current_angle_deg: 50.0,
            velocity_deg_s: 3.0,
            torque_nm: 1.2,
        }],
        decoder: BciDecoderModel {
            model_id: "pipeline_dec".to_string(),
            decoder_type: "hybrid".to_string(),
            kalman_state: Some(KalmanFilterState {
                state_dim: 2,
                state_estimate: vec![50.0, 3.0],
                covariance_diag: vec![0.5, 0.1],
                process_noise: vec![0.01, 0.005],
                measurement_noise: vec![0.2, 0.1],
            }),
            nn_layers: vec![NeuralNetLayer {
                layer_name: "readout".to_string(),
                input_dim: 10,
                output_dim: 2,
                weights_flat: vec![0.1; 20],
                bias: vec![0.0, 0.0],
                activation: "tanh".to_string(),
            }],
            input_channels: vec!["sorted_unit_1".to_string()],
            output_dimensions: 1,
            decode_interval_ms: 25.0,
        },
        latency_ms: 8.0,
        active: true,
    };

    // Roundtrip each component
    let bytes_t = encode_to_vec(&telemetry).expect("encode telemetry pipeline");
    let (dec_t, _): (ImplantTelemetry, usize) =
        decode_from_slice(&bytes_t).expect("decode telemetry pipeline");
    assert_eq!(telemetry, dec_t);

    let bytes_s = encode_to_vec(&sorting).expect("encode sorting pipeline");
    let (dec_s, _): (SpikeSortingResult, usize) =
        decode_from_slice(&bytes_s).expect("decode sorting pipeline");
    assert_eq!(sorting, dec_s);

    let bytes_p = encode_to_vec(&prosthetic).expect("encode prosthetic pipeline");
    let (dec_p, _): (NeuroprostheticControl, usize) =
        decode_from_slice(&bytes_p).expect("decode prosthetic pipeline");
    assert_eq!(prosthetic, dec_p);
}
