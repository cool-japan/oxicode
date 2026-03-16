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
// Domain types  — bioinformatics / protein structure / molecular biology
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AminoAcid {
    code: u8,
    hydrophobicity_x100: i32,
    molecular_weight_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProteinResidue {
    residue_id: u32,
    amino_acid_code: u8,
    x_pm: i32,
    y_pm: i32,
    z_pm: i32,
    b_factor_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProteinChain {
    chain_id: u8,
    residues: Vec<ProteinResidue>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MutationRecord {
    position: u32,
    original_aa: u8,
    mutant_aa: u8,
    delta_stability_x100: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExpressionLevel {
    gene_id: u64,
    sample_id: u32,
    tpm_x1000: u32,
    log2_fold_change_x1000: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SecondaryStructure {
    Helix,
    Sheet,
    Loop,
    Turn,
    Coil,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BindingSiteType {
    Active,
    Allosteric,
    Cofactor,
    MetalIon,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MutationType {
    Missense,
    Nonsense,
    Frameshift,
    Synonymous,
}

// ---------------------------------------------------------------------------
// Strategy helpers
// ---------------------------------------------------------------------------

fn arb_protein_residue() -> impl Strategy<Value = ProteinResidue> {
    (
        any::<u32>(),
        any::<u8>(),
        any::<i32>(),
        any::<i32>(),
        any::<i32>(),
        any::<u32>(),
    )
        .prop_map(
            |(residue_id, amino_acid_code, x_pm, y_pm, z_pm, b_factor_x100)| ProteinResidue {
                residue_id,
                amino_acid_code,
                x_pm,
                y_pm,
                z_pm,
                b_factor_x100,
            },
        )
}

fn arb_amino_acid() -> impl Strategy<Value = AminoAcid> {
    (any::<u8>(), any::<i32>(), any::<u32>()).prop_map(
        |(code, hydrophobicity_x100, molecular_weight_x100)| AminoAcid {
            code,
            hydrophobicity_x100,
            molecular_weight_x100,
        },
    )
}

fn arb_protein_chain() -> impl Strategy<Value = ProteinChain> {
    (
        any::<u8>(),
        prop::collection::vec(arb_protein_residue(), 0..5),
    )
        .prop_map(|(chain_id, residues)| ProteinChain { chain_id, residues })
}

fn arb_mutation_record() -> impl Strategy<Value = MutationRecord> {
    (any::<u32>(), any::<u8>(), any::<u8>(), any::<i32>()).prop_map(
        |(position, original_aa, mutant_aa, delta_stability_x100)| MutationRecord {
            position,
            original_aa,
            mutant_aa,
            delta_stability_x100,
        },
    )
}

fn arb_expression_level() -> impl Strategy<Value = ExpressionLevel> {
    (any::<u64>(), any::<u32>(), any::<u32>(), any::<i32>()).prop_map(
        |(gene_id, sample_id, tpm_x1000, log2_fold_change_x1000)| ExpressionLevel {
            gene_id,
            sample_id,
            tpm_x1000,
            log2_fold_change_x1000,
        },
    )
}

fn arb_secondary_structure() -> impl Strategy<Value = SecondaryStructure> {
    prop_oneof![
        Just(SecondaryStructure::Helix),
        Just(SecondaryStructure::Sheet),
        Just(SecondaryStructure::Loop),
        Just(SecondaryStructure::Turn),
        Just(SecondaryStructure::Coil),
    ]
}

fn arb_binding_site_type() -> impl Strategy<Value = BindingSiteType> {
    prop_oneof![
        Just(BindingSiteType::Active),
        Just(BindingSiteType::Allosteric),
        Just(BindingSiteType::Cofactor),
        Just(BindingSiteType::MetalIon),
    ]
}

fn arb_mutation_type() -> impl Strategy<Value = MutationType> {
    prop_oneof![
        Just(MutationType::Missense),
        Just(MutationType::Nonsense),
        Just(MutationType::Frameshift),
        Just(MutationType::Synonymous),
    ]
}

// ---------------------------------------------------------------------------
// Property-based tests
// ---------------------------------------------------------------------------

proptest! {
    // 1. AminoAcid roundtrip
    #[test]
    fn test_amino_acid_roundtrip(val in arb_amino_acid()) {
        let bytes = encode_to_vec(&val).expect("AminoAcid encode failed");
        let (decoded, _): (AminoAcid, usize) =
            decode_from_slice(&bytes).expect("AminoAcid decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 2. ProteinResidue roundtrip
    #[test]
    fn test_protein_residue_roundtrip(val in arb_protein_residue()) {
        let bytes = encode_to_vec(&val).expect("ProteinResidue encode failed");
        let (decoded, _): (ProteinResidue, usize) =
            decode_from_slice(&bytes).expect("ProteinResidue decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 3. ProteinChain roundtrip (Vec<ProteinResidue> of size 0..5)
    #[test]
    fn test_protein_chain_roundtrip(val in arb_protein_chain()) {
        let bytes = encode_to_vec(&val).expect("ProteinChain encode failed");
        let (decoded, _): (ProteinChain, usize) =
            decode_from_slice(&bytes).expect("ProteinChain decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 4. MutationRecord roundtrip
    #[test]
    fn test_mutation_record_roundtrip(val in arb_mutation_record()) {
        let bytes = encode_to_vec(&val).expect("MutationRecord encode failed");
        let (decoded, _): (MutationRecord, usize) =
            decode_from_slice(&bytes).expect("MutationRecord decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 5. ExpressionLevel roundtrip
    #[test]
    fn test_expression_level_roundtrip(val in arb_expression_level()) {
        let bytes = encode_to_vec(&val).expect("ExpressionLevel encode failed");
        let (decoded, _): (ExpressionLevel, usize) =
            decode_from_slice(&bytes).expect("ExpressionLevel decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 6. SecondaryStructure enum roundtrip
    #[test]
    fn test_secondary_structure_roundtrip(val in arb_secondary_structure()) {
        let bytes = encode_to_vec(&val).expect("SecondaryStructure encode failed");
        let (decoded, _): (SecondaryStructure, usize) =
            decode_from_slice(&bytes).expect("SecondaryStructure decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 7. BindingSiteType enum roundtrip
    #[test]
    fn test_binding_site_type_roundtrip(val in arb_binding_site_type()) {
        let bytes = encode_to_vec(&val).expect("BindingSiteType encode failed");
        let (decoded, _): (BindingSiteType, usize) =
            decode_from_slice(&bytes).expect("BindingSiteType decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 8. MutationType enum roundtrip
    #[test]
    fn test_mutation_type_roundtrip(val in arb_mutation_type()) {
        let bytes = encode_to_vec(&val).expect("MutationType encode failed");
        let (decoded, _): (MutationType, usize) =
            decode_from_slice(&bytes).expect("MutationType decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 9. Deterministic encoding — AminoAcid encoded twice must produce identical bytes
    #[test]
    fn test_amino_acid_deterministic(val in arb_amino_acid()) {
        let bytes_a = encode_to_vec(&val).expect("first AminoAcid encode failed");
        let bytes_b = encode_to_vec(&val).expect("second AminoAcid encode failed");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 10. Deterministic encoding — ProteinChain
    #[test]
    fn test_protein_chain_deterministic(val in arb_protein_chain()) {
        let bytes_a = encode_to_vec(&val).expect("first ProteinChain encode failed");
        let bytes_b = encode_to_vec(&val).expect("second ProteinChain encode failed");
        prop_assert_eq!(bytes_a, bytes_b);
    }

    // 11. Consumed bytes == encoded length for ProteinResidue
    #[test]
    fn test_protein_residue_consumed_bytes(val in arb_protein_residue()) {
        let bytes = encode_to_vec(&val).expect("ProteinResidue encode failed");
        let encoded_len = bytes.len();
        let (_, consumed): (ProteinResidue, usize) =
            decode_from_slice(&bytes).expect("ProteinResidue decode failed");
        prop_assert_eq!(consumed, encoded_len);
    }

    // 12. Consumed bytes == encoded length for ExpressionLevel
    #[test]
    fn test_expression_level_consumed_bytes(val in arb_expression_level()) {
        let bytes = encode_to_vec(&val).expect("ExpressionLevel encode failed");
        let encoded_len = bytes.len();
        let (_, consumed): (ExpressionLevel, usize) =
            decode_from_slice(&bytes).expect("ExpressionLevel decode failed");
        prop_assert_eq!(consumed, encoded_len);
    }

    // 13. Vec<AminoAcid> roundtrip (0..8 elements)
    #[test]
    fn test_vec_amino_acid_roundtrip(
        vals in prop::collection::vec(arb_amino_acid(), 0..8)
    ) {
        let bytes = encode_to_vec(&vals).expect("Vec<AminoAcid> encode failed");
        let (decoded, _): (Vec<AminoAcid>, usize) =
            decode_from_slice(&bytes).expect("Vec<AminoAcid> decode failed");
        prop_assert_eq!(vals, decoded);
    }

    // 14. Vec<MutationRecord> roundtrip (0..8 elements)
    #[test]
    fn test_vec_mutation_record_roundtrip(
        vals in prop::collection::vec(arb_mutation_record(), 0..8)
    ) {
        let bytes = encode_to_vec(&vals).expect("Vec<MutationRecord> encode failed");
        let (decoded, _): (Vec<MutationRecord>, usize) =
            decode_from_slice(&bytes).expect("Vec<MutationRecord> decode failed");
        prop_assert_eq!(vals, decoded);
    }

    // 15. Vec<SecondaryStructure> roundtrip (0..8 elements)
    #[test]
    fn test_vec_secondary_structure_roundtrip(
        vals in prop::collection::vec(arb_secondary_structure(), 0..8)
    ) {
        let bytes = encode_to_vec(&vals).expect("Vec<SecondaryStructure> encode failed");
        let (decoded, _): (Vec<SecondaryStructure>, usize) =
            decode_from_slice(&bytes).expect("Vec<SecondaryStructure> decode failed");
        prop_assert_eq!(vals, decoded);
    }

    // 16. Option<AminoAcid> roundtrip — Some variant
    #[test]
    fn test_option_amino_acid_some_roundtrip(val in arb_amino_acid()) {
        let opt: Option<AminoAcid> = Some(val);
        let bytes = encode_to_vec(&opt).expect("Option<AminoAcid> Some encode failed");
        let (decoded, _): (Option<AminoAcid>, usize) =
            decode_from_slice(&bytes).expect("Option<AminoAcid> Some decode failed");
        prop_assert_eq!(opt, decoded);
    }

    // 17. Option<MutationRecord> roundtrip — None variant
    #[test]
    fn test_option_mutation_record_none_roundtrip(_dummy in any::<u8>()) {
        let opt: Option<MutationRecord> = None;
        let bytes = encode_to_vec(&opt).expect("Option<MutationRecord> None encode failed");
        let (decoded, _): (Option<MutationRecord>, usize) =
            decode_from_slice(&bytes).expect("Option<MutationRecord> None decode failed");
        prop_assert_eq!(opt, decoded);
    }

    // 18. Primitive i32 roundtrip (covers signed integer path used by coordinates / deltas)
    #[test]
    fn test_primitive_i32_roundtrip(val in any::<i32>()) {
        let bytes = encode_to_vec(&val).expect("i32 encode failed");
        let (decoded, _): (i32, usize) =
            decode_from_slice(&bytes).expect("i32 decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 19. Primitive u64 roundtrip (covers gene_id, large identifiers)
    #[test]
    fn test_primitive_u64_roundtrip(val in any::<u64>()) {
        let bytes = encode_to_vec(&val).expect("u64 encode failed");
        let (decoded, _): (u64, usize) =
            decode_from_slice(&bytes).expect("u64 decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 20. Primitive f32 roundtrip (covers B-factors / scores stored as floats)
    #[test]
    fn test_primitive_f32_roundtrip(val in any::<f32>()) {
        let bytes = encode_to_vec(&val).expect("f32 encode failed");
        let (decoded, _): (f32, usize) =
            decode_from_slice(&bytes).expect("f32 decode failed");
        // NaN != NaN by IEEE 754, so compare bit patterns instead
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    // 21. Primitive bool roundtrip (flag fields — e.g. is_active_site, is_conserved)
    #[test]
    fn test_primitive_bool_roundtrip(val in any::<bool>()) {
        let bytes = encode_to_vec(&val).expect("bool encode failed");
        let (decoded, _): (bool, usize) =
            decode_from_slice(&bytes).expect("bool decode failed");
        prop_assert_eq!(val, decoded);
    }

    // 22. String roundtrip (covers gene/chain name, PDB identifiers, FASTA headers)
    #[test]
    fn test_string_roundtrip(val in "[A-Za-z0-9_\\-]{0,64}") {
        let bytes = encode_to_vec(&val).expect("String encode failed");
        let (decoded, _): (String, usize) =
            decode_from_slice(&bytes).expect("String decode failed");
        prop_assert_eq!(val, decoded);
    }
}
