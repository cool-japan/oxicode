#![cfg(feature = "compression-zstd")]
//! Advanced Zstd compression tests for the genomics / DNA sequence analysis domain.
//!
//! Covers nucleotide sequences, gene variants, SNP data, genome assemblies,
//! alignment records, quality scores, annotation tracks, and VCF records.

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

// ---------------------------------------------------------------------------
// Domain structs and enums
// ---------------------------------------------------------------------------

/// Four canonical DNA bases plus ambiguity codes.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Nucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
    /// IUPAC ambiguity: any base
    N,
    /// Deletion / gap in alignment
    Gap,
}

/// A raw nucleotide sequence with an identifier.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NucleotideSequence {
    id: String,
    bases: Vec<Nucleotide>,
    length: u64,
}

/// Single-nucleotide polymorphism record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SnpRecord {
    chromosome: String,
    position: u64,
    reference_allele: u8,
    alternate_allele: u8,
    quality_score: f32,
    filter_pass: bool,
}

/// Alignment record analogous to a SAM/BAM entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AlignmentRecord {
    read_name: String,
    flag: u16,
    reference_name: String,
    position: u64,
    mapping_quality: u8,
    cigar: String,
    sequence: Vec<u8>,
    base_qualities: Vec<u8>,
}

/// Per-base quality score encoding (Phred+33).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QualityScoreTrack {
    sample_id: String,
    scores: Vec<u8>,
    mean_quality: f64,
}

/// Genome annotation record (GFF-like).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AnnotationRecord {
    seqname: String,
    source: String,
    feature: String,
    start: u64,
    end: u64,
    score: Option<f64>,
    strand: i8,
    frame: Option<u8>,
    attributes: Vec<(String, String)>,
}

/// VCF INFO field entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VcfInfoValue {
    Integer(i64),
    Float(f64),
    Flag,
    Text(String),
    IntegerVec(Vec<i64>),
}

/// A VCF record (Variant Call Format).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VcfRecord {
    chrom: String,
    pos: u64,
    id: Option<String>,
    reference: Vec<u8>,
    alt_alleles: Vec<Vec<u8>>,
    qual: Option<f64>,
    filter: Vec<String>,
    info: Vec<(String, VcfInfoValue)>,
    genotypes: Vec<Vec<u8>>,
}

/// Genome assembly contig.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Contig {
    name: String,
    sequence: Vec<u8>,
    gc_content: f64,
    coverage_depth: f32,
}

/// Gene variant classification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VariantEffect {
    Synonymous,
    Missense {
        codon_ref: [u8; 3],
        codon_alt: [u8; 3],
    },
    Nonsense {
        position_in_cds: u32,
    },
    Frameshift {
        net_indel: i32,
    },
    SpliceSite,
    Intronic,
    Intergenic,
}

/// Gene variant with effect annotation.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GeneVariant {
    gene_id: String,
    transcript_id: String,
    variant_type: VariantEffect,
    population_frequency: f64,
    clinically_significant: bool,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_nucleotide_sequence(n: usize) -> NucleotideSequence {
    let bases: Vec<Nucleotide> = (0..n)
        .map(|i| match i % 4 {
            0 => Nucleotide::Adenine,
            1 => Nucleotide::Cytosine,
            2 => Nucleotide::Guanine,
            _ => Nucleotide::Thymine,
        })
        .collect();
    NucleotideSequence {
        id: format!("seq_{n}"),
        bases,
        length: n as u64,
    }
}

fn make_snp_record(pos: u64) -> SnpRecord {
    SnpRecord {
        chromosome: "chr1".to_string(),
        position: pos,
        reference_allele: b'A',
        alternate_allele: b'G',
        quality_score: 30.5,
        filter_pass: true,
    }
}

fn make_alignment_record(name: &str) -> AlignmentRecord {
    AlignmentRecord {
        read_name: name.to_string(),
        flag: 0x0,
        reference_name: "chr1".to_string(),
        position: 1_000_000,
        mapping_quality: 60,
        cigar: "150M".to_string(),
        sequence: b"ACGTACGTACGT".to_vec(),
        base_qualities: (33u8..=44).collect(),
    }
}

fn make_vcf_record(pos: u64) -> VcfRecord {
    VcfRecord {
        chrom: "chr1".to_string(),
        pos,
        id: Some(format!("rs{pos}")),
        reference: b"A".to_vec(),
        alt_alleles: vec![b"G".to_vec()],
        qual: Some(99.0),
        filter: vec!["PASS".to_string()],
        info: vec![
            ("AF".to_string(), VcfInfoValue::Float(0.42)),
            ("DP".to_string(), VcfInfoValue::Integer(42)),
        ],
        genotypes: vec![b"0/1".to_vec()],
    }
}

fn make_contig(name: &str, len: usize) -> Contig {
    Contig {
        name: name.to_string(),
        sequence: (0..len).map(|i| b"ACGT"[i % 4]).collect(),
        gc_content: 0.50,
        coverage_depth: 30.0,
    }
}

// ---------------------------------------------------------------------------
// Test 1: basic NucleotideSequence compress/decompress roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_nucleotide_sequence_basic_roundtrip() {
    let seq = make_nucleotide_sequence(120);
    let encoded = encode_to_vec(&seq).expect("encode NucleotideSequence failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (NucleotideSequence, usize) =
        decode_from_slice(&decompressed).expect("decode NucleotideSequence failed");
    assert_eq!(seq, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: SnpRecord roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_snp_record_roundtrip() {
    let snp = make_snp_record(123_456);
    let encoded = encode_to_vec(&snp).expect("encode SnpRecord failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (SnpRecord, usize) =
        decode_from_slice(&decompressed).expect("decode SnpRecord failed");
    assert_eq!(snp, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: AlignmentRecord roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_alignment_record_roundtrip() {
    let aln = make_alignment_record("read_001");
    let encoded = encode_to_vec(&aln).expect("encode AlignmentRecord failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (AlignmentRecord, usize) =
        decode_from_slice(&decompressed).expect("decode AlignmentRecord failed");
    assert_eq!(aln, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: QualityScoreTrack roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_quality_score_track_roundtrip() {
    let track = QualityScoreTrack {
        sample_id: "SAMPLE_42".to_string(),
        scores: (33u8..=75).cycle().take(300).collect(),
        mean_quality: 38.7,
    };
    let encoded = encode_to_vec(&track).expect("encode QualityScoreTrack failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (QualityScoreTrack, usize) =
        decode_from_slice(&decompressed).expect("decode QualityScoreTrack failed");
    assert_eq!(track, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: AnnotationRecord roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_annotation_record_roundtrip() {
    let ann = AnnotationRecord {
        seqname: "chr7".to_string(),
        source: "Ensembl".to_string(),
        feature: "exon".to_string(),
        start: 117_559_590,
        end: 117_559_732,
        score: Some(1000.0),
        strand: 1,
        frame: Some(0),
        attributes: vec![
            ("gene_id".to_string(), "ENSG00000001626".to_string()),
            ("transcript_id".to_string(), "ENST00000003084".to_string()),
        ],
    };
    let encoded = encode_to_vec(&ann).expect("encode AnnotationRecord failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (AnnotationRecord, usize) =
        decode_from_slice(&decompressed).expect("decode AnnotationRecord failed");
    assert_eq!(ann, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: VcfRecord roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_vcf_record_roundtrip() {
    let vcf = make_vcf_record(1_000_000);
    let encoded = encode_to_vec(&vcf).expect("encode VcfRecord failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (VcfRecord, usize) =
        decode_from_slice(&decompressed).expect("decode VcfRecord failed");
    assert_eq!(vcf, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: GeneVariant – Synonymous variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_gene_variant_synonymous_roundtrip() {
    let variant = GeneVariant {
        gene_id: "ENSG00000141510".to_string(),
        transcript_id: "ENST00000269305".to_string(),
        variant_type: VariantEffect::Synonymous,
        population_frequency: 0.012,
        clinically_significant: false,
    };
    let encoded = encode_to_vec(&variant).expect("encode GeneVariant(Synonymous) failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (GeneVariant, usize) =
        decode_from_slice(&decompressed).expect("decode GeneVariant(Synonymous) failed");
    assert_eq!(variant, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: GeneVariant – Missense variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_gene_variant_missense_roundtrip() {
    let variant = GeneVariant {
        gene_id: "ENSG00000157764".to_string(),
        transcript_id: "ENST00000288602".to_string(),
        variant_type: VariantEffect::Missense {
            codon_ref: [b'G', b'T', b'G'],
            codon_alt: [b'G', b'A', b'G'],
        },
        population_frequency: 0.0001,
        clinically_significant: true,
    };
    let encoded = encode_to_vec(&variant).expect("encode GeneVariant(Missense) failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (GeneVariant, usize) =
        decode_from_slice(&decompressed).expect("decode GeneVariant(Missense) failed");
    assert_eq!(variant, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: GeneVariant – Nonsense variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_gene_variant_nonsense_roundtrip() {
    let variant = GeneVariant {
        gene_id: "ENSG00000012048".to_string(),
        transcript_id: "ENST00000357654".to_string(),
        variant_type: VariantEffect::Nonsense {
            position_in_cds: 1135,
        },
        population_frequency: 0.000001,
        clinically_significant: true,
    };
    let encoded = encode_to_vec(&variant).expect("encode GeneVariant(Nonsense) failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (GeneVariant, usize) =
        decode_from_slice(&decompressed).expect("decode GeneVariant(Nonsense) failed");
    assert_eq!(variant, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: GeneVariant – Frameshift variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_gene_variant_frameshift_roundtrip() {
    let variant = GeneVariant {
        gene_id: "ENSG00000139618".to_string(),
        transcript_id: "ENST00000380152".to_string(),
        variant_type: VariantEffect::Frameshift { net_indel: -2 },
        population_frequency: 0.000005,
        clinically_significant: true,
    };
    let encoded = encode_to_vec(&variant).expect("encode GeneVariant(Frameshift) failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (GeneVariant, usize) =
        decode_from_slice(&decompressed).expect("decode GeneVariant(Frameshift) failed");
    assert_eq!(variant, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: GeneVariant – SpliceSite variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_gene_variant_splice_site_roundtrip() {
    let variant = GeneVariant {
        gene_id: "ENSG00000185518".to_string(),
        transcript_id: "ENST00000335295".to_string(),
        variant_type: VariantEffect::SpliceSite,
        population_frequency: 0.0003,
        clinically_significant: true,
    };
    let encoded = encode_to_vec(&variant).expect("encode GeneVariant(SpliceSite) failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (GeneVariant, usize) =
        decode_from_slice(&decompressed).expect("decode GeneVariant(SpliceSite) failed");
    assert_eq!(variant, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: GeneVariant – Intronic / Intergenic variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_gene_variant_intronic_intergenic_roundtrip() {
    let intronic = GeneVariant {
        gene_id: "ENSG00000100094".to_string(),
        transcript_id: "ENST00000216181".to_string(),
        variant_type: VariantEffect::Intronic,
        population_frequency: 0.15,
        clinically_significant: false,
    };
    let intergenic = GeneVariant {
        gene_id: "intergenic".to_string(),
        transcript_id: "N/A".to_string(),
        variant_type: VariantEffect::Intergenic,
        population_frequency: 0.30,
        clinically_significant: false,
    };
    for variant in [&intronic, &intergenic] {
        let encoded = encode_to_vec(variant).expect("encode GeneVariant failed");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
        let decompressed = decompress(&compressed).expect("zstd decompress failed");
        let (decoded, _): (GeneVariant, usize) =
            decode_from_slice(&decompressed).expect("decode GeneVariant failed");
        assert_eq!(*variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 13: VcfInfoValue enum – each variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_vcf_info_value_all_variants_roundtrip() {
    let variants: Vec<VcfInfoValue> = vec![
        VcfInfoValue::Integer(42),
        VcfInfoValue::Float(3.14),
        VcfInfoValue::Flag,
        VcfInfoValue::Text("SOMATIC".to_string()),
        VcfInfoValue::IntegerVec(vec![1, 2, 3, 4, 5]),
    ];
    for v in &variants {
        let encoded = encode_to_vec(v).expect("encode VcfInfoValue failed");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
        let decompressed = decompress(&compressed).expect("zstd decompress failed");
        let (decoded, _): (VcfInfoValue, usize) =
            decode_from_slice(&decompressed).expect("decode VcfInfoValue failed");
        assert_eq!(*v, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 14: Nucleotide enum – each variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_nucleotide_enum_all_variants_roundtrip() {
    let bases = vec![
        Nucleotide::Adenine,
        Nucleotide::Cytosine,
        Nucleotide::Guanine,
        Nucleotide::Thymine,
        Nucleotide::N,
        Nucleotide::Gap,
    ];
    for base in &bases {
        let encoded = encode_to_vec(base).expect("encode Nucleotide failed");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
        let decompressed = decompress(&compressed).expect("zstd decompress failed");
        let (decoded, _): (Nucleotide, usize) =
            decode_from_slice(&decompressed).expect("decode Nucleotide failed");
        assert_eq!(*base, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 15: Large repetitive SNP data – compression ratio
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_large_snp_vec_compression_ratio() {
    let snps: Vec<SnpRecord> = (0u64..1_000)
        .map(|i| SnpRecord {
            chromosome: "chr1".to_string(),
            position: i * 1000,
            reference_allele: b'A',
            alternate_allele: b'G',
            quality_score: 30.0,
            filter_pass: true,
        })
        .collect();
    let encoded = encode_to_vec(&snps).expect("encode Vec<SnpRecord> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "zstd compressed ({} bytes) should be smaller than encoded ({} bytes) for 1000 repetitive SNPs",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<SnpRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<SnpRecord> failed");
    assert_eq!(snps, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Large repetitive NucleotideSequence – compression ratio
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_large_nucleotide_sequence_compression_ratio() {
    // Highly repetitive: ACGT repeated 1250 times = 5000 bases, maps to many identical Nucleotide enum values
    let seq = make_nucleotide_sequence(5_000);
    let encoded = encode_to_vec(&seq).expect("encode large NucleotideSequence failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "zstd compressed ({} bytes) should be smaller than encoded ({} bytes) for 5000-base repetitive sequence",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (NucleotideSequence, usize) =
        decode_from_slice(&decompressed).expect("decode large NucleotideSequence failed");
    assert_eq!(seq, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Empty NucleotideSequence (zero bases)
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_empty_nucleotide_sequence_roundtrip() {
    let seq = NucleotideSequence {
        id: "empty".to_string(),
        bases: vec![],
        length: 0,
    };
    let encoded = encode_to_vec(&seq).expect("encode empty NucleotideSequence failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (NucleotideSequence, usize) =
        decode_from_slice(&decompressed).expect("decode empty NucleotideSequence failed");
    assert_eq!(seq, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: Vec<AlignmentRecord> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_vec_alignment_records_roundtrip() {
    let records: Vec<AlignmentRecord> = (0..50)
        .map(|i| make_alignment_record(&format!("read_{i:04}")))
        .collect();
    let encoded = encode_to_vec(&records).expect("encode Vec<AlignmentRecord> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<AlignmentRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<AlignmentRecord> failed");
    assert_eq!(records, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Vec<VcfRecord> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_vec_vcf_records_roundtrip() {
    let records: Vec<VcfRecord> = (0u64..80)
        .map(|i| make_vcf_record(100_000 + i * 500))
        .collect();
    let encoded = encode_to_vec(&records).expect("encode Vec<VcfRecord> failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<VcfRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<VcfRecord> failed");
    assert_eq!(records, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Contig genome assembly roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_contig_roundtrip() {
    let contig = make_contig("chr1_scaffold_1", 2_000);
    let encoded = encode_to_vec(&contig).expect("encode Contig failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Contig, usize) =
        decode_from_slice(&decompressed).expect("decode Contig failed");
    assert_eq!(contig, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Truncated compressed data returns an error
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_truncated_data_returns_error() {
    let seq = make_nucleotide_sequence(200);
    let encoded = encode_to_vec(&seq).expect("encode NucleotideSequence failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");

    // Keep only the first quarter of the compressed bytes – guaranteed truncation.
    let truncated = &compressed[..compressed.len() / 4];
    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "decompress() must return an error for truncated zstd data, got Ok instead"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Corrupted (bit-flipped) compressed data returns an error
// ---------------------------------------------------------------------------

#[test]
fn test_zstd_corrupted_data_returns_error() {
    let snp = make_snp_record(999_999);
    let encoded = encode_to_vec(&snp).expect("encode SnpRecord failed");
    let mut compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");

    // Flip every other byte in the payload region (skip the 5-byte OxiCode header).
    let header_len = 5;
    for i in (header_len..compressed.len()).step_by(2) {
        compressed[i] ^= 0xFF;
    }

    let result = decompress(&compressed);
    assert!(
        result.is_err(),
        "decompress() must return an error for corrupted zstd data, got Ok instead"
    );
}
