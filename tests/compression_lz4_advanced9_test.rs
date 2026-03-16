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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum NucleotideBase {
    Adenine,
    Thymine,
    Cytosine,
    Guanine,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DnaSequence {
    id: u64,
    bases: Vec<NucleotideBase>,
    quality_scores: Vec<u8>,
    chromosome: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GenomicVariant {
    position: u64,
    ref_base: NucleotideBase,
    alt_base: NucleotideBase,
    quality: f32,
    sample_id: u32,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn make_dna_sequence(id: u64, chromosome: u8, len: usize) -> DnaSequence {
    let bases_cycle = [
        NucleotideBase::Adenine,
        NucleotideBase::Thymine,
        NucleotideBase::Cytosine,
        NucleotideBase::Guanine,
    ];
    let bases = (0..len)
        .map(|i| match bases_cycle[i % 4] {
            NucleotideBase::Adenine => NucleotideBase::Adenine,
            NucleotideBase::Thymine => NucleotideBase::Thymine,
            NucleotideBase::Cytosine => NucleotideBase::Cytosine,
            NucleotideBase::Guanine => NucleotideBase::Guanine,
        })
        .collect();
    let quality_scores = (0..len).map(|i| (i % 40 + 20) as u8).collect();
    DnaSequence {
        id,
        bases,
        quality_scores,
        chromosome,
    }
}

// ---------------------------------------------------------------------------
// Test 1: basic DnaSequence encode → compress → decompress → decode roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_dna_sequence_lz4_roundtrip() {
    let seq = make_dna_sequence(1, 1, 64);
    let encoded = encode_to_vec(&seq).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (DnaSequence, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(seq, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: GenomicVariant encode → compress → decompress → decode roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_genomic_variant_lz4_roundtrip() {
    let variant = GenomicVariant {
        position: 123_456_789,
        ref_base: NucleotideBase::Adenine,
        alt_base: NucleotideBase::Guanine,
        quality: 99.5,
        sample_id: 42,
    };
    let encoded = encode_to_vec(&variant).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (GenomicVariant, _) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(variant, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: large repetitive sequence achieves meaningful compression
// ---------------------------------------------------------------------------
#[test]
fn test_large_repetitive_sequence_compression_ratio() {
    // All-Adenine sequence is highly repetitive and should compress well
    let seq = DnaSequence {
        id: 2,
        bases: vec![NucleotideBase::Adenine; 10_000],
        quality_scores: vec![40u8; 10_000],
        chromosome: 3,
    };
    let encoded = encode_to_vec(&seq).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    // Compressed payload must be smaller than original encoded bytes
    assert!(
        compressed.len() < encoded.len(),
        "expected compression to reduce size: compressed={} encoded={}",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// Test 4: compression ratio > 1.0 for repetitive genomic data
// ---------------------------------------------------------------------------
#[test]
fn test_compression_ratio_value_for_repetitive_data() {
    let seq = DnaSequence {
        id: 3,
        bases: vec![NucleotideBase::Cytosine; 8_000],
        quality_scores: vec![30u8; 8_000],
        chromosome: 5,
    };
    let encoded = encode_to_vec(&seq).expect("encode failed");
    let original_size = encoded.len();
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let compressed_size = compressed.len();
    let ratio = original_size as f64 / compressed_size as f64;
    assert!(
        ratio > 1.0,
        "compression ratio should exceed 1.0 but got {:.2}",
        ratio
    );
}

// ---------------------------------------------------------------------------
// Test 5: empty DnaSequence (no bases, no quality scores)
// ---------------------------------------------------------------------------
#[test]
fn test_empty_dna_sequence_roundtrip() {
    let seq = DnaSequence {
        id: 0,
        bases: vec![],
        quality_scores: vec![],
        chromosome: 0,
    };
    let encoded = encode_to_vec(&seq).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (DnaSequence, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(seq, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: NucleotideBase::Adenine variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_nucleotide_adenine_roundtrip() {
    let base = NucleotideBase::Adenine;
    let encoded = encode_to_vec(&base).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (NucleotideBase, _) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(base, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: NucleotideBase::Thymine variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_nucleotide_thymine_roundtrip() {
    let base = NucleotideBase::Thymine;
    let encoded = encode_to_vec(&base).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (NucleotideBase, _) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(base, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: NucleotideBase::Cytosine variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_nucleotide_cytosine_roundtrip() {
    let base = NucleotideBase::Cytosine;
    let encoded = encode_to_vec(&base).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (NucleotideBase, _) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(base, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: NucleotideBase::Guanine variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_nucleotide_guanine_roundtrip() {
    let base = NucleotideBase::Guanine;
    let encoded = encode_to_vec(&base).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (NucleotideBase, _) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(base, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Vec<DnaSequence> roundtrip through LZ4
// ---------------------------------------------------------------------------
#[test]
fn test_vec_of_dna_sequences_roundtrip() {
    let sequences: Vec<DnaSequence> = (0..16)
        .map(|i| make_dna_sequence(i as u64, (i % 24) as u8, 32 + i * 4))
        .collect();
    let encoded = encode_to_vec(&sequences).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<DnaSequence>, _) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(sequences, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Vec<GenomicVariant> roundtrip through LZ4
// ---------------------------------------------------------------------------
#[test]
fn test_vec_of_genomic_variants_roundtrip() {
    let variants: Vec<GenomicVariant> = (0u64..20)
        .map(|i| {
            let ref_base = match i % 4 {
                0 => NucleotideBase::Adenine,
                1 => NucleotideBase::Thymine,
                2 => NucleotideBase::Cytosine,
                _ => NucleotideBase::Guanine,
            };
            let alt_base = match (i + 1) % 4 {
                0 => NucleotideBase::Adenine,
                1 => NucleotideBase::Thymine,
                2 => NucleotideBase::Cytosine,
                _ => NucleotideBase::Guanine,
            };
            GenomicVariant {
                position: i * 1_000,
                ref_base,
                alt_base,
                quality: i as f32 * 2.5,
                sample_id: (i % 5) as u32,
            }
        })
        .collect();
    let encoded = encode_to_vec(&variants).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<GenomicVariant>, _) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(variants, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: decompressed bytes match original encoded bytes exactly
// ---------------------------------------------------------------------------
#[test]
fn test_decompressed_matches_original_bytes() {
    let seq = make_dna_sequence(99, 7, 200);
    let encoded = encode_to_vec(&seq).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    assert_eq!(
        encoded, decompressed,
        "decompressed bytes must be identical to original encoded bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 13: idempotent encode/compress/decompress/decode cycle (double pass)
// ---------------------------------------------------------------------------
#[test]
fn test_idempotent_compress_decompress_cycle() {
    let variant = GenomicVariant {
        position: 500_000,
        ref_base: NucleotideBase::Thymine,
        alt_base: NucleotideBase::Cytosine,
        quality: 37.8,
        sample_id: 7,
    };
    // First pass
    let encoded1 = encode_to_vec(&variant).expect("encode pass 1 failed");
    let compressed1 = compress(&encoded1, Compression::Lz4).expect("compress pass 1 failed");
    let decompressed1 = decompress(&compressed1).expect("decompress pass 1 failed");
    let (decoded1, _): (GenomicVariant, _) =
        decode_from_slice(&decompressed1).expect("decode pass 1 failed");
    // Second pass — re-encode decoded value and repeat
    let encoded2 = encode_to_vec(&decoded1).expect("encode pass 2 failed");
    let compressed2 = compress(&encoded2, Compression::Lz4).expect("compress pass 2 failed");
    let decompressed2 = decompress(&compressed2).expect("decompress pass 2 failed");
    let (decoded2, _): (GenomicVariant, _) =
        decode_from_slice(&decompressed2).expect("decode pass 2 failed");
    assert_eq!(variant, decoded1);
    assert_eq!(decoded1, decoded2);
}

// ---------------------------------------------------------------------------
// Test 14: corruption detection — flipping all bytes after index 4 causes error
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_detected_after_flip() {
    let seq = make_dna_sequence(10, 2, 50);
    let encoded = encode_to_vec(&seq).expect("encode failed");
    let mut corrupted = compress(&encoded, Compression::Lz4).expect("compress failed");
    // Flip all bytes after the 5-byte header to corrupt the payload
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = decompress(&corrupted);
    assert!(
        result.is_err(),
        "expected error on corrupted compressed data but got Ok"
    );
}

// ---------------------------------------------------------------------------
// Test 15: single-base DnaSequence (length 1) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_single_base_dna_sequence_roundtrip() {
    let seq = DnaSequence {
        id: 7,
        bases: vec![NucleotideBase::Guanine],
        quality_scores: vec![38],
        chromosome: 22,
    };
    let encoded = encode_to_vec(&seq).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (DnaSequence, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(seq, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: all four NucleotideBase variants in one sequence roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_all_nucleotide_variants_in_sequence() {
    let seq = DnaSequence {
        id: 42,
        bases: vec![
            NucleotideBase::Adenine,
            NucleotideBase::Thymine,
            NucleotideBase::Cytosine,
            NucleotideBase::Guanine,
        ],
        quality_scores: vec![25, 30, 35, 40],
        chromosome: 12,
    };
    let encoded = encode_to_vec(&seq).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (DnaSequence, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(seq, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: compressed size is strictly positive (never empty)
// ---------------------------------------------------------------------------
#[test]
fn test_compressed_output_is_non_empty() {
    let seq = make_dna_sequence(55, 4, 10);
    let encoded = encode_to_vec(&seq).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    assert!(
        !compressed.is_empty(),
        "compressed output must not be empty"
    );
}

// ---------------------------------------------------------------------------
// Test 18: mixed-variant Vec<NucleotideBase> compression roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_nucleotide_bases_compression_roundtrip() {
    let bases: Vec<NucleotideBase> = (0..512)
        .map(|i| match i % 4 {
            0 => NucleotideBase::Adenine,
            1 => NucleotideBase::Thymine,
            2 => NucleotideBase::Cytosine,
            _ => NucleotideBase::Guanine,
        })
        .collect();
    let encoded = encode_to_vec(&bases).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<NucleotideBase>, _) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(bases, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: high-quality variant at chromosome boundary fields
// ---------------------------------------------------------------------------
#[test]
fn test_genomic_variant_boundary_values_roundtrip() {
    let variant = GenomicVariant {
        position: u64::MAX,
        ref_base: NucleotideBase::Adenine,
        alt_base: NucleotideBase::Thymine,
        quality: f32::MAX,
        sample_id: u32::MAX,
    };
    let encoded = encode_to_vec(&variant).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (GenomicVariant, _) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(variant, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: large diverse sequence (random-like quality scores) compresses and decodes
// ---------------------------------------------------------------------------
#[test]
fn test_large_diverse_sequence_roundtrip() {
    // Non-repetitive quality scores make this harder to compress but still correct
    let len = 5_000usize;
    let bases = (0..len)
        .map(|i| match (i * 7 + i / 3) % 4 {
            0 => NucleotideBase::Adenine,
            1 => NucleotideBase::Thymine,
            2 => NucleotideBase::Cytosine,
            _ => NucleotideBase::Guanine,
        })
        .collect();
    let quality_scores = (0..len).map(|i| ((i * 13 + 7) % 41 + 20) as u8).collect();
    let seq = DnaSequence {
        id: 999,
        bases,
        quality_scores,
        chromosome: 16,
    };
    let encoded = encode_to_vec(&seq).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (DnaSequence, _) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(seq, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: multiple chromosomes — Vec<DnaSequence> with chromosome field preserved
// ---------------------------------------------------------------------------
#[test]
fn test_multi_chromosome_sequences_preserve_chromosome_field() {
    let sequences: Vec<DnaSequence> = (1u8..=23)
        .map(|chr| make_dna_sequence(chr as u64 * 1000, chr, 48))
        .collect();
    let encoded = encode_to_vec(&sequences).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<DnaSequence>, _) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(sequences.len(), decoded.len());
    for (original, restored) in sequences.iter().zip(decoded.iter()) {
        assert_eq!(original.chromosome, restored.chromosome);
        assert_eq!(original.id, restored.id);
        assert_eq!(original, restored);
    }
}

// ---------------------------------------------------------------------------
// Test 22: partial-header corruption (truncated data) returns error on decompress
// ---------------------------------------------------------------------------
#[test]
fn test_truncated_compressed_data_returns_error() {
    let seq = make_dna_sequence(5, 9, 30);
    let encoded = encode_to_vec(&seq).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    // Truncate to only 2 bytes — far less than the 5-byte header
    let truncated = &compressed[..2];
    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "expected error on truncated compressed data but got Ok"
    );
}
