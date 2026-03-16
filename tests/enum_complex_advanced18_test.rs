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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

#[derive(Debug, PartialEq, Encode, Decode)]
enum QubitState {
    Zero,
    One,
    Plus,
    Minus,
    Custom { real: i32, imag: i32 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum QuantumGate {
    Hadamard,
    PauliX,
    PauliY,
    PauliZ,
    CNOT,
    Toffoli,
    Phase { angle_mrad: i32 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct QubitRegister {
    num_qubits: u8,
    states: Vec<QubitState>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GateOperation {
    gate: QuantumGate,
    target_qubit: u8,
    control_qubits: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct QuantumCircuit {
    name: String,
    num_qubits: u8,
    operations: Vec<GateOperation>,
    measurement_qubits: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MeasurementResult {
    qubit_id: u8,
    outcome: bool,
    probability_percent: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct QuantumExperiment {
    circuit: QuantumCircuit,
    shots: u32,
    results: Vec<MeasurementResult>,
}

// Test 1: QubitState::Zero roundtrip
#[test]
fn test_qubit_state_zero_roundtrip() {
    let val = QubitState::Zero;
    let encoded = encode_to_vec(&val).expect("encode QubitState::Zero");
    let (decoded, _) = decode_from_slice::<QubitState>(&encoded).expect("decode QubitState::Zero");
    assert_eq!(val, decoded);
}

// Test 2: QubitState::One roundtrip
#[test]
fn test_qubit_state_one_roundtrip() {
    let val = QubitState::One;
    let encoded = encode_to_vec(&val).expect("encode QubitState::One");
    let (decoded, _) = decode_from_slice::<QubitState>(&encoded).expect("decode QubitState::One");
    assert_eq!(val, decoded);
}

// Test 3: QubitState::Plus roundtrip
#[test]
fn test_qubit_state_plus_roundtrip() {
    let val = QubitState::Plus;
    let encoded = encode_to_vec(&val).expect("encode QubitState::Plus");
    let (decoded, _) = decode_from_slice::<QubitState>(&encoded).expect("decode QubitState::Plus");
    assert_eq!(val, decoded);
}

// Test 4: QubitState::Minus roundtrip
#[test]
fn test_qubit_state_minus_roundtrip() {
    let val = QubitState::Minus;
    let encoded = encode_to_vec(&val).expect("encode QubitState::Minus");
    let (decoded, _) = decode_from_slice::<QubitState>(&encoded).expect("decode QubitState::Minus");
    assert_eq!(val, decoded);
}

// Test 5: QubitState::Custom roundtrip
#[test]
fn test_qubit_state_custom_roundtrip() {
    let val = QubitState::Custom {
        real: 707,
        imag: -707,
    };
    let encoded = encode_to_vec(&val).expect("encode QubitState::Custom");
    let (decoded, _) =
        decode_from_slice::<QubitState>(&encoded).expect("decode QubitState::Custom");
    assert_eq!(val, decoded);
}

// Test 6: QuantumGate variants — Hadamard, PauliX, PauliY, PauliZ roundtrips
#[test]
fn test_quantum_gate_simple_variants_roundtrip() {
    let gates = vec![
        QuantumGate::Hadamard,
        QuantumGate::PauliX,
        QuantumGate::PauliY,
        QuantumGate::PauliZ,
    ];
    for gate in gates {
        let encoded = encode_to_vec(&gate).expect("encode QuantumGate simple variant");
        let (decoded, _) =
            decode_from_slice::<QuantumGate>(&encoded).expect("decode QuantumGate simple variant");
        assert_eq!(gate, decoded);
    }
}

// Test 7: QuantumGate::CNOT and Toffoli roundtrip
#[test]
fn test_quantum_gate_cnot_toffoli_roundtrip() {
    let cnot = QuantumGate::CNOT;
    let encoded = encode_to_vec(&cnot).expect("encode CNOT");
    let (decoded, _) = decode_from_slice::<QuantumGate>(&encoded).expect("decode CNOT");
    assert_eq!(cnot, decoded);

    let toffoli = QuantumGate::Toffoli;
    let encoded = encode_to_vec(&toffoli).expect("encode Toffoli");
    let (decoded, _) = decode_from_slice::<QuantumGate>(&encoded).expect("decode Toffoli");
    assert_eq!(toffoli, decoded);
}

// Test 8: QuantumGate::Phase roundtrip with various angles
#[test]
fn test_quantum_gate_phase_roundtrip() {
    let val = QuantumGate::Phase { angle_mrad: 1570 }; // ~pi/2 in mrad
    let encoded = encode_to_vec(&val).expect("encode Phase gate");
    let (decoded, _) = decode_from_slice::<QuantumGate>(&encoded).expect("decode Phase gate");
    assert_eq!(val, decoded);

    let val_neg = QuantumGate::Phase { angle_mrad: -3141 };
    let encoded_neg = encode_to_vec(&val_neg).expect("encode negative Phase gate");
    let (decoded_neg, _) =
        decode_from_slice::<QuantumGate>(&encoded_neg).expect("decode negative Phase gate");
    assert_eq!(val_neg, decoded_neg);
}

// Test 9: QubitRegister roundtrip with multiple states
#[test]
fn test_qubit_register_roundtrip() {
    let val = QubitRegister {
        num_qubits: 3,
        states: vec![QubitState::Zero, QubitState::One, QubitState::Plus],
    };
    let encoded = encode_to_vec(&val).expect("encode QubitRegister");
    let (decoded, _) = decode_from_slice::<QubitRegister>(&encoded).expect("decode QubitRegister");
    assert_eq!(val, decoded);
}

// Test 10: GateOperation roundtrip — single-qubit gate, no control qubits
#[test]
fn test_gate_operation_single_qubit_roundtrip() {
    let val = GateOperation {
        gate: QuantumGate::Hadamard,
        target_qubit: 0,
        control_qubits: vec![],
    };
    let encoded = encode_to_vec(&val).expect("encode GateOperation single qubit");
    let (decoded, _) =
        decode_from_slice::<GateOperation>(&encoded).expect("decode GateOperation single qubit");
    assert_eq!(val, decoded);
}

// Test 11: GateOperation roundtrip — CNOT with one control qubit
#[test]
fn test_gate_operation_cnot_roundtrip() {
    let val = GateOperation {
        gate: QuantumGate::CNOT,
        target_qubit: 1,
        control_qubits: vec![0],
    };
    let encoded = encode_to_vec(&val).expect("encode GateOperation CNOT");
    let (decoded, _) =
        decode_from_slice::<GateOperation>(&encoded).expect("decode GateOperation CNOT");
    assert_eq!(val, decoded);
}

// Test 12: QuantumCircuit roundtrip — empty operations
#[test]
fn test_quantum_circuit_empty_roundtrip() {
    let val = QuantumCircuit {
        name: String::from("empty_circuit"),
        num_qubits: 2,
        operations: vec![],
        measurement_qubits: vec![0, 1],
    };
    let encoded = encode_to_vec(&val).expect("encode empty QuantumCircuit");
    let (decoded, _) =
        decode_from_slice::<QuantumCircuit>(&encoded).expect("decode empty QuantumCircuit");
    assert_eq!(val, decoded);
}

// Test 13: QuantumCircuit roundtrip — small circuit (Bell state preparation)
#[test]
fn test_quantum_circuit_bell_state_roundtrip() {
    let val = QuantumCircuit {
        name: String::from("bell_state"),
        num_qubits: 2,
        operations: vec![
            GateOperation {
                gate: QuantumGate::Hadamard,
                target_qubit: 0,
                control_qubits: vec![],
            },
            GateOperation {
                gate: QuantumGate::CNOT,
                target_qubit: 1,
                control_qubits: vec![0],
            },
        ],
        measurement_qubits: vec![0, 1],
    };
    let encoded = encode_to_vec(&val).expect("encode bell state QuantumCircuit");
    let (decoded, _) =
        decode_from_slice::<QuantumCircuit>(&encoded).expect("decode bell state QuantumCircuit");
    assert_eq!(val, decoded);
}

// Test 14: QuantumCircuit roundtrip — large circuit with many operations
#[test]
fn test_quantum_circuit_large_roundtrip() {
    let operations: Vec<GateOperation> = (0u8..8)
        .map(|i| GateOperation {
            gate: if i % 2 == 0 {
                QuantumGate::PauliX
            } else {
                QuantumGate::Phase {
                    angle_mrad: i as i32 * 100,
                }
            },
            target_qubit: i % 4,
            control_qubits: if i % 3 == 0 {
                vec![]
            } else {
                vec![(i + 1) % 4]
            },
        })
        .collect();

    let val = QuantumCircuit {
        name: String::from("large_test_circuit"),
        num_qubits: 4,
        operations,
        measurement_qubits: vec![0, 1, 2, 3],
    };
    let encoded = encode_to_vec(&val).expect("encode large QuantumCircuit");
    let (decoded, _) =
        decode_from_slice::<QuantumCircuit>(&encoded).expect("decode large QuantumCircuit");
    assert_eq!(val, decoded);
}

// Test 15: MeasurementResult roundtrip
#[test]
fn test_measurement_result_roundtrip() {
    let val = MeasurementResult {
        qubit_id: 3,
        outcome: true,
        probability_percent: 75,
    };
    let encoded = encode_to_vec(&val).expect("encode MeasurementResult");
    let (decoded, _) =
        decode_from_slice::<MeasurementResult>(&encoded).expect("decode MeasurementResult");
    assert_eq!(val, decoded);
}

// Test 16: QuantumExperiment roundtrip
#[test]
fn test_quantum_experiment_roundtrip() {
    let val = QuantumExperiment {
        circuit: QuantumCircuit {
            name: String::from("grover_2q"),
            num_qubits: 2,
            operations: vec![
                GateOperation {
                    gate: QuantumGate::Hadamard,
                    target_qubit: 0,
                    control_qubits: vec![],
                },
                GateOperation {
                    gate: QuantumGate::Hadamard,
                    target_qubit: 1,
                    control_qubits: vec![],
                },
                GateOperation {
                    gate: QuantumGate::Toffoli,
                    target_qubit: 0,
                    control_qubits: vec![1],
                },
            ],
            measurement_qubits: vec![0, 1],
        },
        shots: 1024,
        results: vec![
            MeasurementResult {
                qubit_id: 0,
                outcome: false,
                probability_percent: 50,
            },
            MeasurementResult {
                qubit_id: 1,
                outcome: true,
                probability_percent: 50,
            },
        ],
    };
    let encoded = encode_to_vec(&val).expect("encode QuantumExperiment");
    let (decoded, _) =
        decode_from_slice::<QuantumExperiment>(&encoded).expect("decode QuantumExperiment");
    assert_eq!(val, decoded);
}

// Test 17: Big-endian config roundtrip for QuantumCircuit
#[test]
fn test_quantum_circuit_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let val = QuantumCircuit {
        name: String::from("be_circuit"),
        num_qubits: 1,
        operations: vec![GateOperation {
            gate: QuantumGate::PauliZ,
            target_qubit: 0,
            control_qubits: vec![],
        }],
        measurement_qubits: vec![0],
    };
    let encoded = encode_to_vec_with_config(&val, cfg).expect("encode QuantumCircuit big-endian");
    let (decoded, _) = decode_from_slice_with_config::<QuantumCircuit, _>(&encoded, cfg)
        .expect("decode QuantumCircuit big-endian");
    assert_eq!(val, decoded);
}

// Test 18: Fixed-int encoding config roundtrip for QuantumExperiment
#[test]
fn test_quantum_experiment_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = QuantumExperiment {
        circuit: QuantumCircuit {
            name: String::from("fixed_int_circuit"),
            num_qubits: 2,
            operations: vec![],
            measurement_qubits: vec![0, 1],
        },
        shots: 512,
        results: vec![MeasurementResult {
            qubit_id: 0,
            outcome: false,
            probability_percent: 100,
        }],
    };
    let encoded = encode_to_vec_with_config(&val, cfg).expect("encode QuantumExperiment fixed-int");
    let (decoded, _) = decode_from_slice_with_config::<QuantumExperiment, _>(&encoded, cfg)
        .expect("decode QuantumExperiment fixed-int");
    assert_eq!(val, decoded);
}

// Test 19: Consumed bytes check for QubitRegister
#[test]
fn test_qubit_register_consumed_bytes() {
    let val = QubitRegister {
        num_qubits: 2,
        states: vec![QubitState::Zero, QubitState::One],
    };
    let encoded = encode_to_vec(&val).expect("encode QubitRegister for bytes check");
    let (decoded, consumed) =
        decode_from_slice::<QubitRegister>(&encoded).expect("decode QubitRegister for bytes check");
    assert_eq!(val, decoded);
    assert_eq!(consumed, encoded.len(), "all bytes must be consumed");
}

// Test 20: Consumed bytes check for GateOperation with Phase gate
#[test]
fn test_gate_operation_phase_consumed_bytes() {
    let val = GateOperation {
        gate: QuantumGate::Phase { angle_mrad: 3141 },
        target_qubit: 2,
        control_qubits: vec![0, 1],
    };
    let encoded = encode_to_vec(&val).expect("encode GateOperation Phase for bytes check");
    let (decoded, consumed) = decode_from_slice::<GateOperation>(&encoded)
        .expect("decode GateOperation Phase for bytes check");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        encoded.len(),
        "all bytes consumed for Phase GateOperation"
    );
}

// Test 21: Vec<QuantumCircuit> roundtrip — multiple circuits in a single encoded buffer
#[test]
fn test_vec_quantum_circuit_roundtrip() {
    let circuits = vec![
        QuantumCircuit {
            name: String::from("circuit_alpha"),
            num_qubits: 1,
            operations: vec![GateOperation {
                gate: QuantumGate::Hadamard,
                target_qubit: 0,
                control_qubits: vec![],
            }],
            measurement_qubits: vec![0],
        },
        QuantumCircuit {
            name: String::from("circuit_beta"),
            num_qubits: 2,
            operations: vec![
                GateOperation {
                    gate: QuantumGate::PauliX,
                    target_qubit: 0,
                    control_qubits: vec![],
                },
                GateOperation {
                    gate: QuantumGate::CNOT,
                    target_qubit: 1,
                    control_qubits: vec![0],
                },
            ],
            measurement_qubits: vec![0, 1],
        },
        QuantumCircuit {
            name: String::from("circuit_gamma"),
            num_qubits: 3,
            operations: vec![],
            measurement_qubits: vec![0, 1, 2],
        },
    ];
    let encoded = encode_to_vec(&circuits).expect("encode Vec<QuantumCircuit>");
    let (decoded, consumed) =
        decode_from_slice::<Vec<QuantumCircuit>>(&encoded).expect("decode Vec<QuantumCircuit>");
    assert_eq!(circuits, decoded);
    assert_eq!(
        consumed,
        encoded.len(),
        "all bytes consumed for Vec<QuantumCircuit>"
    );
}

// Test 22: Distinct discriminants — different QubitState and QuantumGate variants produce distinct encodings
#[test]
fn test_distinct_discriminants() {
    let qubit_states = vec![
        QubitState::Zero,
        QubitState::One,
        QubitState::Plus,
        QubitState::Minus,
        QubitState::Custom { real: 1, imag: 0 },
    ];
    let mut state_encodings: Vec<Vec<u8>> = Vec::new();
    for state in &qubit_states {
        let enc = encode_to_vec(state).expect("encode QubitState variant for discriminant check");
        state_encodings.push(enc);
    }
    // All encodings must be pairwise distinct
    for i in 0..state_encodings.len() {
        for j in (i + 1)..state_encodings.len() {
            assert_ne!(
                state_encodings[i], state_encodings[j],
                "QubitState variants {} and {} must have distinct encodings",
                i, j
            );
        }
    }

    let quantum_gates = vec![
        QuantumGate::Hadamard,
        QuantumGate::PauliX,
        QuantumGate::PauliY,
        QuantumGate::PauliZ,
        QuantumGate::CNOT,
        QuantumGate::Toffoli,
        QuantumGate::Phase { angle_mrad: 0 },
    ];
    let mut gate_encodings: Vec<Vec<u8>> = Vec::new();
    for gate in &quantum_gates {
        let enc = encode_to_vec(gate).expect("encode QuantumGate variant for discriminant check");
        gate_encodings.push(enc);
    }
    // All gate encodings must be pairwise distinct
    for i in 0..gate_encodings.len() {
        for j in (i + 1)..gate_encodings.len() {
            assert_ne!(
                gate_encodings[i], gate_encodings[j],
                "QuantumGate variants {} and {} must have distinct encodings",
                i, j
            );
        }
    }
}
