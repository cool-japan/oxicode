//! Advanced file I/O tests — materials science / crystallography domain
//! 22 top-level #[test] functions, no cfg(test) wrapper, no module wrapper.

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

fn tmp(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("{}_{}", name, std::process::id()))
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum CrystalSystem {
    Cubic,
    Tetragonal,
    Orthorhombic,
    Hexagonal,
    Trigonal,
    Monoclinic,
    Triclinic,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum BondType {
    Covalent,
    Ionic,
    Metallic,
    VanDerWaals,
    Hydrogen,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LatticeParams {
    a_pm: u32,
    b_pm: u32,
    c_pm: u32,
    alpha_mdeg: u32,
    beta_mdeg: u32,
    gamma_mdeg: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AtomicSite {
    element_z: u8,
    x_frac_micro: i32,
    y_frac_micro: i32,
    z_frac_micro: i32,
    occupancy_micro: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CrystalStructure {
    name: String,
    system: CrystalSystem,
    space_group: u16,
    lattice: LatticeParams,
    atoms: Vec<AtomicSite>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DiffractionPeak {
    h: i8,
    k: i8,
    l: i8,
    intensity: u32,
    d_spacing_pm: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DiffractionPattern {
    structure_name: String,
    wavelength_pm: u32,
    peaks: Vec<DiffractionPeak>,
}

// ── test 1: CrystalSystem::Cubic ────────────────────────────────────────────
#[test]
fn test_crystal_system_cubic_file_roundtrip() {
    let path = tmp("test_crystal_system_cubic_1.bin");
    let value = CrystalSystem::Cubic;
    encode_to_file(&value, &path).expect("encode CrystalSystem::Cubic failed");
    let decoded: CrystalSystem =
        decode_from_file(&path).expect("decode CrystalSystem::Cubic failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 2: CrystalSystem::Tetragonal ────────────────────────────────────────
#[test]
fn test_crystal_system_tetragonal_file_roundtrip() {
    let path = tmp("test_crystal_system_tetragonal_2.bin");
    let value = CrystalSystem::Tetragonal;
    encode_to_file(&value, &path).expect("encode CrystalSystem::Tetragonal failed");
    let decoded: CrystalSystem =
        decode_from_file(&path).expect("decode CrystalSystem::Tetragonal failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 3: CrystalSystem::Orthorhombic ──────────────────────────────────────
#[test]
fn test_crystal_system_orthorhombic_file_roundtrip() {
    let path = tmp("test_crystal_system_orthorhombic_3.bin");
    let value = CrystalSystem::Orthorhombic;
    encode_to_file(&value, &path).expect("encode CrystalSystem::Orthorhombic failed");
    let decoded: CrystalSystem =
        decode_from_file(&path).expect("decode CrystalSystem::Orthorhombic failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 4: CrystalSystem::Hexagonal ─────────────────────────────────────────
#[test]
fn test_crystal_system_hexagonal_file_roundtrip() {
    let path = tmp("test_crystal_system_hexagonal_4.bin");
    let value = CrystalSystem::Hexagonal;
    encode_to_file(&value, &path).expect("encode CrystalSystem::Hexagonal failed");
    let decoded: CrystalSystem =
        decode_from_file(&path).expect("decode CrystalSystem::Hexagonal failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 5: CrystalSystem::Trigonal ──────────────────────────────────────────
#[test]
fn test_crystal_system_trigonal_file_roundtrip() {
    let path = tmp("test_crystal_system_trigonal_5.bin");
    let value = CrystalSystem::Trigonal;
    encode_to_file(&value, &path).expect("encode CrystalSystem::Trigonal failed");
    let decoded: CrystalSystem =
        decode_from_file(&path).expect("decode CrystalSystem::Trigonal failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 6: CrystalSystem::Monoclinic ────────────────────────────────────────
#[test]
fn test_crystal_system_monoclinic_file_roundtrip() {
    let path = tmp("test_crystal_system_monoclinic_6.bin");
    let value = CrystalSystem::Monoclinic;
    encode_to_file(&value, &path).expect("encode CrystalSystem::Monoclinic failed");
    let decoded: CrystalSystem =
        decode_from_file(&path).expect("decode CrystalSystem::Monoclinic failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 7: CrystalSystem::Triclinic ─────────────────────────────────────────
#[test]
fn test_crystal_system_triclinic_file_roundtrip() {
    let path = tmp("test_crystal_system_triclinic_7.bin");
    let value = CrystalSystem::Triclinic;
    encode_to_file(&value, &path).expect("encode CrystalSystem::Triclinic failed");
    let decoded: CrystalSystem =
        decode_from_file(&path).expect("decode CrystalSystem::Triclinic failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 8: all BondType variants written to individual files ─────────────────
#[test]
fn test_bond_types_all_variants_file_roundtrip() {
    let variants = [
        (BondType::Covalent, "covalent"),
        (BondType::Ionic, "ionic"),
        (BondType::Metallic, "metallic"),
        (BondType::VanDerWaals, "vanderwaals"),
        (BondType::Hydrogen, "hydrogen"),
    ];
    for (variant, label) in &variants {
        let path = tmp(&format!("test_bond_type_{}_8.bin", label));
        encode_to_file(variant, &path).expect("encode BondType variant failed");
        let decoded: BondType = decode_from_file(&path).expect("decode BondType variant failed");
        assert_eq!(variant, &decoded);
        std::fs::remove_file(&path).ok();
    }
}

// ── test 9: LatticeParams file roundtrip ─────────────────────────────────────
#[test]
fn test_lattice_params_file_roundtrip() {
    let path = tmp("test_lattice_params_9.bin");
    // NaCl (rock-salt): a = b = c = 564 pm, all angles 90 000 mdeg
    let lattice = LatticeParams {
        a_pm: 564,
        b_pm: 564,
        c_pm: 564,
        alpha_mdeg: 90_000,
        beta_mdeg: 90_000,
        gamma_mdeg: 90_000,
    };
    encode_to_file(&lattice, &path).expect("encode LatticeParams failed");
    let decoded: LatticeParams = decode_from_file(&path).expect("decode LatticeParams failed");
    assert_eq!(lattice, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 10: AtomicSite file roundtrip ───────────────────────────────────────
#[test]
fn test_atomic_site_file_roundtrip() {
    let path = tmp("test_atomic_site_10.bin");
    // Silicon at (0.125, 0.125, 0.125) fractional, full occupancy
    let site = AtomicSite {
        element_z: 14,
        x_frac_micro: 125_000,
        y_frac_micro: 125_000,
        z_frac_micro: 125_000,
        occupancy_micro: 1_000_000,
    };
    encode_to_file(&site, &path).expect("encode AtomicSite failed");
    let decoded: AtomicSite = decode_from_file(&path).expect("decode AtomicSite failed");
    assert_eq!(site, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 11: CrystalStructure with empty atoms list ──────────────────────────
#[test]
fn test_crystal_structure_empty_atoms_file_roundtrip() {
    let path = tmp("test_crystal_struct_empty_11.bin");
    let structure = CrystalStructure {
        name: String::from("EmptyStruct"),
        system: CrystalSystem::Cubic,
        space_group: 225,
        lattice: LatticeParams {
            a_pm: 400,
            b_pm: 400,
            c_pm: 400,
            alpha_mdeg: 90_000,
            beta_mdeg: 90_000,
            gamma_mdeg: 90_000,
        },
        atoms: vec![],
    };
    encode_to_file(&structure, &path).expect("encode empty CrystalStructure failed");
    let decoded: CrystalStructure =
        decode_from_file(&path).expect("decode empty CrystalStructure failed");
    assert_eq!(structure, decoded);
    assert!(decoded.atoms.is_empty());
    std::fs::remove_file(&path).ok();
}

// ── test 12: CrystalStructure with 5 atoms ───────────────────────────────────
#[test]
fn test_crystal_structure_five_atoms_file_roundtrip() {
    let path = tmp("test_crystal_struct_5atoms_12.bin");
    // Simplified perovskite BaTiO3 (tetragonal) with 5 atoms per unit cell
    let atoms = vec![
        AtomicSite {
            element_z: 56,
            x_frac_micro: 0,
            y_frac_micro: 0,
            z_frac_micro: 0,
            occupancy_micro: 1_000_000,
        }, // Ba
        AtomicSite {
            element_z: 22,
            x_frac_micro: 500_000,
            y_frac_micro: 500_000,
            z_frac_micro: 512_000,
            occupancy_micro: 1_000_000,
        }, // Ti
        AtomicSite {
            element_z: 8,
            x_frac_micro: 500_000,
            y_frac_micro: 500_000,
            z_frac_micro: 0,
            occupancy_micro: 1_000_000,
        }, // O1
        AtomicSite {
            element_z: 8,
            x_frac_micro: 500_000,
            y_frac_micro: 0,
            z_frac_micro: 512_000,
            occupancy_micro: 1_000_000,
        }, // O2
        AtomicSite {
            element_z: 8,
            x_frac_micro: 0,
            y_frac_micro: 500_000,
            z_frac_micro: 512_000,
            occupancy_micro: 1_000_000,
        }, // O3
    ];
    let structure = CrystalStructure {
        name: String::from("BaTiO3"),
        system: CrystalSystem::Tetragonal,
        space_group: 99,
        lattice: LatticeParams {
            a_pm: 399,
            b_pm: 399,
            c_pm: 403,
            alpha_mdeg: 90_000,
            beta_mdeg: 90_000,
            gamma_mdeg: 90_000,
        },
        atoms,
    };
    encode_to_file(&structure, &path).expect("encode 5-atom CrystalStructure failed");
    let decoded: CrystalStructure =
        decode_from_file(&path).expect("decode 5-atom CrystalStructure failed");
    assert_eq!(structure, decoded);
    assert_eq!(decoded.atoms.len(), 5);
    std::fs::remove_file(&path).ok();
}

// ── test 13: DiffractionPeak file roundtrip ───────────────────────────────────
#[test]
fn test_diffraction_peak_file_roundtrip() {
    let path = tmp("test_diffraction_peak_13.bin");
    // Copper (111) reflection, Cu Kα wavelength 154 pm
    let peak = DiffractionPeak {
        h: 1,
        k: 1,
        l: 1,
        intensity: 100_000,
        d_spacing_pm: 208,
    };
    encode_to_file(&peak, &path).expect("encode DiffractionPeak failed");
    let decoded: DiffractionPeak = decode_from_file(&path).expect("decode DiffractionPeak failed");
    assert_eq!(peak, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 14: DiffractionPattern with 10 peaks ────────────────────────────────
#[test]
fn test_diffraction_pattern_ten_peaks_file_roundtrip() {
    let path = tmp("test_diffraction_pattern_10peaks_14.bin");
    // Aluminium FCC powder diffraction (selected reflections)
    let peaks = vec![
        DiffractionPeak {
            h: 1,
            k: 1,
            l: 1,
            intensity: 100_000,
            d_spacing_pm: 233,
        },
        DiffractionPeak {
            h: 2,
            k: 0,
            l: 0,
            intensity: 57_000,
            d_spacing_pm: 202,
        },
        DiffractionPeak {
            h: 2,
            k: 2,
            l: 0,
            intensity: 35_000,
            d_spacing_pm: 143,
        },
        DiffractionPeak {
            h: 3,
            k: 1,
            l: 1,
            intensity: 28_000,
            d_spacing_pm: 122,
        },
        DiffractionPeak {
            h: 2,
            k: 2,
            l: 2,
            intensity: 8_000,
            d_spacing_pm: 117,
        },
        DiffractionPeak {
            h: 4,
            k: 0,
            l: 0,
            intensity: 4_000,
            d_spacing_pm: 101,
        },
        DiffractionPeak {
            h: 3,
            k: 3,
            l: 1,
            intensity: 11_000,
            d_spacing_pm: 93,
        },
        DiffractionPeak {
            h: 4,
            k: 2,
            l: 0,
            intensity: 10_000,
            d_spacing_pm: 90,
        },
        DiffractionPeak {
            h: 4,
            k: 2,
            l: 2,
            intensity: 6_000,
            d_spacing_pm: 83,
        },
        DiffractionPeak {
            h: 3,
            k: 3,
            l: 3,
            intensity: 3_000,
            d_spacing_pm: 78,
        },
    ];
    let pattern = DiffractionPattern {
        structure_name: String::from("Al-FCC"),
        wavelength_pm: 154,
        peaks,
    };
    encode_to_file(&pattern, &path).expect("encode DiffractionPattern 10 peaks failed");
    let decoded: DiffractionPattern =
        decode_from_file(&path).expect("decode DiffractionPattern 10 peaks failed");
    assert_eq!(pattern, decoded);
    assert_eq!(decoded.peaks.len(), 10);
    std::fs::remove_file(&path).ok();
}

// ── test 15: large CrystalStructure (50 atoms) ───────────────────────────────
#[test]
fn test_large_crystal_structure_50_atoms_file_roundtrip() {
    let path = tmp("test_crystal_struct_50atoms_15.bin");
    // Simulate a 50-atom supercell; element_z cycles through common elements
    let elements: [u8; 5] = [26, 28, 29, 22, 8]; // Fe, Ni, Cu, Ti, O
    let atoms: Vec<AtomicSite> = (0u32..50)
        .map(|i| AtomicSite {
            element_z: elements[(i as usize) % elements.len()],
            x_frac_micro: ((i * 20_000) % 1_000_000) as i32,
            y_frac_micro: ((i * 13_000) % 1_000_000) as i32,
            z_frac_micro: ((i * 7_000) % 1_000_000) as i32,
            occupancy_micro: 1_000_000,
        })
        .collect();
    let structure = CrystalStructure {
        name: String::from("FeNiCuTiO-Supercell-50"),
        system: CrystalSystem::Orthorhombic,
        space_group: 62,
        lattice: LatticeParams {
            a_pm: 590,
            b_pm: 780,
            c_pm: 550,
            alpha_mdeg: 90_000,
            beta_mdeg: 90_000,
            gamma_mdeg: 90_000,
        },
        atoms,
    };
    encode_to_file(&structure, &path).expect("encode 50-atom CrystalStructure failed");
    let decoded: CrystalStructure =
        decode_from_file(&path).expect("decode 50-atom CrystalStructure failed");
    assert_eq!(structure, decoded);
    assert_eq!(decoded.atoms.len(), 50);
    std::fs::remove_file(&path).ok();
}

// ── test 16: large DiffractionPattern (100 peaks) ────────────────────────────
#[test]
fn test_large_diffraction_pattern_100_peaks_file_roundtrip() {
    let path = tmp("test_diffraction_pattern_100peaks_16.bin");
    let peaks: Vec<DiffractionPeak> = (0i32..100)
        .map(|i| DiffractionPeak {
            h: (i % 10 - 5) as i8,
            k: (i / 10 - 5) as i8,
            l: (i % 7 - 3) as i8,
            intensity: (100_000u32).saturating_sub(i as u32 * 900),
            d_spacing_pm: 300u32.saturating_sub(i as u32 * 2),
        })
        .collect();
    let pattern = DiffractionPattern {
        structure_name: String::from("SyntheticPhase-100peaks"),
        wavelength_pm: 71, // Mo Kα
        peaks,
    };
    encode_to_file(&pattern, &path).expect("encode 100-peak DiffractionPattern failed");
    let decoded: DiffractionPattern =
        decode_from_file(&path).expect("decode 100-peak DiffractionPattern failed");
    assert_eq!(pattern, decoded);
    assert_eq!(decoded.peaks.len(), 100);
    std::fs::remove_file(&path).ok();
}

// ── test 17: Vec<CrystalStructure> file roundtrip ────────────────────────────
#[test]
fn test_vec_crystal_structure_file_roundtrip() {
    let path = tmp("test_vec_crystal_struct_17.bin");
    let structures = vec![
        CrystalStructure {
            name: String::from("Diamond"),
            system: CrystalSystem::Cubic,
            space_group: 227,
            lattice: LatticeParams {
                a_pm: 357,
                b_pm: 357,
                c_pm: 357,
                alpha_mdeg: 90_000,
                beta_mdeg: 90_000,
                gamma_mdeg: 90_000,
            },
            atoms: vec![
                AtomicSite {
                    element_z: 6,
                    x_frac_micro: 0,
                    y_frac_micro: 0,
                    z_frac_micro: 0,
                    occupancy_micro: 1_000_000,
                },
                AtomicSite {
                    element_z: 6,
                    x_frac_micro: 250_000,
                    y_frac_micro: 250_000,
                    z_frac_micro: 250_000,
                    occupancy_micro: 1_000_000,
                },
            ],
        },
        CrystalStructure {
            name: String::from("Quartz-SiO2"),
            system: CrystalSystem::Trigonal,
            space_group: 154,
            lattice: LatticeParams {
                a_pm: 491,
                b_pm: 491,
                c_pm: 541,
                alpha_mdeg: 90_000,
                beta_mdeg: 90_000,
                gamma_mdeg: 120_000,
            },
            atoms: vec![
                AtomicSite {
                    element_z: 14,
                    x_frac_micro: 465_000,
                    y_frac_micro: 0,
                    z_frac_micro: 333_333,
                    occupancy_micro: 1_000_000,
                },
                AtomicSite {
                    element_z: 8,
                    x_frac_micro: 414_000,
                    y_frac_micro: 267_000,
                    z_frac_micro: 119_000,
                    occupancy_micro: 1_000_000,
                },
            ],
        },
        CrystalStructure {
            name: String::from("Calcite-CaCO3"),
            system: CrystalSystem::Trigonal,
            space_group: 167,
            lattice: LatticeParams {
                a_pm: 499,
                b_pm: 499,
                c_pm: 1706,
                alpha_mdeg: 90_000,
                beta_mdeg: 90_000,
                gamma_mdeg: 120_000,
            },
            atoms: vec![],
        },
    ];
    encode_to_file(&structures, &path).expect("encode Vec<CrystalStructure> failed");
    let decoded: Vec<CrystalStructure> =
        decode_from_file(&path).expect("decode Vec<CrystalStructure> failed");
    assert_eq!(structures, decoded);
    assert_eq!(decoded.len(), 3);
    std::fs::remove_file(&path).ok();
}

// ── test 18: overwrite file — encode twice, decode returns second value ───────
#[test]
fn test_overwrite_file_decode_returns_second_value() {
    let path = tmp("test_overwrite_18.bin");
    let first = CrystalStructure {
        name: String::from("FirstWrite"),
        system: CrystalSystem::Cubic,
        space_group: 1,
        lattice: LatticeParams {
            a_pm: 100,
            b_pm: 100,
            c_pm: 100,
            alpha_mdeg: 90_000,
            beta_mdeg: 90_000,
            gamma_mdeg: 90_000,
        },
        atoms: vec![],
    };
    let second = CrystalStructure {
        name: String::from("SecondWrite"),
        system: CrystalSystem::Hexagonal,
        space_group: 194,
        lattice: LatticeParams {
            a_pm: 321,
            b_pm: 321,
            c_pm: 521,
            alpha_mdeg: 90_000,
            beta_mdeg: 90_000,
            gamma_mdeg: 120_000,
        },
        atoms: vec![AtomicSite {
            element_z: 12,
            x_frac_micro: 333_333,
            y_frac_micro: 666_667,
            z_frac_micro: 250_000,
            occupancy_micro: 1_000_000,
        }],
    };
    encode_to_file(&first, &path).expect("first encode_to_file failed");
    encode_to_file(&second, &path).expect("second (overwrite) encode_to_file failed");
    let decoded: CrystalStructure = decode_from_file(&path).expect("decode after overwrite failed");
    assert_eq!(second, decoded);
    assert_ne!(first.name, decoded.name);
    std::fs::remove_file(&path).ok();
}

// ── test 19: file path uniqueness — parallel structures in separate files ─────
#[test]
fn test_unique_file_paths_no_collision() {
    let systems = [
        (CrystalSystem::Cubic, "cubic"),
        (CrystalSystem::Tetragonal, "tetragonal"),
        (CrystalSystem::Orthorhombic, "orthorhombic"),
        (CrystalSystem::Hexagonal, "hexagonal"),
        (CrystalSystem::Trigonal, "trigonal"),
        (CrystalSystem::Monoclinic, "monoclinic"),
        (CrystalSystem::Triclinic, "triclinic"),
    ];
    let paths: Vec<std::path::PathBuf> = systems
        .iter()
        .map(|(_, label)| tmp(&format!("test_unique_path_{}_19.bin", label)))
        .collect();
    // Write each system to its own unique path
    for ((system, _), path) in systems.iter().zip(paths.iter()) {
        encode_to_file(system, path).expect("encode for unique path test failed");
    }
    // Verify each file decodes to the expected variant
    for ((expected, _), path) in systems.iter().zip(paths.iter()) {
        let decoded: CrystalSystem =
            decode_from_file(path).expect("decode for unique path test failed");
        assert_eq!(expected, &decoded);
    }
    // Cleanup
    for path in &paths {
        std::fs::remove_file(path).ok();
    }
}

// ── test 20: space_group boundary values (1 and 230) ─────────────────────────
#[test]
fn test_space_group_boundary_values_file_roundtrip() {
    let path_low = tmp("test_sg_boundary_low_20.bin");
    let path_high = tmp("test_sg_boundary_high_20.bin");

    let sg_1 = CrystalStructure {
        name: String::from("SpaceGroup1-Triclinic"),
        system: CrystalSystem::Triclinic,
        space_group: 1, // lowest possible
        lattice: LatticeParams {
            a_pm: 500,
            b_pm: 600,
            c_pm: 700,
            alpha_mdeg: 83_000,
            beta_mdeg: 96_000,
            gamma_mdeg: 107_000,
        },
        atoms: vec![AtomicSite {
            element_z: 1,
            x_frac_micro: 100_000,
            y_frac_micro: 200_000,
            z_frac_micro: 300_000,
            occupancy_micro: 1_000_000,
        }],
    };
    let sg_230 = CrystalStructure {
        name: String::from("SpaceGroup230-Cubic"),
        system: CrystalSystem::Cubic,
        space_group: 230, // highest possible
        lattice: LatticeParams {
            a_pm: 1050,
            b_pm: 1050,
            c_pm: 1050,
            alpha_mdeg: 90_000,
            beta_mdeg: 90_000,
            gamma_mdeg: 90_000,
        },
        atoms: vec![AtomicSite {
            element_z: 26,
            x_frac_micro: 0,
            y_frac_micro: 0,
            z_frac_micro: 0,
            occupancy_micro: 1_000_000,
        }],
    };

    encode_to_file(&sg_1, &path_low).expect("encode sg=1 failed");
    encode_to_file(&sg_230, &path_high).expect("encode sg=230 failed");

    let decoded_low: CrystalStructure = decode_from_file(&path_low).expect("decode sg=1 failed");
    let decoded_high: CrystalStructure =
        decode_from_file(&path_high).expect("decode sg=230 failed");

    assert_eq!(sg_1, decoded_low);
    assert_eq!(sg_230, decoded_high);
    assert_eq!(decoded_low.space_group, 1);
    assert_eq!(decoded_high.space_group, 230);

    std::fs::remove_file(&path_low).ok();
    std::fs::remove_file(&path_high).ok();
}

// ── test 21: occupancy edge cases (0 and 1_000_000) ──────────────────────────
#[test]
fn test_occupancy_edge_cases_file_roundtrip() {
    let path = tmp("test_occupancy_edge_21.bin");
    // Partial occupancy model: one fully occupied site and one vacancy
    let atoms = vec![
        AtomicSite {
            element_z: 47, // Ag
            x_frac_micro: 0,
            y_frac_micro: 0,
            z_frac_micro: 0,
            occupancy_micro: 1_000_000, // full occupancy
        },
        AtomicSite {
            element_z: 47, // Ag vacancy
            x_frac_micro: 500_000,
            y_frac_micro: 500_000,
            z_frac_micro: 500_000,
            occupancy_micro: 0, // zero occupancy (vacancy)
        },
        AtomicSite {
            element_z: 47,
            x_frac_micro: 250_000,
            y_frac_micro: 250_000,
            z_frac_micro: 250_000,
            occupancy_micro: 500_000, // half occupancy (disorder)
        },
    ];
    let structure = CrystalStructure {
        name: String::from("Ag-PartialOccupancy"),
        system: CrystalSystem::Cubic,
        space_group: 225,
        lattice: LatticeParams {
            a_pm: 409,
            b_pm: 409,
            c_pm: 409,
            alpha_mdeg: 90_000,
            beta_mdeg: 90_000,
            gamma_mdeg: 90_000,
        },
        atoms,
    };
    encode_to_file(&structure, &path).expect("encode occupancy edge cases failed");
    let decoded: CrystalStructure =
        decode_from_file(&path).expect("decode occupancy edge cases failed");
    assert_eq!(structure, decoded);
    assert_eq!(decoded.atoms[0].occupancy_micro, 1_000_000);
    assert_eq!(decoded.atoms[1].occupancy_micro, 0);
    assert_eq!(decoded.atoms[2].occupancy_micro, 500_000);
    std::fs::remove_file(&path).ok();
}

// ── test 22: encode_to_vec / decode_from_slice consistency with file data ─────
#[test]
fn test_encode_to_vec_matches_file_bytes_and_decode_from_slice() {
    let path = tmp("test_vec_slice_file_consistency_22.bin");
    // Graphite (hexagonal): two carbon atoms per unit cell
    let structure = CrystalStructure {
        name: String::from("Graphite-2H"),
        system: CrystalSystem::Hexagonal,
        space_group: 194,
        lattice: LatticeParams {
            a_pm: 246,
            b_pm: 246,
            c_pm: 671,
            alpha_mdeg: 90_000,
            beta_mdeg: 90_000,
            gamma_mdeg: 120_000,
        },
        atoms: vec![
            AtomicSite {
                element_z: 6,
                x_frac_micro: 0,
                y_frac_micro: 0,
                z_frac_micro: 250_000,
                occupancy_micro: 1_000_000,
            },
            AtomicSite {
                element_z: 6,
                x_frac_micro: 333_333,
                y_frac_micro: 666_667,
                z_frac_micro: 750_000,
                occupancy_micro: 1_000_000,
            },
        ],
    };

    // Write to file
    encode_to_file(&structure, &path).expect("encode_to_file for consistency test failed");

    // Read file bytes back and compare with encode_to_vec output
    let file_bytes = std::fs::read(&path).expect("read file bytes failed");
    let vec_bytes = encode_to_vec(&structure).expect("encode_to_vec failed");
    assert_eq!(
        file_bytes, vec_bytes,
        "file bytes must match encode_to_vec bytes"
    );

    // decode_from_slice must yield the same value as decode_from_file
    let (from_slice, _): (CrystalStructure, _) =
        decode_from_slice(&vec_bytes).expect("decode_from_slice failed");
    let from_file: CrystalStructure =
        decode_from_file(&path).expect("decode_from_file for consistency test failed");
    assert_eq!(from_slice, from_file);
    assert_eq!(structure, from_file);

    std::fs::remove_file(&path).ok();
}
