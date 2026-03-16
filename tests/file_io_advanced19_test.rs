//! Advanced file I/O tests for Genomics / DNA sequencing domain

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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NucleotideBase {
    A,
    T,
    C,
    G,
    N,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QualityScore {
    base: NucleotideBase,
    score: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SequenceRead {
    read_id: u64,
    sequence: Vec<QualityScore>,
    length: u32,
    is_paired: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AlignmentRecord {
    read_id: u64,
    chromosome: String,
    position: u64,
    mapping_quality: u8,
    cigar: String,
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 1: basic SequenceRead write/read roundtrip
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_sequence_read_basic_roundtrip() {
    let path = temp_dir().join("oxicode_genomics_001.bin");
    let read = SequenceRead {
        read_id: 1,
        sequence: vec![
            QualityScore {
                base: NucleotideBase::A,
                score: 40,
            },
            QualityScore {
                base: NucleotideBase::T,
                score: 38,
            },
            QualityScore {
                base: NucleotideBase::C,
                score: 35,
            },
            QualityScore {
                base: NucleotideBase::G,
                score: 37,
            },
        ],
        length: 4,
        is_paired: false,
    };
    encode_to_file(&read, &path).expect("encode SequenceRead basic failed");
    let decoded: SequenceRead = decode_from_file(&path).expect("decode SequenceRead basic failed");
    assert_eq!(read, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 2: basic AlignmentRecord write/read roundtrip
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_alignment_record_basic_roundtrip() {
    let path = temp_dir().join("oxicode_genomics_002.bin");
    let record = AlignmentRecord {
        read_id: 42,
        chromosome: "chr1".to_string(),
        position: 100_000,
        mapping_quality: 60,
        cigar: "150M".to_string(),
    };
    encode_to_file(&record, &path).expect("encode AlignmentRecord basic failed");
    let decoded: AlignmentRecord =
        decode_from_file(&path).expect("decode AlignmentRecord basic failed");
    assert_eq!(record, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 3: NucleotideBase variant A roundtrips correctly
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_nucleotide_base_variant_a() {
    let path = temp_dir().join("oxicode_genomics_003.bin");
    let base = NucleotideBase::A;
    encode_to_file(&base, &path).expect("encode NucleotideBase::A failed");
    let decoded: NucleotideBase = decode_from_file(&path).expect("decode NucleotideBase::A failed");
    assert_eq!(NucleotideBase::A, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 4: NucleotideBase variant T roundtrips correctly
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_nucleotide_base_variant_t() {
    let path = temp_dir().join("oxicode_genomics_004.bin");
    let base = NucleotideBase::T;
    encode_to_file(&base, &path).expect("encode NucleotideBase::T failed");
    let decoded: NucleotideBase = decode_from_file(&path).expect("decode NucleotideBase::T failed");
    assert_eq!(NucleotideBase::T, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 5: NucleotideBase variant C roundtrips correctly
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_nucleotide_base_variant_c() {
    let path = temp_dir().join("oxicode_genomics_005.bin");
    let base = NucleotideBase::C;
    encode_to_file(&base, &path).expect("encode NucleotideBase::C failed");
    let decoded: NucleotideBase = decode_from_file(&path).expect("decode NucleotideBase::C failed");
    assert_eq!(NucleotideBase::C, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 6: NucleotideBase variant G roundtrips correctly
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_nucleotide_base_variant_g() {
    let path = temp_dir().join("oxicode_genomics_006.bin");
    let base = NucleotideBase::G;
    encode_to_file(&base, &path).expect("encode NucleotideBase::G failed");
    let decoded: NucleotideBase = decode_from_file(&path).expect("decode NucleotideBase::G failed");
    assert_eq!(NucleotideBase::G, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 7: NucleotideBase variant N (ambiguous) roundtrips correctly
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_nucleotide_base_variant_n() {
    let path = temp_dir().join("oxicode_genomics_007.bin");
    let base = NucleotideBase::N;
    encode_to_file(&base, &path).expect("encode NucleotideBase::N failed");
    let decoded: NucleotideBase = decode_from_file(&path).expect("decode NucleotideBase::N failed");
    assert_eq!(NucleotideBase::N, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 8: large SequenceRead with 1000 bases
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_sequence_read_large_1000_bases() {
    let path = temp_dir().join("oxicode_genomics_008.bin");
    let bases = [
        NucleotideBase::A,
        NucleotideBase::T,
        NucleotideBase::C,
        NucleotideBase::G,
        NucleotideBase::N,
    ];
    let sequence: Vec<QualityScore> = (0u32..1000)
        .map(|i| QualityScore {
            base: bases[(i % 5) as usize].clone(),
            score: (i % 40 + 1) as u8,
        })
        .collect();
    let read = SequenceRead {
        read_id: 1000,
        sequence,
        length: 1000,
        is_paired: true,
    };
    encode_to_file(&read, &path).expect("encode large 1000-base SequenceRead failed");
    let decoded: SequenceRead =
        decode_from_file(&path).expect("decode large 1000-base SequenceRead failed");
    assert_eq!(decoded.sequence.len(), 1000);
    assert_eq!(decoded.length, 1000);
    assert_eq!(read, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 9: file bytes match encode_to_vec for SequenceRead
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_sequence_read_file_bytes_match_encode_to_vec() {
    let path = temp_dir().join("oxicode_genomics_009.bin");
    let read = SequenceRead {
        read_id: 99,
        sequence: vec![
            QualityScore {
                base: NucleotideBase::G,
                score: 30,
            },
            QualityScore {
                base: NucleotideBase::N,
                score: 5,
            },
        ],
        length: 2,
        is_paired: false,
    };
    encode_to_file(&read, &path).expect("encode SequenceRead for bytes check failed");
    let file_bytes = std::fs::read(&path).expect("read SequenceRead file bytes failed");
    let vec_bytes = encode_to_vec(&read).expect("encode_to_vec SequenceRead failed");
    assert_eq!(file_bytes, vec_bytes);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 10: file bytes match encode_to_vec for AlignmentRecord
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_alignment_record_file_bytes_match_encode_to_vec() {
    let path = temp_dir().join("oxicode_genomics_010.bin");
    let record = AlignmentRecord {
        read_id: 77,
        chromosome: "chrX".to_string(),
        position: 5_000_000,
        mapping_quality: 255,
        cigar: "100M2I48M".to_string(),
    };
    encode_to_file(&record, &path).expect("encode AlignmentRecord for bytes check failed");
    let file_bytes = std::fs::read(&path).expect("read AlignmentRecord file bytes failed");
    let vec_bytes = encode_to_vec(&record).expect("encode_to_vec AlignmentRecord failed");
    assert_eq!(file_bytes, vec_bytes);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 11: overwrite SequenceRead file produces new value on decode
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_sequence_read_overwrite() {
    let path = temp_dir().join("oxicode_genomics_011.bin");
    let first = SequenceRead {
        read_id: 1,
        sequence: vec![QualityScore {
            base: NucleotideBase::A,
            score: 20,
        }],
        length: 1,
        is_paired: false,
    };
    let second = SequenceRead {
        read_id: 999,
        sequence: vec![
            QualityScore {
                base: NucleotideBase::T,
                score: 10,
            },
            QualityScore {
                base: NucleotideBase::C,
                score: 15,
            },
        ],
        length: 2,
        is_paired: true,
    };
    encode_to_file(&first, &path).expect("first encode overwrite SequenceRead failed");
    encode_to_file(&second, &path).expect("second encode overwrite SequenceRead failed");
    let decoded: SequenceRead =
        decode_from_file(&path).expect("decode overwrite SequenceRead failed");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 12: overwrite AlignmentRecord file produces new value on decode
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_alignment_record_overwrite() {
    let path = temp_dir().join("oxicode_genomics_012.bin");
    let first = AlignmentRecord {
        read_id: 1,
        chromosome: "chr1".to_string(),
        position: 1000,
        mapping_quality: 10,
        cigar: "50M".to_string(),
    };
    let second = AlignmentRecord {
        read_id: 8888,
        chromosome: "chrY".to_string(),
        position: 99_999_999,
        mapping_quality: 0,
        cigar: "75M5D20M".to_string(),
    };
    encode_to_file(&first, &path).expect("first encode overwrite AlignmentRecord failed");
    encode_to_file(&second, &path).expect("second encode overwrite AlignmentRecord failed");
    let decoded: AlignmentRecord =
        decode_from_file(&path).expect("decode overwrite AlignmentRecord failed");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 13: error on missing file for SequenceRead
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_error_on_missing_file_sequence_read() {
    let path = temp_dir().join("oxicode_genomics_013_missing_xyz.bin");
    // Ensure it does not exist
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
    let result = decode_from_file::<SequenceRead>(&path);
    assert!(
        result.is_err(),
        "Expected error decoding from non-existent file"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 14: error on missing file for AlignmentRecord
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_error_on_missing_file_alignment_record() {
    let path = temp_dir().join("oxicode_genomics_014_missing_xyz.bin");
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
    let result = decode_from_file::<AlignmentRecord>(&path);
    assert!(
        result.is_err(),
        "Expected error decoding AlignmentRecord from non-existent file"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 15: multiple SequenceRead files written and read independently
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_multiple_sequence_read_files() {
    let reads = vec![
        SequenceRead {
            read_id: 10,
            sequence: vec![QualityScore {
                base: NucleotideBase::A,
                score: 40,
            }],
            length: 1,
            is_paired: false,
        },
        SequenceRead {
            read_id: 20,
            sequence: vec![
                QualityScore {
                    base: NucleotideBase::T,
                    score: 30,
                },
                QualityScore {
                    base: NucleotideBase::G,
                    score: 28,
                },
            ],
            length: 2,
            is_paired: false,
        },
        SequenceRead {
            read_id: 30,
            sequence: vec![
                QualityScore {
                    base: NucleotideBase::C,
                    score: 25,
                },
                QualityScore {
                    base: NucleotideBase::N,
                    score: 5,
                },
                QualityScore {
                    base: NucleotideBase::A,
                    score: 35,
                },
            ],
            length: 3,
            is_paired: true,
        },
    ];
    let paths: Vec<_> = (0..3)
        .map(|n| temp_dir().join(format!("oxicode_genomics_015_{n}.bin")))
        .collect();

    for (read, path) in reads.iter().zip(paths.iter()) {
        encode_to_file(read, path).expect("encode multi-file SequenceRead failed");
    }
    for (read, path) in reads.iter().zip(paths.iter()) {
        let decoded: SequenceRead =
            decode_from_file(path).expect("decode multi-file SequenceRead failed");
        assert_eq!(*read, decoded);
        if path.exists() {
            std::fs::remove_file(path).ok();
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 16: multiple AlignmentRecord files written and read independently
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_multiple_alignment_record_files() {
    let records = vec![
        AlignmentRecord {
            read_id: 101,
            chromosome: "chr1".to_string(),
            position: 1_000_000,
            mapping_quality: 60,
            cigar: "100M".to_string(),
        },
        AlignmentRecord {
            read_id: 102,
            chromosome: "chr2".to_string(),
            position: 2_000_000,
            mapping_quality: 30,
            cigar: "50M5I45M".to_string(),
        },
        AlignmentRecord {
            read_id: 103,
            chromosome: "chrM".to_string(),
            position: 500,
            mapping_quality: 0,
            cigar: "150M".to_string(),
        },
    ];
    let paths: Vec<_> = (0..3)
        .map(|n| temp_dir().join(format!("oxicode_genomics_016_{n}.bin")))
        .collect();

    for (record, path) in records.iter().zip(paths.iter()) {
        encode_to_file(record, path).expect("encode multi-file AlignmentRecord failed");
    }
    for (record, path) in records.iter().zip(paths.iter()) {
        let decoded: AlignmentRecord =
            decode_from_file(path).expect("decode multi-file AlignmentRecord failed");
        assert_eq!(*record, decoded);
        if path.exists() {
            std::fs::remove_file(path).ok();
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 17: QualityScore with minimum (0) and maximum (255) scores
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_quality_score_boundary_values() {
    let path = temp_dir().join("oxicode_genomics_017.bin");
    let sequence = vec![
        QualityScore {
            base: NucleotideBase::A,
            score: 0,
        },
        QualityScore {
            base: NucleotideBase::T,
            score: 255,
        },
        QualityScore {
            base: NucleotideBase::C,
            score: 0,
        },
        QualityScore {
            base: NucleotideBase::G,
            score: 255,
        },
        QualityScore {
            base: NucleotideBase::N,
            score: 0,
        },
    ];
    encode_to_file(&sequence, &path).expect("encode QualityScore boundary values failed");
    let decoded: Vec<QualityScore> =
        decode_from_file(&path).expect("decode QualityScore boundary values failed");
    assert_eq!(decoded[0].score, 0);
    assert_eq!(decoded[1].score, 255);
    assert_eq!(sequence, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 18: SequenceRead with empty sequence (zero-length read)
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_sequence_read_empty_sequence() {
    let path = temp_dir().join("oxicode_genomics_018.bin");
    let read = SequenceRead {
        read_id: 0,
        sequence: vec![],
        length: 0,
        is_paired: false,
    };
    encode_to_file(&read, &path).expect("encode empty SequenceRead failed");
    let decoded: SequenceRead = decode_from_file(&path).expect("decode empty SequenceRead failed");
    assert!(decoded.sequence.is_empty());
    assert_eq!(decoded.length, 0);
    assert_eq!(read, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 19: decode_from_slice matches decode_from_file for SequenceRead
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_decode_from_slice_matches_file_sequence_read() {
    let path = temp_dir().join("oxicode_genomics_019.bin");
    let read = SequenceRead {
        read_id: 512,
        sequence: vec![
            QualityScore {
                base: NucleotideBase::G,
                score: 37,
            },
            QualityScore {
                base: NucleotideBase::C,
                score: 32,
            },
            QualityScore {
                base: NucleotideBase::T,
                score: 29,
            },
        ],
        length: 3,
        is_paired: true,
    };
    encode_to_file(&read, &path).expect("encode SequenceRead for slice compare failed");
    let file_bytes = std::fs::read(&path).expect("read SequenceRead file for slice compare failed");
    let (slice_decoded, _): (SequenceRead, _) =
        decode_from_slice(&file_bytes).expect("decode_from_slice SequenceRead failed");
    let file_decoded: SequenceRead =
        decode_from_file(&path).expect("decode_from_file SequenceRead failed");
    assert_eq!(slice_decoded, file_decoded);
    assert_eq!(read, slice_decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 20: decode_from_slice matches decode_from_file for AlignmentRecord
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_decode_from_slice_matches_file_alignment_record() {
    let path = temp_dir().join("oxicode_genomics_020.bin");
    let record = AlignmentRecord {
        read_id: 256,
        chromosome: "chr22".to_string(),
        position: 33_333_333,
        mapping_quality: 45,
        cigar: "30M3D117M".to_string(),
    };
    encode_to_file(&record, &path).expect("encode AlignmentRecord for slice compare failed");
    let file_bytes =
        std::fs::read(&path).expect("read AlignmentRecord file for slice compare failed");
    let (slice_decoded, _): (AlignmentRecord, _) =
        decode_from_slice(&file_bytes).expect("decode_from_slice AlignmentRecord failed");
    let file_decoded: AlignmentRecord =
        decode_from_file(&path).expect("decode_from_file AlignmentRecord failed");
    assert_eq!(slice_decoded, file_decoded);
    assert_eq!(record, slice_decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 21: paired-end SequenceRead flag preserved correctly
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_sequence_read_paired_end_flag() {
    let path_paired = temp_dir().join("oxicode_genomics_021a.bin");
    let path_unpaired = temp_dir().join("oxicode_genomics_021b.bin");

    let paired = SequenceRead {
        read_id: 1001,
        sequence: vec![QualityScore {
            base: NucleotideBase::A,
            score: 40,
        }],
        length: 1,
        is_paired: true,
    };
    let unpaired = SequenceRead {
        read_id: 1002,
        sequence: vec![QualityScore {
            base: NucleotideBase::T,
            score: 38,
        }],
        length: 1,
        is_paired: false,
    };

    encode_to_file(&paired, &path_paired).expect("encode paired SequenceRead failed");
    encode_to_file(&unpaired, &path_unpaired).expect("encode unpaired SequenceRead failed");

    let decoded_paired: SequenceRead =
        decode_from_file(&path_paired).expect("decode paired SequenceRead failed");
    let decoded_unpaired: SequenceRead =
        decode_from_file(&path_unpaired).expect("decode unpaired SequenceRead failed");

    assert!(decoded_paired.is_paired, "Expected is_paired=true");
    assert!(!decoded_unpaired.is_paired, "Expected is_paired=false");
    assert_eq!(paired, decoded_paired);
    assert_eq!(unpaired, decoded_unpaired);

    if path_paired.exists() {
        std::fs::remove_file(&path_paired).ok();
    }
    if path_unpaired.exists() {
        std::fs::remove_file(&path_unpaired).ok();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 22: large batch of AlignmentRecords — file and vec encoding consistency
// ──────────────────────────────────────────────────────────────────────────────
#[test]
fn test_large_alignment_batch_file_and_vec_consistency() {
    let path = temp_dir().join("oxicode_genomics_022.bin");
    let chromosomes = ["chr1", "chr2", "chr3", "chrX", "chrY", "chrM"];
    let records: Vec<AlignmentRecord> = (0u64..400)
        .map(|i| AlignmentRecord {
            read_id: i,
            chromosome: chromosomes[(i % 6) as usize].to_string(),
            position: (i + 1) * 1_000,
            mapping_quality: (i % 256) as u8,
            cigar: format!("{}M", 100 + i % 51),
        })
        .collect();

    encode_to_file(&records, &path).expect("encode large AlignmentRecord batch to file failed");
    let file_bytes = std::fs::read(&path).expect("read large AlignmentRecord batch file failed");
    let vec_bytes =
        encode_to_vec(&records).expect("encode_to_vec large AlignmentRecord batch failed");
    assert_eq!(
        file_bytes, vec_bytes,
        "file and vec encodings must be identical for large AlignmentRecord batch"
    );
    let decoded: Vec<AlignmentRecord> =
        decode_from_file(&path).expect("decode large AlignmentRecord batch from file failed");
    assert_eq!(records.len(), decoded.len());
    assert_eq!(records, decoded);
    if path.exists() {
        std::fs::remove_file(&path).ok();
    }
}
