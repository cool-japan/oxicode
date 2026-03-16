#![cfg(all(feature = "compression-lz4", feature = "compression-zstd"))]
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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CrystalSystem {
    Cubic,
    Tetragonal,
    Orthorhombic,
    Hexagonal,
    Trigonal,
    Monoclinic,
    Triclinic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BondType {
    Ionic,
    Covalent,
    Metallic,
    VanDerWaals,
    Hydrogen,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MaterialPhase {
    Solid,
    Liquid,
    Gas,
    Plasma,
    Amorphous,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DefectType {
    Vacancy,
    Interstitial,
    Substitution,
    Dislocation,
    Grain,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AtomPosition {
    x_pm: i32,
    y_pm: i32,
    z_pm: i32,
    element: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UnitCell {
    a_pm: u32,
    b_pm: u32,
    c_pm: u32,
    alpha_x1000: u32,
    beta_x1000: u32,
    gamma_x1000: u32,
    crystal_system: CrystalSystem,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiffractionPeak {
    two_theta_x100: u32,
    intensity: u32,
    hkl: [i8; 3],
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MaterialSample {
    sample_id: u32,
    name: String,
    phase: MaterialPhase,
    unit_cell: UnitCell,
    diffraction_peaks: Vec<DiffractionPeak>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MolecularDynamicsFrame {
    frame_id: u64,
    time_ps: u64,
    atoms: Vec<AtomPosition>,
    temperature_k: u32,
    pressure_mpa: u32,
}

// --- Test 1: CrystalSystem enum roundtrip with LZ4 ---
#[test]
fn test_crystal_system_lz4_roundtrip() {
    let systems = vec![
        CrystalSystem::Cubic,
        CrystalSystem::Tetragonal,
        CrystalSystem::Orthorhombic,
        CrystalSystem::Hexagonal,
        CrystalSystem::Trigonal,
        CrystalSystem::Monoclinic,
        CrystalSystem::Triclinic,
    ];
    for system in &systems {
        let encoded = encode_to_vec(system).expect("encode CrystalSystem");
        let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress CrystalSystem");
        let decompressed = decompress(&compressed).expect("lz4 decompress CrystalSystem");
        let (decoded, _): (CrystalSystem, usize) =
            decode_from_slice(&decompressed).expect("decode CrystalSystem");
        assert_eq!(system, &decoded);
    }
}

// --- Test 2: CrystalSystem enum roundtrip with Zstd ---
#[test]
fn test_crystal_system_zstd_roundtrip() {
    let systems = vec![
        CrystalSystem::Cubic,
        CrystalSystem::Tetragonal,
        CrystalSystem::Orthorhombic,
        CrystalSystem::Hexagonal,
        CrystalSystem::Trigonal,
        CrystalSystem::Monoclinic,
        CrystalSystem::Triclinic,
    ];
    for system in &systems {
        let encoded = encode_to_vec(system).expect("encode CrystalSystem");
        let compressed =
            compress(&encoded, Compression::Zstd).expect("zstd compress CrystalSystem");
        let decompressed = decompress(&compressed).expect("zstd decompress CrystalSystem");
        let (decoded, _): (CrystalSystem, usize) =
            decode_from_slice(&decompressed).expect("decode CrystalSystem");
        assert_eq!(system, &decoded);
    }
}

// --- Test 3: BondType enum roundtrip with LZ4 ---
#[test]
fn test_bond_type_lz4_roundtrip() {
    let bond_types = vec![
        BondType::Ionic,
        BondType::Covalent,
        BondType::Metallic,
        BondType::VanDerWaals,
        BondType::Hydrogen,
    ];
    for bond in &bond_types {
        let encoded = encode_to_vec(bond).expect("encode BondType");
        let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress BondType");
        let decompressed = decompress(&compressed).expect("lz4 decompress BondType");
        let (decoded, _): (BondType, usize) =
            decode_from_slice(&decompressed).expect("decode BondType");
        assert_eq!(bond, &decoded);
    }
}

// --- Test 4: BondType enum roundtrip with Zstd ---
#[test]
fn test_bond_type_zstd_roundtrip() {
    let bond_types = vec![
        BondType::Ionic,
        BondType::Covalent,
        BondType::Metallic,
        BondType::VanDerWaals,
        BondType::Hydrogen,
    ];
    for bond in &bond_types {
        let encoded = encode_to_vec(bond).expect("encode BondType");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress BondType");
        let decompressed = decompress(&compressed).expect("zstd decompress BondType");
        let (decoded, _): (BondType, usize) =
            decode_from_slice(&decompressed).expect("decode BondType");
        assert_eq!(bond, &decoded);
    }
}

// --- Test 5: MaterialPhase enum roundtrip with LZ4 ---
#[test]
fn test_material_phase_lz4_roundtrip() {
    let phases = vec![
        MaterialPhase::Solid,
        MaterialPhase::Liquid,
        MaterialPhase::Gas,
        MaterialPhase::Plasma,
        MaterialPhase::Amorphous,
    ];
    for phase in &phases {
        let encoded = encode_to_vec(phase).expect("encode MaterialPhase");
        let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress MaterialPhase");
        let decompressed = decompress(&compressed).expect("lz4 decompress MaterialPhase");
        let (decoded, _): (MaterialPhase, usize) =
            decode_from_slice(&decompressed).expect("decode MaterialPhase");
        assert_eq!(phase, &decoded);
    }
}

// --- Test 6: MaterialPhase enum roundtrip with Zstd ---
#[test]
fn test_material_phase_zstd_roundtrip() {
    let phases = vec![
        MaterialPhase::Solid,
        MaterialPhase::Liquid,
        MaterialPhase::Gas,
        MaterialPhase::Plasma,
        MaterialPhase::Amorphous,
    ];
    for phase in &phases {
        let encoded = encode_to_vec(phase).expect("encode MaterialPhase");
        let compressed =
            compress(&encoded, Compression::Zstd).expect("zstd compress MaterialPhase");
        let decompressed = decompress(&compressed).expect("zstd decompress MaterialPhase");
        let (decoded, _): (MaterialPhase, usize) =
            decode_from_slice(&decompressed).expect("decode MaterialPhase");
        assert_eq!(phase, &decoded);
    }
}

// --- Test 7: DefectType enum roundtrip with LZ4 ---
#[test]
fn test_defect_type_lz4_roundtrip() {
    let defects = vec![
        DefectType::Vacancy,
        DefectType::Interstitial,
        DefectType::Substitution,
        DefectType::Dislocation,
        DefectType::Grain,
    ];
    for defect in &defects {
        let encoded = encode_to_vec(defect).expect("encode DefectType");
        let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress DefectType");
        let decompressed = decompress(&compressed).expect("lz4 decompress DefectType");
        let (decoded, _): (DefectType, usize) =
            decode_from_slice(&decompressed).expect("decode DefectType");
        assert_eq!(defect, &decoded);
    }
}

// --- Test 8: DefectType enum roundtrip with Zstd ---
#[test]
fn test_defect_type_zstd_roundtrip() {
    let defects = vec![
        DefectType::Vacancy,
        DefectType::Interstitial,
        DefectType::Substitution,
        DefectType::Dislocation,
        DefectType::Grain,
    ];
    for defect in &defects {
        let encoded = encode_to_vec(defect).expect("encode DefectType");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress DefectType");
        let decompressed = decompress(&compressed).expect("zstd decompress DefectType");
        let (decoded, _): (DefectType, usize) =
            decode_from_slice(&decompressed).expect("decode DefectType");
        assert_eq!(defect, &decoded);
    }
}

// --- Test 9: AtomPosition struct roundtrip with LZ4 ---
#[test]
fn test_atom_position_lz4_roundtrip() {
    // Silicon atom at origin in diamond cubic lattice (a = 543 pm)
    let atom = AtomPosition {
        x_pm: 0,
        y_pm: 0,
        z_pm: 0,
        element: 14,
    };
    let encoded = encode_to_vec(&atom).expect("encode AtomPosition");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress AtomPosition");
    let decompressed = decompress(&compressed).expect("lz4 decompress AtomPosition");
    let (decoded, _): (AtomPosition, usize) =
        decode_from_slice(&decompressed).expect("decode AtomPosition");
    assert_eq!(atom, decoded);
}

// --- Test 10: AtomPosition struct roundtrip with Zstd ---
#[test]
fn test_atom_position_zstd_roundtrip() {
    // Carbon atom in graphene (bond length ~142 pm)
    let atom = AtomPosition {
        x_pm: 142,
        y_pm: 82,
        z_pm: 0,
        element: 6,
    };
    let encoded = encode_to_vec(&atom).expect("encode AtomPosition");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress AtomPosition");
    let decompressed = decompress(&compressed).expect("zstd decompress AtomPosition");
    let (decoded, _): (AtomPosition, usize) =
        decode_from_slice(&decompressed).expect("decode AtomPosition");
    assert_eq!(atom, decoded);
}

// --- Test 11: UnitCell struct roundtrip with LZ4 (NaCl cubic) ---
#[test]
fn test_unit_cell_lz4_roundtrip() {
    // NaCl: cubic, a = b = c = 564 pm, all angles 90 degrees
    let cell = UnitCell {
        a_pm: 564,
        b_pm: 564,
        c_pm: 564,
        alpha_x1000: 90_000,
        beta_x1000: 90_000,
        gamma_x1000: 90_000,
        crystal_system: CrystalSystem::Cubic,
    };
    let encoded = encode_to_vec(&cell).expect("encode UnitCell");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress UnitCell");
    let decompressed = decompress(&compressed).expect("lz4 decompress UnitCell");
    let (decoded, _): (UnitCell, usize) =
        decode_from_slice(&decompressed).expect("decode UnitCell");
    assert_eq!(cell, decoded);
}

// --- Test 12: UnitCell struct roundtrip with Zstd (Quartz hexagonal) ---
#[test]
fn test_unit_cell_zstd_roundtrip() {
    // Quartz (SiO2): hexagonal, a = b = 491 pm, c = 541 pm, gamma = 120 degrees
    let cell = UnitCell {
        a_pm: 491,
        b_pm: 491,
        c_pm: 541,
        alpha_x1000: 90_000,
        beta_x1000: 90_000,
        gamma_x1000: 120_000,
        crystal_system: CrystalSystem::Hexagonal,
    };
    let encoded = encode_to_vec(&cell).expect("encode UnitCell");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress UnitCell");
    let decompressed = decompress(&compressed).expect("zstd decompress UnitCell");
    let (decoded, _): (UnitCell, usize) =
        decode_from_slice(&decompressed).expect("decode UnitCell");
    assert_eq!(cell, decoded);
}

// --- Test 13: DiffractionPeak struct roundtrip with LZ4 ---
#[test]
fn test_diffraction_peak_lz4_roundtrip() {
    // XRD peak for NaCl (200) reflection at 2theta = 31.7 degrees
    let peak = DiffractionPeak {
        two_theta_x100: 3170,
        intensity: 98500,
        hkl: [2, 0, 0],
    };
    let encoded = encode_to_vec(&peak).expect("encode DiffractionPeak");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress DiffractionPeak");
    let decompressed = decompress(&compressed).expect("lz4 decompress DiffractionPeak");
    let (decoded, _): (DiffractionPeak, usize) =
        decode_from_slice(&decompressed).expect("decode DiffractionPeak");
    assert_eq!(peak, decoded);
}

// --- Test 14: DiffractionPeak struct roundtrip with Zstd ---
#[test]
fn test_diffraction_peak_zstd_roundtrip() {
    // Negative Miller indices for triclinic system
    let peak = DiffractionPeak {
        two_theta_x100: 2430,
        intensity: 12750,
        hkl: [-1, 2, -3],
    };
    let encoded = encode_to_vec(&peak).expect("encode DiffractionPeak");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress DiffractionPeak");
    let decompressed = decompress(&compressed).expect("zstd decompress DiffractionPeak");
    let (decoded, _): (DiffractionPeak, usize) =
        decode_from_slice(&decompressed).expect("decode DiffractionPeak");
    assert_eq!(peak, decoded);
}

// --- Test 15: MaterialSample struct roundtrip with LZ4 ---
#[test]
fn test_material_sample_lz4_roundtrip() {
    let sample = MaterialSample {
        sample_id: 10042,
        name: String::from("TiO2-Rutile-Nano"),
        phase: MaterialPhase::Solid,
        unit_cell: UnitCell {
            a_pm: 459,
            b_pm: 459,
            c_pm: 296,
            alpha_x1000: 90_000,
            beta_x1000: 90_000,
            gamma_x1000: 90_000,
            crystal_system: CrystalSystem::Tetragonal,
        },
        diffraction_peaks: vec![
            DiffractionPeak {
                two_theta_x100: 2741,
                intensity: 999,
                hkl: [1, 1, 0],
            },
            DiffractionPeak {
                two_theta_x100: 3605,
                intensity: 450,
                hkl: [1, 0, 1],
            },
            DiffractionPeak {
                two_theta_x100: 5419,
                intensity: 200,
                hkl: [2, 1, 1],
            },
        ],
    };
    let encoded = encode_to_vec(&sample).expect("encode MaterialSample");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress MaterialSample");
    let decompressed = decompress(&compressed).expect("lz4 decompress MaterialSample");
    let (decoded, _): (MaterialSample, usize) =
        decode_from_slice(&decompressed).expect("decode MaterialSample");
    assert_eq!(sample, decoded);
}

// --- Test 16: MaterialSample struct roundtrip with Zstd ---
#[test]
fn test_material_sample_zstd_roundtrip() {
    let sample = MaterialSample {
        sample_id: 20078,
        name: String::from("ZnO-Wurtzite-Quantum-Dot"),
        phase: MaterialPhase::Solid,
        unit_cell: UnitCell {
            a_pm: 325,
            b_pm: 325,
            c_pm: 521,
            alpha_x1000: 90_000,
            beta_x1000: 90_000,
            gamma_x1000: 120_000,
            crystal_system: CrystalSystem::Hexagonal,
        },
        diffraction_peaks: vec![
            DiffractionPeak {
                two_theta_x100: 3177,
                intensity: 1000,
                hkl: [1, 0, 0],
            },
            DiffractionPeak {
                two_theta_x100: 3421,
                intensity: 875,
                hkl: [0, 0, 2],
            },
            DiffractionPeak {
                two_theta_x100: 3627,
                intensity: 999,
                hkl: [1, 0, 1],
            },
            DiffractionPeak {
                two_theta_x100: 4783,
                intensity: 320,
                hkl: [1, 1, 0],
            },
        ],
    };
    let encoded = encode_to_vec(&sample).expect("encode MaterialSample");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress MaterialSample");
    let decompressed = decompress(&compressed).expect("zstd decompress MaterialSample");
    let (decoded, _): (MaterialSample, usize) =
        decode_from_slice(&decompressed).expect("decode MaterialSample");
    assert_eq!(sample, decoded);
}

// --- Test 17: MolecularDynamicsFrame struct roundtrip with LZ4 ---
#[test]
fn test_md_frame_lz4_roundtrip() {
    let frame = MolecularDynamicsFrame {
        frame_id: 500,
        time_ps: 250,
        atoms: vec![
            AtomPosition {
                x_pm: 0,
                y_pm: 0,
                z_pm: 0,
                element: 26,
            }, // Fe
            AtomPosition {
                x_pm: 143,
                y_pm: 143,
                z_pm: 143,
                element: 26,
            }, // Fe BCC body center
        ],
        temperature_k: 1200,
        pressure_mpa: 101,
    };
    let encoded = encode_to_vec(&frame).expect("encode MolecularDynamicsFrame");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress MolecularDynamicsFrame");
    let decompressed = decompress(&compressed).expect("lz4 decompress MolecularDynamicsFrame");
    let (decoded, _): (MolecularDynamicsFrame, usize) =
        decode_from_slice(&decompressed).expect("decode MolecularDynamicsFrame");
    assert_eq!(frame, decoded);
}

// --- Test 18: MolecularDynamicsFrame struct roundtrip with Zstd ---
#[test]
fn test_md_frame_zstd_roundtrip() {
    let frame = MolecularDynamicsFrame {
        frame_id: 9999,
        time_ps: 5000,
        atoms: vec![
            AtomPosition {
                x_pm: 100,
                y_pm: 200,
                z_pm: 300,
                element: 13,
            }, // Al
            AtomPosition {
                x_pm: 400,
                y_pm: 100,
                z_pm: 200,
                element: 13,
            }, // Al
            AtomPosition {
                x_pm: 200,
                y_pm: 400,
                z_pm: 100,
                element: 13,
            }, // Al
            AtomPosition {
                x_pm: 300,
                y_pm: 300,
                z_pm: 300,
                element: 13,
            }, // Al FCC
        ],
        temperature_k: 933,
        pressure_mpa: 10132,
    };
    let encoded = encode_to_vec(&frame).expect("encode MolecularDynamicsFrame");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress MolecularDynamicsFrame");
    let decompressed = decompress(&compressed).expect("zstd decompress MolecularDynamicsFrame");
    let (decoded, _): (MolecularDynamicsFrame, usize) =
        decode_from_slice(&decompressed).expect("decode MolecularDynamicsFrame");
    assert_eq!(frame, decoded);
}

// --- Test 19: Large MD simulation with 1000+ atoms — compression ratio test ---
#[test]
fn test_large_simulation_compression_ratio() {
    // Simulate a large periodic nano-crystallite: 1024 atoms in BCC Fe lattice
    // with highly repetitive positions (good for compression)
    let lattice_a_pm: i32 = 286; // BCC Fe lattice parameter in pm
    let mut atoms = Vec::with_capacity(1024);
    for i in 0..8i32 {
        for j in 0..8i32 {
            for k in 0..8i32 {
                // Corner atom
                atoms.push(AtomPosition {
                    x_pm: i * lattice_a_pm,
                    y_pm: j * lattice_a_pm,
                    z_pm: k * lattice_a_pm,
                    element: 26,
                });
                // Body-center atom
                atoms.push(AtomPosition {
                    x_pm: i * lattice_a_pm + lattice_a_pm / 2,
                    y_pm: j * lattice_a_pm + lattice_a_pm / 2,
                    z_pm: k * lattice_a_pm + lattice_a_pm / 2,
                    element: 26,
                });
            }
        }
    }
    assert!(
        atoms.len() >= 1000,
        "must have at least 1000 atoms for ratio test"
    );

    let frame = MolecularDynamicsFrame {
        frame_id: 1,
        time_ps: 1,
        atoms,
        temperature_k: 300,
        pressure_mpa: 101,
    };

    let encoded = encode_to_vec(&frame).expect("encode large MD frame");
    let original_len = encoded.len();

    let lz4_compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress large MD frame");
    let zstd_compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress large MD frame");

    // Both compressed forms should be smaller than the original
    assert!(
        lz4_compressed.len() < original_len,
        "LZ4 compressed ({}) should be smaller than original ({})",
        lz4_compressed.len(),
        original_len
    );
    assert!(
        zstd_compressed.len() < original_len,
        "Zstd compressed ({}) should be smaller than original ({})",
        zstd_compressed.len(),
        original_len
    );

    // Verify both decompress back correctly
    let lz4_decompressed = decompress(&lz4_compressed).expect("lz4 decompress large MD frame");
    let (lz4_decoded, _): (MolecularDynamicsFrame, usize) =
        decode_from_slice(&lz4_decompressed).expect("decode lz4 large MD frame");

    let zstd_decompressed = decompress(&zstd_compressed).expect("zstd decompress large MD frame");
    let (zstd_decoded, _): (MolecularDynamicsFrame, usize) =
        decode_from_slice(&zstd_decompressed).expect("decode zstd large MD frame");

    assert_eq!(
        lz4_decoded, zstd_decoded,
        "LZ4 and Zstd decoded results must be identical"
    );
}

// --- Test 20: LZ4 vs Zstd produce different compressed bytes but identical decoded result ---
#[test]
fn test_lz4_vs_zstd_different_bytes_same_decoded() {
    // Fe3O4 (magnetite) unit cell: inverse spinel, cubic Fd-3m
    let sample = MaterialSample {
        sample_id: 30001,
        name: String::from("Fe3O4-Magnetite-Nanoparticle"),
        phase: MaterialPhase::Solid,
        unit_cell: UnitCell {
            a_pm: 839,
            b_pm: 839,
            c_pm: 839,
            alpha_x1000: 90_000,
            beta_x1000: 90_000,
            gamma_x1000: 90_000,
            crystal_system: CrystalSystem::Cubic,
        },
        diffraction_peaks: vec![
            DiffractionPeak {
                two_theta_x100: 1852,
                intensity: 500,
                hkl: [2, 2, 0],
            },
            DiffractionPeak {
                two_theta_x100: 3024,
                intensity: 1000,
                hkl: [3, 1, 1],
            },
            DiffractionPeak {
                two_theta_x100: 3569,
                intensity: 300,
                hkl: [4, 0, 0],
            },
            DiffractionPeak {
                two_theta_x100: 4328,
                intensity: 200,
                hkl: [4, 2, 2],
            },
            DiffractionPeak {
                two_theta_x100: 5360,
                intensity: 400,
                hkl: [5, 1, 1],
            },
        ],
    };

    let encoded = encode_to_vec(&sample).expect("encode MaterialSample for lz4 vs zstd test");

    let lz4_compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let zstd_compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");

    // The compressed byte sequences must differ (different algorithms)
    assert_ne!(
        lz4_compressed, zstd_compressed,
        "LZ4 and Zstd must produce different compressed byte sequences"
    );

    // But decoding must yield identical results
    let lz4_dec = decompress(&lz4_compressed).expect("lz4 decompress");
    let (lz4_result, _): (MaterialSample, usize) =
        decode_from_slice(&lz4_dec).expect("decode lz4 result");

    let zstd_dec = decompress(&zstd_compressed).expect("zstd decompress");
    let (zstd_result, _): (MaterialSample, usize) =
        decode_from_slice(&zstd_dec).expect("decode zstd result");

    assert_eq!(
        lz4_result, zstd_result,
        "decoded results from LZ4 and Zstd must be identical"
    );
    assert_eq!(lz4_result, sample);
}

// --- Test 21: Corrupted compressed data returns error (not panic) ---
#[test]
fn test_corrupted_compressed_data_returns_error() {
    let cell = UnitCell {
        a_pm: 405,
        b_pm: 405,
        c_pm: 405,
        alpha_x1000: 90_000,
        beta_x1000: 90_000,
        gamma_x1000: 90_000,
        crystal_system: CrystalSystem::Cubic,
    };

    let encoded = encode_to_vec(&cell).expect("encode UnitCell for corruption test");

    // Corrupt LZ4 by truncation (guarantees decompression failure)
    let lz4_compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress for corruption test");
    let truncated_lz4 = &lz4_compressed[..4.min(lz4_compressed.len())];

    let lz4_result = decompress(truncated_lz4);
    assert!(
        lz4_result.is_err(),
        "decompress of corrupted LZ4 data must return Err"
    );

    // Corrupt Zstd by truncation (guarantees decompression failure)
    let zstd_compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress for corruption test");
    let truncated_zstd = &zstd_compressed[..4.min(zstd_compressed.len())];

    let zstd_result = decompress(truncated_zstd);
    assert!(
        zstd_result.is_err(),
        "decompress of corrupted Zstd data must return Err"
    );
}

// --- Test 22: Empty atoms vec edge case with both LZ4 and Zstd ---
#[test]
fn test_empty_atoms_vec_edge_case() {
    // Represents a simulation frame with no tracked atoms (e.g., after filtering)
    let empty_frame = MolecularDynamicsFrame {
        frame_id: 0,
        time_ps: 0,
        atoms: Vec::new(),
        temperature_k: 0,
        pressure_mpa: 0,
    };

    // LZ4 roundtrip with empty atoms
    let encoded_lz4 = encode_to_vec(&empty_frame).expect("encode empty frame for lz4");
    let lz4_compressed =
        compress(&encoded_lz4, Compression::Lz4).expect("lz4 compress empty frame");
    let lz4_decompressed = decompress(&lz4_compressed).expect("lz4 decompress empty frame");
    let (lz4_decoded, _): (MolecularDynamicsFrame, usize) =
        decode_from_slice(&lz4_decompressed).expect("decode lz4 empty frame");
    assert_eq!(empty_frame, lz4_decoded);
    assert!(
        lz4_decoded.atoms.is_empty(),
        "atoms vec must remain empty after lz4 roundtrip"
    );

    // Zstd roundtrip with empty atoms
    let encoded_zstd = encode_to_vec(&empty_frame).expect("encode empty frame for zstd");
    let zstd_compressed =
        compress(&encoded_zstd, Compression::Zstd).expect("zstd compress empty frame");
    let zstd_decompressed = decompress(&zstd_compressed).expect("zstd decompress empty frame");
    let (zstd_decoded, _): (MolecularDynamicsFrame, usize) =
        decode_from_slice(&zstd_decompressed).expect("decode zstd empty frame");
    assert_eq!(empty_frame, zstd_decoded);
    assert!(
        zstd_decoded.atoms.is_empty(),
        "atoms vec must remain empty after zstd roundtrip"
    );

    // Also confirm both compression paths agree on decoded value
    assert_eq!(lz4_decoded, zstd_decoded);
}
