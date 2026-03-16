//! Advanced property-based roundtrip tests (set 50) using proptest.
//!
//! Domain: Pharmaceutical / drug discovery data.
//! Tests verify encode → decode roundtrips for domain types and structural
//! invariants such as consumed == bytes.len() and deterministic encoding.

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MoleculeType {
    SmallMolecule,
    Peptide,
    Antibody,
    NucleicAcid,
    Lipid,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TrialPhase {
    Preclinical,
    PhaseI,
    PhaseII,
    PhaseIII,
    Approved,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Compound {
    compound_id: u64,
    name: String,
    mol_weight: f64,
    solubility: f32,
    molecule_type: MoleculeType,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClinicalTrial {
    trial_id: u64,
    compound_id: u64,
    phase: TrialPhase,
    participants: u32,
    success_rate: f32,
}

// ---------------------------------------------------------------------------
// Arbitrary strategies
// ---------------------------------------------------------------------------

fn arb_molecule_type() -> impl Strategy<Value = MoleculeType> {
    prop_oneof![
        Just(MoleculeType::SmallMolecule),
        Just(MoleculeType::Peptide),
        Just(MoleculeType::Antibody),
        Just(MoleculeType::NucleicAcid),
        Just(MoleculeType::Lipid),
    ]
}

fn arb_trial_phase() -> impl Strategy<Value = TrialPhase> {
    prop_oneof![
        Just(TrialPhase::Preclinical),
        Just(TrialPhase::PhaseI),
        Just(TrialPhase::PhaseII),
        Just(TrialPhase::PhaseIII),
        Just(TrialPhase::Approved),
    ]
}

fn arb_compound() -> impl Strategy<Value = Compound> {
    (
        any::<u64>(),
        any::<String>(),
        proptest::num::f64::NORMAL,
        proptest::num::f32::NORMAL,
        arb_molecule_type(),
    )
        .prop_map(
            |(compound_id, name, mol_weight, solubility, molecule_type)| Compound {
                compound_id,
                name,
                mol_weight,
                solubility,
                molecule_type,
            },
        )
}

fn arb_clinical_trial() -> impl Strategy<Value = ClinicalTrial> {
    (
        any::<u64>(),
        any::<u64>(),
        arb_trial_phase(),
        any::<u32>(),
        proptest::num::f32::NORMAL,
    )
        .prop_map(
            |(trial_id, compound_id, phase, participants, success_rate)| ClinicalTrial {
                trial_id,
                compound_id,
                phase,
                participants,
                success_rate,
            },
        )
}

// ---------------------------------------------------------------------------
// Test 1: MoleculeType roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_molecule_type_roundtrip(mol_type in arb_molecule_type()) {
        let encoded = encode_to_vec(&mol_type).expect("encode MoleculeType failed");
        let (decoded, consumed): (MoleculeType, usize) =
            decode_from_slice(&encoded).expect("decode MoleculeType failed");
        prop_assert_eq!(decoded, mol_type);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 2: TrialPhase roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_trial_phase_roundtrip(phase in arb_trial_phase()) {
        let encoded = encode_to_vec(&phase).expect("encode TrialPhase failed");
        let (decoded, consumed): (TrialPhase, usize) =
            decode_from_slice(&encoded).expect("decode TrialPhase failed");
        prop_assert_eq!(decoded, phase);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 3: Compound roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_compound_roundtrip(compound in arb_compound()) {
        let encoded = encode_to_vec(&compound).expect("encode Compound failed");
        let (decoded, consumed): (Compound, usize) =
            decode_from_slice(&encoded).expect("decode Compound failed");
        prop_assert_eq!(decoded, compound);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 4: ClinicalTrial roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_clinical_trial_roundtrip(trial in arb_clinical_trial()) {
        let encoded = encode_to_vec(&trial).expect("encode ClinicalTrial failed");
        let (decoded, consumed): (ClinicalTrial, usize) =
            decode_from_slice(&encoded).expect("decode ClinicalTrial failed");
        prop_assert_eq!(decoded, trial);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 5: consumed == bytes.len() for MoleculeType
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_molecule_type_consumed_equals_len(mol_type in arb_molecule_type()) {
        let encoded = encode_to_vec(&mol_type).expect("encode MoleculeType failed");
        let (_decoded, consumed): (MoleculeType, usize) =
            decode_from_slice(&encoded).expect("decode MoleculeType failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 6: consumed == bytes.len() for Compound
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_compound_consumed_equals_len(compound in arb_compound()) {
        let encoded = encode_to_vec(&compound).expect("encode Compound failed");
        let (_decoded, consumed): (Compound, usize) =
            decode_from_slice(&encoded).expect("decode Compound failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 7: consumed == bytes.len() for ClinicalTrial
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_clinical_trial_consumed_equals_len(trial in arb_clinical_trial()) {
        let encoded = encode_to_vec(&trial).expect("encode ClinicalTrial failed");
        let (_decoded, consumed): (ClinicalTrial, usize) =
            decode_from_slice(&encoded).expect("decode ClinicalTrial failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 8: deterministic encoding for MoleculeType (two encodes produce identical bytes)
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_molecule_type_deterministic(mol_type in arb_molecule_type()) {
        let encoded_a = encode_to_vec(&mol_type).expect("first encode MoleculeType failed");
        let encoded_b = encode_to_vec(&mol_type).expect("second encode MoleculeType failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }
}

// ---------------------------------------------------------------------------
// Test 9: deterministic encoding for Compound
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_compound_deterministic(compound in arb_compound()) {
        let encoded_a = encode_to_vec(&compound).expect("first encode Compound failed");
        let encoded_b = encode_to_vec(&compound).expect("second encode Compound failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }
}

// ---------------------------------------------------------------------------
// Test 10: deterministic encoding for ClinicalTrial
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_clinical_trial_deterministic(trial in arb_clinical_trial()) {
        let encoded_a = encode_to_vec(&trial).expect("first encode ClinicalTrial failed");
        let encoded_b = encode_to_vec(&trial).expect("second encode ClinicalTrial failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }
}

// ---------------------------------------------------------------------------
// Test 11: Vec<Compound> roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_vec_compound_roundtrip(
        compounds in proptest::collection::vec(arb_compound(), 0..10)
    ) {
        let encoded = encode_to_vec(&compounds).expect("encode Vec<Compound> failed");
        let (decoded, consumed): (Vec<Compound>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Compound> failed");
        prop_assert_eq!(decoded, compounds);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 12: Option<ClinicalTrial> roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_option_clinical_trial_roundtrip(
        maybe_trial in proptest::option::of(arb_clinical_trial())
    ) {
        let encoded = encode_to_vec(&maybe_trial).expect("encode Option<ClinicalTrial> failed");
        let (decoded, consumed): (Option<ClinicalTrial>, usize) =
            decode_from_slice(&encoded).expect("decode Option<ClinicalTrial> failed");
        prop_assert_eq!(decoded, maybe_trial);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 13: re-encode idempotency for Compound
//   decode(encode(v)) == v, then encode(decoded) == encode(v)
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_compound_reencode_idempotency(compound in arb_compound()) {
        let encoded_first = encode_to_vec(&compound).expect("first encode Compound failed");
        let (decoded, _consumed): (Compound, usize) =
            decode_from_slice(&encoded_first).expect("decode Compound failed");
        let encoded_second = encode_to_vec(&decoded).expect("re-encode Compound failed");
        prop_assert_eq!(encoded_first, encoded_second);
    }
}

// ---------------------------------------------------------------------------
// Test 14: re-encode idempotency for ClinicalTrial
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_clinical_trial_reencode_idempotency(trial in arb_clinical_trial()) {
        let encoded_first = encode_to_vec(&trial).expect("first encode ClinicalTrial failed");
        let (decoded, _consumed): (ClinicalTrial, usize) =
            decode_from_slice(&encoded_first).expect("decode ClinicalTrial failed");
        let encoded_second = encode_to_vec(&decoded).expect("re-encode ClinicalTrial failed");
        prop_assert_eq!(encoded_first, encoded_second);
    }
}

// ---------------------------------------------------------------------------
// Test 15: all MoleculeType variants encode and decode correctly (exhaustive)
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_all_molecule_type_variants(_dummy: bool) {
        let variants = [
            MoleculeType::SmallMolecule,
            MoleculeType::Peptide,
            MoleculeType::Antibody,
            MoleculeType::NucleicAcid,
            MoleculeType::Lipid,
        ];
        for variant in &variants {
            let encoded = encode_to_vec(variant).expect("encode MoleculeType variant failed");
            let (decoded, consumed): (MoleculeType, usize) =
                decode_from_slice(&encoded).expect("decode MoleculeType variant failed");
            prop_assert_eq!(&decoded, variant);
            prop_assert_eq!(consumed, encoded.len());
        }
    }
}

// ---------------------------------------------------------------------------
// Test 16: all TrialPhase variants encode and decode correctly (exhaustive)
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_all_trial_phase_variants(_dummy: bool) {
        let variants = [
            TrialPhase::Preclinical,
            TrialPhase::PhaseI,
            TrialPhase::PhaseII,
            TrialPhase::PhaseIII,
            TrialPhase::Approved,
        ];
        for variant in &variants {
            let encoded = encode_to_vec(variant).expect("encode TrialPhase variant failed");
            let (decoded, consumed): (TrialPhase, usize) =
                decode_from_slice(&encoded).expect("decode TrialPhase variant failed");
            prop_assert_eq!(&decoded, variant);
            prop_assert_eq!(consumed, encoded.len());
        }
    }
}

// ---------------------------------------------------------------------------
// Test 17: Compound with empty name roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_compound_empty_name_roundtrip(
        compound_id: u64,
        mol_weight in proptest::num::f64::NORMAL,
        solubility in proptest::num::f32::NORMAL,
        molecule_type in arb_molecule_type(),
    ) {
        let compound = Compound {
            compound_id,
            name: String::new(),
            mol_weight,
            solubility,
            molecule_type,
        };
        let encoded = encode_to_vec(&compound).expect("encode Compound (empty name) failed");
        let (decoded, consumed): (Compound, usize) =
            decode_from_slice(&encoded).expect("decode Compound (empty name) failed");
        prop_assert_eq!(decoded, compound);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 18: ClinicalTrial with zero participants roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_clinical_trial_zero_participants(
        trial_id: u64,
        compound_id: u64,
        phase in arb_trial_phase(),
        success_rate in proptest::num::f32::NORMAL,
    ) {
        let trial = ClinicalTrial {
            trial_id,
            compound_id,
            phase,
            participants: 0,
            success_rate,
        };
        let encoded = encode_to_vec(&trial).expect("encode ClinicalTrial (0 participants) failed");
        let (decoded, consumed): (ClinicalTrial, usize) =
            decode_from_slice(&encoded).expect("decode ClinicalTrial (0 participants) failed");
        prop_assert_eq!(decoded, trial);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 19: Vec<ClinicalTrial> roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_vec_clinical_trial_roundtrip(
        trials in proptest::collection::vec(arb_clinical_trial(), 0..8)
    ) {
        let encoded = encode_to_vec(&trials).expect("encode Vec<ClinicalTrial> failed");
        let (decoded, consumed): (Vec<ClinicalTrial>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<ClinicalTrial> failed");
        prop_assert_eq!(decoded, trials);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 20: Option<Compound> roundtrip (None and Some variants)
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_option_compound_roundtrip(
        maybe_compound in proptest::option::of(arb_compound())
    ) {
        let encoded = encode_to_vec(&maybe_compound).expect("encode Option<Compound> failed");
        let (decoded, consumed): (Option<Compound>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Compound> failed");
        prop_assert_eq!(decoded, maybe_compound);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 21: Compound with maximum field values roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_compound_max_id_roundtrip(
        name: String,
        mol_weight in proptest::num::f64::NORMAL,
        solubility in proptest::num::f32::NORMAL,
        molecule_type in arb_molecule_type(),
    ) {
        let compound = Compound {
            compound_id: u64::MAX,
            name,
            mol_weight,
            solubility,
            molecule_type,
        };
        let encoded = encode_to_vec(&compound).expect("encode Compound (max id) failed");
        let (decoded, consumed): (Compound, usize) =
            decode_from_slice(&encoded).expect("decode Compound (max id) failed");
        prop_assert_eq!(decoded, compound);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 22: Pair of (Compound, ClinicalTrial) tuple roundtrip
// ---------------------------------------------------------------------------
proptest! {
    #[test]
    fn prop_pharm_compound_trial_pair_roundtrip(
        compound in arb_compound(),
        trial in arb_clinical_trial(),
    ) {
        let pair = (compound, trial);
        let encoded = encode_to_vec(&pair).expect("encode (Compound, ClinicalTrial) failed");
        let (decoded, consumed): ((Compound, ClinicalTrial), usize) =
            decode_from_slice(&encoded).expect("decode (Compound, ClinicalTrial) failed");
        prop_assert_eq!(decoded, pair);
        prop_assert_eq!(consumed, encoded.len());
    }
}
