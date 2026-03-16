#![cfg(feature = "compression-zstd")]
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

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum ReactionType {
    Synthesis,
    Decomposition,
    SingleReplacement,
    DoubleReplacement,
    Combustion,
    Redox,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum ReactorType {
    BatchReactor,
    CstrReactor,
    PfrReactor,
    SemibatchReactor,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct ChemicalSpecies {
    formula: String,
    molecular_weight_mg: u32,
    concentration_umol: u64,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct ReactionStep {
    step_id: u16,
    reaction_type: ReactionType,
    temperature_mk: u32,
    pressure_kpa: u32,
    duration_s: u32,
    conversion_pct: u8,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct ReactorRun {
    run_id: u64,
    reactor_type: ReactorType,
    reactants: Vec<ChemicalSpecies>,
    products: Vec<ChemicalSpecies>,
    steps: Vec<ReactionStep>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct ProcessDataLog {
    log_id: u64,
    runs: Vec<ReactorRun>,
    operator_id: u32,
}

// Test 1: Each ReactionType — Synthesis compress/decompress roundtrip
#[test]
fn test_synthesis_reaction_compress_decompress() {
    let step = ReactionStep {
        step_id: 1,
        reaction_type: ReactionType::Synthesis,
        temperature_mk: 298_000,
        pressure_kpa: 101,
        duration_s: 3600,
        conversion_pct: 85,
    };
    let encoded = encode_to_vec(&step).expect("encode synthesis step");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress synthesis step");
    let decompressed = decompress(&compressed).expect("decompress synthesis step");
    let decoded: ReactionStep = decode_from_slice(&decompressed)
        .expect("decode synthesis step")
        .0;
    assert_eq!(decoded, step);
}

// Test 2: Each ReactionType — Decomposition compress/decompress roundtrip
#[test]
fn test_decomposition_reaction_compress_decompress() {
    let step = ReactionStep {
        step_id: 2,
        reaction_type: ReactionType::Decomposition,
        temperature_mk: 500_000,
        pressure_kpa: 50,
        duration_s: 1800,
        conversion_pct: 92,
    };
    let encoded = encode_to_vec(&step).expect("encode decomposition step");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress decomposition step");
    let decompressed = decompress(&compressed).expect("decompress decomposition step");
    let decoded: ReactionStep = decode_from_slice(&decompressed)
        .expect("decode decomposition step")
        .0;
    assert_eq!(decoded, step);
}

// Test 3: Each ReactionType — SingleReplacement compress/decompress roundtrip
#[test]
fn test_single_replacement_reaction_compress_decompress() {
    let step = ReactionStep {
        step_id: 3,
        reaction_type: ReactionType::SingleReplacement,
        temperature_mk: 350_000,
        pressure_kpa: 200,
        duration_s: 7200,
        conversion_pct: 70,
    };
    let encoded = encode_to_vec(&step).expect("encode single replacement step");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress single replacement step");
    let decompressed = decompress(&compressed).expect("decompress single replacement step");
    let decoded: ReactionStep = decode_from_slice(&decompressed)
        .expect("decode single replacement step")
        .0;
    assert_eq!(decoded, step);
}

// Test 4: Each ReactionType — DoubleReplacement compress/decompress roundtrip
#[test]
fn test_double_replacement_reaction_compress_decompress() {
    let step = ReactionStep {
        step_id: 4,
        reaction_type: ReactionType::DoubleReplacement,
        temperature_mk: 310_000,
        pressure_kpa: 101,
        duration_s: 600,
        conversion_pct: 99,
    };
    let encoded = encode_to_vec(&step).expect("encode double replacement step");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress double replacement step");
    let decompressed = decompress(&compressed).expect("decompress double replacement step");
    let decoded: ReactionStep = decode_from_slice(&decompressed)
        .expect("decode double replacement step")
        .0;
    assert_eq!(decoded, step);
}

// Test 5: Each ReactorType — BatchReactor compress/decompress roundtrip
#[test]
fn test_batch_reactor_type_compress_decompress() {
    let run = ReactorRun {
        run_id: 1001,
        reactor_type: ReactorType::BatchReactor,
        reactants: vec![ChemicalSpecies {
            formula: "H2O".to_string(),
            molecular_weight_mg: 18_000,
            concentration_umol: 55_500_000,
        }],
        products: vec![],
        steps: vec![],
    };
    let encoded = encode_to_vec(&run).expect("encode batch reactor run");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress batch reactor run");
    let decompressed = decompress(&compressed).expect("decompress batch reactor run");
    let decoded: ReactorRun = decode_from_slice(&decompressed)
        .expect("decode batch reactor run")
        .0;
    assert_eq!(decoded, run);
}

// Test 6: Each ReactorType — CstrReactor compress/decompress roundtrip
#[test]
fn test_cstr_reactor_type_compress_decompress() {
    let run = ReactorRun {
        run_id: 1002,
        reactor_type: ReactorType::CstrReactor,
        reactants: vec![ChemicalSpecies {
            formula: "CH4".to_string(),
            molecular_weight_mg: 16_043,
            concentration_umol: 1_000_000,
        }],
        products: vec![],
        steps: vec![],
    };
    let encoded = encode_to_vec(&run).expect("encode CSTR reactor run");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress CSTR reactor run");
    let decompressed = decompress(&compressed).expect("decompress CSTR reactor run");
    let decoded: ReactorRun = decode_from_slice(&decompressed)
        .expect("decode CSTR reactor run")
        .0;
    assert_eq!(decoded, run);
}

// Test 7: Each ReactorType — PfrReactor compress/decompress roundtrip
#[test]
fn test_pfr_reactor_type_compress_decompress() {
    let run = ReactorRun {
        run_id: 1003,
        reactor_type: ReactorType::PfrReactor,
        reactants: vec![ChemicalSpecies {
            formula: "C6H12O6".to_string(),
            molecular_weight_mg: 180_156,
            concentration_umol: 500_000,
        }],
        products: vec![],
        steps: vec![],
    };
    let encoded = encode_to_vec(&run).expect("encode PFR reactor run");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress PFR reactor run");
    let decompressed = decompress(&compressed).expect("decompress PFR reactor run");
    let decoded: ReactorRun = decode_from_slice(&decompressed)
        .expect("decode PFR reactor run")
        .0;
    assert_eq!(decoded, run);
}

// Test 8: Each ReactorType — SemibatchReactor compress/decompress roundtrip
#[test]
fn test_semibatch_reactor_type_compress_decompress() {
    let run = ReactorRun {
        run_id: 1004,
        reactor_type: ReactorType::SemibatchReactor,
        reactants: vec![ChemicalSpecies {
            formula: "NH3".to_string(),
            molecular_weight_mg: 17_031,
            concentration_umol: 2_000_000,
        }],
        products: vec![],
        steps: vec![],
    };
    let encoded = encode_to_vec(&run).expect("encode semibatch reactor run");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress semibatch reactor run");
    let decompressed = decompress(&compressed).expect("decompress semibatch reactor run");
    let decoded: ReactorRun = decode_from_slice(&decompressed)
        .expect("decode semibatch reactor run")
        .0;
    assert_eq!(decoded, run);
}

// Test 9: ChemicalSpecies compress roundtrip
#[test]
fn test_chemical_species_compress_roundtrip() {
    let species = ChemicalSpecies {
        formula: "C2H5OH".to_string(),
        molecular_weight_mg: 46_068,
        concentration_umol: 750_000,
    };
    let encoded = encode_to_vec(&species).expect("encode chemical species");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress chemical species");
    let decompressed = decompress(&compressed).expect("decompress chemical species");
    let decoded: ChemicalSpecies = decode_from_slice(&decompressed)
        .expect("decode chemical species")
        .0;
    assert_eq!(decoded, species);
}

// Test 10: ReactionStep compress/decompress roundtrip
#[test]
fn test_reaction_step_compress_decompress() {
    let step = ReactionStep {
        step_id: 42,
        reaction_type: ReactionType::Redox,
        temperature_mk: 450_000,
        pressure_kpa: 500,
        duration_s: 10800,
        conversion_pct: 75,
    };
    let encoded = encode_to_vec(&step).expect("encode reaction step");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress reaction step");
    let decompressed = decompress(&compressed).expect("decompress reaction step");
    let decoded: ReactionStep = decode_from_slice(&decompressed)
        .expect("decode reaction step")
        .0;
    assert_eq!(decoded, step);
}

// Test 11: ReactorRun with 5 steps compress/decompress roundtrip
#[test]
fn test_reactor_run_with_five_steps() {
    let run = ReactorRun {
        run_id: 9999,
        reactor_type: ReactorType::PfrReactor,
        reactants: vec![
            ChemicalSpecies {
                formula: "N2".to_string(),
                molecular_weight_mg: 28_014,
                concentration_umol: 3_000_000,
            },
            ChemicalSpecies {
                formula: "H2".to_string(),
                molecular_weight_mg: 2_016,
                concentration_umol: 9_000_000,
            },
        ],
        products: vec![ChemicalSpecies {
            formula: "NH3".to_string(),
            molecular_weight_mg: 17_031,
            concentration_umol: 6_000_000,
        }],
        steps: vec![
            ReactionStep {
                step_id: 1,
                reaction_type: ReactionType::Synthesis,
                temperature_mk: 700_000,
                pressure_kpa: 20_000,
                duration_s: 300,
                conversion_pct: 15,
            },
            ReactionStep {
                step_id: 2,
                reaction_type: ReactionType::Synthesis,
                temperature_mk: 720_000,
                pressure_kpa: 20_000,
                duration_s: 300,
                conversion_pct: 30,
            },
            ReactionStep {
                step_id: 3,
                reaction_type: ReactionType::Synthesis,
                temperature_mk: 710_000,
                pressure_kpa: 20_000,
                duration_s: 300,
                conversion_pct: 45,
            },
            ReactionStep {
                step_id: 4,
                reaction_type: ReactionType::Synthesis,
                temperature_mk: 700_000,
                pressure_kpa: 20_000,
                duration_s: 300,
                conversion_pct: 55,
            },
            ReactionStep {
                step_id: 5,
                reaction_type: ReactionType::Synthesis,
                temperature_mk: 690_000,
                pressure_kpa: 20_000,
                duration_s: 300,
                conversion_pct: 62,
            },
        ],
    };
    let encoded = encode_to_vec(&run).expect("encode reactor run with 5 steps");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress reactor run with 5 steps");
    let decompressed = decompress(&compressed).expect("decompress reactor run with 5 steps");
    let decoded: ReactorRun = decode_from_slice(&decompressed)
        .expect("decode reactor run with 5 steps")
        .0;
    assert_eq!(decoded, run);
}

// Test 12: ProcessDataLog compress/decompress roundtrip
#[test]
fn test_process_data_log_compress_decompress() {
    let log = ProcessDataLog {
        log_id: 123456,
        runs: vec![ReactorRun {
            run_id: 1,
            reactor_type: ReactorType::BatchReactor,
            reactants: vec![ChemicalSpecies {
                formula: "SO2".to_string(),
                molecular_weight_mg: 64_066,
                concentration_umol: 400_000,
            }],
            products: vec![ChemicalSpecies {
                formula: "SO3".to_string(),
                molecular_weight_mg: 80_066,
                concentration_umol: 380_000,
            }],
            steps: vec![ReactionStep {
                step_id: 1,
                reaction_type: ReactionType::Redox,
                temperature_mk: 720_000,
                pressure_kpa: 101,
                duration_s: 1800,
                conversion_pct: 95,
            }],
        }],
        operator_id: 42,
    };
    let encoded = encode_to_vec(&log).expect("encode process data log");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress process data log");
    let decompressed = decompress(&compressed).expect("decompress process data log");
    let decoded: ProcessDataLog = decode_from_slice(&decompressed)
        .expect("decode process data log")
        .0;
    assert_eq!(decoded, log);
}

// Test 13: Large log (100 runs) compression ratio check — compressed <= raw
#[test]
fn test_large_log_compression_ratio() {
    let template_run = ReactorRun {
        run_id: 0,
        reactor_type: ReactorType::CstrReactor,
        reactants: vec![
            ChemicalSpecies {
                formula: "C3H8".to_string(),
                molecular_weight_mg: 44_097,
                concentration_umol: 1_000_000,
            },
            ChemicalSpecies {
                formula: "O2".to_string(),
                molecular_weight_mg: 31_998,
                concentration_umol: 5_000_000,
            },
        ],
        products: vec![
            ChemicalSpecies {
                formula: "CO2".to_string(),
                molecular_weight_mg: 44_010,
                concentration_umol: 3_000_000,
            },
            ChemicalSpecies {
                formula: "H2O".to_string(),
                molecular_weight_mg: 18_015,
                concentration_umol: 4_000_000,
            },
        ],
        steps: vec![ReactionStep {
            step_id: 1,
            reaction_type: ReactionType::Combustion,
            temperature_mk: 1_000_000,
            pressure_kpa: 101,
            duration_s: 60,
            conversion_pct: 99,
        }],
    };
    let runs: Vec<ReactorRun> = (0u64..100)
        .map(|i| ReactorRun {
            run_id: i,
            ..template_run.clone()
        })
        .collect();
    let log = ProcessDataLog {
        log_id: 999,
        runs,
        operator_id: 1,
    };
    let encoded = encode_to_vec(&log).expect("encode large process data log");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress large process data log");
    assert!(
        compressed.len() <= encoded.len(),
        "compressed ({}) should be <= raw ({}) for large repetitive log",
        compressed.len(),
        encoded.len()
    );
}

// Test 14: Repetitive reactor data (1000+ elements) compresses smaller than raw
#[test]
fn test_repetitive_reactor_data_compresses_smaller() {
    let species_vec: Vec<ChemicalSpecies> = (0..1000)
        .map(|_| ChemicalSpecies {
            formula: "C6H12O6".to_string(),
            molecular_weight_mg: 180_156,
            concentration_umol: 250_000,
        })
        .collect();
    let encoded = encode_to_vec(&species_vec).expect("encode repetitive species vec");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress repetitive species vec");
    assert!(
        compressed.len() < encoded.len(),
        "repetitive data ({} bytes compressed) must compress smaller than raw ({} bytes)",
        compressed.len(),
        encoded.len()
    );
}

// Test 15: Empty reactor run compress/decompress roundtrip
#[test]
fn test_empty_reactor_run() {
    let run = ReactorRun {
        run_id: 0,
        reactor_type: ReactorType::BatchReactor,
        reactants: vec![],
        products: vec![],
        steps: vec![],
    };
    let encoded = encode_to_vec(&run).expect("encode empty reactor run");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress empty reactor run");
    let decompressed = decompress(&compressed).expect("decompress empty reactor run");
    let decoded: ReactorRun = decode_from_slice(&decompressed)
        .expect("decode empty reactor run")
        .0;
    assert_eq!(decoded, run);
}

// Test 16: Vec<ChemicalSpecies> compress/decompress roundtrip
#[test]
fn test_vec_chemical_species_compress() {
    let species_vec = vec![
        ChemicalSpecies {
            formula: "H2SO4".to_string(),
            molecular_weight_mg: 98_079,
            concentration_umol: 100_000,
        },
        ChemicalSpecies {
            formula: "NaOH".to_string(),
            molecular_weight_mg: 39_997,
            concentration_umol: 200_000,
        },
        ChemicalSpecies {
            formula: "Na2SO4".to_string(),
            molecular_weight_mg: 142_042,
            concentration_umol: 100_000,
        },
        ChemicalSpecies {
            formula: "H2O".to_string(),
            molecular_weight_mg: 18_015,
            concentration_umol: 100_000,
        },
    ];
    let encoded = encode_to_vec(&species_vec).expect("encode species vec");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress species vec");
    let decompressed = decompress(&compressed).expect("decompress species vec");
    let decoded: Vec<ChemicalSpecies> = decode_from_slice(&decompressed)
        .expect("decode species vec")
        .0;
    assert_eq!(decoded, species_vec);
}

// Test 17: Vec<ReactionStep> compress/decompress roundtrip
#[test]
fn test_vec_reaction_steps_compress() {
    let steps = vec![
        ReactionStep {
            step_id: 10,
            reaction_type: ReactionType::Synthesis,
            temperature_mk: 298_000,
            pressure_kpa: 101,
            duration_s: 600,
            conversion_pct: 20,
        },
        ReactionStep {
            step_id: 11,
            reaction_type: ReactionType::Redox,
            temperature_mk: 350_000,
            pressure_kpa: 202,
            duration_s: 1200,
            conversion_pct: 40,
        },
        ReactionStep {
            step_id: 12,
            reaction_type: ReactionType::Decomposition,
            temperature_mk: 500_000,
            pressure_kpa: 50,
            duration_s: 1800,
            conversion_pct: 80,
        },
    ];
    let encoded = encode_to_vec(&steps).expect("encode reaction steps vec");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress reaction steps vec");
    let decompressed = decompress(&compressed).expect("decompress reaction steps vec");
    let decoded: Vec<ReactionStep> = decode_from_slice(&decompressed)
        .expect("decode reaction steps vec")
        .0;
    assert_eq!(decoded, steps);
}

// Test 18: Decompress gives exactly the original encoded bytes
#[test]
fn test_decompress_gives_original_bytes() {
    let species = ChemicalSpecies {
        formula: "Fe2O3".to_string(),
        molecular_weight_mg: 159_688,
        concentration_umol: 50_000,
    };
    let original_bytes = encode_to_vec(&species).expect("encode iron oxide species");
    let compressed =
        compress(&original_bytes, Compression::Zstd).expect("compress iron oxide species");
    let recovered_bytes = decompress(&compressed).expect("decompress iron oxide species");
    assert_eq!(
        original_bytes, recovered_bytes,
        "decompressed bytes must exactly match original encoded bytes"
    );
}

// Test 19: Combustion reaction roundtrip with high conversion
#[test]
fn test_combustion_reaction_roundtrip() {
    let step = ReactionStep {
        step_id: 5,
        reaction_type: ReactionType::Combustion,
        temperature_mk: 1_200_000,
        pressure_kpa: 300,
        duration_s: 120,
        conversion_pct: 98,
    };
    let encoded = encode_to_vec(&step).expect("encode combustion step");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress combustion step");
    let decompressed = decompress(&compressed).expect("decompress combustion step");
    let decoded: ReactionStep = decode_from_slice(&decompressed)
        .expect("decode combustion step")
        .0;
    assert_eq!(decoded, step);
    assert_eq!(decoded.reaction_type, ReactionType::Combustion);
}

// Test 20: High-temperature step compress/decompress
#[test]
fn test_high_temperature_step() {
    let step = ReactionStep {
        step_id: 100,
        reaction_type: ReactionType::Combustion,
        temperature_mk: 3_000_000,
        pressure_kpa: 5_000,
        duration_s: 30,
        conversion_pct: 100,
    };
    let encoded = encode_to_vec(&step).expect("encode high-temperature step");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress high-temperature step");
    let decompressed = decompress(&compressed).expect("decompress high-temperature step");
    let decoded: ReactionStep = decode_from_slice(&decompressed)
        .expect("decode high-temperature step")
        .0;
    assert_eq!(decoded.temperature_mk, 3_000_000);
    assert_eq!(decoded, step);
}

// Test 21: High-pressure step compress/decompress
#[test]
fn test_high_pressure_step() {
    let step = ReactionStep {
        step_id: 200,
        reaction_type: ReactionType::Synthesis,
        temperature_mk: 700_000,
        pressure_kpa: 35_000,
        duration_s: 900,
        conversion_pct: 50,
    };
    let encoded = encode_to_vec(&step).expect("encode high-pressure step");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress high-pressure step");
    let decompressed = decompress(&compressed).expect("decompress high-pressure step");
    let decoded: ReactionStep = decode_from_slice(&decompressed)
        .expect("decode high-pressure step")
        .0;
    assert_eq!(decoded.pressure_kpa, 35_000);
    assert_eq!(decoded, step);
}

// Test 22: Operator ID boundary values in ProcessDataLog compress/decompress
#[test]
fn test_operator_id_boundary() {
    let log_min = ProcessDataLog {
        log_id: 0,
        runs: vec![],
        operator_id: 0,
    };
    let log_max = ProcessDataLog {
        log_id: u64::MAX,
        runs: vec![],
        operator_id: u32::MAX,
    };

    let encoded_min = encode_to_vec(&log_min).expect("encode min operator log");
    let compressed_min =
        compress(&encoded_min, Compression::Zstd).expect("compress min operator log");
    let decompressed_min = decompress(&compressed_min).expect("decompress min operator log");
    let decoded_min: ProcessDataLog = decode_from_slice(&decompressed_min)
        .expect("decode min operator log")
        .0;
    assert_eq!(decoded_min, log_min);

    let encoded_max = encode_to_vec(&log_max).expect("encode max operator log");
    let compressed_max =
        compress(&encoded_max, Compression::Zstd).expect("compress max operator log");
    let decompressed_max = decompress(&compressed_max).expect("decompress max operator log");
    let decoded_max: ProcessDataLog = decode_from_slice(&decompressed_max)
        .expect("decode max operator log")
        .0;
    assert_eq!(decoded_max, log_max);
    assert_eq!(decoded_max.operator_id, u32::MAX);
}
