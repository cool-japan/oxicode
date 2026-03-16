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
use proptest::prelude::*;

// ── Domain types ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Atom {
    atom_id: u32,
    element: u8,
    charge_x1000: i32,
    x_pm: i32,
    y_pm: i32,
    z_pm: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Bond {
    bond_id: u32,
    atom_a: u32,
    atom_b: u32,
    bond_order_x100: u32,
    length_pm: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Molecule {
    mol_id: u64,
    formula_hash: u64,
    num_atoms: u16,
    num_bonds: u16,
    mol_weight_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DockingScore {
    ligand_id: u64,
    receptor_id: u32,
    binding_energy_x1000: i32,
    rmsd_pm_x100: u32,
    pose_count: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PharmType {
    HydrogenDonor,
    HydrogenAcceptor,
    Hydrophobic,
    Aromatic,
    Cationic,
    Anionic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PharmacophoreFeature {
    feature_id: u32,
    feature_type: PharmType,
    x_pm: i32,
    y_pm: i32,
    z_pm: i32,
    radius_pm: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ForceField {
    AMBER,
    CHARMM,
    GROMOS,
    OPLS,
    Dreiding,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SolvationModel {
    Vacuum,
    ImplicitWater,
    ExplicitWater,
    GBSA,
    PBSA,
}

// ── Strategies ──────────────────────────────────────────────────────────────

fn arb_atom() -> impl Strategy<Value = Atom> {
    (
        any::<u32>(),
        any::<u8>(),
        any::<i32>(),
        any::<i32>(),
        any::<i32>(),
        any::<i32>(),
    )
        .prop_map(|(atom_id, element, charge_x1000, x_pm, y_pm, z_pm)| Atom {
            atom_id,
            element,
            charge_x1000,
            x_pm,
            y_pm,
            z_pm,
        })
}

fn arb_bond() -> impl Strategy<Value = Bond> {
    (
        any::<u32>(),
        any::<u32>(),
        any::<u32>(),
        any::<u32>(),
        any::<u32>(),
    )
        .prop_map(
            |(bond_id, atom_a, atom_b, bond_order_x100, length_pm)| Bond {
                bond_id,
                atom_a,
                atom_b,
                bond_order_x100,
                length_pm,
            },
        )
}

fn arb_molecule() -> impl Strategy<Value = Molecule> {
    (
        any::<u64>(),
        any::<u64>(),
        any::<u16>(),
        any::<u16>(),
        any::<u32>(),
    )
        .prop_map(
            |(mol_id, formula_hash, num_atoms, num_bonds, mol_weight_x100)| Molecule {
                mol_id,
                formula_hash,
                num_atoms,
                num_bonds,
                mol_weight_x100,
            },
        )
}

fn arb_docking_score() -> impl Strategy<Value = DockingScore> {
    (
        any::<u64>(),
        any::<u32>(),
        any::<i32>(),
        any::<u32>(),
        any::<u16>(),
    )
        .prop_map(
            |(ligand_id, receptor_id, binding_energy_x1000, rmsd_pm_x100, pose_count)| {
                DockingScore {
                    ligand_id,
                    receptor_id,
                    binding_energy_x1000,
                    rmsd_pm_x100,
                    pose_count,
                }
            },
        )
}

fn arb_pharm_type() -> impl Strategy<Value = PharmType> {
    prop_oneof![
        Just(PharmType::HydrogenDonor),
        Just(PharmType::HydrogenAcceptor),
        Just(PharmType::Hydrophobic),
        Just(PharmType::Aromatic),
        Just(PharmType::Cationic),
        Just(PharmType::Anionic),
    ]
}

fn arb_pharmacophore_feature() -> impl Strategy<Value = PharmacophoreFeature> {
    (
        any::<u32>(),
        arb_pharm_type(),
        any::<i32>(),
        any::<i32>(),
        any::<i32>(),
        any::<u32>(),
    )
        .prop_map(|(feature_id, feature_type, x_pm, y_pm, z_pm, radius_pm)| {
            PharmacophoreFeature {
                feature_id,
                feature_type,
                x_pm,
                y_pm,
                z_pm,
                radius_pm,
            }
        })
}

fn arb_force_field() -> impl Strategy<Value = ForceField> {
    prop_oneof![
        Just(ForceField::AMBER),
        Just(ForceField::CHARMM),
        Just(ForceField::GROMOS),
        Just(ForceField::OPLS),
        Just(ForceField::Dreiding),
    ]
}

fn arb_solvation_model() -> impl Strategy<Value = SolvationModel> {
    prop_oneof![
        Just(SolvationModel::Vacuum),
        Just(SolvationModel::ImplicitWater),
        Just(SolvationModel::ExplicitWater),
        Just(SolvationModel::GBSA),
        Just(SolvationModel::PBSA),
    ]
}

// ── Tests ────────────────────────────────────────────────────────────────────

proptest! {
    // 1. Atom roundtrip
    #[test]
    fn test_atom_roundtrip(atom in arb_atom()) {
        let bytes = encode_to_vec(&atom).expect("encode Atom");
        let (decoded, _): (Atom, usize) = decode_from_slice(&bytes).expect("decode Atom");
        prop_assert_eq!(atom, decoded);
    }

    // 2. Atom deterministic encoding
    #[test]
    fn test_atom_deterministic(atom in arb_atom()) {
        let bytes_a = encode_to_vec(&atom).expect("encode Atom first");
        let bytes_b = encode_to_vec(&atom).expect("encode Atom second");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 3. Atom consumed bytes equals total length
    #[test]
    fn test_atom_consumed_bytes(atom in arb_atom()) {
        let bytes = encode_to_vec(&atom).expect("encode Atom");
        let len = bytes.len();
        let (_, consumed): (Atom, usize) = decode_from_slice(&bytes).expect("decode Atom");
        prop_assert_eq!(consumed, len);
    }

    // 4. Bond roundtrip
    #[test]
    fn test_bond_roundtrip(bond in arb_bond()) {
        let bytes = encode_to_vec(&bond).expect("encode Bond");
        let (decoded, _): (Bond, usize) = decode_from_slice(&bytes).expect("decode Bond");
        prop_assert_eq!(bond, decoded);
    }

    // 5. Bond deterministic encoding
    #[test]
    fn test_bond_deterministic(bond in arb_bond()) {
        let bytes_a = encode_to_vec(&bond).expect("encode Bond first");
        let bytes_b = encode_to_vec(&bond).expect("encode Bond second");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 6. Molecule roundtrip
    #[test]
    fn test_molecule_roundtrip(mol in arb_molecule()) {
        let bytes = encode_to_vec(&mol).expect("encode Molecule");
        let (decoded, _): (Molecule, usize) = decode_from_slice(&bytes).expect("decode Molecule");
        prop_assert_eq!(mol, decoded);
    }

    // 7. Molecule consumed bytes equals total length
    #[test]
    fn test_molecule_consumed_bytes(mol in arb_molecule()) {
        let bytes = encode_to_vec(&mol).expect("encode Molecule");
        let len = bytes.len();
        let (_, consumed): (Molecule, usize) = decode_from_slice(&bytes).expect("decode Molecule");
        prop_assert_eq!(consumed, len);
    }

    // 8. DockingScore roundtrip
    #[test]
    fn test_docking_score_roundtrip(score in arb_docking_score()) {
        let bytes = encode_to_vec(&score).expect("encode DockingScore");
        let (decoded, _): (DockingScore, usize) = decode_from_slice(&bytes).expect("decode DockingScore");
        prop_assert_eq!(score, decoded);
    }

    // 9. DockingScore deterministic encoding
    #[test]
    fn test_docking_score_deterministic(score in arb_docking_score()) {
        let bytes_a = encode_to_vec(&score).expect("encode DockingScore first");
        let bytes_b = encode_to_vec(&score).expect("encode DockingScore second");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 10. PharmType enum roundtrip
    #[test]
    fn test_pharm_type_roundtrip(pt in arb_pharm_type()) {
        let bytes = encode_to_vec(&pt).expect("encode PharmType");
        let (decoded, _): (PharmType, usize) = decode_from_slice(&bytes).expect("decode PharmType");
        prop_assert_eq!(pt, decoded);
    }

    // 11. PharmacophoreFeature roundtrip
    #[test]
    fn test_pharmacophore_feature_roundtrip(feat in arb_pharmacophore_feature()) {
        let bytes = encode_to_vec(&feat).expect("encode PharmacophoreFeature");
        let (decoded, _): (PharmacophoreFeature, usize) = decode_from_slice(&bytes).expect("decode PharmacophoreFeature");
        prop_assert_eq!(feat, decoded);
    }

    // 12. PharmacophoreFeature consumed bytes equals total length
    #[test]
    fn test_pharmacophore_feature_consumed_bytes(feat in arb_pharmacophore_feature()) {
        let bytes = encode_to_vec(&feat).expect("encode PharmacophoreFeature");
        let len = bytes.len();
        let (_, consumed): (PharmacophoreFeature, usize) = decode_from_slice(&bytes).expect("decode PharmacophoreFeature");
        prop_assert_eq!(consumed, len);
    }

    // 13. ForceField enum roundtrip
    #[test]
    fn test_force_field_roundtrip(ff in arb_force_field()) {
        let bytes = encode_to_vec(&ff).expect("encode ForceField");
        let (decoded, _): (ForceField, usize) = decode_from_slice(&bytes).expect("decode ForceField");
        prop_assert_eq!(ff, decoded);
    }

    // 14. ForceField deterministic encoding
    #[test]
    fn test_force_field_deterministic(ff in arb_force_field()) {
        let bytes_a = encode_to_vec(&ff).expect("encode ForceField first");
        let bytes_b = encode_to_vec(&ff).expect("encode ForceField second");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 15. SolvationModel enum roundtrip
    #[test]
    fn test_solvation_model_roundtrip(sm in arb_solvation_model()) {
        let bytes = encode_to_vec(&sm).expect("encode SolvationModel");
        let (decoded, _): (SolvationModel, usize) = decode_from_slice(&bytes).expect("decode SolvationModel");
        prop_assert_eq!(sm, decoded);
    }

    // 16. Vec<Atom> roundtrip (0..6 elements)
    #[test]
    fn test_vec_atom_roundtrip(atoms in proptest::collection::vec(arb_atom(), 0..6)) {
        let bytes = encode_to_vec(&atoms).expect("encode Vec<Atom>");
        let (decoded, _): (Vec<Atom>, usize) = decode_from_slice(&bytes).expect("decode Vec<Atom>");
        prop_assert_eq!(atoms, decoded);
    }

    // 17. Vec<Bond> roundtrip (0..6 elements)
    #[test]
    fn test_vec_bond_roundtrip(bonds in proptest::collection::vec(arb_bond(), 0..6)) {
        let bytes = encode_to_vec(&bonds).expect("encode Vec<Bond>");
        let (decoded, _): (Vec<Bond>, usize) = decode_from_slice(&bytes).expect("decode Vec<Bond>");
        prop_assert_eq!(bonds, decoded);
    }

    // 18. Vec<DockingScore> roundtrip (0..6 elements)
    #[test]
    fn test_vec_docking_score_roundtrip(scores in proptest::collection::vec(arb_docking_score(), 0..6)) {
        let bytes = encode_to_vec(&scores).expect("encode Vec<DockingScore>");
        let (decoded, _): (Vec<DockingScore>, usize) = decode_from_slice(&bytes).expect("decode Vec<DockingScore>");
        prop_assert_eq!(scores, decoded);
    }

    // 19. Option<Atom> roundtrip (Some and None)
    #[test]
    fn test_option_atom_roundtrip(opt in proptest::option::of(arb_atom())) {
        let bytes = encode_to_vec(&opt).expect("encode Option<Atom>");
        let (decoded, _): (Option<Atom>, usize) = decode_from_slice(&bytes).expect("decode Option<Atom>");
        prop_assert_eq!(opt, decoded);
    }

    // 20. Option<DockingScore> roundtrip (Some and None)
    #[test]
    fn test_option_docking_score_roundtrip(opt in proptest::option::of(arb_docking_score())) {
        let bytes = encode_to_vec(&opt).expect("encode Option<DockingScore>");
        let (decoded, _): (Option<DockingScore>, usize) = decode_from_slice(&bytes).expect("decode Option<DockingScore>");
        prop_assert_eq!(opt, decoded);
    }

    // 21. Primitive u64 (mol_id / ligand_id domain) roundtrip
    #[test]
    fn test_primitive_u64_roundtrip(v in any::<u64>()) {
        let bytes = encode_to_vec(&v).expect("encode u64");
        let (decoded, consumed): (u64, usize) = decode_from_slice(&bytes).expect("decode u64");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }

    // 22. Primitive i32 (coordinate / charge domain) roundtrip
    #[test]
    fn test_primitive_i32_roundtrip(v in any::<i32>()) {
        let bytes = encode_to_vec(&v).expect("encode i32");
        let (decoded, consumed): (i32, usize) = decode_from_slice(&bytes).expect("decode i32");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(consumed, bytes.len());
    }
}
