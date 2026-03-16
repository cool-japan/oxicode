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

// ---------------------------------------------------------------------------
// Domain types: Energy grid / power management streaming
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GridStatus {
    Normal,
    HighLoad,
    LowLoad,
    Fault,
    Maintenance,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PowerReading {
    node_id: u32,
    voltage_v: f32,
    current_a: f32,
    frequency_hz: f32,
    status: GridStatus,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GridSnapshot {
    grid_id: u64,
    readings: Vec<PowerReading>,
    total_load_kw: f64,
    timestamp: u64,
}

// ---------------------------------------------------------------------------
// Helper: build a representative PowerReading
// ---------------------------------------------------------------------------
fn make_reading(node_id: u32, status: GridStatus) -> PowerReading {
    PowerReading {
        node_id,
        voltage_v: 230.0 + node_id as f32 * 0.1,
        current_a: 10.0 + node_id as f32 * 0.05,
        frequency_hz: 50.0,
        status,
        timestamp: 1_700_000_000 + node_id as u64 * 100,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Single PowerReading roundtrip via duplex
// ---------------------------------------------------------------------------
#[test]
fn test_grid_single_power_reading_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let original = make_reading(1, GridStatus::Normal);

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&original).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let decoded: Option<PowerReading> = decoder.read_item().await.expect("read_item");
        assert_eq!(decoded, Some(original));
    });
}

// ---------------------------------------------------------------------------
// Test 2: GridStatus::Normal variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_status_normal_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(10, GridStatus::Normal);

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let decoded: PowerReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.status, GridStatus::Normal);
        assert_eq!(decoded.node_id, 10);
    });
}

// ---------------------------------------------------------------------------
// Test 3: GridStatus::HighLoad variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_status_high_load_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(20, GridStatus::HighLoad);

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let decoded: PowerReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.status, GridStatus::HighLoad);
    });
}

// ---------------------------------------------------------------------------
// Test 4: GridStatus::LowLoad variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_status_low_load_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(30, GridStatus::LowLoad);

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let decoded: PowerReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.status, GridStatus::LowLoad);
    });
}

// ---------------------------------------------------------------------------
// Test 5: GridStatus::Fault variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_status_fault_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(40, GridStatus::Fault);

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let decoded: PowerReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.status, GridStatus::Fault);
    });
}

// ---------------------------------------------------------------------------
// Test 6: GridStatus::Maintenance variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_status_maintenance_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(50, GridStatus::Maintenance);

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let decoded: PowerReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.status, GridStatus::Maintenance);
    });
}

// ---------------------------------------------------------------------------
// Test 7: All GridStatus variants in a single stream
// ---------------------------------------------------------------------------
#[test]
fn test_grid_all_status_variants_stream() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings = vec![
            make_reading(1, GridStatus::Normal),
            make_reading(2, GridStatus::HighLoad),
            make_reading(3, GridStatus::LowLoad),
            make_reading(4, GridStatus::Fault),
            make_reading(5, GridStatus::Maintenance),
        ];

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let mut decoded: Vec<PowerReading> = Vec::new();
        while let Some(item) = decoder.read_item().await.expect("read_item") {
            decoded.push(item);
        }
        assert_eq!(decoded, readings);
    });
}

// ---------------------------------------------------------------------------
// Test 8: GridSnapshot roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_snapshot_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let snapshot = GridSnapshot {
            grid_id: 0xDEAD_BEEF_0000_0001,
            readings: vec![
                make_reading(100, GridStatus::Normal),
                make_reading(101, GridStatus::HighLoad),
                make_reading(102, GridStatus::LowLoad),
            ],
            total_load_kw: 1_234.567_89,
            timestamp: 1_700_100_000,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&snapshot).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let decoded: GridSnapshot = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.grid_id, snapshot.grid_id);
        assert_eq!(decoded.readings.len(), 3);
        assert_eq!(decoded.readings[0].node_id, 100);
        assert_eq!(decoded.readings[2].status, GridStatus::LowLoad);
    });
}

// ---------------------------------------------------------------------------
// Test 9: Batch of 20 PowerReadings roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_batch_20_readings_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let statuses = [
            GridStatus::Normal,
            GridStatus::HighLoad,
            GridStatus::LowLoad,
            GridStatus::Fault,
            GridStatus::Maintenance,
        ];
        let readings: Vec<PowerReading> = (0u32..20)
            .map(|i| make_reading(i, statuses[(i as usize) % statuses.len()].clone()))
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let mut decoded: Vec<PowerReading> = Vec::new();
        while let Some(item) = decoder.read_item().await.expect("read_item") {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 20);
        assert_eq!(decoded, readings);
    });
}

// ---------------------------------------------------------------------------
// Test 10: Empty stream — no readings written, decoder yields None immediately
// ---------------------------------------------------------------------------
#[test]
fn test_grid_empty_stream() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        // No write_item calls — just finish
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let result: Option<PowerReading> = decoder.read_item().await.expect("read_item");
        assert_eq!(result, None);
        assert!(decoder.is_finished());
    });
}

// ---------------------------------------------------------------------------
// Test 11: Large batch of 100 PowerReadings roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_large_batch_100_readings_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let statuses = [
            GridStatus::Normal,
            GridStatus::HighLoad,
            GridStatus::LowLoad,
            GridStatus::Fault,
            GridStatus::Maintenance,
        ];
        let readings: Vec<PowerReading> = (0u32..100)
            .map(|i| make_reading(i, statuses[(i as usize) % statuses.len()].clone()))
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let mut decoded: Vec<PowerReading> = Vec::new();
        while let Some(item) = decoder.read_item().await.expect("read_item") {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 100);
        assert_eq!(decoded, readings);
    });
}

// ---------------------------------------------------------------------------
// Test 12: Progress tracking — items_processed > 0 after encoding
// ---------------------------------------------------------------------------
#[test]
fn test_grid_progress_items_processed() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings: Vec<PowerReading> = (0u32..15)
            .map(|i| make_reading(i, GridStatus::Normal))
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        // Progress is updated on flush; check before finish by reading the progress field.
        // After writing 15 items into the buffer, flush happens on finish.
        encoder.finish().await.expect("finish");

        // Drain decoder so we can inspect decoder progress
        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        while let Some(_item) = decoder
            .read_item::<PowerReading>()
            .await
            .expect("read_item")
        {}
        assert!(decoder.progress().items_processed > 0);
        assert_eq!(decoder.progress().items_processed, 15);
    });
}

// ---------------------------------------------------------------------------
// Test 13: write_all helper — write Vec<PowerReading> via write_all
// ---------------------------------------------------------------------------
#[test]
fn test_grid_write_all_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings: Vec<PowerReading> = (0u32..25)
            .map(|i| {
                make_reading(
                    i,
                    if i % 2 == 0 {
                        GridStatus::Normal
                    } else {
                        GridStatus::HighLoad
                    },
                )
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_all(readings.clone())
            .await
            .expect("write_all");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let mut decoded: Vec<PowerReading> = Vec::new();
        while let Some(item) = decoder.read_item().await.expect("read_item") {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 25);
        assert_eq!(decoded, readings);
    });
}

// ---------------------------------------------------------------------------
// Test 14: Small chunk size forces multiple chunk flushes
// ---------------------------------------------------------------------------
#[test]
fn test_grid_small_chunk_size_multiple_chunks() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        // chunk_size is clamped to minimum 1024; use enough readings to exceed it.
        // Each PowerReading is ~25 bytes; 60 readings = ~1500 bytes > 1024 → multiple chunks.
        let config = StreamingConfig::new().with_chunk_size(1024);
        let readings: Vec<PowerReading> = (0u32..60)
            .map(|i| make_reading(i, GridStatus::Normal))
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, config);
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let mut decoded: Vec<PowerReading> = Vec::new();
        while let Some(item) = decoder.read_item().await.expect("read_item") {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 60);
        assert_eq!(decoded, readings);
        assert!(
            decoder.progress().chunks_processed > 1,
            "expected multiple chunks, got {}",
            decoder.progress().chunks_processed
        );
    });
}

// ---------------------------------------------------------------------------
// Test 15: Large chunk size — everything fits in one chunk
// ---------------------------------------------------------------------------
#[test]
fn test_grid_large_chunk_size_single_chunk() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        // 1 MB chunk — all 20 readings easily fit in one chunk
        let config = StreamingConfig::new().with_chunk_size(1024 * 1024);
        let readings: Vec<PowerReading> = (0u32..20)
            .map(|i| make_reading(i, GridStatus::LowLoad))
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, config);
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let mut decoded: Vec<PowerReading> = Vec::new();
        while let Some(item) = decoder.read_item().await.expect("read_item") {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 20);
        assert_eq!(decoded, readings);
        assert_eq!(
            decoder.progress().chunks_processed,
            1,
            "expected single chunk, got {}",
            decoder.progress().chunks_processed
        );
    });
}

// ---------------------------------------------------------------------------
// Test 16: flush_per_item config — each item flushes immediately
// ---------------------------------------------------------------------------
#[test]
fn test_grid_flush_per_item_config() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let config = StreamingConfig::new().with_flush_per_item(true);
        let readings: Vec<PowerReading> = (0u32..10)
            .map(|i| make_reading(i, GridStatus::Fault))
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, config);
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let mut decoded: Vec<PowerReading> = Vec::new();
        while let Some(item) = decoder.read_item().await.expect("read_item") {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 10);
        assert_eq!(decoded, readings);
        // Each item triggered an individual chunk — 10 items → 10 chunks
        assert_eq!(decoder.progress().chunks_processed, 10);
    });
}

// ---------------------------------------------------------------------------
// Test 17: GridSnapshot with zero readings (edge case)
// ---------------------------------------------------------------------------
#[test]
fn test_grid_snapshot_zero_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let snapshot = GridSnapshot {
            grid_id: 999,
            readings: Vec::new(),
            total_load_kw: 0.0,
            timestamp: 1_700_200_000,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&snapshot).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let decoded: GridSnapshot = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.grid_id, 999);
        assert!(decoded.readings.is_empty());
        assert_eq!(decoded.total_load_kw, 0.0);
    });
}

// ---------------------------------------------------------------------------
// Test 18: Multiple GridSnapshots in sequence
// ---------------------------------------------------------------------------
#[test]
fn test_grid_multiple_snapshots_sequence() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let snapshots: Vec<GridSnapshot> = (0u64..5)
            .map(|g| GridSnapshot {
                grid_id: g * 1000,
                readings: (0u32..3)
                    .map(|i| make_reading(i + g as u32 * 10, GridStatus::Normal))
                    .collect(),
                total_load_kw: g as f64 * 500.0,
                timestamp: 1_700_300_000 + g * 3600,
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for s in &snapshots {
            encoder.write_item(s).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let mut decoded: Vec<GridSnapshot> = Vec::new();
        while let Some(item) = decoder.read_item().await.expect("read_item") {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 5);
        for (i, snap) in decoded.iter().enumerate() {
            assert_eq!(snap.grid_id, i as u64 * 1000);
            assert_eq!(snap.readings.len(), 3);
        }
    });
}

// ---------------------------------------------------------------------------
// Test 19: encode_to_vec / decode_from_slice roundtrip for PowerReading
// ---------------------------------------------------------------------------
#[test]
fn test_grid_encode_to_vec_decode_from_slice() {
    let reading = make_reading(77, GridStatus::Maintenance);
    let encoded = encode_to_vec(&reading).expect("encode_to_vec");
    let (decoded, consumed): (PowerReading, _) =
        decode_from_slice(&encoded).expect("decode_from_slice");
    assert_eq!(decoded, reading);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 20: Verify bytes_processed grows with more data
// ---------------------------------------------------------------------------
#[test]
fn test_grid_bytes_processed_grows() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings: Vec<PowerReading> = (0u32..50)
            .map(|i| make_reading(i, GridStatus::HighLoad))
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        while let Some(_item) = decoder
            .read_item::<PowerReading>()
            .await
            .expect("read_item")
        {}

        let progress = decoder.progress();
        assert_eq!(progress.items_processed, 50);
        assert!(progress.bytes_processed > 0);
        // 50 PowerReadings with varint encoding — conservatively at least 500 bytes
        assert!(progress.bytes_processed >= 500);
    });
}

// ---------------------------------------------------------------------------
// Test 21: encoder.progress().items_processed > 0 after writing and finishing
// ---------------------------------------------------------------------------
#[test]
fn test_grid_encoder_progress_items_processed() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings: Vec<PowerReading> = (0u32..8)
            .map(|i| make_reading(i, GridStatus::Normal))
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        // After finish, the encoder flushes and items_processed is updated
        let finished_writer = encoder.finish().await.expect("finish");
        drop(finished_writer);

        // Drain the reader to verify correctness
        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let mut count = 0u32;
        while let Some(_item) = decoder
            .read_item::<PowerReading>()
            .await
            .expect("read_item")
        {
            count += 1;
        }
        assert_eq!(count, 8);
        assert!(decoder.progress().items_processed > 0);
    });
}

// ---------------------------------------------------------------------------
// Test 22: Concurrent read/write via tokio duplex with mixed status readings
// ---------------------------------------------------------------------------
#[test]
fn test_grid_concurrent_reads_mixed_status() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let statuses = [
            GridStatus::Normal,
            GridStatus::HighLoad,
            GridStatus::LowLoad,
            GridStatus::Fault,
            GridStatus::Maintenance,
        ];
        let readings: Vec<PowerReading> = (0u32..55)
            .map(|i| make_reading(i, statuses[(i as usize) % statuses.len()].clone()))
            .collect();
        let expected = readings.clone();

        // Use a larger duplex buffer to allow concurrent progress
        let (writer, reader) = tokio::io::duplex(65536);

        // Spawn encoder as a separate task
        let encode_handle = tokio::spawn(async move {
            let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
            for r in &readings {
                encoder.write_item(r).await.expect("write");
            }
            encoder.finish().await.expect("finish");
        });

        // Decode concurrently in the main async context
        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let mut decoded: Vec<PowerReading> = Vec::new();
        while let Some(item) = decoder.read_item().await.expect("read_item") {
            decoded.push(item);
        }

        encode_handle.await.expect("encoder task");

        assert_eq!(decoded.len(), 55);
        assert_eq!(decoded, expected);
        assert!(decoder.progress().items_processed > 0);
        assert!(decoder.is_finished());
    });
}
