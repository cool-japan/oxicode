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

// --- Domain Enums ---

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BatchStatus {
    InProgress,
    Completed,
    Failed,
    Quarantine,
    Released,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum QualityDecision {
    Accept,
    Reject,
    ReTest,
    Hold,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ProcessStep {
    Weighing,
    Mixing,
    Granulation,
    Drying,
    Compression,
    Coating,
    Packaging,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EquipmentStatus {
    Idle,
    Running,
    Cleaning,
    Maintenance,
    OOS,
}

// --- Domain Structs ---

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatchRecord {
    batch_id: u64,
    product_code: String,
    status: BatchStatus,
    start_time: u64,
    end_time: Option<u64>,
    yield_kg_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InProcessTest {
    test_id: u64,
    batch_id: u64,
    step: ProcessStep,
    parameter: String,
    result_x1000: i32,
    limit_lo_x1000: i32,
    limit_hi_x1000: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EquipmentLog {
    log_id: u64,
    equipment_id: u32,
    status: EquipmentStatus,
    operator_id: u32,
    timestamp: u64,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QualityEvent {
    event_id: u64,
    batch_id: u64,
    decision: QualityDecision,
    test_count: u16,
    failures: u16,
    reviewer_id: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IngredientsUsage {
    usage_id: u64,
    batch_id: u64,
    ingredient_code: String,
    lot_number: String,
    quantity_g_x100: u64,
    tare_g_x100: u32,
}

// --- Test 1: BatchRecord roundtrip with Completed status ---
#[test]
fn test_batch_record_completed_roundtrip() {
    let record = BatchRecord {
        batch_id: 100001,
        product_code: "AMOX-500-CAP".to_string(),
        status: BatchStatus::Completed,
        start_time: 1_700_000_000,
        end_time: Some(1_700_003_600),
        yield_kg_x100: 5000,
    };

    let encoded = encode_to_vec(&record).expect("encode BatchRecord Completed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress BatchRecord Completed");
    let decompressed = decompress(&compressed).expect("decompress BatchRecord Completed");
    let (decoded, _): (BatchRecord, usize) =
        decode_from_slice(&decompressed).expect("decode BatchRecord Completed");

    assert_eq!(record, decoded);
}

// --- Test 2: BatchRecord roundtrip with InProgress status and no end_time ---
#[test]
fn test_batch_record_in_progress_no_end_time() {
    let record = BatchRecord {
        batch_id: 100002,
        product_code: "IBUPROFEN-200-TAB".to_string(),
        status: BatchStatus::InProgress,
        start_time: 1_700_010_000,
        end_time: None,
        yield_kg_x100: 0,
    };

    let encoded = encode_to_vec(&record).expect("encode BatchRecord InProgress");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress BatchRecord InProgress");
    let decompressed = decompress(&compressed).expect("decompress BatchRecord InProgress");
    let (decoded, _): (BatchRecord, usize) =
        decode_from_slice(&decompressed).expect("decode BatchRecord InProgress");

    assert_eq!(record, decoded);
    assert_eq!(decoded.end_time, None);
}

// --- Test 3: BatchRecord with Failed status ---
#[test]
fn test_batch_record_failed_status() {
    let record = BatchRecord {
        batch_id: 100003,
        product_code: "METFORMIN-850-TAB".to_string(),
        status: BatchStatus::Failed,
        start_time: 1_700_020_000,
        end_time: Some(1_700_021_500),
        yield_kg_x100: 0,
    };

    let encoded = encode_to_vec(&record).expect("encode BatchRecord Failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress BatchRecord Failed");
    let decompressed = decompress(&compressed).expect("decompress BatchRecord Failed");
    let (decoded, _): (BatchRecord, usize) =
        decode_from_slice(&decompressed).expect("decode BatchRecord Failed");

    assert_eq!(record, decoded);
    assert_eq!(decoded.status, BatchStatus::Failed);
}

// --- Test 4: BatchRecord with Quarantine status ---
#[test]
fn test_batch_record_quarantine_status() {
    let record = BatchRecord {
        batch_id: 100004,
        product_code: "ASPIRIN-100-EC-TAB".to_string(),
        status: BatchStatus::Quarantine,
        start_time: 1_700_030_000,
        end_time: Some(1_700_035_000),
        yield_kg_x100: 4800,
    };

    let encoded = encode_to_vec(&record).expect("encode BatchRecord Quarantine");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress BatchRecord Quarantine");
    let decompressed = decompress(&compressed).expect("decompress BatchRecord Quarantine");
    let (decoded, _): (BatchRecord, usize) =
        decode_from_slice(&decompressed).expect("decode BatchRecord Quarantine");

    assert_eq!(record, decoded);
    assert_eq!(decoded.status, BatchStatus::Quarantine);
}

// --- Test 5: BatchRecord with Released status ---
#[test]
fn test_batch_record_released_status() {
    let record = BatchRecord {
        batch_id: 100005,
        product_code: "PARACETAMOL-500-TAB".to_string(),
        status: BatchStatus::Released,
        start_time: 1_700_040_000,
        end_time: Some(1_700_046_000),
        yield_kg_x100: 9950,
    };

    let encoded = encode_to_vec(&record).expect("encode BatchRecord Released");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress BatchRecord Released");
    let decompressed = decompress(&compressed).expect("decompress BatchRecord Released");
    let (decoded, _): (BatchRecord, usize) =
        decode_from_slice(&decompressed).expect("decode BatchRecord Released");

    assert_eq!(record, decoded);
    assert_eq!(decoded.status, BatchStatus::Released);
}

// --- Test 6: InProcessTest roundtrip for Granulation step ---
#[test]
fn test_in_process_test_granulation_roundtrip() {
    let test_entry = InProcessTest {
        test_id: 200001,
        batch_id: 100001,
        step: ProcessStep::Granulation,
        parameter: "moisture_content_pct".to_string(),
        result_x1000: 2500,
        limit_lo_x1000: 1000,
        limit_hi_x1000: 4000,
    };

    let encoded = encode_to_vec(&test_entry).expect("encode InProcessTest Granulation");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress InProcessTest Granulation");
    let decompressed = decompress(&compressed).expect("decompress InProcessTest Granulation");
    let (decoded, _): (InProcessTest, usize) =
        decode_from_slice(&decompressed).expect("decode InProcessTest Granulation");

    assert_eq!(test_entry, decoded);
    assert_eq!(decoded.step, ProcessStep::Granulation);
}

// --- Test 7: InProcessTest for Compression step with result at lower limit ---
#[test]
fn test_in_process_test_compression_step_at_lower_limit() {
    let test_entry = InProcessTest {
        test_id: 200002,
        batch_id: 100001,
        step: ProcessStep::Compression,
        parameter: "hardness_N".to_string(),
        result_x1000: 50000,
        limit_lo_x1000: 50000,
        limit_hi_x1000: 150000,
    };

    let encoded =
        encode_to_vec(&test_entry).expect("encode InProcessTest Compression at lower limit");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress InProcessTest Compression step");
    let decompressed = decompress(&compressed).expect("decompress InProcessTest Compression step");
    let (decoded, _): (InProcessTest, usize) =
        decode_from_slice(&decompressed).expect("decode InProcessTest Compression step");

    assert_eq!(test_entry, decoded);
    assert_eq!(decoded.result_x1000, decoded.limit_lo_x1000);
}

// --- Test 8: InProcessTest covering all ProcessStep variants ---
#[test]
fn test_all_process_steps_encode_decode() {
    let steps = vec![
        ProcessStep::Weighing,
        ProcessStep::Mixing,
        ProcessStep::Granulation,
        ProcessStep::Drying,
        ProcessStep::Compression,
        ProcessStep::Coating,
        ProcessStep::Packaging,
    ];

    for (idx, step) in steps.into_iter().enumerate() {
        let test_entry = InProcessTest {
            test_id: 200010 + idx as u64,
            batch_id: 100010,
            step: step.clone(),
            parameter: "test_param".to_string(),
            result_x1000: 1000 * (idx as i32 + 1),
            limit_lo_x1000: 500,
            limit_hi_x1000: 10000,
        };

        let encoded = encode_to_vec(&test_entry).expect("encode InProcessTest all steps");
        let compressed =
            compress(&encoded, Compression::Lz4).expect("compress InProcessTest all steps");
        let decompressed = decompress(&compressed).expect("decompress InProcessTest all steps");
        let (decoded, _): (InProcessTest, usize) =
            decode_from_slice(&decompressed).expect("decode InProcessTest all steps");

        assert_eq!(test_entry, decoded);
        assert_eq!(decoded.step, step);
    }
}

// --- Test 9: EquipmentLog roundtrip for Cleaning status ---
#[test]
fn test_equipment_log_cleaning_roundtrip() {
    let log = EquipmentLog {
        log_id: 300001,
        equipment_id: 42,
        status: EquipmentStatus::Cleaning,
        operator_id: 7001,
        timestamp: 1_700_050_000,
        notes: "Post-batch CIP cycle completed per SOP-EQ-042".to_string(),
    };

    let encoded = encode_to_vec(&log).expect("encode EquipmentLog Cleaning");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress EquipmentLog Cleaning");
    let decompressed = decompress(&compressed).expect("decompress EquipmentLog Cleaning");
    let (decoded, _): (EquipmentLog, usize) =
        decode_from_slice(&decompressed).expect("decode EquipmentLog Cleaning");

    assert_eq!(log, decoded);
    assert_eq!(decoded.status, EquipmentStatus::Cleaning);
}

// --- Test 10: EquipmentLog covering all EquipmentStatus variants ---
#[test]
fn test_all_equipment_statuses_encode_decode() {
    let statuses = vec![
        EquipmentStatus::Idle,
        EquipmentStatus::Running,
        EquipmentStatus::Cleaning,
        EquipmentStatus::Maintenance,
        EquipmentStatus::OOS,
    ];

    for (idx, status) in statuses.into_iter().enumerate() {
        let log = EquipmentLog {
            log_id: 300010 + idx as u64,
            equipment_id: 10 + idx as u32,
            status: status.clone(),
            operator_id: 7000 + idx as u32,
            timestamp: 1_700_060_000 + idx as u64 * 3600,
            notes: format!("Status log entry {}", idx),
        };

        let encoded = encode_to_vec(&log).expect("encode EquipmentLog all statuses");
        let compressed =
            compress(&encoded, Compression::Lz4).expect("compress EquipmentLog all statuses");
        let decompressed = decompress(&compressed).expect("decompress EquipmentLog all statuses");
        let (decoded, _): (EquipmentLog, usize) =
            decode_from_slice(&decompressed).expect("decode EquipmentLog all statuses");

        assert_eq!(log, decoded);
        assert_eq!(decoded.status, status);
    }
}

// --- Test 11: QualityEvent roundtrip with Accept decision ---
#[test]
fn test_quality_event_accept_roundtrip() {
    let event = QualityEvent {
        event_id: 400001,
        batch_id: 100005,
        decision: QualityDecision::Accept,
        test_count: 48,
        failures: 0,
        reviewer_id: 9001,
    };

    let encoded = encode_to_vec(&event).expect("encode QualityEvent Accept");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress QualityEvent Accept");
    let decompressed = decompress(&compressed).expect("decompress QualityEvent Accept");
    let (decoded, _): (QualityEvent, usize) =
        decode_from_slice(&decompressed).expect("decode QualityEvent Accept");

    assert_eq!(event, decoded);
    assert_eq!(decoded.failures, 0);
}

// --- Test 12: QualityEvent covering all QualityDecision variants ---
#[test]
fn test_all_quality_decisions_encode_decode() {
    let decisions = vec![
        QualityDecision::Accept,
        QualityDecision::Reject,
        QualityDecision::ReTest,
        QualityDecision::Hold,
    ];

    for (idx, decision) in decisions.into_iter().enumerate() {
        let event = QualityEvent {
            event_id: 400010 + idx as u64,
            batch_id: 100020 + idx as u64,
            decision: decision.clone(),
            test_count: 24 + idx as u16,
            failures: idx as u16,
            reviewer_id: 9000 + idx as u32,
        };

        let encoded = encode_to_vec(&event).expect("encode QualityEvent all decisions");
        let compressed =
            compress(&encoded, Compression::Lz4).expect("compress QualityEvent all decisions");
        let decompressed = decompress(&compressed).expect("decompress QualityEvent all decisions");
        let (decoded, _): (QualityEvent, usize) =
            decode_from_slice(&decompressed).expect("decode QualityEvent all decisions");

        assert_eq!(event, decoded);
        assert_eq!(decoded.decision, decision);
    }
}

// --- Test 13: IngredientsUsage roundtrip ---
#[test]
fn test_ingredients_usage_roundtrip() {
    let usage = IngredientsUsage {
        usage_id: 500001,
        batch_id: 100001,
        ingredient_code: "AMOXICILLIN-TRIHYDRATE-EP".to_string(),
        lot_number: "RAW-2024-001-LOT-7".to_string(),
        quantity_g_x100: 50_000_000,
        tare_g_x100: 25_000,
    };

    let encoded = encode_to_vec(&usage).expect("encode IngredientsUsage");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress IngredientsUsage");
    let decompressed = decompress(&compressed).expect("decompress IngredientsUsage");
    let (decoded, _): (IngredientsUsage, usize) =
        decode_from_slice(&decompressed).expect("decode IngredientsUsage");

    assert_eq!(usage, decoded);
    assert_eq!(decoded.ingredient_code, "AMOXICILLIN-TRIHYDRATE-EP");
}

// --- Test 14: Verify compressed bytes differ from original encoded bytes ---
#[test]
fn test_compressed_differs_from_encoded() {
    let record = BatchRecord {
        batch_id: 100050,
        product_code: "CEFUROXIME-250-TAB".to_string(),
        status: BatchStatus::Completed,
        start_time: 1_700_100_000,
        end_time: Some(1_700_108_000),
        yield_kg_x100: 7500,
    };

    let encoded = encode_to_vec(&record).expect("encode for diff check");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress for diff check");

    assert_ne!(
        encoded, compressed,
        "Compressed output must differ from raw encoded bytes"
    );
}

// --- Test 15: Verify decompressed length matches original encoded length ---
#[test]
fn test_decompressed_length_matches_original() {
    let log = EquipmentLog {
        log_id: 300100,
        equipment_id: 55,
        status: EquipmentStatus::Maintenance,
        operator_id: 7050,
        timestamp: 1_700_200_000,
        notes: "Scheduled PM per maintenance plan MPL-2024-Q4".to_string(),
    };

    let encoded = encode_to_vec(&log).expect("encode for length check");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress for length check");
    let decompressed = decompress(&compressed).expect("decompress for length check");

    assert_eq!(
        encoded.len(),
        decompressed.len(),
        "Decompressed length must match original encoded length"
    );
}

// --- Test 16: Option<u64> end_time — Some case explicit check ---
#[test]
fn test_batch_record_option_end_time_some() {
    let record = BatchRecord {
        batch_id: 100060,
        product_code: "OMEPRAZOLE-20-CAP".to_string(),
        status: BatchStatus::Completed,
        start_time: 1_700_300_000,
        end_time: Some(1_700_310_800),
        yield_kg_x100: 6300,
    };

    let encoded = encode_to_vec(&record).expect("encode option some end_time");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress option some end_time");
    let decompressed = decompress(&compressed).expect("decompress option some end_time");
    let (decoded, _): (BatchRecord, usize) =
        decode_from_slice(&decompressed).expect("decode option some end_time");

    assert_eq!(decoded.end_time, Some(1_700_310_800));
}

// --- Test 17: Option<u64> end_time — None case explicit check ---
#[test]
fn test_batch_record_option_end_time_none() {
    let record = BatchRecord {
        batch_id: 100061,
        product_code: "ATORVASTATIN-40-TAB".to_string(),
        status: BatchStatus::InProgress,
        start_time: 1_700_320_000,
        end_time: None,
        yield_kg_x100: 0,
    };

    let encoded = encode_to_vec(&record).expect("encode option none end_time");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress option none end_time");
    let decompressed = decompress(&compressed).expect("decompress option none end_time");
    let (decoded, _): (BatchRecord, usize) =
        decode_from_slice(&decompressed).expect("decode option none end_time");

    assert_eq!(decoded.end_time, None);
    assert_eq!(decoded.status, BatchStatus::InProgress);
}

// --- Test 18: Multiple compress/decompress cycles preserve data integrity ---
#[test]
fn test_multiple_compress_decompress_cycles() {
    let event = QualityEvent {
        event_id: 400100,
        batch_id: 100070,
        decision: QualityDecision::ReTest,
        test_count: 32,
        failures: 3,
        reviewer_id: 9010,
    };

    let encoded = encode_to_vec(&event).expect("encode for multi-cycle");
    let mut data = encoded.clone();

    for cycle in 0..3_usize {
        let compressed = compress(&data, Compression::Lz4)
            .unwrap_or_else(|_| panic!("compress cycle {}", cycle));
        let decompressed =
            decompress(&compressed).unwrap_or_else(|_| panic!("decompress cycle {}", cycle));
        data = decompressed;
    }

    let (decoded, _): (QualityEvent, usize) =
        decode_from_slice(&data).expect("decode after multi-cycle");
    assert_eq!(event, decoded);
}

// --- Test 19: Large batch record archive — compression ratio test (1000+ records) ---
#[test]
fn test_large_batch_record_archive_compression_ratio() {
    let mut records: Vec<BatchRecord> = Vec::with_capacity(1000);
    for i in 0..1000_u64 {
        records.push(BatchRecord {
            batch_id: 200000 + i,
            product_code: "GENERIC-TABLET-500MG".to_string(),
            status: BatchStatus::Completed,
            start_time: 1_700_000_000 + i * 3600,
            end_time: Some(1_700_003_600 + i * 3600),
            yield_kg_x100: 5000,
        });
    }

    let encoded = encode_to_vec(&records).expect("encode large batch archive");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress large batch archive");
    let decompressed = decompress(&compressed).expect("decompress large batch archive");
    let (decoded, _): (Vec<BatchRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode large batch archive");

    assert_eq!(records.len(), decoded.len());
    assert_eq!(records[0], decoded[0]);
    assert_eq!(records[999], decoded[999]);

    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 1.0,
        "Compression ratio {} should be < 1.0 for 1000 repetitive batch records",
        ratio
    );
}

// --- Test 20: Large in-process test log — compression ratio (1200+ entries) ---
#[test]
fn test_large_in_process_test_log_compression_ratio() {
    let mut entries: Vec<InProcessTest> = Vec::with_capacity(1200);
    for i in 0..1200_u64 {
        entries.push(InProcessTest {
            test_id: 300000 + i,
            batch_id: 200000 + (i / 10),
            step: ProcessStep::Compression,
            parameter: "hardness_N".to_string(),
            result_x1000: 100000,
            limit_lo_x1000: 50000,
            limit_hi_x1000: 150000,
        });
    }

    let encoded = encode_to_vec(&entries).expect("encode large test log");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress large test log");
    let decompressed = decompress(&compressed).expect("decompress large test log");
    let (decoded, _): (Vec<InProcessTest>, usize) =
        decode_from_slice(&decompressed).expect("decode large test log");

    assert_eq!(entries.len(), decoded.len());
    assert_eq!(entries[0], decoded[0]);
    assert_eq!(entries[1199], decoded[1199]);

    let ratio = compressed.len() as f64 / encoded.len() as f64;
    assert!(
        ratio < 1.0,
        "Compression ratio {} should be < 1.0 for 1200 repetitive in-process test entries",
        ratio
    );
}

// --- Test 21: Full GMP batch lifecycle — from InProgress to Released ---
#[test]
fn test_full_gmp_batch_lifecycle_roundtrip() {
    let lifecycle: Vec<BatchRecord> = vec![
        BatchRecord {
            batch_id: 100200,
            product_code: "LANSOPRAZOLE-30-CAP".to_string(),
            status: BatchStatus::InProgress,
            start_time: 1_701_000_000,
            end_time: None,
            yield_kg_x100: 0,
        },
        BatchRecord {
            batch_id: 100200,
            product_code: "LANSOPRAZOLE-30-CAP".to_string(),
            status: BatchStatus::Quarantine,
            start_time: 1_701_000_000,
            end_time: Some(1_701_014_400),
            yield_kg_x100: 8800,
        },
        BatchRecord {
            batch_id: 100200,
            product_code: "LANSOPRAZOLE-30-CAP".to_string(),
            status: BatchStatus::Released,
            start_time: 1_701_000_000,
            end_time: Some(1_701_014_400),
            yield_kg_x100: 8800,
        },
    ];

    let encoded = encode_to_vec(&lifecycle).expect("encode lifecycle");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress lifecycle");
    let decompressed = decompress(&compressed).expect("decompress lifecycle");
    let (decoded, _): (Vec<BatchRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode lifecycle");

    assert_eq!(lifecycle.len(), decoded.len());
    assert_eq!(decoded[0].status, BatchStatus::InProgress);
    assert_eq!(decoded[0].end_time, None);
    assert_eq!(decoded[1].status, BatchStatus::Quarantine);
    assert_eq!(decoded[2].status, BatchStatus::Released);
    assert_eq!(decoded[2].yield_kg_x100, 8800);
}

// --- Test 22: Mixed pharmaceutical data pack — all struct types in one roundtrip ---
#[test]
fn test_mixed_pharmaceutical_data_pack_roundtrip() {
    let batch = BatchRecord {
        batch_id: 100300,
        product_code: "DOXYCYCLINE-100-CAP".to_string(),
        status: BatchStatus::Completed,
        start_time: 1_702_000_000,
        end_time: Some(1_702_010_800),
        yield_kg_x100: 4400,
    };

    let test_entry = InProcessTest {
        test_id: 500001,
        batch_id: 100300,
        step: ProcessStep::Coating,
        parameter: "film_weight_gain_pct".to_string(),
        result_x1000: 3200,
        limit_lo_x1000: 2500,
        limit_hi_x1000: 5000,
    };

    let equipment_log = EquipmentLog {
        log_id: 600001,
        equipment_id: 88,
        status: EquipmentStatus::Running,
        operator_id: 7200,
        timestamp: 1_702_001_000,
        notes: "Coating pan speed 12 RPM, inlet temp 60C".to_string(),
    };

    let quality_event = QualityEvent {
        event_id: 700001,
        batch_id: 100300,
        decision: QualityDecision::Accept,
        test_count: 60,
        failures: 0,
        reviewer_id: 9050,
    };

    let ingredient = IngredientsUsage {
        usage_id: 800001,
        batch_id: 100300,
        ingredient_code: "DOXYCYCLINE-HYCLATE-EP".to_string(),
        lot_number: "RAW-2024-055-LOT-2".to_string(),
        quantity_g_x100: 12_000_000,
        tare_g_x100: 15_000,
    };

    // Encode, compress, decompress, decode each independently
    let enc_b = encode_to_vec(&batch).expect("encode batch mixed pack");
    let cmp_b = compress(&enc_b, Compression::Lz4).expect("compress batch mixed pack");
    let dec_b = decompress(&cmp_b).expect("decompress batch mixed pack");
    let (decoded_batch, _): (BatchRecord, usize) =
        decode_from_slice(&dec_b).expect("decode batch mixed pack");
    assert_eq!(batch, decoded_batch);

    let enc_t = encode_to_vec(&test_entry).expect("encode test entry mixed pack");
    let cmp_t = compress(&enc_t, Compression::Lz4).expect("compress test entry mixed pack");
    let dec_t = decompress(&cmp_t).expect("decompress test entry mixed pack");
    let (decoded_test, _): (InProcessTest, usize) =
        decode_from_slice(&dec_t).expect("decode test entry mixed pack");
    assert_eq!(test_entry, decoded_test);

    let enc_e = encode_to_vec(&equipment_log).expect("encode equipment log mixed pack");
    let cmp_e = compress(&enc_e, Compression::Lz4).expect("compress equipment log mixed pack");
    let dec_e = decompress(&cmp_e).expect("decompress equipment log mixed pack");
    let (decoded_equip, _): (EquipmentLog, usize) =
        decode_from_slice(&dec_e).expect("decode equipment log mixed pack");
    assert_eq!(equipment_log, decoded_equip);

    let enc_q = encode_to_vec(&quality_event).expect("encode quality event mixed pack");
    let cmp_q = compress(&enc_q, Compression::Lz4).expect("compress quality event mixed pack");
    let dec_q = decompress(&cmp_q).expect("decompress quality event mixed pack");
    let (decoded_qe, _): (QualityEvent, usize) =
        decode_from_slice(&dec_q).expect("decode quality event mixed pack");
    assert_eq!(quality_event, decoded_qe);

    let enc_i = encode_to_vec(&ingredient).expect("encode ingredient mixed pack");
    let cmp_i = compress(&enc_i, Compression::Lz4).expect("compress ingredient mixed pack");
    let dec_i = decompress(&cmp_i).expect("decompress ingredient mixed pack");
    let (decoded_ing, _): (IngredientsUsage, usize) =
        decode_from_slice(&dec_i).expect("decode ingredient mixed pack");
    assert_eq!(ingredient, decoded_ing);

    // Final integrity checks across all decoded values
    assert_eq!(decoded_batch.batch_id, decoded_test.batch_id);
    assert_eq!(decoded_batch.batch_id, decoded_qe.batch_id);
    assert_eq!(decoded_batch.batch_id, decoded_ing.batch_id);
    assert_eq!(decoded_qe.failures, 0);
    assert_eq!(decoded_qe.decision, QualityDecision::Accept);
    assert_eq!(decoded_equip.status, EquipmentStatus::Running);
    assert_eq!(decoded_test.step, ProcessStep::Coating);
}
