#![cfg(feature = "versioning")]
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
use oxicode::versioning::Version;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GateType {
    Hadamard,
    PauliX,
    PauliY,
    PauliZ,
    CNOT,
    Toffoli,
    Phase,
    Measure,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ErrorSyndrome {
    None,
    BitFlip,
    PhaseFlip,
    Both,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum QubitState {
    Ground,
    Excited,
    Superposition,
    Entangled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Qubit {
    qubit_id: u32,
    state: QubitState,
    fidelity_x1000: u32,
    coherence_time_us: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QuantumGate {
    gate_id: u64,
    gate_type: GateType,
    target_qubit: u32,
    control_qubit: Option<u32>,
    phase_rad_x1000: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QuantumCircuit {
    circuit_id: u64,
    name: String,
    num_qubits: u16,
    gates: Vec<QuantumGate>,
    depth: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ErrorCorrectionCycle {
    cycle_id: u64,
    timestamp: u64,
    syndromes: Vec<ErrorSyndrome>,
    corrections_applied: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QuantumResult {
    result_id: u64,
    circuit_id: u64,
    measurements: Vec<u8>,
    fidelity_x1000: u32,
    shots: u32,
}

#[test]
fn test_qubit_ground_state_roundtrip() {
    let qubit = Qubit {
        qubit_id: 0,
        state: QubitState::Ground,
        fidelity_x1000: 999,
        coherence_time_us: 150,
    };
    let bytes = encode_to_vec(&qubit).expect("encode qubit ground state");
    let (decoded, consumed) =
        decode_from_slice::<Qubit>(&bytes).expect("decode qubit ground state");
    assert_eq!(decoded, qubit);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_qubit_superposition_versioned_v1() {
    let qubit = Qubit {
        qubit_id: 7,
        state: QubitState::Superposition,
        fidelity_x1000: 987,
        coherence_time_us: 200,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&qubit, ver).expect("encode qubit superposition v1");
    let (decoded, version, consumed) =
        decode_versioned_value::<Qubit>(&bytes).expect("decode qubit superposition v1");
    assert_eq!(decoded, qubit);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_qubit_entangled_versioned_v2() {
    let qubit = Qubit {
        qubit_id: 42,
        state: QubitState::Entangled,
        fidelity_x1000: 995,
        coherence_time_us: 300,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&qubit, ver).expect("encode qubit entangled v2");
    let (decoded, version, consumed) =
        decode_versioned_value::<Qubit>(&bytes).expect("decode qubit entangled v2");
    assert_eq!(decoded, qubit);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_gate_type_hadamard_roundtrip() {
    let gate = QuantumGate {
        gate_id: 1001,
        gate_type: GateType::Hadamard,
        target_qubit: 0,
        control_qubit: None,
        phase_rad_x1000: 0,
    };
    let bytes = encode_to_vec(&gate).expect("encode hadamard gate");
    let (decoded, consumed) =
        decode_from_slice::<QuantumGate>(&bytes).expect("decode hadamard gate");
    assert_eq!(decoded, gate);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_gate_cnot_with_control_qubit() {
    let gate = QuantumGate {
        gate_id: 2002,
        gate_type: GateType::CNOT,
        target_qubit: 1,
        control_qubit: Some(0),
        phase_rad_x1000: 0,
    };
    let bytes = encode_to_vec(&gate).expect("encode CNOT gate");
    let (decoded, consumed) = decode_from_slice::<QuantumGate>(&bytes).expect("decode CNOT gate");
    assert_eq!(decoded.control_qubit, Some(0));
    assert_eq!(decoded.gate_type, GateType::CNOT);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_gate_toffoli_versioned_v1_2_3() {
    let gate = QuantumGate {
        gate_id: 3003,
        gate_type: GateType::Toffoli,
        target_qubit: 2,
        control_qubit: Some(1),
        phase_rad_x1000: 0,
    };
    let ver = Version::new(1, 2, 3);
    let bytes = encode_versioned_value(&gate, ver).expect("encode toffoli gate v1.2.3");
    let (decoded, version, consumed) =
        decode_versioned_value::<QuantumGate>(&bytes).expect("decode toffoli gate v1.2.3");
    assert_eq!(decoded, gate);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert!(consumed > 0);
}

#[test]
fn test_gate_phase_with_negative_phase() {
    let gate = QuantumGate {
        gate_id: 4004,
        gate_type: GateType::Phase,
        target_qubit: 3,
        control_qubit: None,
        phase_rad_x1000: -1571,
    };
    let bytes = encode_to_vec(&gate).expect("encode phase gate");
    let (decoded, consumed) = decode_from_slice::<QuantumGate>(&bytes).expect("decode phase gate");
    assert_eq!(decoded.phase_rad_x1000, -1571);
    assert_eq!(decoded.gate_type, GateType::Phase);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_gate_measure_no_control_qubit() {
    let gate = QuantumGate {
        gate_id: 5005,
        gate_type: GateType::Measure,
        target_qubit: 4,
        control_qubit: None,
        phase_rad_x1000: 0,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&gate, ver).expect("encode measure gate v2");
    let (decoded, version, consumed) =
        decode_versioned_value::<QuantumGate>(&bytes).expect("decode measure gate v2");
    assert_eq!(decoded.control_qubit, None);
    assert_eq!(version.major, 2);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_quantum_circuit_empty_gates() {
    let circuit = QuantumCircuit {
        circuit_id: 100,
        name: String::from("empty_circuit"),
        num_qubits: 4,
        gates: vec![],
        depth: 0,
    };
    let bytes = encode_to_vec(&circuit).expect("encode empty quantum circuit");
    let (decoded, consumed) =
        decode_from_slice::<QuantumCircuit>(&bytes).expect("decode empty quantum circuit");
    assert_eq!(decoded, circuit);
    assert_eq!(decoded.gates.len(), 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_quantum_circuit_multiple_gates_versioned_v1() {
    let circuit = QuantumCircuit {
        circuit_id: 200,
        name: String::from("bell_state"),
        num_qubits: 2,
        gates: vec![
            QuantumGate {
                gate_id: 1,
                gate_type: GateType::Hadamard,
                target_qubit: 0,
                control_qubit: None,
                phase_rad_x1000: 0,
            },
            QuantumGate {
                gate_id: 2,
                gate_type: GateType::CNOT,
                target_qubit: 1,
                control_qubit: Some(0),
                phase_rad_x1000: 0,
            },
        ],
        depth: 2,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&circuit, ver).expect("encode bell state circuit v1");
    let (decoded, version, consumed) =
        decode_versioned_value::<QuantumCircuit>(&bytes).expect("decode bell state circuit v1");
    assert_eq!(decoded, circuit);
    assert_eq!(decoded.gates.len(), 2);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_quantum_circuit_ghz_state_versioned_v2() {
    let circuit = QuantumCircuit {
        circuit_id: 300,
        name: String::from("ghz_state"),
        num_qubits: 3,
        gates: vec![
            QuantumGate {
                gate_id: 10,
                gate_type: GateType::Hadamard,
                target_qubit: 0,
                control_qubit: None,
                phase_rad_x1000: 0,
            },
            QuantumGate {
                gate_id: 11,
                gate_type: GateType::CNOT,
                target_qubit: 1,
                control_qubit: Some(0),
                phase_rad_x1000: 0,
            },
            QuantumGate {
                gate_id: 12,
                gate_type: GateType::CNOT,
                target_qubit: 2,
                control_qubit: Some(0),
                phase_rad_x1000: 0,
            },
        ],
        depth: 3,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&circuit, ver).expect("encode GHZ state circuit v2");
    let (decoded, version, consumed) =
        decode_versioned_value::<QuantumCircuit>(&bytes).expect("decode GHZ state circuit v2");
    assert_eq!(decoded.name, "ghz_state");
    assert_eq!(decoded.gates.len(), 3);
    assert_eq!(version.major, 2);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_error_syndrome_none_roundtrip() {
    let syndrome = ErrorSyndrome::None;
    let bytes = encode_to_vec(&syndrome).expect("encode error syndrome none");
    let (decoded, consumed) =
        decode_from_slice::<ErrorSyndrome>(&bytes).expect("decode error syndrome none");
    assert_eq!(decoded, ErrorSyndrome::None);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_error_correction_cycle_bit_flips_versioned_v1_2_3() {
    let cycle = ErrorCorrectionCycle {
        cycle_id: 9001,
        timestamp: 1_700_000_000,
        syndromes: vec![
            ErrorSyndrome::BitFlip,
            ErrorSyndrome::None,
            ErrorSyndrome::PhaseFlip,
            ErrorSyndrome::Both,
        ],
        corrections_applied: 2,
    };
    let ver = Version::new(1, 2, 3);
    let bytes = encode_versioned_value(&cycle, ver).expect("encode error correction cycle v1.2.3");
    let (decoded, version, consumed) = decode_versioned_value::<ErrorCorrectionCycle>(&bytes)
        .expect("decode error correction cycle v1.2.3");
    assert_eq!(decoded, cycle);
    assert_eq!(decoded.syndromes.len(), 4);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_error_correction_cycle_empty_syndromes() {
    let cycle = ErrorCorrectionCycle {
        cycle_id: 9002,
        timestamp: 1_700_000_100,
        syndromes: vec![],
        corrections_applied: 0,
    };
    let bytes = encode_to_vec(&cycle).expect("encode error correction cycle empty");
    let (decoded, consumed) = decode_from_slice::<ErrorCorrectionCycle>(&bytes)
        .expect("decode error correction cycle empty");
    assert_eq!(decoded, cycle);
    assert_eq!(decoded.syndromes.len(), 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_quantum_result_single_shot_versioned_v1() {
    let result = QuantumResult {
        result_id: 7001,
        circuit_id: 200,
        measurements: vec![0, 1],
        fidelity_x1000: 982,
        shots: 1,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&result, ver).expect("encode quantum result single shot v1");
    let (decoded, version, consumed) = decode_versioned_value::<QuantumResult>(&bytes)
        .expect("decode quantum result single shot v1");
    assert_eq!(decoded, result);
    assert_eq!(decoded.shots, 1);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_quantum_result_multi_shot_versioned_v2() {
    let result = QuantumResult {
        result_id: 7002,
        circuit_id: 300,
        measurements: vec![0, 0, 1, 1, 0, 1, 1, 0],
        fidelity_x1000: 976,
        shots: 1024,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&result, ver).expect("encode quantum result multi shot v2");
    let (decoded, version, consumed) = decode_versioned_value::<QuantumResult>(&bytes)
        .expect("decode quantum result multi shot v2");
    assert_eq!(decoded, result);
    assert_eq!(decoded.shots, 1024);
    assert_eq!(version.major, 2);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_version_fields_accessible_directly() {
    let ver = Version::new(3, 7, 11);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 7);
    assert_eq!(ver.patch, 11);
}

#[test]
fn test_version_preserved_across_encode_decode_v1_2_3() {
    let qubit = Qubit {
        qubit_id: 99,
        state: QubitState::Excited,
        fidelity_x1000: 940,
        coherence_time_us: 80,
    };
    let ver = Version::new(1, 2, 3);
    let bytes = encode_versioned_value(&qubit, ver).expect("encode qubit v1.2.3");
    let (_, version, _) = decode_versioned_value::<Qubit>(&bytes).expect("decode qubit v1.2.3");
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
}

#[test]
fn test_vec_of_qubits_versioned_v1() {
    let qubits = vec![
        Qubit {
            qubit_id: 0,
            state: QubitState::Ground,
            fidelity_x1000: 999,
            coherence_time_us: 100,
        },
        Qubit {
            qubit_id: 1,
            state: QubitState::Excited,
            fidelity_x1000: 997,
            coherence_time_us: 120,
        },
        Qubit {
            qubit_id: 2,
            state: QubitState::Superposition,
            fidelity_x1000: 995,
            coherence_time_us: 90,
        },
    ];
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&qubits, ver).expect("encode vec of qubits v1");
    let (decoded, version, consumed) =
        decode_versioned_value::<Vec<Qubit>>(&bytes).expect("decode vec of qubits v1");
    assert_eq!(decoded, qubits);
    assert_eq!(decoded.len(), 3);
    assert_eq!(version.major, 1);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_vec_of_error_syndromes_versioned_v2() {
    let syndromes = vec![
        ErrorSyndrome::None,
        ErrorSyndrome::BitFlip,
        ErrorSyndrome::PhaseFlip,
        ErrorSyndrome::Both,
        ErrorSyndrome::BitFlip,
    ];
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&syndromes, ver).expect("encode vec of syndromes v2");
    let (decoded, version, consumed) =
        decode_versioned_value::<Vec<ErrorSyndrome>>(&bytes).expect("decode vec of syndromes v2");
    assert_eq!(decoded, syndromes);
    assert_eq!(decoded.len(), 5);
    assert_eq!(version.major, 2);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_circuit_pauli_gates_roundtrip() {
    let circuit = QuantumCircuit {
        circuit_id: 400,
        name: String::from("pauli_sequence"),
        num_qubits: 1,
        gates: vec![
            QuantumGate {
                gate_id: 20,
                gate_type: GateType::PauliX,
                target_qubit: 0,
                control_qubit: None,
                phase_rad_x1000: 0,
            },
            QuantumGate {
                gate_id: 21,
                gate_type: GateType::PauliY,
                target_qubit: 0,
                control_qubit: None,
                phase_rad_x1000: 0,
            },
            QuantumGate {
                gate_id: 22,
                gate_type: GateType::PauliZ,
                target_qubit: 0,
                control_qubit: None,
                phase_rad_x1000: 0,
            },
        ],
        depth: 3,
    };
    let bytes = encode_to_vec(&circuit).expect("encode pauli sequence circuit");
    let (decoded, consumed) =
        decode_from_slice::<QuantumCircuit>(&bytes).expect("decode pauli sequence circuit");
    assert_eq!(decoded, circuit);
    assert_eq!(decoded.gates[0].gate_type, GateType::PauliX);
    assert_eq!(decoded.gates[1].gate_type, GateType::PauliY);
    assert_eq!(decoded.gates[2].gate_type, GateType::PauliZ);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_consumed_bytes_nonzero_for_all_types() {
    let qubit = Qubit {
        qubit_id: 1,
        state: QubitState::Ground,
        fidelity_x1000: 990,
        coherence_time_us: 50,
    };
    let gate = QuantumGate {
        gate_id: 1,
        gate_type: GateType::Hadamard,
        target_qubit: 0,
        control_qubit: None,
        phase_rad_x1000: 0,
    };
    let cycle = ErrorCorrectionCycle {
        cycle_id: 1,
        timestamp: 0,
        syndromes: vec![ErrorSyndrome::None],
        corrections_applied: 0,
    };

    let qubit_bytes = encode_to_vec(&qubit).expect("encode qubit consumed");
    let gate_bytes = encode_to_vec(&gate).expect("encode gate consumed");
    let cycle_bytes = encode_to_vec(&cycle).expect("encode cycle consumed");

    let (_, qubit_consumed) =
        decode_from_slice::<Qubit>(&qubit_bytes).expect("decode qubit consumed");
    let (_, gate_consumed) =
        decode_from_slice::<QuantumGate>(&gate_bytes).expect("decode gate consumed");
    let (_, cycle_consumed) =
        decode_from_slice::<ErrorCorrectionCycle>(&cycle_bytes).expect("decode cycle consumed");

    assert!(qubit_consumed > 0);
    assert!(gate_consumed > 0);
    assert!(cycle_consumed > 0);
}

#[test]
fn test_quantum_result_empty_measurements_versioned_v1_2_3() {
    let result = QuantumResult {
        result_id: 8001,
        circuit_id: 100,
        measurements: vec![],
        fidelity_x1000: 0,
        shots: 0,
    };
    let ver = Version::new(1, 2, 3);
    let bytes = encode_versioned_value(&result, ver).expect("encode quantum result empty v1.2.3");
    let (decoded, version, consumed) = decode_versioned_value::<QuantumResult>(&bytes)
        .expect("decode quantum result empty v1.2.3");
    assert_eq!(decoded, result);
    assert_eq!(decoded.measurements.len(), 0);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert_eq!(consumed, bytes.len());
}
