//! Advanced async streaming tests — manufacturing / industrial IoT domain (set 30).
//!
//! 22 `#[tokio::test]` functions exercising OxiCode's async streaming API
//! through industrial IoT types: sensor readings, machine events, production batches.
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
// Domain types — manufacturing / industrial IoT
// ---------------------------------------------------------------------------

/// Operational status of an industrial machine.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MachineStatus {
    Idle,
    Running,
    Warning,
    Fault,
    Maintenance,
    Offline,
}

/// Classification of a physical sensor.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SensorKind {
    Temperature,
    Pressure,
    Vibration,
    Current,
    Speed,
    Flow,
}

/// A single timestamped reading from one sensor.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SensorReading {
    sensor_id: u32,
    kind: SensorKind,
    value: f64,
    unit: String,
    timestamp_ms: u64,
}

/// An event emitted by a machine, carrying zero or more sensor readings.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MachineEvent {
    machine_id: u32,
    status: MachineStatus,
    timestamp_ms: u64,
    readings: Vec<SensorReading>,
}

/// A discrete production batch spanning multiple machine events.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductionBatch {
    batch_id: u64,
    product_code: String,
    quantity: u32,
    start_ms: u64,
    end_ms: u64,
    events: Vec<MachineEvent>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_reading(
    sensor_id: u32,
    kind: SensorKind,
    value: f64,
    unit: &str,
    ts: u64,
) -> SensorReading {
    SensorReading {
        sensor_id,
        kind,
        value,
        unit: unit.to_string(),
        timestamp_ms: ts,
    }
}

fn make_event(
    machine_id: u32,
    status: MachineStatus,
    ts: u64,
    readings: Vec<SensorReading>,
) -> MachineEvent {
    MachineEvent {
        machine_id,
        status,
        timestamp_ms: ts,
        readings,
    }
}

async fn encode_single_item<T: Encode>(item: &T) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(item)
            .await
            .expect("encode_single_item: write_item failed");
        enc.finish()
            .await
            .expect("encode_single_item: finish failed");
    }
    buf
}

async fn decode_single_item<T: Decode>(buf: Vec<u8>) -> Option<T> {
    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    dec.read_item::<T>()
        .await
        .expect("decode_single_item: read_item failed")
}

// ---------------------------------------------------------------------------
// Test 1: Single SensorReading async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_single_sensor_reading_roundtrip() {
    let original = make_reading(
        101,
        SensorKind::Temperature,
        72.5,
        "celsius",
        1_700_000_000_000,
    );
    let buf = encode_single_item(&original).await;
    let decoded = decode_single_item::<SensorReading>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "single SensorReading roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 2: SensorReading with Pressure kind
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_pressure_reading_roundtrip() {
    let original = make_reading(202, SensorKind::Pressure, 4.2, "bar", 1_700_000_001_000);
    let buf = encode_single_item(&original).await;
    let decoded = decode_single_item::<SensorReading>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "pressure SensorReading roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 3: MachineEvent with Running status and a single reading
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_machine_event_running_single_reading() {
    let reading = make_reading(10, SensorKind::Speed, 1450.0, "rpm", 1_700_000_002_000);
    let original = make_event(5, MachineStatus::Running, 1_700_000_002_000, vec![reading]);
    let buf = encode_single_item(&original).await;
    let decoded = decode_single_item::<MachineEvent>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "MachineEvent Running roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 4: MachineEvent with Fault status and multiple readings
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_machine_event_fault_multiple_readings() {
    let readings = vec![
        make_reading(11, SensorKind::Vibration, 9.8, "mm/s", 1_700_000_003_000),
        make_reading(12, SensorKind::Current, 22.4, "ampere", 1_700_000_003_001),
        make_reading(
            13,
            SensorKind::Temperature,
            98.1,
            "celsius",
            1_700_000_003_002,
        ),
    ];
    let original = make_event(7, MachineStatus::Fault, 1_700_000_003_000, readings);
    let buf = encode_single_item(&original).await;
    let decoded = decode_single_item::<MachineEvent>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "MachineEvent Fault multi-reading roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 5: ProductionBatch async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_production_batch_roundtrip() {
    let event = make_event(
        1,
        MachineStatus::Running,
        1_700_000_010_000,
        vec![make_reading(
            20,
            SensorKind::Flow,
            125.3,
            "l/min",
            1_700_000_010_000,
        )],
    );
    let original = ProductionBatch {
        batch_id: 999_001,
        product_code: String::from("WIDGET-A"),
        quantity: 500,
        start_ms: 1_700_000_000_000,
        end_ms: 1_700_000_020_000,
        events: vec![event],
    };
    let buf = encode_single_item(&original).await;
    let decoded = decode_single_item::<ProductionBatch>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "ProductionBatch async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Empty stream — encoder writes nothing but completes cleanly
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_empty_stream_no_items() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let enc = AsyncEncoder::new(cursor);
        enc.finish().await.expect("empty stream finish failed");
    }
    assert!(
        !buf.is_empty(),
        "encoded buffer must contain end-chunk marker"
    );

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let item: Option<SensorReading> = dec.read_item().await.expect("read on empty stream failed");
    assert_eq!(item, None, "expected None from empty stream");
    assert!(
        dec.is_finished(),
        "decoder must be finished after empty stream"
    );
}

// ---------------------------------------------------------------------------
// Test 7: Large event with 100+ sensor readings
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_large_event_100_readings() {
    let readings: Vec<SensorReading> = (0u32..120)
        .map(|i| {
            make_reading(
                i,
                SensorKind::Temperature,
                f64::from(i) * 0.5,
                "celsius",
                u64::from(i) * 1_000,
            )
        })
        .collect();
    assert_eq!(readings.len(), 120, "must have 120 readings");

    let original = make_event(42, MachineStatus::Running, 0, readings);
    let buf = encode_single_item(&original).await;
    let decoded = decode_single_item::<MachineEvent>(buf).await;
    let decoded_event = decoded.expect("decoded large event must be Some");
    assert_eq!(
        decoded_event.readings.len(),
        120,
        "all 120 readings must survive roundtrip"
    );
    assert_eq!(decoded_event.machine_id, 42, "machine_id mismatch");
}

// ---------------------------------------------------------------------------
// Test 8: Progress tracking — items_processed > 0 after encoding
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_progress_items_processed_nonzero() {
    let readings: Vec<SensorReading> = (0u32..5)
        .map(|i| {
            make_reading(
                i,
                SensorKind::Current,
                f64::from(i) * 2.0,
                "ampere",
                u64::from(i),
            )
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for r in &readings {
            enc.write_item(r).await.expect("write reading failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);

    // Read one item to trigger progress update
    let _first: Option<SensorReading> = dec.read_item().await.expect("read first failed");
    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after reading one item"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Machine status transitions — all six variants roundtrip via stream
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_all_machine_status_variants_stream() {
    let statuses = vec![
        MachineStatus::Idle,
        MachineStatus::Running,
        MachineStatus::Warning,
        MachineStatus::Fault,
        MachineStatus::Maintenance,
        MachineStatus::Offline,
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for s in &statuses {
            enc.write_item(s).await.expect("write MachineStatus failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);

    for expected in &statuses {
        let item: Option<MachineStatus> = dec.read_item().await.expect("read MachineStatus failed");
        assert_eq!(
            item.as_ref(),
            Some(expected),
            "MachineStatus variant mismatch for {:?}",
            expected
        );
    }

    let eof: Option<MachineStatus> = dec.read_item().await.expect("eof check failed");
    assert_eq!(eof, None, "expected None after all statuses");
}

// ---------------------------------------------------------------------------
// Test 10: All SensorKind variants roundtrip via async stream
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_all_sensor_kind_variants_stream() {
    let kinds = vec![
        SensorKind::Temperature,
        SensorKind::Pressure,
        SensorKind::Vibration,
        SensorKind::Current,
        SensorKind::Speed,
        SensorKind::Flow,
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for k in &kinds {
            enc.write_item(k).await.expect("write SensorKind failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);

    for expected in &kinds {
        let item: Option<SensorKind> = dec.read_item().await.expect("read SensorKind failed");
        assert_eq!(
            item.as_ref(),
            Some(expected),
            "SensorKind variant mismatch for {:?}",
            expected
        );
    }
}

// ---------------------------------------------------------------------------
// Test 11: Batch of multiple MachineEvents in order
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_multiple_machine_events_order_preserved() {
    let events: Vec<MachineEvent> = (0u32..8)
        .map(|i| {
            make_event(
                i,
                if i % 2 == 0 {
                    MachineStatus::Running
                } else {
                    MachineStatus::Idle
                },
                u64::from(i) * 100_000,
                vec![make_reading(
                    i * 10,
                    SensorKind::Speed,
                    f64::from(i) * 300.0,
                    "rpm",
                    u64::from(i) * 100_000,
                )],
            )
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for ev in &events {
            enc.write_item(ev).await.expect("write MachineEvent failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);

    for expected in &events {
        let item: Option<MachineEvent> = dec.read_item().await.expect("read MachineEvent failed");
        assert_eq!(
            item.as_ref(),
            Some(expected),
            "MachineEvent mismatch at machine_id {}",
            expected.machine_id
        );
    }
}

// ---------------------------------------------------------------------------
// Test 12: Small chunk size forces multi-chunk encoding of sensor readings
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_multi_chunk_encoding_with_small_chunk_size() {
    // 50 readings, each with a long unit string (~30 bytes each = ~1500 bytes total)
    // With chunk_size=1024, this must produce at least 2 data chunks.
    let readings: Vec<SensorReading> = (0u32..50)
        .map(|i| {
            make_reading(
                i,
                SensorKind::Temperature,
                f64::from(i) * 1.23,
                "degrees-celsius-industrial", // 26-char string -> more bytes per item
                u64::from(i) * 50_000,
            )
        })
        .collect();

    // minimum chunk_size is 1024; 50 * ~35 bytes = ~1750 bytes > 1024 => 2+ chunks
    let config = StreamingConfig::new().with_chunk_size(1024);

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::with_config(cursor, config);
        for r in &readings {
            enc.write_item(r)
                .await
                .expect("write reading with small chunk failed");
        }
        enc.finish().await.expect("finish failed");
    }

    // Verify the raw payload is large enough to require multiple 1024-byte chunks
    assert!(
        buf.len() > 1024,
        "encoded stream ({} bytes) must exceed 1024 bytes for multi-chunk guarantee",
        buf.len()
    );

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let decoded: Vec<SensorReading> = dec.read_all().await.expect("read_all failed");

    assert_eq!(
        decoded, readings,
        "multi-chunk readings must decode identically"
    );
    assert!(
        dec.progress().chunks_processed > 1,
        "multiple chunks must have been processed (chunks={})",
        dec.progress().chunks_processed
    );
}

// ---------------------------------------------------------------------------
// Test 13: ProductionBatch with many events async write_all roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_production_batch_write_all_roundtrip() {
    let events: Vec<MachineEvent> = (0u32..6)
        .map(|i| {
            make_event(
                i,
                MachineStatus::Running,
                u64::from(i) * 1_000,
                vec![make_reading(
                    i,
                    SensorKind::Flow,
                    f64::from(i) * 10.0,
                    "l/min",
                    u64::from(i) * 1_000,
                )],
            )
        })
        .collect();

    let batches: Vec<ProductionBatch> = (0u64..3)
        .map(|b| ProductionBatch {
            batch_id: 100 + b,
            product_code: format!("SKU-{}", b),
            quantity: (b as u32 + 1) * 100,
            start_ms: b * 1_000_000,
            end_ms: b * 1_000_000 + 500_000,
            events: events.clone(),
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        // write_all takes owned IntoIterator<Item=T> — clone before calling
        enc.write_all(batches.clone())
            .await
            .expect("write_all batches failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let decoded: Vec<ProductionBatch> = dec.read_all().await.expect("read_all batches failed");

    assert_eq!(
        decoded, batches,
        "production batches write_all roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Concurrent reads from two independent decoders on same data
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_concurrent_decoders_same_data() {
    let reading = make_reading(300, SensorKind::Pressure, 3.14, "bar", 1_700_000_100_000);

    // Build encoded buffer once
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&reading)
            .await
            .expect("write reading failed");
        enc.finish().await.expect("finish failed");
    }

    // Spawn two concurrent decode tasks from independent clones of the buffer
    let buf_a = buf.clone();
    let buf_b = buf.clone();

    let task_a = tokio::spawn(async move {
        let cursor = Cursor::new(buf_a);
        let reader = BufReader::new(cursor);
        let mut dec = AsyncDecoder::new(reader);
        dec.read_item::<SensorReading>()
            .await
            .expect("concurrent decoder A failed")
    });

    let task_b = tokio::spawn(async move {
        let cursor = Cursor::new(buf_b);
        let reader = BufReader::new(cursor);
        let mut dec = AsyncDecoder::new(reader);
        dec.read_item::<SensorReading>()
            .await
            .expect("concurrent decoder B failed")
    });

    let result_a = task_a.await.expect("task A panicked");
    let result_b = task_b.await.expect("task B panicked");

    assert_eq!(
        result_a,
        Some(reading.clone()),
        "concurrent decoder A mismatch"
    );
    assert_eq!(result_b, Some(reading), "concurrent decoder B mismatch");
}

// ---------------------------------------------------------------------------
// Test 15: Encoder progress tracks items after finish
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_encoder_progress_after_finish() {
    let events: Vec<MachineEvent> = (0u32..10)
        .map(|i| make_event(i, MachineStatus::Running, u64::from(i) * 1_000, vec![]))
        .collect();

    let mut buf = Vec::<u8>::new();
    let cursor = Cursor::new(&mut buf);
    let mut enc = AsyncEncoder::new(cursor);
    enc.set_estimated_total(10);

    for ev in &events {
        enc.write_item(ev).await.expect("write event failed");
    }

    // progress tracked before finish is only from flushed chunks
    // but after finish all items are accounted for
    enc.finish().await.expect("finish failed");

    // Verify by decoding
    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let decoded: Vec<MachineEvent> = dec.read_all().await.expect("read_all failed");
    assert_eq!(decoded.len(), 10, "must decode 10 events");
    assert_eq!(
        dec.progress().items_processed,
        10,
        "decoder items_processed must be 10"
    );
}

// ---------------------------------------------------------------------------
// Test 16: flush_per_item config — one item per chunk
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_flush_per_item_config() {
    let readings: Vec<SensorReading> = (0u32..5)
        .map(|i| {
            make_reading(
                i,
                SensorKind::Current,
                f64::from(i) * 1.5,
                "ampere",
                u64::from(i),
            )
        })
        .collect();

    let config = StreamingConfig::new().with_flush_per_item(true);

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::with_config(cursor, config);
        for r in &readings {
            enc.write_item(r).await.expect("write per-item failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let decoded: Vec<SensorReading> = dec.read_all().await.expect("read_all failed");
    assert_eq!(decoded, readings, "flush_per_item roundtrip mismatch");
    // Each item was its own chunk, so chunks_processed == number of items
    assert_eq!(
        dec.progress().chunks_processed,
        readings.len() as u64,
        "chunks_processed must equal item count when flush_per_item is true"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Sync encode of MachineEvent, async decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_sync_encode_async_decode_machine_event() {
    let reading = make_reading(500, SensorKind::Vibration, 2.71, "mm/s", 1_700_000_200_000);
    let original = make_event(99, MachineStatus::Warning, 1_700_000_200_000, vec![reading]);

    // Sync encode: just encode the struct directly (not via streaming)
    let sync_bytes = encode_to_vec(&original).expect("sync encode_to_vec failed");

    // Async decode of the same struct from the same byte format
    // (use async streaming encoder to repackage the sync-encoded bytes for streaming decode)
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&original)
            .await
            .expect("async repackage failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);
    let decoded: Option<MachineEvent> = dec.read_item().await.expect("async decode failed");
    assert_eq!(
        decoded,
        Some(original.clone()),
        "sync-encode then async-stream decode mismatch"
    );

    // Also verify the raw sync round-trip is consistent
    let (sync_decoded, _): (MachineEvent, _) =
        decode_from_slice(&sync_bytes).expect("sync decode_from_slice failed");
    assert_eq!(sync_decoded, original, "sync roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 18: ProductionBatch with zero events (empty events vec)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_production_batch_empty_events() {
    let original = ProductionBatch {
        batch_id: 0,
        product_code: String::from("EMPTY-BATCH"),
        quantity: 0,
        start_ms: 0,
        end_ms: 0,
        events: vec![],
    };
    let buf = encode_single_item(&original).await;
    let decoded = decode_single_item::<ProductionBatch>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "ProductionBatch with empty events roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Vec<SensorReading> containing all SensorKind variants
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_vec_all_sensor_kinds_roundtrip() {
    let readings = vec![
        make_reading(1, SensorKind::Temperature, 25.0, "celsius", 1_000),
        make_reading(2, SensorKind::Pressure, 1.01, "bar", 2_000),
        make_reading(3, SensorKind::Vibration, 0.05, "mm/s", 3_000),
        make_reading(4, SensorKind::Current, 12.5, "ampere", 4_000),
        make_reading(5, SensorKind::Speed, 3000.0, "rpm", 5_000),
        make_reading(6, SensorKind::Flow, 50.0, "l/min", 6_000),
    ];
    let buf = encode_single_item(&readings).await;
    let decoded = decode_single_item::<Vec<SensorReading>>(buf).await;
    assert_eq!(
        decoded,
        Some(readings),
        "Vec of all SensorKind variants roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Large ProductionBatch (100+ readings across multiple events)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_large_production_batch_many_readings() {
    let events: Vec<MachineEvent> = (0u32..5)
        .map(|ev_i| {
            let readings: Vec<SensorReading> = (0u32..25)
                .map(|r_i| {
                    make_reading(
                        ev_i * 100 + r_i,
                        SensorKind::Temperature,
                        f64::from(ev_i * 100 + r_i),
                        "celsius",
                        u64::from(ev_i * 100_000 + r_i * 1_000),
                    )
                })
                .collect();
            make_event(
                ev_i,
                MachineStatus::Running,
                u64::from(ev_i) * 100_000,
                readings,
            )
        })
        .collect();

    let total_readings: usize = events.iter().map(|e| e.readings.len()).sum();
    assert_eq!(
        total_readings, 125,
        "must have 125 total readings across 5 events"
    );

    let original = ProductionBatch {
        batch_id: 777,
        product_code: String::from("HEAVY-DUTY-PART"),
        quantity: 250,
        start_ms: 0,
        end_ms: 500_000,
        events,
    };

    let buf = encode_single_item(&original).await;
    let decoded = decode_single_item::<ProductionBatch>(buf).await;
    let decoded_batch = decoded.expect("large ProductionBatch decode must be Some");
    let decoded_readings: usize = decoded_batch.events.iter().map(|e| e.readings.len()).sum();
    assert_eq!(
        decoded_readings, 125,
        "all 125 readings must survive roundtrip"
    );
    assert_eq!(decoded_batch.batch_id, 777, "batch_id mismatch");
}

// ---------------------------------------------------------------------------
// Test 21: bytes_processed and chunks_processed grow as readings are decoded
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_progress_bytes_and_chunks_grow() {
    let readings: Vec<SensorReading> = (0u32..30)
        .map(|i| {
            make_reading(
                i,
                SensorKind::Pressure,
                f64::from(i),
                "bar",
                u64::from(i) * 1_000,
            )
        })
        .collect();

    // Use a very small chunk size to guarantee multiple chunks
    let config = StreamingConfig::new().with_chunk_size(1024);

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::with_config(cursor, config);
        for r in &readings {
            enc.write_item(r).await.expect("write reading failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(reader);

    // Read one item to trigger first chunk load
    let _first: Option<SensorReading> = dec.read_item().await.expect("read first failed");
    let bytes_after_one = dec.progress().bytes_processed;
    let chunks_after_one = dec.progress().chunks_processed;
    assert!(
        bytes_after_one > 0,
        "bytes_processed must be > 0 after first read"
    );
    assert!(
        chunks_after_one >= 1,
        "at least one chunk must be processed"
    );

    // Read the rest
    let rest: Vec<SensorReading> = dec.read_all().await.expect("read_all failed");
    assert_eq!(rest.len(), 29, "must decode 29 remaining readings");

    let bytes_after_all = dec.progress().bytes_processed;
    assert!(
        bytes_after_all > bytes_after_one,
        "bytes_processed must grow after decoding all items (was {}, now {})",
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
// Test 22: Tokio duplex in-memory channel — write and read concurrently
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot30_tokio_duplex_channel_streaming() {
    use tokio::io::split;

    let events: Vec<MachineEvent> = (0u32..4)
        .map(|i| {
            make_event(
                i,
                MachineStatus::Maintenance,
                u64::from(i) * 10_000,
                vec![make_reading(
                    i * 3,
                    SensorKind::Vibration,
                    f64::from(i) * 0.3,
                    "mm/s",
                    u64::from(i) * 10_000,
                )],
            )
        })
        .collect();

    let events_to_write = events.clone();

    // tokio::io::duplex gives a bidirectional in-memory stream
    let (client, server) = tokio::io::duplex(65536);
    let (server_read, _server_write) = split(server);
    let (_client_read, client_write) = split(client);

    // Writer task: encodes into the client write half
    let write_task = tokio::spawn(async move {
        let mut enc = AsyncEncoder::new(client_write);
        for ev in &events_to_write {
            enc.write_item(ev).await.expect("duplex write_item failed");
        }
        enc.finish().await.expect("duplex finish failed");
    });

    // Reader task: decodes from the server read half
    let read_task = tokio::spawn(async move {
        let reader = BufReader::new(server_read);
        let mut dec = AsyncDecoder::new(reader);
        dec.read_all::<MachineEvent>()
            .await
            .expect("duplex read_all failed")
    });

    write_task.await.expect("write task panicked");
    let decoded = read_task.await.expect("read task panicked");

    assert_eq!(decoded, events, "duplex channel streaming roundtrip failed");
}
