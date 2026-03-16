//! Advanced complex enum tests for OxiCode — crop genetics and plant breeding domain.
//! 22 test functions covering deeply nested enums, enums with named fields, and enums containing enums.

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InheritancePattern {
    Dominant,
    Recessive,
    Codominant,
    IncompleteDominant,
    Polygenic {
        loci_count: u16,
        heritability: f64,
    },
    Epistatic {
        modifier_gene: String,
        target_gene: String,
    },
    Maternal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MolecularMarker {
    Ssr {
        locus: String,
        repeat_motif: String,
        allele_sizes: Vec<u16>,
    },
    Snp {
        chromosome: u8,
        position: u64,
        ref_allele: u8,
        alt_allele: u8,
    },
    Aflp {
        primer_combination: String,
        fragment_size: u16,
        presence: bool,
    },
    Kasp {
        assay_id: String,
        fam_allele: u8,
        hex_allele: u8,
        call: String,
    },
    InDel {
        chromosome: u8,
        position: u64,
        inserted_bases: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Generation {
    Parental,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    Bc {
        recurrent_parent: String,
        backcross_number: u8,
    },
    Dh,
    Synthetic {
        cycle: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BreedingStage {
    Crossing {
        female: String,
        male: String,
        date: String,
    },
    Segregating {
        generation: Generation,
        population_size: u32,
    },
    PreliminaryYieldTrial {
        entries: u16,
        locations: u8,
    },
    AdvancedYieldTrial {
        entries: u16,
        locations: u8,
        years: u8,
    },
    PreRelease {
        candidate_name: String,
    },
    VarietyRelease {
        name: String,
        year: u16,
        region: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ResistanceLevel {
    Immune,
    HighlyResistant,
    Resistant,
    ModeratelyResistant,
    ModeratelySusceptible,
    Susceptible,
    HighlySusceptible,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DiseaseType {
    Fungal {
        pathogen: String,
        race: Option<String>,
    },
    Bacterial {
        pathogen: String,
        pathovar: Option<String>,
    },
    Viral {
        virus_name: String,
        strain: Option<String>,
    },
    Nematode {
        species: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DiseaseResistance {
    disease: DiseaseType,
    level: ResistanceLevel,
    gene: Option<String>,
    inheritance: InheritancePattern,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ToleranceScore {
    Score1Susceptible,
    Score3Poor,
    Score5Moderate,
    Score7Good,
    Score9Excellent,
    Quantitative { score: f64, std_error: f64 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AbioticStress {
    Drought {
        tolerance: ToleranceScore,
        screening_stage: String,
    },
    Heat {
        tolerance: ToleranceScore,
        max_temp_c: f64,
    },
    Salinity {
        tolerance: ToleranceScore,
        ec_ds_m: f64,
    },
    Frost {
        tolerance: ToleranceScore,
        min_temp_c: f64,
    },
    Waterlogging {
        tolerance: ToleranceScore,
        duration_days: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlotData {
    plot_id: String,
    entry_id: u32,
    rep: u8,
    block: u8,
    yield_kg_ha: f64,
    moisture_pct: f64,
    plant_height_cm: f64,
    days_to_heading: u16,
    days_to_maturity: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FieldLayout {
    Rcbd {
        reps: u8,
        entries: u16,
    },
    AlphaLattice {
        reps: u8,
        blocks_per_rep: u8,
        entries: u16,
    },
    Augmented {
        checks: Vec<String>,
        test_entries: u16,
    },
    SplitPlot {
        main_factor: String,
        sub_factor: String,
    },
    StripPlot {
        horizontal: String,
        vertical: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct YieldTrial {
    trial_name: String,
    location: String,
    year: u16,
    layout: FieldLayout,
    plots: Vec<PlotData>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum QtlEffect {
    Additive(f64),
    Dominance(f64),
    Overdominance {
        additive: f64,
        dominance: f64,
    },
    Epistatic {
        effect_a: f64,
        effect_b: f64,
        interaction: f64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QtlResult {
    trait_name: String,
    chromosome: u8,
    position_cm: f64,
    lod_score: f64,
    pve_percent: f64,
    flanking_markers: (String, String),
    effect: QtlEffect,
    confidence_interval: (f64, f64),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GwasSignificance {
    GenomeWide { p_value: f64, neg_log10_p: f64 },
    Suggestive { p_value: f64, neg_log10_p: f64 },
    NotSignificant,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GwasHit {
    snp_id: String,
    chromosome: u8,
    position: u64,
    trait_name: String,
    significance: GwasSignificance,
    effect_size: f64,
    maf: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SeedCertification {
    BreederSeed,
    FoundationSeed {
        generation: u8,
        lot_number: String,
    },
    RegisteredSeed {
        lot_number: String,
        purity_pct: f64,
    },
    CertifiedSeed {
        lot_number: String,
        purity_pct: f64,
        germination_pct: f64,
    },
    QualityDeclared {
        declaration_id: String,
    },
    Uncertified,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IntellectualProperty {
    Pvp {
        certificate_number: String,
        expiry_year: u16,
        species: String,
    },
    UtilityPatent {
        patent_number: String,
        claims: Vec<String>,
    },
    PlantPatent {
        patent_number: String,
    },
    TradeSecret,
    OpenSource {
        license: String,
    },
    NoProtection,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PedigreeNode {
    Named {
        name: String,
        origin: String,
    },
    Cross {
        female: Box<PedigreeNode>,
        male: Box<PedigreeNode>,
    },
    Backcross {
        recurrent: Box<PedigreeNode>,
        donor: Box<PedigreeNode>,
        generations: u8,
    },
    Unknown,
    Landrace {
        region: String,
        collected_year: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TissueCultureStage {
    ExplantInitiation {
        tissue_type: String,
        sterilization_protocol: String,
    },
    CallusInduction {
        medium: String,
        hormone_mg_l: f64,
    },
    SomaticEmbryogenesis {
        embryo_count: u32,
    },
    ShootRegeneration {
        shoot_count: u32,
        medium: String,
    },
    Rooting {
        root_count: u32,
        days: u16,
    },
    Acclimatization {
        survival_pct: f64,
    },
    FieldTransfer {
        plants_transferred: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GmoEvent {
    SingleGene {
        gene: String,
        source_organism: String,
        promoter: String,
    },
    GeneStack {
        events: Vec<String>,
        traits_conferred: Vec<String>,
    },
    CrisprEdit {
        target_gene: String,
        edit_type: String,
        guide_rna: String,
    },
    RnaInterference {
        target_gene: String,
        silencing_pct: f64,
    },
    Cisgenic {
        gene: String,
        source_species: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GmoRegulatory {
    Approved {
        country: String,
        event: GmoEvent,
        approval_year: u16,
    },
    PendingApproval {
        country: String,
        event: GmoEvent,
        submission_year: u16,
    },
    FieldTrialOnly {
        permit_id: String,
        event: GmoEvent,
    },
    Deregulated {
        event: GmoEvent,
    },
    Banned {
        country: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GrainQualityClass {
    HighProtein {
        protein_pct: f64,
        gluten_index: f64,
    },
    HighOil {
        oil_pct: f64,
        fatty_acid_profile: String,
    },
    HighStarch {
        starch_pct: f64,
        amylose_pct: f64,
    },
    FeedGrade {
        crude_protein_pct: f64,
        fiber_pct: f64,
    },
    SpecialtyUse {
        use_type: String,
        key_parameter: String,
        value: f64,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GrainQualityReport {
    sample_id: String,
    crop: String,
    quality_class: GrainQualityClass,
    test_weight_kg_hl: f64,
    moisture_pct: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BreedingObjective {
    YieldImprovement {
        target_increase_pct: f64,
    },
    DiseaseResistanceProgram {
        targets: Vec<DiseaseResistance>,
    },
    QualityEnhancement {
        quality_target: GrainQualityClass,
    },
    StressAdaptation {
        stress_targets: Vec<AbioticStress>,
    },
    HybridDevelopment {
        heterosis_pct: f64,
        cms_system: String,
    },
    MutationBreeding {
        mutagen: String,
        dose: String,
    },
    SpeedBreeding {
        generations_per_year: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BreedingProgram {
    program_id: String,
    crop: String,
    stage: BreedingStage,
    objective: BreedingObjective,
    ip_status: IntellectualProperty,
    pedigree: PedigreeNode,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_inheritance_patterns() {
    let patterns = vec![
        InheritancePattern::Dominant,
        InheritancePattern::Recessive,
        InheritancePattern::Codominant,
        InheritancePattern::IncompleteDominant,
        InheritancePattern::Polygenic {
            loci_count: 12,
            heritability: 0.65,
        },
        InheritancePattern::Epistatic {
            modifier_gene: "Rf1".to_string(),
            target_gene: "Ms26".to_string(),
        },
        InheritancePattern::Maternal,
    ];
    for p in &patterns {
        let bytes = encode_to_vec(p).expect("encode inheritance pattern");
        let (decoded, _) =
            decode_from_slice::<InheritancePattern>(&bytes).expect("decode inheritance pattern");
        assert_eq!(p, &decoded);
    }
}

#[test]
fn test_molecular_markers() {
    let markers = vec![
        MolecularMarker::Ssr {
            locus: "Xgwm261".to_string(),
            repeat_motif: "CT".to_string(),
            allele_sizes: vec![174, 192, 210],
        },
        MolecularMarker::Snp {
            chromosome: 3,
            position: 45_892_001,
            ref_allele: b'A',
            alt_allele: b'G',
        },
        MolecularMarker::Aflp {
            primer_combination: "EcoRI-AGG/MseI-CTT".to_string(),
            fragment_size: 287,
            presence: true,
        },
        MolecularMarker::Kasp {
            assay_id: "snpKASP_001".to_string(),
            fam_allele: b'C',
            hex_allele: b'T',
            call: "FAM:FAM".to_string(),
        },
        MolecularMarker::InDel {
            chromosome: 7,
            position: 102_400_500,
            inserted_bases: "AATCG".to_string(),
        },
    ];
    for m in &markers {
        let bytes = encode_to_vec(m).expect("encode marker");
        let (decoded, _) = decode_from_slice::<MolecularMarker>(&bytes).expect("decode marker");
        assert_eq!(m, &decoded);
    }
}

#[test]
fn test_breeding_stages_with_generations() {
    let stages = vec![
        BreedingStage::Crossing {
            female: "IR64".to_string(),
            male: "Nipponbare".to_string(),
            date: "2025-06-15".to_string(),
        },
        BreedingStage::Segregating {
            generation: Generation::F2,
            population_size: 5000,
        },
        BreedingStage::Segregating {
            generation: Generation::Bc {
                recurrent_parent: "IR64".to_string(),
                backcross_number: 3,
            },
            population_size: 200,
        },
        BreedingStage::PreliminaryYieldTrial {
            entries: 400,
            locations: 3,
        },
        BreedingStage::AdvancedYieldTrial {
            entries: 25,
            locations: 12,
            years: 3,
        },
        BreedingStage::PreRelease {
            candidate_name: "IRRI-2025-A".to_string(),
        },
        BreedingStage::VarietyRelease {
            name: "NSIC Rc480".to_string(),
            year: 2026,
            region: "Philippines Region III".to_string(),
        },
    ];
    for s in &stages {
        let bytes = encode_to_vec(s).expect("encode breeding stage");
        let (decoded, _) =
            decode_from_slice::<BreedingStage>(&bytes).expect("decode breeding stage");
        assert_eq!(s, &decoded);
    }
}

#[test]
fn test_disease_resistance_classification() {
    let dr = DiseaseResistance {
        disease: DiseaseType::Fungal {
            pathogen: "Puccinia triticina".to_string(),
            race: Some("TBBGS".to_string()),
        },
        level: ResistanceLevel::HighlyResistant,
        gene: Some("Lr34".to_string()),
        inheritance: InheritancePattern::Dominant,
    };
    let bytes = encode_to_vec(&dr).expect("encode disease resistance");
    let (decoded, _) =
        decode_from_slice::<DiseaseResistance>(&bytes).expect("decode disease resistance");
    assert_eq!(dr, decoded);
}

#[test]
fn test_multiple_disease_types() {
    let diseases = vec![
        DiseaseResistance {
            disease: DiseaseType::Bacterial {
                pathogen: "Xanthomonas oryzae".to_string(),
                pathovar: Some("oryzae".to_string()),
            },
            level: ResistanceLevel::Resistant,
            gene: Some("Xa21".to_string()),
            inheritance: InheritancePattern::Dominant,
        },
        DiseaseResistance {
            disease: DiseaseType::Viral {
                virus_name: "Rice tungro".to_string(),
                strain: None,
            },
            level: ResistanceLevel::ModeratelyResistant,
            gene: None,
            inheritance: InheritancePattern::Polygenic {
                loci_count: 4,
                heritability: 0.42,
            },
        },
        DiseaseResistance {
            disease: DiseaseType::Nematode {
                species: "Heterodera avenae".to_string(),
            },
            level: ResistanceLevel::Immune,
            gene: Some("Cre1".to_string()),
            inheritance: InheritancePattern::Recessive,
        },
    ];
    let bytes = encode_to_vec(&diseases).expect("encode disease list");
    let (decoded, _) =
        decode_from_slice::<Vec<DiseaseResistance>>(&bytes).expect("decode disease list");
    assert_eq!(diseases, decoded);
}

#[test]
fn test_abiotic_stress_tolerance() {
    let stresses = vec![
        AbioticStress::Drought {
            tolerance: ToleranceScore::Score7Good,
            screening_stage: "flowering".to_string(),
        },
        AbioticStress::Heat {
            tolerance: ToleranceScore::Quantitative {
                score: 7.8,
                std_error: 0.3,
            },
            max_temp_c: 42.5,
        },
        AbioticStress::Salinity {
            tolerance: ToleranceScore::Score5Moderate,
            ec_ds_m: 8.0,
        },
        AbioticStress::Frost {
            tolerance: ToleranceScore::Score9Excellent,
            min_temp_c: -15.0,
        },
        AbioticStress::Waterlogging {
            tolerance: ToleranceScore::Score3Poor,
            duration_days: 14,
        },
    ];
    for s in &stresses {
        let bytes = encode_to_vec(s).expect("encode abiotic stress");
        let (decoded, _) =
            decode_from_slice::<AbioticStress>(&bytes).expect("decode abiotic stress");
        assert_eq!(s, &decoded);
    }
}

#[test]
fn test_yield_trial_with_rcbd_layout() {
    let trial = YieldTrial {
        trial_name: "IRYT-2025-WS".to_string(),
        location: "IRRI Los Banos".to_string(),
        year: 2025,
        layout: FieldLayout::Rcbd {
            reps: 3,
            entries: 64,
        },
        plots: vec![
            PlotData {
                plot_id: "R1B1P01".to_string(),
                entry_id: 1,
                rep: 1,
                block: 1,
                yield_kg_ha: 6850.0,
                moisture_pct: 14.2,
                plant_height_cm: 98.5,
                days_to_heading: 82,
                days_to_maturity: 115,
            },
            PlotData {
                plot_id: "R1B1P02".to_string(),
                entry_id: 2,
                rep: 1,
                block: 1,
                yield_kg_ha: 7120.0,
                moisture_pct: 13.8,
                plant_height_cm: 102.3,
                days_to_heading: 85,
                days_to_maturity: 118,
            },
        ],
    };
    let bytes = encode_to_vec(&trial).expect("encode yield trial");
    let (decoded, _) = decode_from_slice::<YieldTrial>(&bytes).expect("decode yield trial");
    assert_eq!(trial, decoded);
}

#[test]
fn test_alpha_lattice_layout() {
    let trial = YieldTrial {
        trial_name: "NUVYT-2025".to_string(),
        location: "CIMMYT El Batan".to_string(),
        year: 2025,
        layout: FieldLayout::AlphaLattice {
            reps: 2,
            blocks_per_rep: 10,
            entries: 200,
        },
        plots: vec![PlotData {
            plot_id: "R1B03P07".to_string(),
            entry_id: 37,
            rep: 1,
            block: 3,
            yield_kg_ha: 5400.0,
            moisture_pct: 12.5,
            plant_height_cm: 88.0,
            days_to_heading: 68,
            days_to_maturity: 102,
        }],
    };
    let bytes = encode_to_vec(&trial).expect("encode alpha lattice trial");
    let (decoded, _) = decode_from_slice::<YieldTrial>(&bytes).expect("decode alpha lattice trial");
    assert_eq!(trial, decoded);
}

#[test]
fn test_augmented_and_strip_plot_layouts() {
    let layouts = vec![
        FieldLayout::Augmented {
            checks: vec![
                "IR64".to_string(),
                "PSB Rc82".to_string(),
                "NSIC Rc222".to_string(),
            ],
            test_entries: 500,
        },
        FieldLayout::SplitPlot {
            main_factor: "Nitrogen rate".to_string(),
            sub_factor: "Variety".to_string(),
        },
        FieldLayout::StripPlot {
            horizontal: "Irrigation regime".to_string(),
            vertical: "Planting density".to_string(),
        },
    ];
    for layout in &layouts {
        let bytes = encode_to_vec(layout).expect("encode field layout");
        let (decoded, _) = decode_from_slice::<FieldLayout>(&bytes).expect("decode field layout");
        assert_eq!(layout, &decoded);
    }
}

#[test]
fn test_qtl_mapping_results() {
    let qtls = vec![
        QtlResult {
            trait_name: "Plant height".to_string(),
            chromosome: 4,
            position_cm: 72.3,
            lod_score: 12.8,
            pve_percent: 18.5,
            flanking_markers: ("Xgwm513".to_string(), "Xgwm165".to_string()),
            effect: QtlEffect::Additive(-8.3),
            confidence_interval: (68.1, 76.5),
        },
        QtlResult {
            trait_name: "Grain yield".to_string(),
            chromosome: 1,
            position_cm: 105.7,
            lod_score: 5.2,
            pve_percent: 7.1,
            flanking_markers: ("RM1".to_string(), "RM283".to_string()),
            effect: QtlEffect::Overdominance {
                additive: 120.0,
                dominance: 350.0,
            },
            confidence_interval: (98.0, 113.4),
        },
        QtlResult {
            trait_name: "Heading date".to_string(),
            chromosome: 6,
            position_cm: 33.0,
            lod_score: 20.1,
            pve_percent: 35.0,
            flanking_markers: ("Hd1_F".to_string(), "Hd1_R".to_string()),
            effect: QtlEffect::Dominance(4.5),
            confidence_interval: (31.0, 35.0),
        },
    ];
    let bytes = encode_to_vec(&qtls).expect("encode QTL results");
    let (decoded, _) = decode_from_slice::<Vec<QtlResult>>(&bytes).expect("decode QTL results");
    assert_eq!(qtls, decoded);
}

#[test]
fn test_epistatic_qtl_effect() {
    let qtl = QtlResult {
        trait_name: "Fusarium head blight resistance".to_string(),
        chromosome: 3,
        position_cm: 48.2,
        lod_score: 8.9,
        pve_percent: 12.4,
        flanking_markers: ("Xgwm389".to_string(), "Xbarc147".to_string()),
        effect: QtlEffect::Epistatic {
            effect_a: -0.45,
            effect_b: 0.22,
            interaction: -0.18,
        },
        confidence_interval: (44.0, 52.5),
    };
    let bytes = encode_to_vec(&qtl).expect("encode epistatic QTL");
    let (decoded, _) = decode_from_slice::<QtlResult>(&bytes).expect("decode epistatic QTL");
    assert_eq!(qtl, decoded);
}

#[test]
fn test_gwas_hits() {
    let hits = vec![
        GwasHit {
            snp_id: "S1_283940121".to_string(),
            chromosome: 1,
            position: 283_940_121,
            trait_name: "Thousand kernel weight".to_string(),
            significance: GwasSignificance::GenomeWide {
                p_value: 2.3e-9,
                neg_log10_p: 8.64,
            },
            effect_size: 1.82,
            maf: 0.23,
        },
        GwasHit {
            snp_id: "S5_67201000".to_string(),
            chromosome: 5,
            position: 67_201_000,
            trait_name: "Days to heading".to_string(),
            significance: GwasSignificance::Suggestive {
                p_value: 5.1e-6,
                neg_log10_p: 5.29,
            },
            effect_size: -2.1,
            maf: 0.38,
        },
        GwasHit {
            snp_id: "S7_120000".to_string(),
            chromosome: 7,
            position: 120_000,
            trait_name: "Protein content".to_string(),
            significance: GwasSignificance::NotSignificant,
            effect_size: 0.01,
            maf: 0.05,
        },
    ];
    let bytes = encode_to_vec(&hits).expect("encode GWAS hits");
    let (decoded, _) = decode_from_slice::<Vec<GwasHit>>(&bytes).expect("decode GWAS hits");
    assert_eq!(hits, decoded);
}

#[test]
fn test_seed_certification_grades() {
    let grades = vec![
        SeedCertification::BreederSeed,
        SeedCertification::FoundationSeed {
            generation: 2,
            lot_number: "FS-2025-0042".to_string(),
        },
        SeedCertification::RegisteredSeed {
            lot_number: "RS-2025-1001".to_string(),
            purity_pct: 99.5,
        },
        SeedCertification::CertifiedSeed {
            lot_number: "CS-2025-5500".to_string(),
            purity_pct: 98.0,
            germination_pct: 85.0,
        },
        SeedCertification::QualityDeclared {
            declaration_id: "QDS-TZ-2025-002".to_string(),
        },
        SeedCertification::Uncertified,
    ];
    for g in &grades {
        let bytes = encode_to_vec(g).expect("encode seed certification");
        let (decoded, _) =
            decode_from_slice::<SeedCertification>(&bytes).expect("decode seed certification");
        assert_eq!(g, &decoded);
    }
}

#[test]
fn test_intellectual_property_types() {
    let ip_types = vec![
        IntellectualProperty::Pvp {
            certificate_number: "PVP-2024-00123".to_string(),
            expiry_year: 2044,
            species: "Triticum aestivum".to_string(),
        },
        IntellectualProperty::UtilityPatent {
            patent_number: "US10,123,456".to_string(),
            claims: vec![
                "Drought tolerance gene construct".to_string(),
                "Method of transformation".to_string(),
                "Resulting plant cell".to_string(),
            ],
        },
        IntellectualProperty::PlantPatent {
            patent_number: "USPP34,567".to_string(),
        },
        IntellectualProperty::TradeSecret,
        IntellectualProperty::OpenSource {
            license: "Open Source Seed Initiative (OSSI)".to_string(),
        },
        IntellectualProperty::NoProtection,
    ];
    for ip in &ip_types {
        let bytes = encode_to_vec(ip).expect("encode IP type");
        let (decoded, _) =
            decode_from_slice::<IntellectualProperty>(&bytes).expect("decode IP type");
        assert_eq!(ip, &decoded);
    }
}

#[test]
fn test_pedigree_tree_deeply_nested() {
    let pedigree = PedigreeNode::Cross {
        female: Box::new(PedigreeNode::Cross {
            female: Box::new(PedigreeNode::Named {
                name: "IR8".to_string(),
                origin: "IRRI".to_string(),
            }),
            male: Box::new(PedigreeNode::Named {
                name: "TN1".to_string(),
                origin: "Taiwan".to_string(),
            }),
        }),
        male: Box::new(PedigreeNode::Backcross {
            recurrent: Box::new(PedigreeNode::Named {
                name: "IR64".to_string(),
                origin: "IRRI".to_string(),
            }),
            donor: Box::new(PedigreeNode::Landrace {
                region: "Odisha, India".to_string(),
                collected_year: 1965,
            }),
            generations: 4,
        }),
    };
    let bytes = encode_to_vec(&pedigree).expect("encode pedigree tree");
    let (decoded, _) = decode_from_slice::<PedigreeNode>(&bytes).expect("decode pedigree tree");
    assert_eq!(pedigree, decoded);
}

#[test]
fn test_tissue_culture_pipeline() {
    let stages = vec![
        TissueCultureStage::ExplantInitiation {
            tissue_type: "immature embryo".to_string(),
            sterilization_protocol: "70% ethanol 30s + 0.1% HgCl2 5min".to_string(),
        },
        TissueCultureStage::CallusInduction {
            medium: "MS + 2mg/L 2,4-D".to_string(),
            hormone_mg_l: 2.0,
        },
        TissueCultureStage::SomaticEmbryogenesis { embryo_count: 48 },
        TissueCultureStage::ShootRegeneration {
            shoot_count: 35,
            medium: "MS + 1mg/L BAP + 0.5mg/L NAA".to_string(),
        },
        TissueCultureStage::Rooting {
            root_count: 30,
            days: 21,
        },
        TissueCultureStage::Acclimatization { survival_pct: 87.5 },
        TissueCultureStage::FieldTransfer {
            plants_transferred: 26,
        },
    ];
    let bytes = encode_to_vec(&stages).expect("encode tissue culture stages");
    let (decoded, _) =
        decode_from_slice::<Vec<TissueCultureStage>>(&bytes).expect("decode tissue culture stages");
    assert_eq!(stages, decoded);
}

#[test]
fn test_gmo_event_tracking() {
    let events = vec![
        GmoEvent::SingleGene {
            gene: "cry1Ab".to_string(),
            source_organism: "Bacillus thuringiensis".to_string(),
            promoter: "CaMV 35S".to_string(),
        },
        GmoEvent::GeneStack {
            events: vec!["MON810".to_string(), "NK603".to_string()],
            traits_conferred: vec![
                "Lepidoptera resistance".to_string(),
                "Glyphosate tolerance".to_string(),
            ],
        },
        GmoEvent::CrisprEdit {
            target_gene: "WAXY".to_string(),
            edit_type: "Knockout".to_string(),
            guide_rna: "ATCGATCGATCGATCGATCG".to_string(),
        },
        GmoEvent::RnaInterference {
            target_gene: "PPO".to_string(),
            silencing_pct: 95.0,
        },
        GmoEvent::Cisgenic {
            gene: "Rpi-vnt1".to_string(),
            source_species: "Solanum venturii".to_string(),
        },
    ];
    for e in &events {
        let bytes = encode_to_vec(e).expect("encode GMO event");
        let (decoded, _) = decode_from_slice::<GmoEvent>(&bytes).expect("decode GMO event");
        assert_eq!(e, &decoded);
    }
}

#[test]
fn test_gmo_regulatory_nested() {
    let statuses = vec![
        GmoRegulatory::Approved {
            country: "USA".to_string(),
            event: GmoEvent::SingleGene {
                gene: "cp4-epsps".to_string(),
                source_organism: "Agrobacterium sp. strain CP4".to_string(),
                promoter: "FMV 35S".to_string(),
            },
            approval_year: 1996,
        },
        GmoRegulatory::PendingApproval {
            country: "EU".to_string(),
            event: GmoEvent::CrisprEdit {
                target_gene: "SlCLV3".to_string(),
                edit_type: "Promoter modification".to_string(),
                guide_rna: "GCTAGCTAGCTAGCTAGCTA".to_string(),
            },
            submission_year: 2025,
        },
        GmoRegulatory::FieldTrialOnly {
            permit_id: "FT-KE-2025-012".to_string(),
            event: GmoEvent::RnaInterference {
                target_gene: "BnFAD2".to_string(),
                silencing_pct: 88.0,
            },
        },
        GmoRegulatory::Deregulated {
            event: GmoEvent::Cisgenic {
                gene: "VfAAT".to_string(),
                source_species: "Malus sylvestris".to_string(),
            },
        },
        GmoRegulatory::Banned {
            country: "Zambia".to_string(),
        },
    ];
    let bytes = encode_to_vec(&statuses).expect("encode GMO regulatory");
    let (decoded, _) =
        decode_from_slice::<Vec<GmoRegulatory>>(&bytes).expect("decode GMO regulatory");
    assert_eq!(statuses, decoded);
}

#[test]
fn test_grain_quality_parameters() {
    let reports = vec![
        GrainQualityReport {
            sample_id: "GQ-2025-001".to_string(),
            crop: "Wheat".to_string(),
            quality_class: GrainQualityClass::HighProtein {
                protein_pct: 14.2,
                gluten_index: 88.0,
            },
            test_weight_kg_hl: 80.5,
            moisture_pct: 12.0,
        },
        GrainQualityReport {
            sample_id: "GQ-2025-002".to_string(),
            crop: "Soybean".to_string(),
            quality_class: GrainQualityClass::HighOil {
                oil_pct: 21.5,
                fatty_acid_profile: "High oleic".to_string(),
            },
            test_weight_kg_hl: 72.0,
            moisture_pct: 13.0,
        },
        GrainQualityReport {
            sample_id: "GQ-2025-003".to_string(),
            crop: "Maize".to_string(),
            quality_class: GrainQualityClass::HighStarch {
                starch_pct: 72.0,
                amylose_pct: 25.0,
            },
            test_weight_kg_hl: 74.0,
            moisture_pct: 14.5,
        },
        GrainQualityReport {
            sample_id: "GQ-2025-004".to_string(),
            crop: "Barley".to_string(),
            quality_class: GrainQualityClass::SpecialtyUse {
                use_type: "Malting".to_string(),
                key_parameter: "Diastatic power".to_string(),
                value: 145.0,
            },
            test_weight_kg_hl: 68.0,
            moisture_pct: 11.5,
        },
    ];
    let bytes = encode_to_vec(&reports).expect("encode grain quality reports");
    let (decoded, _) =
        decode_from_slice::<Vec<GrainQualityReport>>(&bytes).expect("decode grain quality reports");
    assert_eq!(reports, decoded);
}

#[test]
fn test_breeding_program_full() {
    let program = BreedingProgram {
        program_id: "WBP-2025-DROUGHT".to_string(),
        crop: "Wheat".to_string(),
        stage: BreedingStage::Segregating {
            generation: Generation::F4,
            population_size: 800,
        },
        objective: BreedingObjective::StressAdaptation {
            stress_targets: vec![
                AbioticStress::Drought {
                    tolerance: ToleranceScore::Score7Good,
                    screening_stage: "anthesis".to_string(),
                },
                AbioticStress::Heat {
                    tolerance: ToleranceScore::Score5Moderate,
                    max_temp_c: 38.0,
                },
            ],
        },
        ip_status: IntellectualProperty::OpenSource {
            license: "OSSI Pledge".to_string(),
        },
        pedigree: PedigreeNode::Cross {
            female: Box::new(PedigreeNode::Named {
                name: "Kingbird".to_string(),
                origin: "CIMMYT".to_string(),
            }),
            male: Box::new(PedigreeNode::Named {
                name: "Vorobey".to_string(),
                origin: "CIMMYT".to_string(),
            }),
        },
    };
    let bytes = encode_to_vec(&program).expect("encode breeding program");
    let (decoded, _) =
        decode_from_slice::<BreedingProgram>(&bytes).expect("decode breeding program");
    assert_eq!(program, decoded);
}

#[test]
fn test_hybrid_breeding_with_cms() {
    let program = BreedingProgram {
        program_id: "RHP-2025-HYBRID".to_string(),
        crop: "Rice".to_string(),
        stage: BreedingStage::AdvancedYieldTrial {
            entries: 18,
            locations: 8,
            years: 2,
        },
        objective: BreedingObjective::HybridDevelopment {
            heterosis_pct: 25.0,
            cms_system: "WA-CMS".to_string(),
        },
        ip_status: IntellectualProperty::Pvp {
            certificate_number: "PVP-PH-2025-009".to_string(),
            expiry_year: 2045,
            species: "Oryza sativa".to_string(),
        },
        pedigree: PedigreeNode::Cross {
            female: Box::new(PedigreeNode::Named {
                name: "IR58025A".to_string(),
                origin: "IRRI".to_string(),
            }),
            male: Box::new(PedigreeNode::Cross {
                female: Box::new(PedigreeNode::Named {
                    name: "IR34686R".to_string(),
                    origin: "IRRI".to_string(),
                }),
                male: Box::new(PedigreeNode::Named {
                    name: "Swarna".to_string(),
                    origin: "ICAR".to_string(),
                }),
            }),
        },
    };
    let bytes = encode_to_vec(&program).expect("encode hybrid program");
    let (decoded, _) = decode_from_slice::<BreedingProgram>(&bytes).expect("decode hybrid program");
    assert_eq!(program, decoded);
}

#[test]
fn test_speed_breeding_mutation_objectives() {
    let objectives = vec![
        BreedingObjective::SpeedBreeding {
            generations_per_year: 6,
        },
        BreedingObjective::MutationBreeding {
            mutagen: "EMS".to_string(),
            dose: "0.8% v/v for 16 hours".to_string(),
        },
        BreedingObjective::YieldImprovement {
            target_increase_pct: 15.0,
        },
        BreedingObjective::DiseaseResistanceProgram {
            targets: vec![
                DiseaseResistance {
                    disease: DiseaseType::Fungal {
                        pathogen: "Magnaporthe oryzae".to_string(),
                        race: Some("IB-49".to_string()),
                    },
                    level: ResistanceLevel::HighlyResistant,
                    gene: Some("Pi9".to_string()),
                    inheritance: InheritancePattern::Dominant,
                },
                DiseaseResistance {
                    disease: DiseaseType::Fungal {
                        pathogen: "Rhizoctonia solani".to_string(),
                        race: None,
                    },
                    level: ResistanceLevel::ModeratelyResistant,
                    gene: None,
                    inheritance: InheritancePattern::Polygenic {
                        loci_count: 8,
                        heritability: 0.35,
                    },
                },
            ],
        },
        BreedingObjective::QualityEnhancement {
            quality_target: GrainQualityClass::FeedGrade {
                crude_protein_pct: 11.0,
                fiber_pct: 3.2,
            },
        },
    ];
    let bytes = encode_to_vec(&objectives).expect("encode breeding objectives");
    let (decoded, _) =
        decode_from_slice::<Vec<BreedingObjective>>(&bytes).expect("decode breeding objectives");
    assert_eq!(objectives, decoded);
}
