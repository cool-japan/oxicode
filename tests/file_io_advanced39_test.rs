//! Advanced file I/O tests for OxiCode — domain: quantum computing infrastructure

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GateType {
    Hadamard,
    PauliX,
    PauliY,
    PauliZ,
    Cnot,
    Toffoli,
    Rz { angle_millirad: i64 },
    Rx { angle_millirad: i64 },
    Swap,
    CZ,
    ISwap,
    Phase { theta_millirad: i64 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ErrorCorrectionCode {
    SurfaceCode { distance: u16 },
    SteaneCode,
    ShorCode,
    ColorCode { dimension: u8 },
    RotatedSurface { distance: u16, rounds: u16 },
    ConcatenatedSteane { levels: u8 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TranspilerOptLevel {
    None,
    Light,
    Medium,
    Heavy,
    Custom { pass_ids: Vec<u16> },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed { error_code: u32 },
    Cancelled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NoiseChannel {
    Depolarizing { probability_x1e9: u64 },
    AmplitudeDamping { gamma_x1e9: u64 },
    PhaseDamping { lambda_x1e9: u64 },
    BitFlip { probability_x1e9: u64 },
    ThermalRelaxation { t1_ns: u64, t2_ns: u64 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QubitRegisterConfig {
    register_name: String,
    num_qubits: u32,
    qubit_ids: Vec<u32>,
    connectivity: Vec<(u32, u32)>,
    native_gates: Vec<GateType>,
    register_type: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QuantumGateDefinition {
    gate_id: u64,
    gate_type: GateType,
    target_qubits: Vec<u32>,
    control_qubits: Vec<u32>,
    duration_ns: u32,
    fidelity_x1e9: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CircuitMetrics {
    circuit_id: u64,
    depth: u32,
    width: u32,
    gate_count: u32,
    two_qubit_gate_count: u32,
    measurement_count: u32,
    classical_bit_count: u32,
    estimated_runtime_ns: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DecoherenceProfile {
    qubit_id: u32,
    t1_ns: u64,
    t2_ns: u64,
    t2_star_ns: u64,
    t_phi_ns: u64,
    measurement_timestamp: u64,
    temperature_micro_kelvin: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CalibrationSnapshot {
    snapshot_id: u64,
    qpu_name: String,
    single_qubit_fidelities_x1e9: Vec<u64>,
    two_qubit_fidelities_x1e9: Vec<u64>,
    readout_errors_x1e9: Vec<u64>,
    readout_assignment_fidelity_x1e9: Vec<u64>,
    calibration_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CryogenicReading {
    system_id: u32,
    mixing_chamber_temp_micro_k: u64,
    still_temp_micro_k: u64,
    cold_plate_temp_micro_k: u64,
    four_k_stage_temp_micro_k: u64,
    fifty_k_stage_temp_micro_k: u64,
    pulse_tube_power_mw: u32,
    helium_pressure_mbar_x100: u32,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PulseSchedule {
    schedule_id: u64,
    qubit_id: u32,
    channel_name: String,
    frequency_hz_x1000: u64,
    amplitude_x1e9: u64,
    sigma_ns: u32,
    duration_ns: u32,
    drag_coefficient_x1e6: i64,
    phase_millirad: i64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QuantumVolumeBenchmark {
    benchmark_id: u64,
    qpu_name: String,
    quantum_volume: u32,
    num_qubits_tested: u32,
    num_trials: u32,
    heavy_output_fraction_x1e6: u64,
    confidence_interval_x1e6: u64,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EntanglementWitness {
    witness_id: u64,
    qubit_pair: (u32, u32),
    witness_value_x1e9: i64,
    negativity_x1e9: u64,
    concurrence_x1e9: u64,
    bell_state_fidelity_x1e9: u64,
    measurement_basis: String,
    num_shots: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NoiseModelParams {
    model_id: u64,
    qubit_id: u32,
    channels: Vec<NoiseChannel>,
    gate_error_rates_x1e9: Vec<u64>,
    measurement_error_x1e9: u64,
    crosstalk_coefficients_x1e9: Vec<i64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct JobQueueEntry {
    job_id: u64,
    user_id: String,
    circuit_name: String,
    status: JobStatus,
    priority: u8,
    num_shots: u32,
    transpiler_opt: TranspilerOptLevel,
    submitted_timestamp: u64,
    started_timestamp: Option<u64>,
    completed_timestamp: Option<u64>,
    estimated_wait_seconds: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QpuTopologyEdge {
    qubit_a: u32,
    qubit_b: u32,
    cx_error_x1e9: u64,
    cx_duration_ns: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QpuTopologyMap {
    qpu_name: String,
    num_qubits: u32,
    edges: Vec<QpuTopologyEdge>,
    qubit_frequencies_hz_x1000: Vec<u64>,
    qubit_anharmonicities_hz_x1000: Vec<i64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClassicalQuantumInterfaceConfig {
    interface_id: u32,
    dac_resolution_bits: u8,
    adc_resolution_bits: u8,
    sample_rate_mhz_x100: u32,
    num_channels: u16,
    feedback_latency_ns: u32,
    real_time_discriminator_enabled: bool,
    kernel_name: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ErrorCorrectionConfig {
    config_id: u64,
    code: ErrorCorrectionCode,
    logical_qubit_count: u16,
    physical_qubits_per_logical: u32,
    syndrome_extraction_rounds: u16,
    decoder_name: String,
    threshold_error_rate_x1e9: u64,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn unique_tmp(name: &str) -> std::path::PathBuf {
    temp_dir().join(name)
}

// ---------------------------------------------------------------------------
// Tests — 22 total
// ---------------------------------------------------------------------------

/// 1. QubitRegisterConfig with native gates — vec roundtrip
#[test]
fn test_qubit_register_config_vec_roundtrip() {
    let config = QubitRegisterConfig {
        register_name: "main_register".to_string(),
        num_qubits: 5,
        qubit_ids: vec![0, 1, 2, 3, 4],
        connectivity: vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4)],
        native_gates: vec![
            GateType::Hadamard,
            GateType::Cnot,
            GateType::Rz {
                angle_millirad: 1571,
            },
        ],
        register_type: "transmon".to_string(),
    };
    let bytes = encode_to_vec(&config).expect("encode QubitRegisterConfig");
    let (decoded, consumed): (QubitRegisterConfig, usize) =
        decode_from_slice(&bytes).expect("decode QubitRegisterConfig");
    assert_eq!(config, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 2. QuantumGateDefinition all gate variants — vec roundtrip
#[test]
fn test_gate_definition_all_variants_vec_roundtrip() {
    let variants = vec![
        GateType::Hadamard,
        GateType::PauliX,
        GateType::PauliY,
        GateType::PauliZ,
        GateType::Cnot,
        GateType::Toffoli,
        GateType::Rz {
            angle_millirad: 785,
        },
        GateType::Rx {
            angle_millirad: -1571,
        },
        GateType::Swap,
        GateType::CZ,
        GateType::ISwap,
        GateType::Phase {
            theta_millirad: 3142,
        },
    ];
    for (i, gate_type) in variants.into_iter().enumerate() {
        let gate = QuantumGateDefinition {
            gate_id: i as u64 + 1000,
            gate_type,
            target_qubits: vec![i as u32],
            control_qubits: if i % 3 == 0 {
                vec![i as u32 + 1]
            } else {
                vec![]
            },
            duration_ns: 20 + i as u32 * 5,
            fidelity_x1e9: 999_000_000 - i as u64 * 1_000_000,
        };
        let bytes = encode_to_vec(&gate).expect("encode QuantumGateDefinition variant");
        let (decoded, _): (QuantumGateDefinition, usize) =
            decode_from_slice(&bytes).expect("decode QuantumGateDefinition variant");
        assert_eq!(gate, decoded);
    }
}

/// 3. CircuitMetrics — file roundtrip
#[test]
fn test_circuit_metrics_file_roundtrip() {
    let path = unique_tmp("circuit_metrics_39.bin");
    let metrics = CircuitMetrics {
        circuit_id: 42_000,
        depth: 1200,
        width: 127,
        gate_count: 8500,
        two_qubit_gate_count: 3200,
        measurement_count: 127,
        classical_bit_count: 127,
        estimated_runtime_ns: 45_000_000,
    };
    encode_to_file(&metrics, &path).expect("encode_to_file CircuitMetrics");
    let decoded: CircuitMetrics = decode_from_file(&path).expect("decode_from_file CircuitMetrics");
    assert_eq!(metrics, decoded);
    std::fs::remove_file(&path).expect("cleanup circuit_metrics_39.bin");
}

/// 4. DecoherenceProfile with realistic T1/T2 values — vec roundtrip
#[test]
fn test_decoherence_profile_vec_roundtrip() {
    let profile = DecoherenceProfile {
        qubit_id: 17,
        t1_ns: 150_000_000,
        t2_ns: 80_000_000,
        t2_star_ns: 50_000_000,
        t_phi_ns: 120_000_000,
        measurement_timestamp: 1_740_100_000,
        temperature_micro_kelvin: 15_000,
    };
    let bytes = encode_to_vec(&profile).expect("encode DecoherenceProfile");
    let (decoded, consumed): (DecoherenceProfile, usize) =
        decode_from_slice(&bytes).expect("decode DecoherenceProfile");
    assert_eq!(profile, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 5. CalibrationSnapshot with multiple qubit fidelities — file roundtrip
#[test]
fn test_calibration_snapshot_file_roundtrip() {
    let path = unique_tmp("calibration_snap_39.bin");
    let snap = CalibrationSnapshot {
        snapshot_id: 99_001,
        qpu_name: "Eagle-r3".to_string(),
        single_qubit_fidelities_x1e9: vec![
            999_400_000,
            999_200_000,
            998_900_000,
            999_100_000,
            999_500_000,
        ],
        two_qubit_fidelities_x1e9: vec![990_000_000, 988_500_000, 991_200_000, 989_700_000],
        readout_errors_x1e9: vec![12_000_000, 15_000_000, 11_500_000, 13_200_000, 14_800_000],
        readout_assignment_fidelity_x1e9: vec![
            988_000_000,
            985_000_000,
            988_500_000,
            986_800_000,
            985_200_000,
        ],
        calibration_timestamp: 1_740_200_000,
    };
    encode_to_file(&snap, &path).expect("encode_to_file CalibrationSnapshot");
    let decoded: CalibrationSnapshot =
        decode_from_file(&path).expect("decode_from_file CalibrationSnapshot");
    assert_eq!(snap, decoded);
    std::fs::remove_file(&path).expect("cleanup calibration_snap_39.bin");
}

/// 6. CryogenicReading — dilution refrigerator temperatures — vec roundtrip
#[test]
fn test_cryogenic_reading_vec_roundtrip() {
    let reading = CryogenicReading {
        system_id: 7,
        mixing_chamber_temp_micro_k: 12_500,
        still_temp_micro_k: 800_000,
        cold_plate_temp_micro_k: 100_000,
        four_k_stage_temp_micro_k: 3_800_000,
        fifty_k_stage_temp_micro_k: 42_000_000,
        pulse_tube_power_mw: 1500,
        helium_pressure_mbar_x100: 105_000,
        timestamp: 1_740_300_000,
    };
    let bytes = encode_to_vec(&reading).expect("encode CryogenicReading");
    let (decoded, consumed): (CryogenicReading, usize) =
        decode_from_slice(&bytes).expect("decode CryogenicReading");
    assert_eq!(reading, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 7. PulseSchedule with DRAG correction — file roundtrip
#[test]
fn test_pulse_schedule_drag_file_roundtrip() {
    let path = unique_tmp("pulse_sched_39.bin");
    let pulse = PulseSchedule {
        schedule_id: 55_001,
        qubit_id: 3,
        channel_name: "d3".to_string(),
        frequency_hz_x1000: 5_120_000_000_000,
        amplitude_x1e9: 450_000_000,
        sigma_ns: 40,
        duration_ns: 160,
        drag_coefficient_x1e6: -250_000,
        phase_millirad: 0,
    };
    encode_to_file(&pulse, &path).expect("encode_to_file PulseSchedule");
    let decoded: PulseSchedule = decode_from_file(&path).expect("decode_from_file PulseSchedule");
    assert_eq!(pulse, decoded);
    std::fs::remove_file(&path).expect("cleanup pulse_sched_39.bin");
}

/// 8. QuantumVolumeBenchmark — vec roundtrip
#[test]
fn test_quantum_volume_benchmark_vec_roundtrip() {
    let benchmark = QuantumVolumeBenchmark {
        benchmark_id: 88_001,
        qpu_name: "Osprey-QPU-Alpha".to_string(),
        quantum_volume: 256,
        num_qubits_tested: 8,
        num_trials: 100,
        heavy_output_fraction_x1e6: 720_000,
        confidence_interval_x1e6: 15_000,
        timestamp: 1_740_400_000,
    };
    let bytes = encode_to_vec(&benchmark).expect("encode QuantumVolumeBenchmark");
    let (decoded, consumed): (QuantumVolumeBenchmark, usize) =
        decode_from_slice(&bytes).expect("decode QuantumVolumeBenchmark");
    assert_eq!(benchmark, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 9. EntanglementWitness with negative witness value — file roundtrip
#[test]
fn test_entanglement_witness_file_roundtrip() {
    let path = unique_tmp("entanglement_witness_39.bin");
    let witness = EntanglementWitness {
        witness_id: 77_001,
        qubit_pair: (4, 7),
        witness_value_x1e9: -350_000_000,
        negativity_x1e9: 420_000_000,
        concurrence_x1e9: 850_000_000,
        bell_state_fidelity_x1e9: 960_000_000,
        measurement_basis: "ZZ".to_string(),
        num_shots: 8192,
    };
    encode_to_file(&witness, &path).expect("encode_to_file EntanglementWitness");
    let decoded: EntanglementWitness =
        decode_from_file(&path).expect("decode_from_file EntanglementWitness");
    assert_eq!(witness, decoded);
    std::fs::remove_file(&path).expect("cleanup entanglement_witness_39.bin");
}

/// 10. NoiseModelParams with multiple noise channels — vec roundtrip
#[test]
fn test_noise_model_params_vec_roundtrip() {
    let model = NoiseModelParams {
        model_id: 66_001,
        qubit_id: 12,
        channels: vec![
            NoiseChannel::Depolarizing {
                probability_x1e9: 1_500_000,
            },
            NoiseChannel::AmplitudeDamping {
                gamma_x1e9: 800_000,
            },
            NoiseChannel::PhaseDamping {
                lambda_x1e9: 1_200_000,
            },
            NoiseChannel::ThermalRelaxation {
                t1_ns: 150_000_000,
                t2_ns: 80_000_000,
            },
        ],
        gate_error_rates_x1e9: vec![600_000, 800_000, 1_100_000],
        measurement_error_x1e9: 12_000_000,
        crosstalk_coefficients_x1e9: vec![-50_000, 120_000, -30_000],
    };
    let bytes = encode_to_vec(&model).expect("encode NoiseModelParams");
    let (decoded, consumed): (NoiseModelParams, usize) =
        decode_from_slice(&bytes).expect("decode NoiseModelParams");
    assert_eq!(model, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 11. ErrorCorrectionConfig — surface code — file roundtrip
#[test]
fn test_error_correction_surface_code_file_roundtrip() {
    let path = unique_tmp("ecc_surface_39.bin");
    let ecc = ErrorCorrectionConfig {
        config_id: 44_001,
        code: ErrorCorrectionCode::SurfaceCode { distance: 17 },
        logical_qubit_count: 4,
        physical_qubits_per_logical: 578,
        syndrome_extraction_rounds: 17,
        decoder_name: "minimum_weight_perfect_matching".to_string(),
        threshold_error_rate_x1e9: 1_100_000,
    };
    encode_to_file(&ecc, &path).expect("encode_to_file ErrorCorrectionConfig");
    let decoded: ErrorCorrectionConfig =
        decode_from_file(&path).expect("decode_from_file ErrorCorrectionConfig");
    assert_eq!(ecc, decoded);
    std::fs::remove_file(&path).expect("cleanup ecc_surface_39.bin");
}

/// 12. ErrorCorrectionCode all variants — vec roundtrip
#[test]
fn test_error_correction_code_all_variants_vec_roundtrip() {
    let codes = vec![
        ErrorCorrectionCode::SurfaceCode { distance: 5 },
        ErrorCorrectionCode::SteaneCode,
        ErrorCorrectionCode::ShorCode,
        ErrorCorrectionCode::ColorCode { dimension: 3 },
        ErrorCorrectionCode::RotatedSurface {
            distance: 7,
            rounds: 7,
        },
        ErrorCorrectionCode::ConcatenatedSteane { levels: 2 },
    ];
    for (i, code) in codes.into_iter().enumerate() {
        let ecc = ErrorCorrectionConfig {
            config_id: 50_000 + i as u64,
            code,
            logical_qubit_count: (i as u16 + 1) * 2,
            physical_qubits_per_logical: 100 + i as u32 * 50,
            syndrome_extraction_rounds: (i as u16 + 3),
            decoder_name: format!("decoder_variant_{}", i),
            threshold_error_rate_x1e9: 1_000_000 + i as u64 * 100_000,
        };
        let bytes = encode_to_vec(&ecc).expect("encode ErrorCorrectionConfig variant");
        let (decoded, _): (ErrorCorrectionConfig, usize) =
            decode_from_slice(&bytes).expect("decode ErrorCorrectionConfig variant");
        assert_eq!(ecc, decoded);
    }
}

/// 13. TranspilerOptLevel all variants — vec roundtrip
#[test]
fn test_transpiler_opt_levels_vec_roundtrip() {
    let levels = vec![
        TranspilerOptLevel::None,
        TranspilerOptLevel::Light,
        TranspilerOptLevel::Medium,
        TranspilerOptLevel::Heavy,
        TranspilerOptLevel::Custom {
            pass_ids: vec![1, 5, 12, 27, 33],
        },
    ];
    for (i, opt) in levels.into_iter().enumerate() {
        let job = JobQueueEntry {
            job_id: 10_000 + i as u64,
            user_id: format!("user_{}", i),
            circuit_name: format!("circuit_opt_test_{}", i),
            status: JobStatus::Queued,
            priority: (i as u8) % 5,
            num_shots: 4096,
            transpiler_opt: opt,
            submitted_timestamp: 1_740_500_000 + i as u64 * 300,
            started_timestamp: None,
            completed_timestamp: None,
            estimated_wait_seconds: 120 + i as u32 * 30,
        };
        let bytes = encode_to_vec(&job).expect("encode JobQueueEntry with opt level");
        let (decoded, _): (JobQueueEntry, usize) =
            decode_from_slice(&bytes).expect("decode JobQueueEntry with opt level");
        assert_eq!(job, decoded);
    }
}

/// 14. JobQueueEntry all statuses — file roundtrip
#[test]
fn test_job_queue_all_statuses_file_roundtrip() {
    let path = unique_tmp("job_queue_statuses_39.bin");
    let statuses = vec![
        JobStatus::Queued,
        JobStatus::Running,
        JobStatus::Completed,
        JobStatus::Failed { error_code: 4001 },
        JobStatus::Cancelled,
    ];
    for (i, status) in statuses.into_iter().enumerate() {
        let job = JobQueueEntry {
            job_id: 20_000 + i as u64,
            user_id: "researcher_alpha".to_string(),
            circuit_name: format!("bell_state_{}", i),
            status,
            priority: 3,
            num_shots: 1024,
            transpiler_opt: TranspilerOptLevel::Medium,
            submitted_timestamp: 1_740_600_000,
            started_timestamp: if i >= 1 { Some(1_740_600_010) } else { None },
            completed_timestamp: if i >= 2 { Some(1_740_600_050) } else { None },
            estimated_wait_seconds: 60,
        };
        encode_to_file(&job, &path).expect("encode_to_file JobQueueEntry status variant");
        let decoded: JobQueueEntry =
            decode_from_file(&path).expect("decode_from_file JobQueueEntry status variant");
        assert_eq!(job, decoded);
    }
    std::fs::remove_file(&path).expect("cleanup job_queue_statuses_39.bin");
}

/// 15. QpuTopologyMap — heavy hex lattice subset — vec roundtrip
#[test]
fn test_qpu_topology_map_vec_roundtrip() {
    let edges: Vec<QpuTopologyEdge> = (0..20)
        .map(|i| QpuTopologyEdge {
            qubit_a: i,
            qubit_b: i + 1,
            cx_error_x1e9: 8_000_000 + (i as u64 % 5) * 500_000,
            cx_duration_ns: 300 + (i % 3) * 20,
        })
        .collect();
    let topo = QpuTopologyMap {
        qpu_name: "HeavyHex-21Q".to_string(),
        num_qubits: 21,
        edges,
        qubit_frequencies_hz_x1000: (0..21)
            .map(|i| 5_000_000_000_000 + i as u64 * 50_000_000)
            .collect(),
        qubit_anharmonicities_hz_x1000: (0..21)
            .map(|i| -340_000_000 + i as i64 * 2_000_000)
            .collect(),
    };
    let bytes = encode_to_vec(&topo).expect("encode QpuTopologyMap");
    let (decoded, consumed): (QpuTopologyMap, usize) =
        decode_from_slice(&bytes).expect("decode QpuTopologyMap");
    assert_eq!(topo, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 16. ClassicalQuantumInterfaceConfig — file roundtrip
#[test]
fn test_classical_quantum_interface_config_file_roundtrip() {
    let path = unique_tmp("cq_interface_39.bin");
    let config = ClassicalQuantumInterfaceConfig {
        interface_id: 1,
        dac_resolution_bits: 14,
        adc_resolution_bits: 12,
        sample_rate_mhz_x100: 200_000,
        num_channels: 32,
        feedback_latency_ns: 250,
        real_time_discriminator_enabled: true,
        kernel_name: "matched_filter_v3".to_string(),
    };
    encode_to_file(&config, &path).expect("encode_to_file ClassicalQuantumInterfaceConfig");
    let decoded: ClassicalQuantumInterfaceConfig =
        decode_from_file(&path).expect("decode_from_file ClassicalQuantumInterfaceConfig");
    assert_eq!(config, decoded);
    std::fs::remove_file(&path).expect("cleanup cq_interface_39.bin");
}

/// 17. Batch of 64 decoherence profiles — vec roundtrip
#[test]
fn test_decoherence_batch_64_qubits_vec_roundtrip() {
    let profiles: Vec<DecoherenceProfile> = (0..64)
        .map(|i| DecoherenceProfile {
            qubit_id: i,
            t1_ns: 100_000_000 + (i as u64 % 10) * 10_000_000,
            t2_ns: 50_000_000 + (i as u64 % 8) * 5_000_000,
            t2_star_ns: 30_000_000 + (i as u64 % 6) * 3_000_000,
            t_phi_ns: 80_000_000 + (i as u64 % 12) * 4_000_000,
            measurement_timestamp: 1_740_700_000 + i as u64,
            temperature_micro_kelvin: 14_500 + (i as u64 % 5) * 200,
        })
        .collect();
    let bytes = encode_to_vec(&profiles).expect("encode decoherence batch");
    let (decoded, consumed): (Vec<DecoherenceProfile>, usize) =
        decode_from_slice(&bytes).expect("decode decoherence batch");
    assert_eq!(profiles, decoded);
    assert_eq!(consumed, bytes.len());
}

/// 18. Cryogenic readings batch — file roundtrip with overwrite semantics
#[test]
fn test_cryogenic_readings_file_overwrite() {
    let path = unique_tmp("cryo_overwrite_39.bin");
    let first = CryogenicReading {
        system_id: 1,
        mixing_chamber_temp_micro_k: 15_000,
        still_temp_micro_k: 900_000,
        cold_plate_temp_micro_k: 120_000,
        four_k_stage_temp_micro_k: 4_100_000,
        fifty_k_stage_temp_micro_k: 48_000_000,
        pulse_tube_power_mw: 1600,
        helium_pressure_mbar_x100: 110_000,
        timestamp: 1_740_800_000,
    };
    encode_to_file(&first, &path).expect("encode_to_file first cryogenic reading");
    let second = CryogenicReading {
        system_id: 2,
        mixing_chamber_temp_micro_k: 11_200,
        still_temp_micro_k: 750_000,
        cold_plate_temp_micro_k: 95_000,
        four_k_stage_temp_micro_k: 3_900_000,
        fifty_k_stage_temp_micro_k: 40_000_000,
        pulse_tube_power_mw: 1450,
        helium_pressure_mbar_x100: 102_000,
        timestamp: 1_740_800_100,
    };
    encode_to_file(&second, &path).expect("encode_to_file second cryogenic reading");
    let decoded: CryogenicReading =
        decode_from_file(&path).expect("decode_from_file overwrite cryogenic reading");
    assert_eq!(second, decoded);
    std::fs::remove_file(&path).expect("cleanup cryo_overwrite_39.bin");
}

/// 19. Encoding determinism — same circuit metrics encodes identically twice
#[test]
fn test_circuit_metrics_encoding_determinism() {
    let metrics = CircuitMetrics {
        circuit_id: 99_999,
        depth: 500,
        width: 65,
        gate_count: 4200,
        two_qubit_gate_count: 1800,
        measurement_count: 65,
        classical_bit_count: 65,
        estimated_runtime_ns: 22_000_000,
    };
    let bytes_a = encode_to_vec(&metrics).expect("encode CircuitMetrics determinism A");
    let bytes_b = encode_to_vec(&metrics).expect("encode CircuitMetrics determinism B");
    assert_eq!(
        bytes_a, bytes_b,
        "deterministic encoding must produce identical bytes"
    );
}

/// 20. Pulse schedule batch — file then slice roundtrip pipeline
#[test]
fn test_pulse_schedule_file_then_slice_pipeline() {
    let path = unique_tmp("pulse_pipeline_39.bin");
    let pulses: Vec<PulseSchedule> = (0..8)
        .map(|i| PulseSchedule {
            schedule_id: 70_000 + i as u64,
            qubit_id: i,
            channel_name: format!("d{}", i),
            frequency_hz_x1000: 5_000_000_000_000 + i as u64 * 100_000_000,
            amplitude_x1e9: 400_000_000 + i as u64 * 10_000_000,
            sigma_ns: 35 + i * 2,
            duration_ns: 140 + i * 8,
            drag_coefficient_x1e6: -200_000 + i as i64 * 25_000,
            phase_millirad: i as i64 * 785,
        })
        .collect();
    encode_to_file(&pulses, &path).expect("encode_to_file pulse batch");
    let decoded_file: Vec<PulseSchedule> =
        decode_from_file(&path).expect("decode_from_file pulse batch");
    assert_eq!(pulses, decoded_file);
    let bytes = encode_to_vec(&pulses).expect("encode_to_vec pulse batch");
    let (decoded_slice, consumed): (Vec<PulseSchedule>, usize) =
        decode_from_slice(&bytes).expect("decode_from_slice pulse batch");
    assert_eq!(pulses, decoded_slice);
    assert_eq!(consumed, bytes.len());
    std::fs::remove_file(&path).expect("cleanup pulse_pipeline_39.bin");
}

/// 21. Large QPU topology — 127 qubit topology — file roundtrip
#[test]
fn test_large_qpu_topology_127q_file_roundtrip() {
    let path = unique_tmp("topo_127q_39.bin");
    let mut edges = Vec::new();
    for i in 0u32..126 {
        if i % 5 != 4 {
            edges.push(QpuTopologyEdge {
                qubit_a: i,
                qubit_b: i + 1,
                cx_error_x1e9: 7_500_000 + (i as u64 % 7) * 300_000,
                cx_duration_ns: 280 + (i % 4) * 15,
            });
        }
    }
    for i in 0u32..100 {
        if i + 27 < 127 {
            edges.push(QpuTopologyEdge {
                qubit_a: i,
                qubit_b: i + 27,
                cx_error_x1e9: 9_000_000 + (i as u64 % 5) * 400_000,
                cx_duration_ns: 320 + (i % 3) * 10,
            });
        }
    }
    let topo = QpuTopologyMap {
        qpu_name: "Eagle-127Q".to_string(),
        num_qubits: 127,
        edges,
        qubit_frequencies_hz_x1000: (0..127)
            .map(|i| 4_800_000_000_000 + i as u64 * 30_000_000)
            .collect(),
        qubit_anharmonicities_hz_x1000: (0..127)
            .map(|i| -330_000_000 + i as i64 * 1_500_000)
            .collect(),
    };
    encode_to_file(&topo, &path).expect("encode_to_file 127Q topology");
    let decoded: QpuTopologyMap = decode_from_file(&path).expect("decode_from_file 127Q topology");
    assert_eq!(topo, decoded);
    std::fs::remove_file(&path).expect("cleanup topo_127q_39.bin");
}

/// 22. Full quantum job lifecycle — compound struct file roundtrip
#[test]
fn test_full_job_lifecycle_compound_file_roundtrip() {
    let path = unique_tmp("job_lifecycle_39.bin");
    let register = QubitRegisterConfig {
        register_name: "grover_register".to_string(),
        num_qubits: 10,
        qubit_ids: (0..10).collect(),
        connectivity: (0..9).map(|i| (i, i + 1)).collect(),
        native_gates: vec![
            GateType::Hadamard,
            GateType::Cnot,
            GateType::Rz {
                angle_millirad: 1571,
            },
        ],
        register_type: "transmon".to_string(),
    };
    let job = JobQueueEntry {
        job_id: 99_001,
        user_id: "grover_team".to_string(),
        circuit_name: "grover_search_10q".to_string(),
        status: JobStatus::Completed,
        priority: 1,
        num_shots: 8192,
        transpiler_opt: TranspilerOptLevel::Heavy,
        submitted_timestamp: 1_740_900_000,
        started_timestamp: Some(1_740_900_010),
        completed_timestamp: Some(1_740_900_095),
        estimated_wait_seconds: 10,
    };
    let witness = EntanglementWitness {
        witness_id: 99_002,
        qubit_pair: (0, 9),
        witness_value_x1e9: -280_000_000,
        negativity_x1e9: 350_000_000,
        concurrence_x1e9: 780_000_000,
        bell_state_fidelity_x1e9: 940_000_000,
        measurement_basis: "XX".to_string(),
        num_shots: 8192,
    };
    let compound = (register.clone(), job.clone(), witness.clone());
    encode_to_file(&compound, &path).expect("encode_to_file compound job lifecycle");
    let decoded: (QubitRegisterConfig, JobQueueEntry, EntanglementWitness) =
        decode_from_file(&path).expect("decode_from_file compound job lifecycle");
    assert_eq!(register, decoded.0);
    assert_eq!(job, decoded.1);
    assert_eq!(witness, decoded.2);
    std::fs::remove_file(&path).expect("cleanup job_lifecycle_39.bin");
}
