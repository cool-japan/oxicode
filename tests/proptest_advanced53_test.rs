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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AminoAcid {
    Ala,
    Arg,
    Asn,
    Asp,
    Cys,
    Gln,
    Glu,
    Gly,
    His,
    Ile,
    Leu,
    Lys,
    Met,
    Phe,
    Pro,
    Ser,
    Thr,
    Trp,
    Tyr,
    Val,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SecondaryStructure {
    AlphaHelix,
    BetaSheet,
    RandomCoil,
    Turn,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Residue {
    aa: AminoAcid,
    structure: SecondaryStructure,
    phi_deg: i32,
    psi_deg: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProteinChain {
    chain_id: u8,
    residues: Vec<Residue>,
    molecular_weight_da: u64,
}

fn amino_acid_strategy() -> impl Strategy<Value = AminoAcid> {
    (0u8..20).prop_map(|v| match v {
        0 => AminoAcid::Ala,
        1 => AminoAcid::Arg,
        2 => AminoAcid::Asn,
        3 => AminoAcid::Asp,
        4 => AminoAcid::Cys,
        5 => AminoAcid::Gln,
        6 => AminoAcid::Glu,
        7 => AminoAcid::Gly,
        8 => AminoAcid::His,
        9 => AminoAcid::Ile,
        10 => AminoAcid::Leu,
        11 => AminoAcid::Lys,
        12 => AminoAcid::Met,
        13 => AminoAcid::Phe,
        14 => AminoAcid::Pro,
        15 => AminoAcid::Ser,
        16 => AminoAcid::Thr,
        17 => AminoAcid::Trp,
        18 => AminoAcid::Tyr,
        _ => AminoAcid::Val,
    })
}

fn secondary_structure_strategy() -> impl Strategy<Value = SecondaryStructure> {
    (0u8..4).prop_map(|v| match v {
        0 => SecondaryStructure::AlphaHelix,
        1 => SecondaryStructure::BetaSheet,
        2 => SecondaryStructure::RandomCoil,
        _ => SecondaryStructure::Turn,
    })
}

fn residue_strategy() -> impl Strategy<Value = Residue> {
    (
        amino_acid_strategy(),
        secondary_structure_strategy(),
        any::<i32>(),
        any::<i32>(),
    )
        .prop_map(|(aa, structure, phi_deg, psi_deg)| Residue {
            aa,
            structure,
            phi_deg,
            psi_deg,
        })
}

fn protein_chain_strategy() -> impl Strategy<Value = ProteinChain> {
    (
        any::<u8>(),
        prop::collection::vec(residue_strategy(), 0..=64),
        any::<u64>(),
    )
        .prop_map(|(chain_id, residues, molecular_weight_da)| ProteinChain {
            chain_id,
            residues,
            molecular_weight_da,
        })
}

proptest! {
    #[test]
    fn test_residue_roundtrip_arbitrary_phi_psi(residue in residue_strategy()) {
        let encoded = encode_to_vec(&residue).expect("encode Residue");
        let (decoded, _) = decode_from_slice::<Residue>(&encoded).expect("decode Residue");
        prop_assert_eq!(residue, decoded);
    }

    #[test]
    fn test_protein_chain_roundtrip_varying_lengths(chain in protein_chain_strategy()) {
        let encoded = encode_to_vec(&chain).expect("encode ProteinChain");
        let (decoded, _) = decode_from_slice::<ProteinChain>(&encoded).expect("decode ProteinChain");
        prop_assert_eq!(chain, decoded);
    }

    #[test]
    fn test_consumed_bytes_equals_encoded_length_residue(residue in residue_strategy()) {
        let encoded = encode_to_vec(&residue).expect("encode Residue for byte check");
        let (_, consumed) = decode_from_slice::<Residue>(&encoded).expect("decode Residue for byte check");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_consumed_bytes_equals_encoded_length_protein_chain(chain in protein_chain_strategy()) {
        let encoded = encode_to_vec(&chain).expect("encode ProteinChain for byte check");
        let (_, consumed) = decode_from_slice::<ProteinChain>(&encoded).expect("decode ProteinChain for byte check");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_encode_deterministic_residue(residue in residue_strategy()) {
        let encoded1 = encode_to_vec(&residue).expect("encode Residue first time");
        let encoded2 = encode_to_vec(&residue).expect("encode Residue second time");
        prop_assert_eq!(encoded1, encoded2);
    }

    #[test]
    fn test_encode_deterministic_protein_chain(chain in protein_chain_strategy()) {
        let encoded1 = encode_to_vec(&chain).expect("encode ProteinChain first time");
        let encoded2 = encode_to_vec(&chain).expect("encode ProteinChain second time");
        prop_assert_eq!(encoded1, encoded2);
    }

    #[test]
    fn test_vec_residue_roundtrip(residues in prop::collection::vec(residue_strategy(), 0..=32)) {
        let encoded = encode_to_vec(&residues).expect("encode Vec<Residue>");
        let (decoded, _) = decode_from_slice::<Vec<Residue>>(&encoded).expect("decode Vec<Residue>");
        prop_assert_eq!(residues, decoded);
    }

    #[test]
    fn test_option_residue_roundtrip(opt in prop::option::of(residue_strategy())) {
        let encoded = encode_to_vec(&opt).expect("encode Option<Residue>");
        let (decoded, _) = decode_from_slice::<Option<Residue>>(&encoded).expect("decode Option<Residue>");
        prop_assert_eq!(opt, decoded);
    }

    #[test]
    fn test_u8_roundtrip(val in any::<u8>()) {
        let encoded = encode_to_vec(&val).expect("encode u8");
        let (decoded, _) = decode_from_slice::<u8>(&encoded).expect("decode u8");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_i32_roundtrip(val in any::<i32>()) {
        let encoded = encode_to_vec(&val).expect("encode i32");
        let (decoded, _) = decode_from_slice::<i32>(&encoded).expect("decode i32");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_u64_roundtrip(val in any::<u64>()) {
        let encoded = encode_to_vec(&val).expect("encode u64");
        let (decoded, _) = decode_from_slice::<u64>(&encoded).expect("decode u64");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_bool_roundtrip(val in any::<bool>()) {
        let encoded = encode_to_vec(&val).expect("encode bool");
        let (decoded, _) = decode_from_slice::<bool>(&encoded).expect("decode bool");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_string_roundtrip(val in ".*") {
        let encoded = encode_to_vec(&val).expect("encode String");
        let (decoded, _) = decode_from_slice::<String>(&encoded).expect("decode String");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_f32_roundtrip(val in any::<f32>()) {
        let encoded = encode_to_vec(&val).expect("encode f32");
        let (decoded, _) = decode_from_slice::<f32>(&encoded).expect("decode f32");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_f64_roundtrip(val in any::<f64>()) {
        let encoded = encode_to_vec(&val).expect("encode f64");
        let (decoded, _) = decode_from_slice::<f64>(&encoded).expect("decode f64");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_vec_u8_roundtrip(val in prop::collection::vec(any::<u8>(), 0..=256)) {
        let encoded = encode_to_vec(&val).expect("encode Vec<u8>");
        let (decoded, _) = decode_from_slice::<Vec<u8>>(&encoded).expect("decode Vec<u8>");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_vec_string_roundtrip(val in prop::collection::vec(".*", 0..=16)) {
        let encoded = encode_to_vec(&val).expect("encode Vec<String>");
        let (decoded, _) = decode_from_slice::<Vec<String>>(&encoded).expect("decode Vec<String>");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_option_u64_roundtrip(val in prop::option::of(any::<u64>())) {
        let encoded = encode_to_vec(&val).expect("encode Option<u64>");
        let (decoded, _) = decode_from_slice::<Option<u64>>(&encoded).expect("decode Option<u64>");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_amino_acid_variant_roundtrip(idx in 0u8..20) {
        let aa = match idx {
            0 => AminoAcid::Ala,
            1 => AminoAcid::Arg,
            2 => AminoAcid::Asn,
            3 => AminoAcid::Asp,
            4 => AminoAcid::Cys,
            5 => AminoAcid::Gln,
            6 => AminoAcid::Glu,
            7 => AminoAcid::Gly,
            8 => AminoAcid::His,
            9 => AminoAcid::Ile,
            10 => AminoAcid::Leu,
            11 => AminoAcid::Lys,
            12 => AminoAcid::Met,
            13 => AminoAcid::Phe,
            14 => AminoAcid::Pro,
            15 => AminoAcid::Ser,
            16 => AminoAcid::Thr,
            17 => AminoAcid::Trp,
            18 => AminoAcid::Tyr,
            _ => AminoAcid::Val,
        };
        let encoded = encode_to_vec(&aa).expect("encode AminoAcid variant");
        let (decoded, _) = decode_from_slice::<AminoAcid>(&encoded).expect("decode AminoAcid variant");
        prop_assert_eq!(aa, decoded);
    }

    #[test]
    fn test_secondary_structure_variant_roundtrip(idx in 0u8..4) {
        let ss = match idx {
            0 => SecondaryStructure::AlphaHelix,
            1 => SecondaryStructure::BetaSheet,
            2 => SecondaryStructure::RandomCoil,
            _ => SecondaryStructure::Turn,
        };
        let encoded = encode_to_vec(&ss).expect("encode SecondaryStructure variant");
        let (decoded, _) = decode_from_slice::<SecondaryStructure>(&encoded).expect("decode SecondaryStructure variant");
        prop_assert_eq!(ss, decoded);
    }

    #[test]
    fn test_distinct_residues_have_distinct_or_equal_encoded_bytes(
        r1 in residue_strategy(),
        r2 in residue_strategy(),
    ) {
        let enc1 = encode_to_vec(&r1).expect("encode Residue r1");
        let enc2 = encode_to_vec(&r2).expect("encode Residue r2");
        if r1 == r2 {
            prop_assert_eq!(&enc1, &enc2);
        } else {
            prop_assert_ne!(&enc1, &enc2);
        }
    }

    #[test]
    fn test_residue_encode_length_is_stable(residue in residue_strategy()) {
        let enc1 = encode_to_vec(&residue).expect("encode Residue for length stability first");
        let enc2 = encode_to_vec(&residue).expect("encode Residue for length stability second");
        prop_assert_eq!(enc1.len(), enc2.len());
    }
}
