#![cfg(feature = "compression-lz4")]
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

#[derive(Debug, PartialEq, Encode, Decode)]
enum AtomElement {
    H,
    C,
    N,
    O,
    S,
    P,
    Fe,
    Ca,
    Na,
    Cl,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum BondOrder {
    Single,
    Double,
    Triple,
    Aromatic,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Atom {
    atom_id: u32,
    element: AtomElement,
    x_pm: i32,
    y_pm: i32,
    z_pm: i32,
    charge_micro: i32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Bond {
    bond_id: u32,
    atom_a: u32,
    atom_b: u32,
    order: BondOrder,
    length_pm: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MoleculeSnapshot {
    frame_id: u64,
    time_ps: u64,
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
    potential_energy_uj: i64,
}

// --- AtomElement compress/decompress roundtrips ---

#[test]
fn test_atom_element_hydrogen_compress_roundtrip() {
    let element = AtomElement::H;
    let encoded = encode_to_vec(&element).expect("encode AtomElement::H failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress AtomElement::H failed");
    let decompressed = decompress(&compressed).expect("decompress AtomElement::H failed");
    let (decoded, _): (AtomElement, usize) =
        decode_from_slice(&decompressed).expect("decode AtomElement::H failed");
    assert_eq!(element, decoded);
}

#[test]
fn test_atom_element_carbon_compress_roundtrip() {
    let element = AtomElement::C;
    let encoded = encode_to_vec(&element).expect("encode AtomElement::C failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress AtomElement::C failed");
    let decompressed = decompress(&compressed).expect("decompress AtomElement::C failed");
    let (decoded, _): (AtomElement, usize) =
        decode_from_slice(&decompressed).expect("decode AtomElement::C failed");
    assert_eq!(element, decoded);
}

#[test]
fn test_atom_element_iron_compress_roundtrip() {
    let element = AtomElement::Fe;
    let encoded = encode_to_vec(&element).expect("encode AtomElement::Fe failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress AtomElement::Fe failed");
    let decompressed = decompress(&compressed).expect("decompress AtomElement::Fe failed");
    let (decoded, _): (AtomElement, usize) =
        decode_from_slice(&decompressed).expect("decode AtomElement::Fe failed");
    assert_eq!(element, decoded);
}

#[test]
fn test_atom_element_calcium_compress_roundtrip() {
    let element = AtomElement::Ca;
    let encoded = encode_to_vec(&element).expect("encode AtomElement::Ca failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress AtomElement::Ca failed");
    let decompressed = decompress(&compressed).expect("decompress AtomElement::Ca failed");
    let (decoded, _): (AtomElement, usize) =
        decode_from_slice(&decompressed).expect("decode AtomElement::Ca failed");
    assert_eq!(element, decoded);
}

// --- BondOrder compress/decompress roundtrips ---

#[test]
fn test_bond_order_single_compress_roundtrip() {
    let order = BondOrder::Single;
    let encoded = encode_to_vec(&order).expect("encode BondOrder::Single failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress BondOrder::Single failed");
    let decompressed = decompress(&compressed).expect("decompress BondOrder::Single failed");
    let (decoded, _): (BondOrder, usize) =
        decode_from_slice(&decompressed).expect("decode BondOrder::Single failed");
    assert_eq!(order, decoded);
}

#[test]
fn test_bond_order_double_compress_roundtrip() {
    let order = BondOrder::Double;
    let encoded = encode_to_vec(&order).expect("encode BondOrder::Double failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress BondOrder::Double failed");
    let decompressed = decompress(&compressed).expect("decompress BondOrder::Double failed");
    let (decoded, _): (BondOrder, usize) =
        decode_from_slice(&decompressed).expect("decode BondOrder::Double failed");
    assert_eq!(order, decoded);
}

#[test]
fn test_bond_order_aromatic_compress_roundtrip() {
    let order = BondOrder::Aromatic;
    let encoded = encode_to_vec(&order).expect("encode BondOrder::Aromatic failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress BondOrder::Aromatic failed");
    let decompressed = decompress(&compressed).expect("decompress BondOrder::Aromatic failed");
    let (decoded, _): (BondOrder, usize) =
        decode_from_slice(&decompressed).expect("decode BondOrder::Aromatic failed");
    assert_eq!(order, decoded);
}

#[test]
fn test_bond_order_triple_compress_roundtrip() {
    let order = BondOrder::Triple;
    let encoded = encode_to_vec(&order).expect("encode BondOrder::Triple failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress BondOrder::Triple failed");
    let decompressed = decompress(&compressed).expect("decompress BondOrder::Triple failed");
    let (decoded, _): (BondOrder, usize) =
        decode_from_slice(&decompressed).expect("decode BondOrder::Triple failed");
    assert_eq!(order, decoded);
}

// --- Atom compress roundtrip ---

#[test]
fn test_atom_compress_roundtrip() {
    let atom = Atom {
        atom_id: 42,
        element: AtomElement::N,
        x_pm: 153,
        y_pm: -240,
        z_pm: 789,
        charge_micro: -500,
    };
    let encoded = encode_to_vec(&atom).expect("encode Atom failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Atom failed");
    let decompressed = decompress(&compressed).expect("decompress Atom failed");
    let (decoded, _): (Atom, usize) = decode_from_slice(&decompressed).expect("decode Atom failed");
    assert_eq!(atom, decoded);
}

// --- Bond compress roundtrip ---

#[test]
fn test_bond_compress_roundtrip() {
    let bond = Bond {
        bond_id: 7,
        atom_a: 1,
        atom_b: 2,
        order: BondOrder::Triple,
        length_pm: 120,
    };
    let encoded = encode_to_vec(&bond).expect("encode Bond failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Bond failed");
    let decompressed = decompress(&compressed).expect("decompress Bond failed");
    let (decoded, _): (Bond, usize) = decode_from_slice(&decompressed).expect("decode Bond failed");
    assert_eq!(bond, decoded);
}

// --- MoleculeSnapshot with 10 atoms + bonds ---

#[test]
fn test_molecule_snapshot_ten_atoms_compress_roundtrip() {
    let atoms: Vec<Atom> = (0..10)
        .map(|i| Atom {
            atom_id: i,
            element: if i % 2 == 0 {
                AtomElement::C
            } else {
                AtomElement::H
            },
            x_pm: (i as i32) * 154,
            y_pm: (i as i32) * -50,
            z_pm: 0,
            charge_micro: 0,
        })
        .collect();
    let bonds: Vec<Bond> = (0..9)
        .map(|i| Bond {
            bond_id: i,
            atom_a: i,
            atom_b: i + 1,
            order: BondOrder::Single,
            length_pm: 154,
        })
        .collect();
    let snapshot = MoleculeSnapshot {
        frame_id: 0,
        time_ps: 100,
        atoms,
        bonds,
        potential_energy_uj: -48_200,
    };
    let encoded = encode_to_vec(&snapshot).expect("encode MoleculeSnapshot(10) failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress MoleculeSnapshot(10) failed");
    let decompressed = decompress(&compressed).expect("decompress MoleculeSnapshot(10) failed");
    let (decoded, _): (MoleculeSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode MoleculeSnapshot(10) failed");
    assert_eq!(snapshot, decoded);
}

// --- Large snapshot (1000 atoms) compression ratio ---

#[test]
fn test_large_snapshot_compression_ratio() {
    let atoms: Vec<Atom> = (0u32..1000)
        .map(|i| Atom {
            atom_id: i,
            element: AtomElement::C,
            x_pm: (i as i32) * 154,
            y_pm: 0,
            z_pm: 0,
            charge_micro: 0,
        })
        .collect();
    let bonds: Vec<Bond> = (0u32..999)
        .map(|i| Bond {
            bond_id: i,
            atom_a: i,
            atom_b: i + 1,
            order: BondOrder::Single,
            length_pm: 154,
        })
        .collect();
    let snapshot = MoleculeSnapshot {
        frame_id: 1,
        time_ps: 1000,
        atoms,
        bonds,
        potential_energy_uj: -982_000,
    };
    let encoded = encode_to_vec(&snapshot).expect("encode large snapshot failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress large snapshot failed");
    assert!(
        compressed.len() <= encoded.len(),
        "compressed ({} bytes) should be <= encoded ({} bytes) for large repetitive snapshot",
        compressed.len(),
        encoded.len()
    );
}

// --- Repetitive atoms compress smaller ---

#[test]
fn test_repetitive_atoms_compress_smaller() {
    let atoms: Vec<Atom> = (0u32..1000)
        .map(|_| Atom {
            atom_id: 0,
            element: AtomElement::H,
            x_pm: 0,
            y_pm: 0,
            z_pm: 0,
            charge_micro: 0,
        })
        .collect();
    let encoded = encode_to_vec(&atoms).expect("encode repetitive atoms failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress repetitive atoms failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({} bytes) should be < encoded ({} bytes) for identical atoms",
        compressed.len(),
        encoded.len()
    );
}

// --- Empty snapshot ---

#[test]
fn test_empty_snapshot_compress_roundtrip() {
    let snapshot = MoleculeSnapshot {
        frame_id: 0,
        time_ps: 0,
        atoms: vec![],
        bonds: vec![],
        potential_energy_uj: 0,
    };
    let encoded = encode_to_vec(&snapshot).expect("encode empty snapshot failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress empty snapshot failed");
    let decompressed = decompress(&compressed).expect("decompress empty snapshot failed");
    let (decoded, _): (MoleculeSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode empty snapshot failed");
    assert_eq!(snapshot, decoded);
}

// --- Vec<Atom> compress roundtrip ---

#[test]
fn test_vec_atom_compress_roundtrip() {
    let atoms = vec![
        Atom {
            atom_id: 0,
            element: AtomElement::O,
            x_pm: 0,
            y_pm: 0,
            z_pm: 0,
            charge_micro: -2000,
        },
        Atom {
            atom_id: 1,
            element: AtomElement::H,
            x_pm: 96,
            y_pm: 0,
            z_pm: 0,
            charge_micro: 1000,
        },
        Atom {
            atom_id: 2,
            element: AtomElement::H,
            x_pm: -96,
            y_pm: 0,
            z_pm: 0,
            charge_micro: 1000,
        },
    ];
    let encoded = encode_to_vec(&atoms).expect("encode Vec<Atom> failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Vec<Atom> failed");
    let decompressed = decompress(&compressed).expect("decompress Vec<Atom> failed");
    let (decoded, _): (Vec<Atom>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<Atom> failed");
    assert_eq!(atoms, decoded);
}

// --- Vec<Bond> compress roundtrip ---

#[test]
fn test_vec_bond_compress_roundtrip() {
    let bonds = vec![
        Bond {
            bond_id: 0,
            atom_a: 0,
            atom_b: 1,
            order: BondOrder::Single,
            length_pm: 154,
        },
        Bond {
            bond_id: 1,
            atom_a: 1,
            atom_b: 2,
            order: BondOrder::Double,
            length_pm: 134,
        },
        Bond {
            bond_id: 2,
            atom_a: 2,
            atom_b: 3,
            order: BondOrder::Aromatic,
            length_pm: 140,
        },
    ];
    let encoded = encode_to_vec(&bonds).expect("encode Vec<Bond> failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Vec<Bond> failed");
    let decompressed = decompress(&compressed).expect("decompress Vec<Bond> failed");
    let (decoded, _): (Vec<Bond>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<Bond> failed");
    assert_eq!(bonds, decoded);
}

// --- Decompress gives original bytes ---

#[test]
fn test_decompress_gives_original_bytes() {
    let atom = Atom {
        atom_id: 99,
        element: AtomElement::S,
        x_pm: 1234,
        y_pm: -5678,
        z_pm: 910,
        charge_micro: 500,
    };
    let encoded = encode_to_vec(&atom).expect("encode sulfur atom failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress sulfur atom failed");
    let decompressed = decompress(&compressed).expect("decompress sulfur atom failed");
    assert_eq!(
        encoded, decompressed,
        "decompressed bytes must exactly match original encoded bytes"
    );
}

// --- Molecular dynamics trajectory (5 frames) ---

#[test]
fn test_molecular_dynamics_trajectory_five_frames() {
    let trajectory: Vec<MoleculeSnapshot> = (0u64..5)
        .map(|frame| MoleculeSnapshot {
            frame_id: frame,
            time_ps: frame * 10,
            atoms: vec![
                Atom {
                    atom_id: 0,
                    element: AtomElement::C,
                    x_pm: (frame as i32) * 10,
                    y_pm: 0,
                    z_pm: 0,
                    charge_micro: 0,
                },
                Atom {
                    atom_id: 1,
                    element: AtomElement::C,
                    x_pm: (frame as i32) * 10 + 154,
                    y_pm: 0,
                    z_pm: 0,
                    charge_micro: 0,
                },
            ],
            bonds: vec![Bond {
                bond_id: 0,
                atom_a: 0,
                atom_b: 1,
                order: BondOrder::Single,
                length_pm: 154,
            }],
            potential_energy_uj: -1000 - (frame as i64) * 50,
        })
        .collect();
    let encoded = encode_to_vec(&trajectory).expect("encode trajectory failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress trajectory failed");
    let decompressed = decompress(&compressed).expect("decompress trajectory failed");
    let (decoded, _): (Vec<MoleculeSnapshot>, usize) =
        decode_from_slice(&decompressed).expect("decode trajectory failed");
    assert_eq!(trajectory, decoded);
    assert_eq!(decoded.len(), 5);
}

// --- Protein backbone simulation ---

#[test]
fn test_protein_backbone_simulation_snapshot() {
    // Simplified backbone: alternating N-CA-C-O pattern (4 residues)
    let elements = [
        AtomElement::N,
        AtomElement::C,
        AtomElement::C,
        AtomElement::O,
    ];
    let atoms: Vec<Atom> = (0u32..20)
        .map(|i| Atom {
            atom_id: i,
            element: match i % 4 {
                0 => AtomElement::N,
                1 => AtomElement::C,
                2 => AtomElement::C,
                _ => AtomElement::O,
            },
            x_pm: (i as i32) * 132,
            y_pm: if i % 2 == 0 { 50 } else { -50 },
            z_pm: (i as i32) * 20,
            charge_micro: if i % 4 == 0 { -300 } else { 0 },
        })
        .collect();
    let _ = elements; // used via closure above for clarity
    let bonds: Vec<Bond> = (0u32..19)
        .map(|i| Bond {
            bond_id: i,
            atom_a: i,
            atom_b: i + 1,
            order: if i % 3 == 2 {
                BondOrder::Double
            } else {
                BondOrder::Single
            },
            length_pm: 132,
        })
        .collect();
    let snapshot = MoleculeSnapshot {
        frame_id: 10,
        time_ps: 250,
        atoms,
        bonds,
        potential_energy_uj: -312_500,
    };
    let encoded = encode_to_vec(&snapshot).expect("encode protein backbone failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress protein backbone failed");
    let decompressed = decompress(&compressed).expect("decompress protein backbone failed");
    let (decoded, _): (MoleculeSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode protein backbone failed");
    assert_eq!(snapshot, decoded);
}

// --- DNA strand snapshot ---

#[test]
fn test_dna_strand_snapshot_compress_roundtrip() {
    // Phosphate backbone: alternating P-O-C pattern
    let atoms: Vec<Atom> = (0u32..30)
        .map(|i| Atom {
            atom_id: i,
            element: match i % 3 {
                0 => AtomElement::P,
                1 => AtomElement::O,
                _ => AtomElement::C,
            },
            x_pm: (i as i32) * 160,
            y_pm: ((i as f32) * 36.0_f32.to_radians().sin() * 100.0) as i32,
            z_pm: ((i as f32) * 36.0_f32.to_radians().cos() * 100.0) as i32,
            charge_micro: if i % 3 == 0 { -1000 } else { 0 },
        })
        .collect();
    let bonds: Vec<Bond> = (0u32..29)
        .map(|i| Bond {
            bond_id: i,
            atom_a: i,
            atom_b: i + 1,
            order: BondOrder::Single,
            length_pm: 160,
        })
        .collect();
    let snapshot = MoleculeSnapshot {
        frame_id: 5,
        time_ps: 500,
        atoms,
        bonds,
        potential_energy_uj: -720_000,
    };
    let encoded = encode_to_vec(&snapshot).expect("encode DNA strand failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress DNA strand failed");
    let decompressed = decompress(&compressed).expect("decompress DNA strand failed");
    let (decoded, _): (MoleculeSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode DNA strand failed");
    assert_eq!(snapshot, decoded);
}

// --- High negative potential energy ---

#[test]
fn test_high_negative_potential_energy_snapshot() {
    let snapshot = MoleculeSnapshot {
        frame_id: 999,
        time_ps: 99_999,
        atoms: vec![
            Atom {
                atom_id: 0,
                element: AtomElement::Fe,
                x_pm: 0,
                y_pm: 0,
                z_pm: 0,
                charge_micro: 2000,
            },
            Atom {
                atom_id: 1,
                element: AtomElement::O,
                x_pm: 200,
                y_pm: 0,
                z_pm: 0,
                charge_micro: -2000,
            },
        ],
        bonds: vec![Bond {
            bond_id: 0,
            atom_a: 0,
            atom_b: 1,
            order: BondOrder::Double,
            length_pm: 162,
        }],
        potential_energy_uj: i64::MIN / 2,
    };
    let encoded = encode_to_vec(&snapshot).expect("encode high-energy snapshot failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress high-energy snapshot failed");
    let decompressed = decompress(&compressed).expect("decompress high-energy snapshot failed");
    let (decoded, _): (MoleculeSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode high-energy snapshot failed");
    assert_eq!(snapshot, decoded);
    assert_eq!(decoded.potential_energy_uj, i64::MIN / 2);
}

// --- Full molecule with all element types ---

#[test]
fn test_full_molecule_all_element_types_compress_roundtrip() {
    let atoms = vec![
        Atom {
            atom_id: 0,
            element: AtomElement::H,
            x_pm: 0,
            y_pm: 0,
            z_pm: 0,
            charge_micro: 1000,
        },
        Atom {
            atom_id: 1,
            element: AtomElement::C,
            x_pm: 110,
            y_pm: 0,
            z_pm: 0,
            charge_micro: 0,
        },
        Atom {
            atom_id: 2,
            element: AtomElement::N,
            x_pm: 220,
            y_pm: 0,
            z_pm: 0,
            charge_micro: -300,
        },
        Atom {
            atom_id: 3,
            element: AtomElement::O,
            x_pm: 330,
            y_pm: 0,
            z_pm: 0,
            charge_micro: -600,
        },
        Atom {
            atom_id: 4,
            element: AtomElement::S,
            x_pm: 440,
            y_pm: 0,
            z_pm: 0,
            charge_micro: -200,
        },
        Atom {
            atom_id: 5,
            element: AtomElement::P,
            x_pm: 550,
            y_pm: 0,
            z_pm: 0,
            charge_micro: -1000,
        },
        Atom {
            atom_id: 6,
            element: AtomElement::Fe,
            x_pm: 660,
            y_pm: 0,
            z_pm: 0,
            charge_micro: 2000,
        },
        Atom {
            atom_id: 7,
            element: AtomElement::Ca,
            x_pm: 770,
            y_pm: 0,
            z_pm: 0,
            charge_micro: 2000,
        },
        Atom {
            atom_id: 8,
            element: AtomElement::Na,
            x_pm: 880,
            y_pm: 0,
            z_pm: 0,
            charge_micro: 1000,
        },
        Atom {
            atom_id: 9,
            element: AtomElement::Cl,
            x_pm: 990,
            y_pm: 0,
            z_pm: 0,
            charge_micro: -1000,
        },
    ];
    let bonds = vec![
        Bond {
            bond_id: 0,
            atom_a: 0,
            atom_b: 1,
            order: BondOrder::Single,
            length_pm: 110,
        },
        Bond {
            bond_id: 1,
            atom_a: 1,
            atom_b: 2,
            order: BondOrder::Double,
            length_pm: 132,
        },
        Bond {
            bond_id: 2,
            atom_a: 2,
            atom_b: 3,
            order: BondOrder::Triple,
            length_pm: 116,
        },
        Bond {
            bond_id: 3,
            atom_a: 3,
            atom_b: 4,
            order: BondOrder::Aromatic,
            length_pm: 140,
        },
    ];
    let snapshot = MoleculeSnapshot {
        frame_id: 42,
        time_ps: 4200,
        atoms,
        bonds,
        potential_energy_uj: -1_234_567,
    };
    let encoded = encode_to_vec(&snapshot).expect("encode full-element snapshot failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress full-element snapshot failed");
    let decompressed = decompress(&compressed).expect("decompress full-element snapshot failed");
    let (decoded, _): (MoleculeSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode full-element snapshot failed");
    assert_eq!(snapshot, decoded);
    assert_eq!(decoded.atoms.len(), 10);
}
