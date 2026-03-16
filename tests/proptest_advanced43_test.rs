//! Advanced property-based tests (set 43) using proptest.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Domain: bioinformatics / genomics.
//!
//! Covers: Nucleotide, Chromosome, GenomicPosition, Variant, SampleGenotype
//! roundtrips, consumed-bytes checks, deterministic encoding, option types,
//! vec collections, and arbitrary quality/depth values.

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

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Nucleotide {
    A,
    T,
    C,
    G,
    N,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Chromosome {
    Chr1,
    Chr2,
    Chr3,
    Chr4,
    Chr5,
    ChrX,
    ChrY,
    ChrMT,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GenomicPosition {
    chromosome: Chromosome,
    position: u64,
    strand_plus: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Variant {
    pos: GenomicPosition,
    ref_allele: String,
    alt_allele: String,
    quality: f32,
    depth: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SampleGenotype {
    sample_id: String,
    variants: Vec<Variant>,
    ploidy: u8,
}

// ── Strategies ───────────────────────────────────────────────────────────────

fn arb_nucleotide() -> impl Strategy<Value = Nucleotide> {
    prop_oneof![
        Just(Nucleotide::A),
        Just(Nucleotide::T),
        Just(Nucleotide::C),
        Just(Nucleotide::G),
        Just(Nucleotide::N),
    ]
}

fn arb_chromosome() -> impl Strategy<Value = Chromosome> {
    prop_oneof![
        Just(Chromosome::Chr1),
        Just(Chromosome::Chr2),
        Just(Chromosome::Chr3),
        Just(Chromosome::Chr4),
        Just(Chromosome::Chr5),
        Just(Chromosome::ChrX),
        Just(Chromosome::ChrY),
        Just(Chromosome::ChrMT),
    ]
}

fn arb_genomic_position() -> impl Strategy<Value = GenomicPosition> {
    (arb_chromosome(), any::<u64>(), any::<bool>()).prop_map(
        |(chromosome, position, strand_plus)| GenomicPosition {
            chromosome,
            position,
            strand_plus,
        },
    )
}

fn arb_allele() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("A".to_string()),
        Just("T".to_string()),
        Just("C".to_string()),
        Just("G".to_string()),
        Just("AT".to_string()),
        Just("GC".to_string()),
        Just("ACGT".to_string()),
    ]
}

fn arb_variant() -> impl Strategy<Value = Variant> {
    (
        arb_genomic_position(),
        arb_allele(),
        arb_allele(),
        any::<f32>(),
        any::<u32>(),
    )
        .prop_map(|(pos, ref_allele, alt_allele, quality, depth)| Variant {
            pos,
            ref_allele,
            alt_allele,
            quality,
            depth,
        })
}

fn arb_sample_genotype() -> impl Strategy<Value = SampleGenotype> {
    (
        "[a-zA-Z0-9_]{1,20}",
        prop::collection::vec(arb_variant(), 0..8usize),
        any::<u8>(),
    )
        .prop_map(|(sample_id, variants, ploidy)| SampleGenotype {
            sample_id,
            variants,
            ploidy,
        })
}

// ── 1. Nucleotide::A roundtrip ────────────────────────────────────────────────

#[test]
fn prop_nucleotide_a_roundtrip() {
    proptest!(|(_dummy: u8)| {
        let nuc = Nucleotide::A;
        let enc = encode_to_vec(&nuc).expect("encode Nucleotide::A failed");
        let (decoded, bytes_read): (Nucleotide, usize) =
            decode_from_slice(&enc).expect("decode Nucleotide::A failed");
        prop_assert_eq!(nuc, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 2. All Nucleotide variants roundtrip ─────────────────────────────────────

#[test]
fn prop_all_nucleotide_variants_roundtrip() {
    proptest!(|(nuc in arb_nucleotide())| {
        let enc = encode_to_vec(&nuc).expect("encode Nucleotide failed");
        let (decoded, bytes_read): (Nucleotide, usize) =
            decode_from_slice(&enc).expect("decode Nucleotide failed");
        prop_assert_eq!(nuc, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 3. All Chromosome variants roundtrip ─────────────────────────────────────

#[test]
fn prop_all_chromosome_variants_roundtrip() {
    proptest!(|(chr in arb_chromosome())| {
        let enc = encode_to_vec(&chr).expect("encode Chromosome failed");
        let (decoded, bytes_read): (Chromosome, usize) =
            decode_from_slice(&enc).expect("decode Chromosome failed");
        prop_assert_eq!(chr, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 4. GenomicPosition roundtrip ─────────────────────────────────────────────

#[test]
fn prop_genomic_position_roundtrip() {
    proptest!(|(gpos in arb_genomic_position())| {
        let enc = encode_to_vec(&gpos).expect("encode GenomicPosition failed");
        let (decoded, bytes_read): (GenomicPosition, usize) =
            decode_from_slice(&enc).expect("decode GenomicPosition failed");
        prop_assert_eq!(gpos, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 5. GenomicPosition consumed == bytes.len() ───────────────────────────────

#[test]
fn prop_genomic_position_consumed_equals_len() {
    proptest!(|(chromosome in arb_chromosome(), position: u64, strand_plus: bool)| {
        let gpos = GenomicPosition { chromosome, position, strand_plus };
        let enc = encode_to_vec(&gpos).expect("encode GenomicPosition for consumed check");
        let (_decoded, consumed): (GenomicPosition, usize) =
            decode_from_slice(&enc).expect("decode GenomicPosition for consumed check");
        prop_assert_eq!(
            consumed,
            enc.len(),
            "consumed bytes ({}) must equal encoded length ({})",
            consumed,
            enc.len()
        );
    });
}

// ── 6. GenomicPosition deterministic encoding ─────────────────────────────────

#[test]
fn prop_genomic_position_deterministic_encoding() {
    proptest!(|(gpos in arb_genomic_position())| {
        let enc1 = encode_to_vec(&gpos).expect("first encode GenomicPosition");
        let enc2 = encode_to_vec(&gpos).expect("second encode GenomicPosition");
        prop_assert_eq!(
            enc1,
            enc2,
            "encoding GenomicPosition twice must yield identical bytes"
        );
    });
}

// ── 7. Variant roundtrip ──────────────────────────────────────────────────────

#[test]
fn prop_variant_roundtrip() {
    proptest!(|(v in arb_variant())| {
        let enc = encode_to_vec(&v).expect("encode Variant failed");
        let (decoded, bytes_read): (Variant, usize) =
            decode_from_slice(&enc).expect("decode Variant failed");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 8. Variant consumed == bytes.len() ───────────────────────────────────────

#[test]
fn prop_variant_consumed_equals_len() {
    proptest!(|(v in arb_variant())| {
        let enc = encode_to_vec(&v).expect("encode Variant for consumed check");
        let (_decoded, consumed): (Variant, usize) =
            decode_from_slice(&enc).expect("decode Variant for consumed check");
        prop_assert_eq!(
            consumed,
            enc.len(),
            "Variant consumed ({}) must equal encoded length ({})",
            consumed,
            enc.len()
        );
    });
}

// ── 9. Variant deterministic encoding ────────────────────────────────────────

#[test]
fn prop_variant_deterministic_encoding() {
    proptest!(|(v in arb_variant())| {
        let enc1 = encode_to_vec(&v).expect("first encode Variant");
        let enc2 = encode_to_vec(&v).expect("second encode Variant");
        prop_assert_eq!(enc1, enc2, "Variant encoding must be deterministic");
    });
}

// ── 10. Vec<Variant> roundtrip ────────────────────────────────────────────────

#[test]
fn prop_vec_variant_roundtrip() {
    proptest!(|(variants in prop::collection::vec(arb_variant(), 0..6usize))| {
        let enc = encode_to_vec(&variants).expect("encode Vec<Variant> failed");
        let (decoded, bytes_read): (Vec<Variant>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Variant> failed");
        prop_assert_eq!(variants, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 11. SampleGenotype roundtrip ─────────────────────────────────────────────

#[test]
fn prop_sample_genotype_roundtrip() {
    proptest!(|(sg in arb_sample_genotype())| {
        let enc = encode_to_vec(&sg).expect("encode SampleGenotype failed");
        let (decoded, bytes_read): (SampleGenotype, usize) =
            decode_from_slice(&enc).expect("decode SampleGenotype failed");
        prop_assert_eq!(sg, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 12. SampleGenotype consumed == bytes.len() ───────────────────────────────

#[test]
fn prop_sample_genotype_consumed_equals_len() {
    proptest!(|(sg in arb_sample_genotype())| {
        let enc = encode_to_vec(&sg).expect("encode SampleGenotype for consumed check");
        let (_decoded, consumed): (SampleGenotype, usize) =
            decode_from_slice(&enc).expect("decode SampleGenotype for consumed check");
        prop_assert_eq!(
            consumed,
            enc.len(),
            "SampleGenotype consumed ({}) must equal encoded length ({})",
            consumed,
            enc.len()
        );
    });
}

// ── 13. SampleGenotype deterministic encoding ─────────────────────────────────

#[test]
fn prop_sample_genotype_deterministic_encoding() {
    proptest!(|(sg in arb_sample_genotype())| {
        let enc1 = encode_to_vec(&sg).expect("first encode SampleGenotype");
        let enc2 = encode_to_vec(&sg).expect("second encode SampleGenotype");
        prop_assert_eq!(enc1, enc2, "SampleGenotype encoding must be deterministic");
    });
}

// ── 14. Option<Nucleotide> roundtrip ─────────────────────────────────────────

#[test]
fn prop_option_nucleotide_roundtrip() {
    proptest!(|(opt in prop::option::of(arb_nucleotide()))| {
        let enc = encode_to_vec(&opt).expect("encode Option<Nucleotide> failed");
        let (decoded, bytes_read): (Option<Nucleotide>, usize) =
            decode_from_slice(&enc).expect("decode Option<Nucleotide> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 15. Option<GenomicPosition> roundtrip ────────────────────────────────────

#[test]
fn prop_option_genomic_position_roundtrip() {
    proptest!(|(opt in prop::option::of(arb_genomic_position()))| {
        let enc = encode_to_vec(&opt).expect("encode Option<GenomicPosition> failed");
        let (decoded, bytes_read): (Option<GenomicPosition>, usize) =
            decode_from_slice(&enc).expect("decode Option<GenomicPosition> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 16. Option<Variant> roundtrip ────────────────────────────────────────────

#[test]
fn prop_option_variant_roundtrip() {
    proptest!(|(opt in prop::option::of(arb_variant()))| {
        let enc = encode_to_vec(&opt).expect("encode Option<Variant> failed");
        let (decoded, bytes_read): (Option<Variant>, usize) =
            decode_from_slice(&enc).expect("decode Option<Variant> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 17. Variant arbitrary quality and depth values ───────────────────────────

#[test]
fn prop_variant_arbitrary_quality_depth_roundtrip() {
    proptest!(|(
        gpos in arb_genomic_position(),
        ref_allele in arb_allele(),
        alt_allele in arb_allele(),
        quality: f32,
        depth: u32,
    )| {
        let v = Variant { pos: gpos, ref_allele, alt_allele, quality, depth };
        let enc = encode_to_vec(&v).expect("encode Variant with arbitrary quality/depth");
        let (decoded, bytes_read): (Variant, usize) =
            decode_from_slice(&enc).expect("decode Variant with arbitrary quality/depth");
        prop_assert_eq!(v, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 18. Vec<Nucleotide> (DNA sequence) roundtrip ─────────────────────────────

#[test]
fn prop_dna_sequence_roundtrip() {
    proptest!(|(seq in prop::collection::vec(arb_nucleotide(), 0..50usize))| {
        let enc = encode_to_vec(&seq).expect("encode DNA sequence (Vec<Nucleotide>) failed");
        let (decoded, bytes_read): (Vec<Nucleotide>, usize) =
            decode_from_slice(&enc).expect("decode DNA sequence (Vec<Nucleotide>) failed");
        prop_assert_eq!(seq, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 19. Vec<SampleGenotype> roundtrip (cohort) ───────────────────────────────

#[test]
fn prop_cohort_roundtrip() {
    proptest!(|(cohort in prop::collection::vec(arb_sample_genotype(), 0..4usize))| {
        let enc = encode_to_vec(&cohort).expect("encode Vec<SampleGenotype> failed");
        let (decoded, bytes_read): (Vec<SampleGenotype>, usize) =
            decode_from_slice(&enc).expect("decode Vec<SampleGenotype> failed");
        prop_assert_eq!(cohort, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 20. Re-encode decoded Variant gives identical bytes ──────────────────────

#[test]
fn prop_variant_reencode_idempotent() {
    proptest!(|(v in arb_variant())| {
        let enc1 = encode_to_vec(&v).expect("first encode Variant for idempotency");
        let (decoded, _): (Variant, usize) =
            decode_from_slice(&enc1).expect("first decode Variant for idempotency");
        let enc2 = encode_to_vec(&decoded).expect("re-encode decoded Variant");
        prop_assert_eq!(enc1, enc2, "re-encoding decoded Variant must yield identical bytes");
    });
}

// ── 21. ChrMT (mitochondrial) GenomicPosition roundtrip ──────────────────────

#[test]
fn prop_chrmt_genomic_position_roundtrip() {
    proptest!(|(position: u64, strand_plus: bool)| {
        let gpos = GenomicPosition {
            chromosome: Chromosome::ChrMT,
            position,
            strand_plus,
        };
        let enc = encode_to_vec(&gpos).expect("encode ChrMT GenomicPosition failed");
        let (decoded, bytes_read): (GenomicPosition, usize) =
            decode_from_slice(&enc).expect("decode ChrMT GenomicPosition failed");
        prop_assert_eq!(gpos, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}

// ── 22. Option<SampleGenotype> roundtrip ─────────────────────────────────────

#[test]
fn prop_option_sample_genotype_roundtrip() {
    proptest!(|(opt in prop::option::of(arb_sample_genotype()))| {
        let enc = encode_to_vec(&opt).expect("encode Option<SampleGenotype> failed");
        let (decoded, bytes_read): (Option<SampleGenotype>, usize) =
            decode_from_slice(&enc).expect("decode Option<SampleGenotype> failed");
        prop_assert_eq!(opt, decoded);
        prop_assert_eq!(bytes_read, enc.len());
    });
}
