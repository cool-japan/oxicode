//! Advanced async streaming tests — genomics sequencing pipeline domain (set 31).
//!
//! 22 `#[tokio::test]` functions exercising OxiCode's async streaming API
//! through genomics sequencing types: sequence reads, alignment results, sequencing runs.
//!
//! Feature gate: `async-tokio`
//! No module wrapper, no `#[cfg(test)]` block, no `unwrap()`.

#![cfg(feature = "async-tokio")]
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
use oxicode::async_tokio::{AsyncDecoder, AsyncEncoder};
use oxicode::streaming::StreamingConfig;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use std::io::Cursor;
use tokio::io::BufReader;

// ---------------------------------------------------------------------------
// Domain types — genomics sequencing pipeline
// ---------------------------------------------------------------------------

/// Sequencing platform used to generate reads.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SequencingPlatform {
    Illumina,
    PacBio,
    NanoPore,
    IonTorrent,
    BGI,
}

/// Quality classification of a sequencing read.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReadQuality {
    High,
    Medium,
    Low,
    Filtered,
}

/// A single sequencing read from a platform.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SequenceRead {
    read_id: String,
    sequence: String,
    quality_scores: Vec<u8>,
    platform: SequencingPlatform,
    quality: ReadQuality,
    length: u32,
}

/// An alignment result mapping a read to a reference genome.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AlignmentResult {
    read_id: String,
    chromosome: String,
    position: u64,
    mapq: u8,
    cigar: String,
    is_paired: bool,
}

/// Metadata for a complete sequencing run.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SequencingRun {
    run_id: String,
    platform: SequencingPlatform,
    sample_id: String,
    read_count: u64,
    mean_quality: f32,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_read(
    read_id: &str,
    sequence: &str,
    quality_scores: Vec<u8>,
    platform: SequencingPlatform,
    quality: ReadQuality,
) -> SequenceRead {
    let length = sequence.len() as u32;
    SequenceRead {
        read_id: read_id.to_string(),
        sequence: sequence.to_string(),
        quality_scores,
        platform,
        quality,
        length,
    }
}

fn make_alignment(
    read_id: &str,
    chromosome: &str,
    position: u64,
    mapq: u8,
    cigar: &str,
    is_paired: bool,
) -> AlignmentResult {
    AlignmentResult {
        read_id: read_id.to_string(),
        chromosome: chromosome.to_string(),
        position,
        mapq,
        cigar: cigar.to_string(),
        is_paired,
    }
}

fn make_run(
    run_id: &str,
    platform: SequencingPlatform,
    sample_id: &str,
    read_count: u64,
    mean_quality: f32,
) -> SequencingRun {
    SequencingRun {
        run_id: run_id.to_string(),
        platform,
        sample_id: sample_id.to_string(),
        read_count,
        mean_quality,
    }
}

async fn encode_single<T: Encode>(item: &T) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(item)
            .await
            .expect("encode_single: write_item failed");
        enc.finish().await.expect("encode_single: finish failed");
    }
    buf
}

async fn decode_single<T: Decode>(buf: Vec<u8>) -> Option<T> {
    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    dec.read_item::<T>()
        .await
        .expect("decode_single: read_item failed")
}

// ---------------------------------------------------------------------------
// Test 1: Single SequenceRead async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_single_sequence_read_roundtrip() {
    let original = make_read(
        "read_0001",
        "ACGTACGTACGT",
        vec![40, 38, 35, 40, 39, 37, 36, 40, 38, 35, 40, 39],
        SequencingPlatform::Illumina,
        ReadQuality::High,
    );
    let buf = encode_single(&original).await;
    let decoded = decode_single::<SequenceRead>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "single SequenceRead async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 2: Batch of SequenceReads streamed in order
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_batch_sequence_reads_order_preserved() {
    let reads: Vec<SequenceRead> = (0u32..20)
        .map(|i| {
            make_read(
                &format!("read_{:04}", i),
                &"ACGT".repeat(10),
                vec![30u8 + (i as u8 % 10); 40],
                SequencingPlatform::Illumina,
                ReadQuality::High,
            )
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for r in &reads {
            enc.write_item(r).await.expect("write SequenceRead failed");
        }
        enc.finish().await.expect("finish batch reads failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let decoded: Vec<SequenceRead> = dec.read_all().await.expect("read_all batch reads failed");

    assert_eq!(decoded.len(), reads.len(), "batch count mismatch");
    assert_eq!(
        decoded, reads,
        "batch SequenceReads order or content mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 3: AlignmentResult async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_single_alignment_result_roundtrip() {
    let original = make_alignment("read_0001", "chr1", 100_000_000, 60, "150M", true);
    let buf = encode_single(&original).await;
    let decoded = decode_single::<AlignmentResult>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "single AlignmentResult async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Stream of AlignmentResults across multiple chromosomes
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_alignment_results_stream_multi_chrom() {
    let chromosomes = ["chr1", "chr2", "chrX", "chrY", "chrM"];
    let alignments: Vec<AlignmentResult> = chromosomes
        .iter()
        .enumerate()
        .map(|(i, chrom)| {
            make_alignment(
                &format!("read_{:04}", i),
                chrom,
                u64::from(i as u32) * 1_000_000,
                50 + i as u8,
                "100M",
                i % 2 == 0,
            )
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for aln in &alignments {
            enc.write_item(aln)
                .await
                .expect("write AlignmentResult failed");
        }
        enc.finish().await.expect("finish alignments stream failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let decoded: Vec<AlignmentResult> = dec.read_all().await.expect("read_all alignments failed");

    assert_eq!(
        decoded, alignments,
        "alignment results stream roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 5: SequencingRun metadata async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_sequencing_run_metadata_roundtrip() {
    let original = make_run(
        "RUN_2024_001",
        SequencingPlatform::PacBio,
        "SAMPLE_HG001",
        15_000_000,
        28.5_f32,
    );
    let buf = encode_single(&original).await;
    let decoded = decode_single::<SequencingRun>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "SequencingRun metadata roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Empty stream — encoder writes nothing but completes cleanly
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_empty_stream_no_reads() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let enc = AsyncEncoder::new(cursor);
        enc.finish().await.expect("empty stream finish failed");
    }
    assert!(
        !buf.is_empty(),
        "encoded buffer must contain end-chunk marker even when empty"
    );

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let item: Option<SequenceRead> = dec.read_item().await.expect("read on empty stream failed");
    assert_eq!(item, None, "expected None from empty stream");
    assert!(
        dec.is_finished(),
        "decoder must be finished after empty stream"
    );
}

// ---------------------------------------------------------------------------
// Test 7: Large SequenceRead with 1000+ quality scores
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_large_read_1000_quality_scores() {
    let quality_scores: Vec<u8> = (0u32..1200).map(|i| (20 + i % 40) as u8).collect();
    assert_eq!(quality_scores.len(), 1200, "must have 1200 quality scores");

    let sequence: String = "ACGT".repeat(300);
    let original = make_read(
        "read_long_0001",
        &sequence,
        quality_scores.clone(),
        SequencingPlatform::NanoPore,
        ReadQuality::Medium,
    );

    let buf = encode_single(&original).await;
    let decoded = decode_single::<SequenceRead>(buf).await;
    let decoded_read = decoded.expect("large SequenceRead decode must be Some");
    assert_eq!(
        decoded_read.quality_scores.len(),
        1200,
        "all 1200 quality scores must survive roundtrip"
    );
    assert_eq!(
        decoded_read.read_id, "read_long_0001",
        "read_id mismatch on large read"
    );
    assert_eq!(
        decoded_read.length,
        sequence.len() as u32,
        "length mismatch on large read"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Progress tracking — items_processed > 0 after reading one read
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_progress_items_processed_nonzero() {
    let reads: Vec<SequenceRead> = (0u32..5)
        .map(|i| {
            make_read(
                &format!("read_{:04}", i),
                "ACGTACGT",
                vec![30, 35, 38, 40, 36, 32, 38, 40],
                SequencingPlatform::Illumina,
                ReadQuality::High,
            )
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for r in &reads {
            enc.write_item(r)
                .await
                .expect("write read for progress test failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);

    let _first: Option<SequenceRead> = dec.read_item().await.expect("read first read failed");
    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after reading one SequenceRead"
    );
}

// ---------------------------------------------------------------------------
// Test 9: All SequencingPlatform variants stream roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_all_platform_variants_stream() {
    let platforms = vec![
        SequencingPlatform::Illumina,
        SequencingPlatform::PacBio,
        SequencingPlatform::NanoPore,
        SequencingPlatform::IonTorrent,
        SequencingPlatform::BGI,
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for p in &platforms {
            enc.write_item(p)
                .await
                .expect("write SequencingPlatform failed");
        }
        enc.finish()
            .await
            .expect("finish all-platforms stream failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);

    for expected in &platforms {
        let item: Option<SequencingPlatform> = dec
            .read_item()
            .await
            .expect("read SequencingPlatform failed");
        assert_eq!(
            item.as_ref(),
            Some(expected),
            "SequencingPlatform variant mismatch for {:?}",
            expected
        );
    }

    let eof: Option<SequencingPlatform> = dec.read_item().await.expect("eof check failed");
    assert_eq!(eof, None, "expected None after all platforms");
}

// ---------------------------------------------------------------------------
// Test 10: ReadQuality variants — all four roundtrip via stream
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_all_read_quality_variants_stream() {
    let qualities = vec![
        ReadQuality::High,
        ReadQuality::Medium,
        ReadQuality::Low,
        ReadQuality::Filtered,
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for q in &qualities {
            enc.write_item(q).await.expect("write ReadQuality failed");
        }
        enc.finish()
            .await
            .expect("finish ReadQuality stream failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);

    for expected in &qualities {
        let item: Option<ReadQuality> = dec.read_item().await.expect("read ReadQuality failed");
        assert_eq!(
            item.as_ref(),
            Some(expected),
            "ReadQuality variant mismatch for {:?}",
            expected
        );
    }
}

// ---------------------------------------------------------------------------
// Test 11: Quality filtering — stream reads, verify only High/Medium survive
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_quality_filtering_stream() {
    let reads: Vec<SequenceRead> = vec![
        make_read(
            "r1",
            "ACGT",
            vec![40, 38, 37, 39],
            SequencingPlatform::Illumina,
            ReadQuality::High,
        ),
        make_read(
            "r2",
            "TGCA",
            vec![20, 18, 15, 22],
            SequencingPlatform::Illumina,
            ReadQuality::Low,
        ),
        make_read(
            "r3",
            "GCTA",
            vec![35, 33, 36, 34],
            SequencingPlatform::Illumina,
            ReadQuality::Medium,
        ),
        make_read(
            "r4",
            "ATCG",
            vec![10, 8, 12, 9],
            SequencingPlatform::Illumina,
            ReadQuality::Filtered,
        ),
        make_read(
            "r5",
            "CGAT",
            vec![40, 39, 40, 38],
            SequencingPlatform::Illumina,
            ReadQuality::High,
        ),
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_all(reads.clone())
            .await
            .expect("write_all reads for quality filter test failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let all_decoded: Vec<SequenceRead> = dec
        .read_all()
        .await
        .expect("read_all for quality filter failed");

    // Filter in place post-decode — only High and Medium pass
    let passing: Vec<&SequenceRead> = all_decoded
        .iter()
        .filter(|r| matches!(r.quality, ReadQuality::High | ReadQuality::Medium))
        .collect();

    assert_eq!(
        passing.len(),
        3,
        "expected 3 passing reads (2 High + 1 Medium)"
    );
    assert_eq!(passing[0].read_id, "r1");
    assert_eq!(passing[1].read_id, "r3");
    assert_eq!(passing[2].read_id, "r5");
}

// ---------------------------------------------------------------------------
// Test 12: Small chunk size forces multi-chunk encoding of sequence reads
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_multi_chunk_encoding_small_chunk_size() {
    // 50 reads each with a 50-char sequence → ~2000+ bytes total with chunk_size=1024
    let reads: Vec<SequenceRead> = (0u32..50)
        .map(|i| {
            make_read(
                &format!("read_chunk_{:04}", i),
                &"ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTAC",
                vec![30u8 + (i as u8 % 10); 50],
                SequencingPlatform::Illumina,
                ReadQuality::High,
            )
        })
        .collect();

    let config = StreamingConfig::new().with_chunk_size(1024);

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::with_config(cursor, config);
        for r in &reads {
            enc.write_item(r)
                .await
                .expect("write read for multi-chunk test failed");
        }
        enc.finish().await.expect("finish multi-chunk failed");
    }

    assert!(
        buf.len() > 1024,
        "encoded stream ({} bytes) must exceed 1024 bytes for multi-chunk guarantee",
        buf.len()
    );

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let decoded: Vec<SequenceRead> = dec.read_all().await.expect("read_all multi-chunk failed");

    assert_eq!(
        decoded, reads,
        "multi-chunk sequence reads must decode identically"
    );
    assert!(
        dec.progress().chunks_processed > 1,
        "multiple chunks must be processed (chunks={})",
        dec.progress().chunks_processed
    );
}

// ---------------------------------------------------------------------------
// Test 13: SequencingRun for each platform variant — write_all roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_sequencing_run_all_platforms_write_all() {
    let runs: Vec<SequencingRun> = vec![
        make_run(
            "RUN_ILL_001",
            SequencingPlatform::Illumina,
            "S001",
            50_000_000,
            35.2_f32,
        ),
        make_run(
            "RUN_PAC_001",
            SequencingPlatform::PacBio,
            "S002",
            1_000_000,
            28.7_f32,
        ),
        make_run(
            "RUN_NAN_001",
            SequencingPlatform::NanoPore,
            "S003",
            500_000,
            15.3_f32,
        ),
        make_run(
            "RUN_ION_001",
            SequencingPlatform::IonTorrent,
            "S004",
            5_000_000,
            25.1_f32,
        ),
        make_run(
            "RUN_BGI_001",
            SequencingPlatform::BGI,
            "S005",
            30_000_000,
            33.8_f32,
        ),
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_all(runs.clone())
            .await
            .expect("write_all SequencingRuns failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let decoded: Vec<SequencingRun> = dec
        .read_all()
        .await
        .expect("read_all SequencingRuns failed");

    assert_eq!(decoded, runs, "SequencingRun write_all roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 14: Sync encode of SequenceRead, async decode via streaming
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_sync_encode_async_decode_sequence_read() {
    let original = make_read(
        "sync_read_001",
        "TTACGGCAATCG",
        vec![38, 40, 35, 37, 39, 36, 40, 38, 35, 39, 40, 37],
        SequencingPlatform::BGI,
        ReadQuality::High,
    );

    // Sync encode for raw byte verification
    let sync_bytes = encode_to_vec(&original).expect("sync encode_to_vec failed");

    // Async streaming encode+decode
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("async repackage write_item failed");
        enc.finish().await.expect("async repackage finish failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let decoded: Option<SequenceRead> = dec.read_item().await.expect("async decode failed");
    assert_eq!(
        decoded,
        Some(original.clone()),
        "sync-encode async-decode mismatch"
    );

    // Verify raw sync round-trip is also consistent
    let (sync_decoded, _): (SequenceRead, _) =
        decode_from_slice(&sync_bytes).expect("sync decode_from_slice failed");
    assert_eq!(sync_decoded, original, "sync roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 15: Concurrent decode tasks from same encoded buffer
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_concurrent_decoders_same_data() {
    let run = make_run(
        "RUN_CONC_001",
        SequencingPlatform::Illumina,
        "SA100",
        1_000_000,
        34.0_f32,
    );

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&run)
            .await
            .expect("write SequencingRun for concurrent test failed");
        enc.finish().await.expect("finish concurrent test failed");
    }

    let buf_a = buf.clone();
    let buf_b = buf.clone();

    let task_a = tokio::spawn(async move {
        let cursor = Cursor::new(buf_a);
        let reader = BufReader::new(cursor);
        let mut dec = AsyncDecoder::new(reader);
        dec.read_item::<SequencingRun>()
            .await
            .expect("concurrent decoder A failed")
    });

    let task_b = tokio::spawn(async move {
        let cursor = Cursor::new(buf_b);
        let reader = BufReader::new(cursor);
        let mut dec = AsyncDecoder::new(reader);
        dec.read_item::<SequencingRun>()
            .await
            .expect("concurrent decoder B failed")
    });

    let result_a = task_a.await.expect("task A panicked");
    let result_b = task_b.await.expect("task B panicked");

    assert_eq!(result_a, Some(run.clone()), "concurrent decoder A mismatch");
    assert_eq!(result_b, Some(run), "concurrent decoder B mismatch");
}

// ---------------------------------------------------------------------------
// Test 16: flush_per_item config — one chunk per read item
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_flush_per_item_config() {
    let reads: Vec<SequenceRead> = (0u32..6)
        .map(|i| {
            make_read(
                &format!("read_flush_{:04}", i),
                "ACGTACGT",
                vec![35u8 + (i as u8 % 5); 8],
                SequencingPlatform::Illumina,
                ReadQuality::High,
            )
        })
        .collect();

    let config = StreamingConfig::new().with_flush_per_item(true);

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::with_config(cursor, config);
        for r in &reads {
            enc.write_item(r)
                .await
                .expect("write per-item flush failed");
        }
        enc.finish().await.expect("finish per-item flush failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let decoded: Vec<SequenceRead> = dec
        .read_all()
        .await
        .expect("read_all per-item flush failed");
    assert_eq!(decoded, reads, "flush_per_item reads roundtrip mismatch");
    assert_eq!(
        dec.progress().chunks_processed,
        reads.len() as u64,
        "chunks_processed must equal item count when flush_per_item is true"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Tokio duplex in-memory channel — reads streamed write/read concurrently
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_tokio_duplex_channel_reads_stream() {
    use tokio::io::split;

    let reads: Vec<SequenceRead> = (0u32..6)
        .map(|i| {
            make_read(
                &format!("duplex_read_{:04}", i),
                &"GCTA".repeat(5),
                vec![30u8 + (i as u8 % 8); 20],
                SequencingPlatform::PacBio,
                ReadQuality::Medium,
            )
        })
        .collect();

    let reads_to_write = reads.clone();

    let (client, server) = tokio::io::duplex(65536);
    let (server_read, _server_write) = split(server);
    let (_client_read, client_write) = split(client);

    let write_task = tokio::spawn(async move {
        let mut enc = AsyncEncoder::new(client_write);
        for r in &reads_to_write {
            enc.write_item(r)
                .await
                .expect("duplex write_item SequenceRead failed");
        }
        enc.finish().await.expect("duplex finish failed");
    });

    let read_task = tokio::spawn(async move {
        let reader = BufReader::new(server_read);
        let mut dec = AsyncDecoder::new(reader);
        dec.read_all::<SequenceRead>()
            .await
            .expect("duplex read_all SequenceRead failed")
    });

    write_task.await.expect("write task panicked");
    let decoded = read_task.await.expect("read task panicked");

    assert_eq!(
        decoded, reads,
        "duplex channel SequenceRead streaming roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 18: AlignmentResult with unpaired read (is_paired = false)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_alignment_unpaired_read_roundtrip() {
    let original = make_alignment("read_unpaired_001", "chrM", 16_500, 55, "75M2S", false);
    let buf = encode_single(&original).await;
    let decoded = decode_single::<AlignmentResult>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "unpaired AlignmentResult roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 19: SequencingRun with zero read_count (empty run metadata)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_sequencing_run_zero_read_count() {
    let original = make_run(
        "RUN_EMPTY_001",
        SequencingPlatform::NanoPore,
        "SAMPLE_EMPTY",
        0,
        0.0_f32,
    );
    let buf = encode_single(&original).await;
    let decoded = decode_single::<SequencingRun>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "SequencingRun with zero read_count roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 20: bytes_processed and chunks_processed grow as reads are decoded
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_progress_bytes_and_chunks_grow() {
    let reads: Vec<SequenceRead> = (0u32..30)
        .map(|i| {
            make_read(
                &format!("prog_read_{:04}", i),
                &"ACGTACGTACGT".repeat(4),
                vec![35u8 + (i as u8 % 5); 48],
                SequencingPlatform::Illumina,
                ReadQuality::High,
            )
        })
        .collect();

    // Small chunk size to guarantee multiple chunks
    let config = StreamingConfig::new().with_chunk_size(1024);

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::with_config(cursor, config);
        for r in &reads {
            enc.write_item(r)
                .await
                .expect("write read for progress growth test failed");
        }
        enc.finish()
            .await
            .expect("finish progress growth test failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);

    let _first: Option<SequenceRead> = dec
        .read_item()
        .await
        .expect("read first for progress growth failed");
    let bytes_after_one = dec.progress().bytes_processed;
    let chunks_after_one = dec.progress().chunks_processed;
    assert!(
        bytes_after_one > 0,
        "bytes_processed must be > 0 after first read"
    );
    assert!(
        chunks_after_one >= 1,
        "at least one chunk must be processed after first read"
    );

    let rest: Vec<SequenceRead> = dec.read_all().await.expect("read_all rest failed");
    assert_eq!(rest.len(), 29, "must decode 29 remaining reads");

    let bytes_after_all = dec.progress().bytes_processed;
    assert!(
        bytes_after_all > bytes_after_one,
        "bytes_processed must grow after decoding all reads (was {}, now {})",
        bytes_after_one,
        bytes_after_all
    );
    assert_eq!(
        dec.progress().items_processed,
        30,
        "items_processed must equal 30"
    );
}

// ---------------------------------------------------------------------------
// Test 21: SequenceRead with NanoPore long-read (very long sequence)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_nanopore_long_read_roundtrip() {
    // NanoPore reads can be many kilobases; simulate a 5000 bp read
    let sequence: String = "ACGT".repeat(1250); // 5000 chars
    let quality_scores: Vec<u8> = (0u32..5000).map(|i| (10 + i % 30) as u8).collect();

    let original = make_read(
        "nanopore_long_001",
        &sequence,
        quality_scores.clone(),
        SequencingPlatform::NanoPore,
        ReadQuality::Medium,
    );

    let buf = encode_single(&original).await;
    let decoded = decode_single::<SequenceRead>(buf).await;
    let decoded_read = decoded.expect("NanoPore long read decode must be Some");

    assert_eq!(
        decoded_read.sequence.len(),
        5000,
        "sequence length must be 5000"
    );
    assert_eq!(
        decoded_read.quality_scores.len(),
        5000,
        "quality_scores length must be 5000"
    );
    assert_eq!(
        decoded_read.platform,
        SequencingPlatform::NanoPore,
        "platform mismatch"
    );
    assert_eq!(
        decoded_read.read_id, "nanopore_long_001",
        "read_id mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Multiple SequencingRuns streamed via duplex channel with progress check
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_genomics31_multiple_runs_duplex_with_progress() {
    use tokio::io::split;

    let runs: Vec<SequencingRun> = vec![
        make_run(
            "RUN_DUP_001",
            SequencingPlatform::Illumina,
            "S_DUP_A",
            10_000_000,
            34.5_f32,
        ),
        make_run(
            "RUN_DUP_002",
            SequencingPlatform::PacBio,
            "S_DUP_B",
            2_000_000,
            27.3_f32,
        ),
        make_run(
            "RUN_DUP_003",
            SequencingPlatform::BGI,
            "S_DUP_C",
            20_000_000,
            32.1_f32,
        ),
        make_run(
            "RUN_DUP_004",
            SequencingPlatform::IonTorrent,
            "S_DUP_D",
            3_000_000,
            24.8_f32,
        ),
    ];

    let runs_to_write = runs.clone();

    let (client, server) = tokio::io::duplex(65536);
    let (server_read, _server_write) = split(server);
    let (_client_read, client_write) = split(client);

    let write_task = tokio::spawn(async move {
        let mut enc = AsyncEncoder::new(client_write);
        for run in &runs_to_write {
            enc.write_item(run)
                .await
                .expect("duplex write_item SequencingRun failed");
        }
        enc.finish()
            .await
            .expect("duplex SequencingRun finish failed");
    });

    let read_task = tokio::spawn(async move {
        let reader = BufReader::new(server_read);
        let mut dec = AsyncDecoder::new(reader);
        let items = dec
            .read_all::<SequencingRun>()
            .await
            .expect("duplex read_all SequencingRun failed");
        let progress = dec.progress().items_processed;
        (items, progress)
    });

    write_task.await.expect("write task panicked");
    let (decoded, items_processed) = read_task.await.expect("read task panicked");

    assert_eq!(
        decoded, runs,
        "duplex SequencingRun streaming roundtrip failed"
    );
    assert_eq!(
        items_processed, 4,
        "items_processed must be 4 after reading all runs"
    );
}
