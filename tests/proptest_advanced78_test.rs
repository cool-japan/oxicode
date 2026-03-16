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
// Domain types: Bioinformatics & Genomics
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Base {
    A,
    C,
    G,
    T,
    N,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QualityScore {
    phred: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SequenceRead {
    read_id: String,
    bases: Vec<Base>,
    quality_scores: Vec<QualityScore>,
    mapping_quality: u8,
    alignment_start: u64,
    alignment_end: u64,
    is_reverse_strand: bool,
    mate_alignment_start: Option<u64>,
    insert_size: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FeatureType {
    Exon,
    Intron,
    Utr5,
    Utr3,
    Cds,
    Promoter,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeneAnnotation {
    gene_id: String,
    gene_name: String,
    chromosome: String,
    start: u64,
    end: u64,
    strand: bool,
    features: Vec<GenomicFeature>,
    transcript_count: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GenomicFeature {
    feature_type: FeatureType,
    start: u64,
    end: u64,
    phase: Option<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SecondaryStructure {
    Helix,
    Sheet,
    Coil,
    Turn,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ResidueContact {
    residue_i: u32,
    residue_j: u32,
    distance_angstrom: u32,
    confidence: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProteinStructurePrediction {
    protein_id: String,
    length: u32,
    ss_assignments: Vec<SecondaryStructure>,
    contact_map: Vec<ResidueContact>,
    confidence_score: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VariantType {
    Snp,
    Insertion,
    Deletion,
    Mnp,
    Complex,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VariantCall {
    chrom: String,
    position: u64,
    ref_allele: Vec<Base>,
    alt_alleles: Vec<Vec<Base>>,
    variant_type: VariantType,
    quality: u32,
    allele_frequency: u32,
    depth: u32,
    genotype: (u8, u8),
    filter_pass: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PhyloNode {
    node_id: u32,
    taxon_name: Option<String>,
    branch_length_thousandths: u32,
    children: Vec<u32>,
    bootstrap_support: Option<u16>,
    is_leaf: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PhylogeneticTree {
    root_id: u32,
    nodes: Vec<PhyloNode>,
    num_leaves: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MetabolicReaction {
    reaction_id: String,
    enzyme_ec: String,
    substrates: Vec<String>,
    products: Vec<String>,
    reversible: bool,
    flux_thousandths: i64,
    pathway_id: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeneExpressionProfile {
    gene_id: String,
    sample_id: String,
    raw_count: u64,
    fpkm_thousandths: u64,
    tpm_thousandths: u64,
    log2_fold_change_thousandths: i64,
    p_value_millionths: u64,
    is_significant: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrisprGuideRna {
    target_gene: String,
    guide_sequence: Vec<Base>,
    pam_sequence: Vec<Base>,
    on_target_score_hundredths: u16,
    off_target_count: u32,
    gc_content_hundredths: u8,
    chromosome: String,
    cut_position: u64,
    strand: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProteinInteraction {
    protein_a: String,
    protein_b: String,
    interaction_score_thousandths: u32,
    detection_method: String,
    is_physical: bool,
    pubmed_ids: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProteinInteractionNetwork {
    network_id: String,
    interactions: Vec<ProteinInteraction>,
    node_count: u32,
    edge_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MethylationContext {
    CpG,
    CHG,
    CHH,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MethylationSite {
    chrom: String,
    position: u64,
    context: MethylationContext,
    methylation_pct_hundredths: u16,
    coverage: u32,
    strand: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EpigeneticPattern {
    sample_id: String,
    sites: Vec<MethylationSite>,
    global_methylation_pct_hundredths: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SingleCellCluster {
    cluster_id: u32,
    cell_barcodes: Vec<String>,
    marker_genes: Vec<String>,
    cell_count: u32,
    umap_centroid_x_thousandths: i64,
    umap_centroid_y_thousandths: i64,
    cell_type_annotation: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScRnaSeqExperiment {
    experiment_id: String,
    clusters: Vec<SingleCellCluster>,
    total_cells: u32,
    genes_detected: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SvType {
    Deletion,
    Duplication,
    Inversion,
    Translocation,
    InsertionLarge,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StructuralVariant {
    sv_id: String,
    sv_type: SvType,
    chrom: String,
    start: u64,
    end: u64,
    length: i64,
    supporting_reads: u32,
    quality: u32,
    genotype: (u8, u8),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PopulationGeneticsStats {
    locus_id: String,
    fst_thousandths: u32,
    observed_het_thousandths: u32,
    expected_het_thousandths: u32,
    hw_p_value_millionths: u64,
    allele_count: u16,
    sample_size: u32,
    populations: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PathogenicityClass {
    Benign,
    LikelyBenign,
    Uncertain,
    LikelyPathogenic,
    Pathogenic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClinicalVariant {
    variant_id: String,
    gene: String,
    hgvs_coding: String,
    hgvs_protein: String,
    classification: PathogenicityClass,
    review_status: u8,
    condition: String,
    evidence_codes: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OtuEntry {
    taxon_id: String,
    taxonomy_lineage: Vec<String>,
    counts: Vec<u64>,
    relative_abundance_millionths: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MetagenomicsOtuTable {
    project_id: String,
    sample_ids: Vec<String>,
    otus: Vec<OtuEntry>,
    total_reads_per_sample: Vec<u64>,
    diversity_index_thousandths: u32,
}

// ---------------------------------------------------------------------------
// prop_compose! strategies
// ---------------------------------------------------------------------------

fn arb_base() -> impl Strategy<Value = Base> {
    prop_oneof![
        Just(Base::A),
        Just(Base::C),
        Just(Base::G),
        Just(Base::T),
        Just(Base::N),
    ]
}

fn arb_base_vec(max_len: usize) -> impl Strategy<Value = Vec<Base>> {
    prop::collection::vec(arb_base(), 1..=max_len)
}

prop_compose! {
    fn arb_quality_score()(phred in 0u8..=60) -> QualityScore {
        QualityScore { phred }
    }
}

prop_compose! {
    fn arb_sequence_read()(
        read_id in "[A-Z0-9]{4,12}",
        base_count in 10usize..=50,
        mapping_quality in 0u8..=60,
        alignment_start in 0u64..9_999_000,
        span in 50u64..500,
        is_reverse_strand in any::<bool>(),
        has_mate in any::<bool>(),
        mate_pos in 0u64..10_000_000,
        insert_size in -1000i32..1000,
    ) -> SequenceRead {
        let bases: Vec<Base> = (0..base_count)
            .map(|i| match i % 5 { 0 => Base::A, 1 => Base::C, 2 => Base::G, 3 => Base::T, _ => Base::N })
            .collect();
        let quality_scores: Vec<QualityScore> = (0..base_count)
            .map(|i| QualityScore { phred: (((i * 7 + 20) % 41) + 2) as u8 })
            .collect();
        SequenceRead {
            read_id,
            bases,
            quality_scores,
            mapping_quality,
            alignment_start,
            alignment_end: alignment_start + span,
            is_reverse_strand,
            mate_alignment_start: if has_mate { Some(mate_pos) } else { None },
            insert_size,
        }
    }
}

prop_compose! {
    fn arb_feature_type()(idx in 0u8..6) -> FeatureType {
        match idx {
            0 => FeatureType::Exon,
            1 => FeatureType::Intron,
            2 => FeatureType::Utr5,
            3 => FeatureType::Utr3,
            4 => FeatureType::Cds,
            _ => FeatureType::Promoter,
        }
    }
}

prop_compose! {
    fn arb_genomic_feature()(
        ft in arb_feature_type(),
        start in 1000u64..500_000,
        len in 50u64..5_000,
        has_phase in any::<bool>(),
        phase_val in 0u8..3,
    ) -> GenomicFeature {
        GenomicFeature {
            feature_type: ft,
            start,
            end: start + len,
            phase: if has_phase { Some(phase_val) } else { None },
        }
    }
}

prop_compose! {
    fn arb_gene_annotation()(
        gene_id in "ENSG[0-9]{6}",
        gene_name in "[A-Z]{2,6}[0-9]{1,3}",
        chrom in "chr(1[0-9]?|2[0-2]?|[3-9]|X|Y)",
        start in 10_000u64..100_000_000,
        span in 1_000u64..500_000,
        strand in any::<bool>(),
        features in prop::collection::vec(arb_genomic_feature(), 1..=6),
        transcript_count in 1u16..20,
    ) -> GeneAnnotation {
        GeneAnnotation {
            gene_id,
            gene_name,
            chromosome: chrom,
            start,
            end: start + span,
            strand,
            features,
            transcript_count,
        }
    }
}

prop_compose! {
    fn arb_residue_contact()(
        residue_i in 0u32..500,
        residue_j in 0u32..500,
        distance in 3u32..20,
        confidence in 0u16..10000,
    ) -> ResidueContact {
        ResidueContact {
            residue_i,
            residue_j,
            distance_angstrom: distance,
            confidence,
        }
    }
}

prop_compose! {
    fn arb_protein_structure()(
        protein_id in "P[0-9]{5}",
        len in 20u32..200,
        contacts in prop::collection::vec(arb_residue_contact(), 1..=8),
        confidence_score in 0u16..10000,
    ) -> ProteinStructurePrediction {
        let ss_assignments: Vec<SecondaryStructure> = (0..len)
            .map(|i| match i % 4 {
                0 => SecondaryStructure::Helix,
                1 => SecondaryStructure::Sheet,
                2 => SecondaryStructure::Coil,
                _ => SecondaryStructure::Turn,
            })
            .collect();
        ProteinStructurePrediction {
            protein_id,
            length: len,
            ss_assignments,
            contact_map: contacts,
            confidence_score,
        }
    }
}

fn arb_variant_type() -> impl Strategy<Value = VariantType> {
    prop_oneof![
        Just(VariantType::Snp),
        Just(VariantType::Insertion),
        Just(VariantType::Deletion),
        Just(VariantType::Mnp),
        Just(VariantType::Complex),
    ]
}

prop_compose! {
    fn arb_variant_call()(
        chrom in "chr(1[0-9]?|2[0-2]?|[3-9]|X|Y)",
        position in 1u64..300_000_000,
        ref_allele in arb_base_vec(5),
        alt_count in 1usize..=3,
        vt in arb_variant_type(),
        quality in 0u32..10000,
        af in 0u32..1000,
        depth in 1u32..5000,
        gt0 in 0u8..2,
        gt1 in 0u8..2,
        filter_pass in any::<bool>(),
    ) -> VariantCall {
        let alt_alleles: Vec<Vec<Base>> = (0..alt_count)
            .map(|i| {
                let len = (i % 4) + 1;
                (0..len).map(|j| match (i + j) % 4 {
                    0 => Base::A, 1 => Base::C, 2 => Base::G, _ => Base::T,
                }).collect()
            })
            .collect();
        VariantCall {
            chrom,
            position,
            ref_allele,
            alt_alleles,
            variant_type: vt,
            quality,
            allele_frequency: af,
            depth,
            genotype: (gt0, gt1),
            filter_pass,
        }
    }
}

prop_compose! {
    fn arb_phylo_node()(
        node_id in 0u32..1000,
        has_taxon in any::<bool>(),
        taxon in "[A-Z][a-z]{3,10}_[a-z]{3,8}",
        branch_len in 0u32..10_000,
        children in prop::collection::vec(0u32..1000, 0..=3),
        has_bootstrap in any::<bool>(),
        bootstrap in 0u16..1000,
        is_leaf in any::<bool>(),
    ) -> PhyloNode {
        PhyloNode {
            node_id,
            taxon_name: if has_taxon { Some(taxon) } else { None },
            branch_length_thousandths: branch_len,
            children,
            bootstrap_support: if has_bootstrap { Some(bootstrap) } else { None },
            is_leaf,
        }
    }
}

prop_compose! {
    fn arb_phylogenetic_tree()(
        root_id in 0u32..100,
        nodes in prop::collection::vec(arb_phylo_node(), 2..=8),
        num_leaves in 1u32..50,
    ) -> PhylogeneticTree {
        PhylogeneticTree { root_id, nodes, num_leaves }
    }
}

prop_compose! {
    fn arb_metabolic_reaction()(
        reaction_id in "R[0-9]{5}",
        enzyme_ec in "[1-6]\\.[0-9]{1,2}\\.[0-9]{1,2}\\.[0-9]{1,3}",
        substrates in prop::collection::vec("[A-Z][a-z]{2,8}", 1..=4),
        products in prop::collection::vec("[A-Z][a-z]{2,8}", 1..=4),
        reversible in any::<bool>(),
        flux in -10_000i64..10_000,
        pathway_id in "map[0-9]{5}",
    ) -> MetabolicReaction {
        MetabolicReaction {
            reaction_id,
            enzyme_ec,
            substrates,
            products,
            reversible,
            flux_thousandths: flux,
            pathway_id,
        }
    }
}

prop_compose! {
    fn arb_gene_expression()(
        gene_id in "ENSG[0-9]{6}",
        sample_id in "SAMPLE_[0-9]{3}",
        raw_count in 0u64..1_000_000,
        fpkm in 0u64..500_000,
        tpm in 0u64..500_000,
        lfc in -10_000i64..10_000,
        pval in 0u64..1_000_000,
        is_significant in any::<bool>(),
    ) -> GeneExpressionProfile {
        GeneExpressionProfile {
            gene_id,
            sample_id,
            raw_count,
            fpkm_thousandths: fpkm,
            tpm_thousandths: tpm,
            log2_fold_change_thousandths: lfc,
            p_value_millionths: pval,
            is_significant,
        }
    }
}

prop_compose! {
    fn arb_crispr_guide()(
        target_gene in "[A-Z]{2,6}[0-9]{1,2}",
        guide_len in 18usize..=22,
        pam_len in 2usize..=4,
        on_target in 0u16..10000,
        off_target_count in 0u32..100,
        gc in 20u8..80,
        chrom in "chr[0-9]{1,2}",
        cut_pos in 1u64..300_000_000,
        strand in any::<bool>(),
    ) -> CrisprGuideRna {
        let guide_sequence: Vec<Base> = (0..guide_len)
            .map(|i| match i % 4 { 0 => Base::G, 1 => Base::A, 2 => Base::T, _ => Base::C })
            .collect();
        let pam_sequence: Vec<Base> = (0..pam_len)
            .map(|i| if i == 0 { Base::N } else { Base::G })
            .collect();
        CrisprGuideRna {
            target_gene,
            guide_sequence,
            pam_sequence,
            on_target_score_hundredths: on_target,
            off_target_count,
            gc_content_hundredths: gc,
            chromosome: chrom,
            cut_position: cut_pos,
            strand,
        }
    }
}

prop_compose! {
    fn arb_protein_interaction()(
        protein_a in "P[0-9]{5}",
        protein_b in "P[0-9]{5}",
        score in 0u32..1000,
        method in "(Y2H|CoIP|PCA|FRET)",
        is_physical in any::<bool>(),
        pubmed_ids in prop::collection::vec(10_000_000u32..40_000_000, 0..=3),
    ) -> ProteinInteraction {
        ProteinInteraction {
            protein_a,
            protein_b,
            interaction_score_thousandths: score,
            detection_method: method,
            is_physical,
            pubmed_ids,
        }
    }
}

prop_compose! {
    fn arb_ppi_network()(
        network_id in "NET_[0-9]{4}",
        interactions in prop::collection::vec(arb_protein_interaction(), 1..=6),
        node_count in 2u32..100,
        edge_count in 1u32..500,
    ) -> ProteinInteractionNetwork {
        ProteinInteractionNetwork {
            network_id,
            interactions,
            node_count,
            edge_count,
        }
    }
}

fn arb_methylation_context() -> impl Strategy<Value = MethylationContext> {
    prop_oneof![
        Just(MethylationContext::CpG),
        Just(MethylationContext::CHG),
        Just(MethylationContext::CHH),
    ]
}

prop_compose! {
    fn arb_methylation_site()(
        chrom in "chr[0-9]{1,2}",
        position in 1u64..300_000_000,
        context in arb_methylation_context(),
        meth_pct in 0u16..10000,
        coverage in 1u32..500,
        strand in any::<bool>(),
    ) -> MethylationSite {
        MethylationSite {
            chrom,
            position,
            context,
            methylation_pct_hundredths: meth_pct,
            coverage,
            strand,
        }
    }
}

prop_compose! {
    fn arb_epigenetic_pattern()(
        sample_id in "METH_[0-9]{4}",
        sites in prop::collection::vec(arb_methylation_site(), 1..=6),
        global_meth in 0u16..10000,
    ) -> EpigeneticPattern {
        EpigeneticPattern {
            sample_id,
            sites,
            global_methylation_pct_hundredths: global_meth,
        }
    }
}

prop_compose! {
    fn arb_sc_cluster()(
        cluster_id in 0u32..50,
        barcodes in prop::collection::vec("[ACGT]{16}", 1..=4),
        markers in prop::collection::vec("[A-Z]{2,5}[0-9]{1,2}", 1..=5),
        cell_count in 10u32..5000,
        ux in -10_000i64..10_000,
        uy in -10_000i64..10_000,
        has_annotation in any::<bool>(),
        annotation in "(T_cell|B_cell|Monocyte|NK_cell|Fibroblast)",
    ) -> SingleCellCluster {
        SingleCellCluster {
            cluster_id,
            cell_barcodes: barcodes,
            marker_genes: markers,
            cell_count,
            umap_centroid_x_thousandths: ux,
            umap_centroid_y_thousandths: uy,
            cell_type_annotation: if has_annotation { Some(annotation) } else { None },
        }
    }
}

prop_compose! {
    fn arb_scrna_experiment()(
        experiment_id in "SC_EXP_[0-9]{4}",
        clusters in prop::collection::vec(arb_sc_cluster(), 1..=5),
        total_cells in 100u32..50_000,
        genes_detected in 500u32..30_000,
    ) -> ScRnaSeqExperiment {
        ScRnaSeqExperiment {
            experiment_id,
            clusters,
            total_cells,
            genes_detected,
        }
    }
}

fn arb_sv_type() -> impl Strategy<Value = SvType> {
    prop_oneof![
        Just(SvType::Deletion),
        Just(SvType::Duplication),
        Just(SvType::Inversion),
        Just(SvType::Translocation),
        Just(SvType::InsertionLarge),
    ]
}

prop_compose! {
    fn arb_structural_variant()(
        sv_id in "SV_[0-9]{6}",
        sv_type in arb_sv_type(),
        chrom in "chr[0-9]{1,2}",
        start in 1u64..200_000_000,
        span in 50u64..5_000_000,
        supporting in 2u32..200,
        quality in 0u32..10000,
        gt0 in 0u8..2,
        gt1 in 0u8..2,
    ) -> StructuralVariant {
        StructuralVariant {
            sv_id,
            sv_type,
            chrom,
            start,
            end: start + span,
            length: span as i64,
            supporting_reads: supporting,
            quality,
            genotype: (gt0, gt1),
        }
    }
}

prop_compose! {
    fn arb_pop_genetics()(
        locus_id in "rs[0-9]{6,8}",
        fst in 0u32..1000,
        obs_het in 0u32..1000,
        exp_het in 0u32..1000,
        hw_pval in 0u64..1_000_000,
        allele_count in 2u16..20,
        sample_size in 10u32..10_000,
        pops in prop::collection::vec("[A-Z]{3}", 2..=5),
    ) -> PopulationGeneticsStats {
        PopulationGeneticsStats {
            locus_id,
            fst_thousandths: fst,
            observed_het_thousandths: obs_het,
            expected_het_thousandths: exp_het,
            hw_p_value_millionths: hw_pval,
            allele_count,
            sample_size,
            populations: pops,
        }
    }
}

fn arb_pathogenicity() -> impl Strategy<Value = PathogenicityClass> {
    prop_oneof![
        Just(PathogenicityClass::Benign),
        Just(PathogenicityClass::LikelyBenign),
        Just(PathogenicityClass::Uncertain),
        Just(PathogenicityClass::LikelyPathogenic),
        Just(PathogenicityClass::Pathogenic),
    ]
}

prop_compose! {
    fn arb_clinical_variant()(
        variant_id in "ClinVar_[0-9]{6}",
        gene in "[A-Z]{2,6}[0-9]{0,2}",
        hgvs_c in "c\\.[0-9]{1,4}[ACGT]>[ACGT]",
        hgvs_p in "p\\.[A-Z][a-z]{2}[0-9]{1,4}[A-Z][a-z]{2}",
        classification in arb_pathogenicity(),
        review_status in 0u8..4,
        condition in "(Cardiomyopathy|BreastCancer|CF|LynchSyndrome|Hemophilia)",
        evidence in prop::collection::vec("(PS1|PM2|PP3|BA1|BS1|BP4)", 1..=4),
    ) -> ClinicalVariant {
        ClinicalVariant {
            variant_id,
            gene,
            hgvs_coding: hgvs_c,
            hgvs_protein: hgvs_p,
            classification,
            review_status,
            condition,
            evidence_codes: evidence,
        }
    }
}

prop_compose! {
    fn arb_otu_entry()(
        taxon_id in "OTU_[0-9]{5}",
        lineage in prop::collection::vec("[A-Z][a-z]{3,10}", 2..=7),
        counts in prop::collection::vec(0u64..100_000, 1..=5),
        rel_abundance in 0u64..1_000_000,
    ) -> OtuEntry {
        OtuEntry {
            taxon_id,
            taxonomy_lineage: lineage,
            counts,
            relative_abundance_millionths: rel_abundance,
        }
    }
}

prop_compose! {
    fn arb_metagenomics_table()(
        project_id in "PRJNA[0-9]{6}",
        sample_ids in prop::collection::vec("SRS[0-9]{6}", 1..=4),
        otus in prop::collection::vec(arb_otu_entry(), 1..=5),
        total_reads in prop::collection::vec(1_000u64..10_000_000, 1..=4),
        diversity in 0u32..10_000,
    ) -> MetagenomicsOtuTable {
        MetagenomicsOtuTable {
            project_id,
            sample_ids,
            otus,
            total_reads_per_sample: total_reads,
            diversity_index_thousandths: diversity,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests (22 total)
// ---------------------------------------------------------------------------

#[test]
fn test_dna_sequence_read_roundtrip() {
    proptest!(|(read in arb_sequence_read())| {
        let encoded = encode_to_vec(&read).expect("encode SequenceRead failed");
        let (decoded, _) = decode_from_slice::<SequenceRead>(&encoded)
            .expect("decode SequenceRead failed");
        prop_assert_eq!(read, decoded);
    });
}

#[test]
fn test_gene_annotation_roundtrip() {
    proptest!(|(ann in arb_gene_annotation())| {
        let encoded = encode_to_vec(&ann).expect("encode GeneAnnotation failed");
        let (decoded, _) = decode_from_slice::<GeneAnnotation>(&encoded)
            .expect("decode GeneAnnotation failed");
        prop_assert_eq!(ann, decoded);
    });
}

#[test]
fn test_protein_structure_prediction_roundtrip() {
    proptest!(|(pred in arb_protein_structure())| {
        let encoded = encode_to_vec(&pred).expect("encode ProteinStructurePrediction failed");
        let (decoded, _) = decode_from_slice::<ProteinStructurePrediction>(&encoded)
            .expect("decode ProteinStructurePrediction failed");
        prop_assert_eq!(pred, decoded);
    });
}

#[test]
fn test_variant_call_roundtrip() {
    proptest!(|(vc in arb_variant_call())| {
        let encoded = encode_to_vec(&vc).expect("encode VariantCall failed");
        let (decoded, _) = decode_from_slice::<VariantCall>(&encoded)
            .expect("decode VariantCall failed");
        prop_assert_eq!(vc, decoded);
    });
}

#[test]
fn test_phylogenetic_tree_roundtrip() {
    proptest!(|(tree in arb_phylogenetic_tree())| {
        let encoded = encode_to_vec(&tree).expect("encode PhylogeneticTree failed");
        let (decoded, _) = decode_from_slice::<PhylogeneticTree>(&encoded)
            .expect("decode PhylogeneticTree failed");
        prop_assert_eq!(tree, decoded);
    });
}

#[test]
fn test_metabolic_reaction_roundtrip() {
    proptest!(|(rxn in arb_metabolic_reaction())| {
        let encoded = encode_to_vec(&rxn).expect("encode MetabolicReaction failed");
        let (decoded, _) = decode_from_slice::<MetabolicReaction>(&encoded)
            .expect("decode MetabolicReaction failed");
        prop_assert_eq!(rxn, decoded);
    });
}

#[test]
fn test_gene_expression_profile_roundtrip() {
    proptest!(|(expr in arb_gene_expression())| {
        let encoded = encode_to_vec(&expr).expect("encode GeneExpressionProfile failed");
        let (decoded, _) = decode_from_slice::<GeneExpressionProfile>(&encoded)
            .expect("decode GeneExpressionProfile failed");
        prop_assert_eq!(expr, decoded);
    });
}

#[test]
fn test_crispr_guide_rna_roundtrip() {
    proptest!(|(guide in arb_crispr_guide())| {
        let encoded = encode_to_vec(&guide).expect("encode CrisprGuideRna failed");
        let (decoded, _) = decode_from_slice::<CrisprGuideRna>(&encoded)
            .expect("decode CrisprGuideRna failed");
        prop_assert_eq!(guide, decoded);
    });
}

#[test]
fn test_protein_interaction_network_roundtrip() {
    proptest!(|(net in arb_ppi_network())| {
        let encoded = encode_to_vec(&net).expect("encode ProteinInteractionNetwork failed");
        let (decoded, _) = decode_from_slice::<ProteinInteractionNetwork>(&encoded)
            .expect("decode ProteinInteractionNetwork failed");
        prop_assert_eq!(net, decoded);
    });
}

#[test]
fn test_epigenetic_pattern_roundtrip() {
    proptest!(|(pattern in arb_epigenetic_pattern())| {
        let encoded = encode_to_vec(&pattern).expect("encode EpigeneticPattern failed");
        let (decoded, _) = decode_from_slice::<EpigeneticPattern>(&encoded)
            .expect("decode EpigeneticPattern failed");
        prop_assert_eq!(pattern, decoded);
    });
}

#[test]
fn test_single_cell_experiment_roundtrip() {
    proptest!(|(exp in arb_scrna_experiment())| {
        let encoded = encode_to_vec(&exp).expect("encode ScRnaSeqExperiment failed");
        let (decoded, _) = decode_from_slice::<ScRnaSeqExperiment>(&encoded)
            .expect("decode ScRnaSeqExperiment failed");
        prop_assert_eq!(exp, decoded);
    });
}

#[test]
fn test_structural_variant_roundtrip() {
    proptest!(|(sv in arb_structural_variant())| {
        let encoded = encode_to_vec(&sv).expect("encode StructuralVariant failed");
        let (decoded, _) = decode_from_slice::<StructuralVariant>(&encoded)
            .expect("decode StructuralVariant failed");
        prop_assert_eq!(sv, decoded);
    });
}

#[test]
fn test_population_genetics_roundtrip() {
    proptest!(|(pg in arb_pop_genetics())| {
        let encoded = encode_to_vec(&pg).expect("encode PopulationGeneticsStats failed");
        let (decoded, _) = decode_from_slice::<PopulationGeneticsStats>(&encoded)
            .expect("decode PopulationGeneticsStats failed");
        prop_assert_eq!(pg, decoded);
    });
}

#[test]
fn test_clinical_variant_roundtrip() {
    proptest!(|(cv in arb_clinical_variant())| {
        let encoded = encode_to_vec(&cv).expect("encode ClinicalVariant failed");
        let (decoded, _) = decode_from_slice::<ClinicalVariant>(&encoded)
            .expect("decode ClinicalVariant failed");
        prop_assert_eq!(cv, decoded);
    });
}

#[test]
fn test_metagenomics_otu_table_roundtrip() {
    proptest!(|(table in arb_metagenomics_table())| {
        let encoded = encode_to_vec(&table).expect("encode MetagenomicsOtuTable failed");
        let (decoded, _) = decode_from_slice::<MetagenomicsOtuTable>(&encoded)
            .expect("decode MetagenomicsOtuTable failed");
        prop_assert_eq!(table, decoded);
    });
}

#[test]
fn test_base_enum_roundtrip() {
    proptest!(|(base in arb_base())| {
        let encoded = encode_to_vec(&base).expect("encode Base failed");
        let (decoded, _) = decode_from_slice::<Base>(&encoded)
            .expect("decode Base failed");
        prop_assert_eq!(base, decoded);
    });
}

#[test]
fn test_vec_of_variant_calls_roundtrip() {
    proptest!(|(calls in prop::collection::vec(arb_variant_call(), 1..=4))| {
        let encoded = encode_to_vec(&calls).expect("encode Vec<VariantCall> failed");
        let (decoded, _) = decode_from_slice::<Vec<VariantCall>>(&encoded)
            .expect("decode Vec<VariantCall> failed");
        prop_assert_eq!(calls, decoded);
    });
}

#[test]
fn test_vec_of_gene_expressions_roundtrip() {
    proptest!(|(profiles in prop::collection::vec(arb_gene_expression(), 1..=5))| {
        let encoded = encode_to_vec(&profiles).expect("encode Vec<GeneExpressionProfile> failed");
        let (decoded, _) = decode_from_slice::<Vec<GeneExpressionProfile>>(&encoded)
            .expect("decode Vec<GeneExpressionProfile> failed");
        prop_assert_eq!(profiles, decoded);
    });
}

#[test]
fn test_optional_clinical_variant_roundtrip() {
    proptest!(|(has_cv in any::<bool>(), cv in arb_clinical_variant())| {
        let val: Option<ClinicalVariant> = if has_cv { Some(cv) } else { None };
        let encoded = encode_to_vec(&val).expect("encode Option<ClinicalVariant> failed");
        let (decoded, _) = decode_from_slice::<Option<ClinicalVariant>>(&encoded)
            .expect("decode Option<ClinicalVariant> failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_methylation_site_roundtrip() {
    proptest!(|(site in arb_methylation_site())| {
        let encoded = encode_to_vec(&site).expect("encode MethylationSite failed");
        let (decoded, _) = decode_from_slice::<MethylationSite>(&encoded)
            .expect("decode MethylationSite failed");
        prop_assert_eq!(site, decoded);
    });
}

#[test]
fn test_nested_phylo_nodes_roundtrip() {
    proptest!(|(nodes in prop::collection::vec(arb_phylo_node(), 2..=6))| {
        let encoded = encode_to_vec(&nodes).expect("encode Vec<PhyloNode> failed");
        let (decoded, _) = decode_from_slice::<Vec<PhyloNode>>(&encoded)
            .expect("decode Vec<PhyloNode> failed");
        prop_assert_eq!(nodes, decoded);
    });
}

#[test]
fn test_protein_interaction_roundtrip() {
    proptest!(|(pi in arb_protein_interaction())| {
        let encoded = encode_to_vec(&pi).expect("encode ProteinInteraction failed");
        let (decoded, _) = decode_from_slice::<ProteinInteraction>(&encoded)
            .expect("decode ProteinInteraction failed");
        prop_assert_eq!(pi, decoded);
    });
}
