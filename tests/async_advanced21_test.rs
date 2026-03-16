//! Advanced async streaming tests (21st set) for OxiCode — sensor network theme.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Domain types unique to this file:
//!   `SensorReading` — struct { sensor_id: u32, value: f64, unit: String, timestamp: u64 }
//!   `SensorEvent`   — enum { Reading(SensorReading), Alarm { … }, Calibrate { … }, Offline { … } }
//!
//! Coverage matrix:
//!   1:   SensorReading single-item write_item / read_item roundtrip
//!   2:   SensorEvent::Reading single-item roundtrip
//!   3:   SensorEvent::Alarm single-item roundtrip
//!   4:   SensorEvent::Calibrate single-item roundtrip
//!   5:   SensorEvent::Offline single-item roundtrip
//!   6:   Multiple SensorReadings via write_all / read_all
//!   7:   Multiple SensorEvents via write_all / read_all
//!   8:   Empty collection — write_all(empty) then read_all returns []
//!   9:   Large collection (100+ SensorReadings) roundtrip
//!  10:   Progress tracking — items_processed equals number written
//!  11:   With config (small chunk size) — data integrity preserved
//!  12:   Mixed SensorEvent variants in one stream
//!  13:   finish() then read_all() — stream exhausted cleanly
//!  14:   Sequential writes with multiple write_item calls
//!  15:   bytes_processed grows after each read_item
//!  16:   SensorReading with boundary values (sensor_id=u32::MAX, timestamp=u64::MAX)
//!  17:   SensorEvent::Alarm threshold boundary values
//!  18:   write_all preserving original via clone
//!  19:   read_all after interleaved write_all on same pipe
//!  20:   Error handling — decoding wrong type returns Err
//!  21:   Flush-per-item config — chunks_processed matches item count
//!  22:   Concurrent encode/decode via tokio::join!

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
use oxicode::async_tokio::{AsyncDecoder, AsyncEncoder, StreamingConfig};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SensorReading {
    sensor_id: u32,
    value: f64,
    unit: String,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SensorEvent {
    Reading(SensorReading),
    Alarm { sensor_id: u32, threshold: f64 },
    Calibrate { sensor_id: u32 },
    Offline { sensor_id: u32, reason: String },
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_reading(sensor_id: u32, value: f64, unit: &str, timestamp: u64) -> SensorReading {
    SensorReading {
        sensor_id,
        value,
        unit: unit.to_string(),
        timestamp,
    }
}

fn make_readings(n: usize) -> Vec<SensorReading> {
    (0..n)
        .map(|i| {
            make_reading(
                i as u32,
                20.0 + (i as f64) * 0.5,
                if i % 2 == 0 { "Celsius" } else { "Kelvin" },
                1_700_000_000u64 + i as u64,
            )
        })
        .collect()
}

fn make_mixed_events(n: usize) -> Vec<SensorEvent> {
    (0..n)
        .map(|i| match i % 4 {
            0 => SensorEvent::Reading(make_reading(i as u32, 100.0 - i as f64, "Pa", i as u64)),
            1 => SensorEvent::Alarm {
                sensor_id: i as u32,
                threshold: 50.0 + i as f64,
            },
            2 => SensorEvent::Calibrate {
                sensor_id: i as u32,
            },
            _ => SensorEvent::Offline {
                sensor_id: i as u32,
                reason: format!("timeout-{}", i),
            },
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Test 1: SensorReading single-item write_item / read_item roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_reading_single_roundtrip() {
    let reading = make_reading(1, 23.5, "Celsius", 1_700_000_001);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&reading).await.expect("write_item");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: SensorReading = dec
        .read_item()
        .await
        .expect("read_item")
        .expect("expected Some(SensorReading)");
    assert_eq!(reading, got);
}

// ---------------------------------------------------------------------------
// Test 2: SensorEvent::Reading single-item roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_event_reading_roundtrip() {
    let event = SensorEvent::Reading(make_reading(42, -5.1, "Celsius", 1_700_100_000));
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event).await.expect("write_item");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: SensorEvent = dec
        .read_item()
        .await
        .expect("read_item")
        .expect("expected Some(SensorEvent)");
    assert_eq!(event, got);
}

// ---------------------------------------------------------------------------
// Test 3: SensorEvent::Alarm single-item roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_event_alarm_roundtrip() {
    let event = SensorEvent::Alarm {
        sensor_id: 7,
        threshold: 80.0,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event).await.expect("write_item");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: SensorEvent = dec
        .read_item()
        .await
        .expect("read_item")
        .expect("expected Some(SensorEvent::Alarm)");
    assert_eq!(event, got);
}

// ---------------------------------------------------------------------------
// Test 4: SensorEvent::Calibrate single-item roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_event_calibrate_roundtrip() {
    let event = SensorEvent::Calibrate { sensor_id: 99 };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event).await.expect("write_item");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: SensorEvent = dec
        .read_item()
        .await
        .expect("read_item")
        .expect("expected Some(SensorEvent::Calibrate)");
    assert_eq!(event, got);
}

// ---------------------------------------------------------------------------
// Test 5: SensorEvent::Offline single-item roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_event_offline_roundtrip() {
    let event = SensorEvent::Offline {
        sensor_id: 3,
        reason: "power_loss".to_string(),
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event).await.expect("write_item");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: SensorEvent = dec
        .read_item()
        .await
        .expect("read_item")
        .expect("expected Some(SensorEvent::Offline)");
    assert_eq!(event, got);
}

// ---------------------------------------------------------------------------
// Test 6: Multiple SensorReadings via write_all / read_all
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_readings_write_all_read_all() {
    let readings = make_readings(8);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(readings.clone().into_iter())
        .await
        .expect("write_all");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorReading> = dec.read_all().await.expect("read_all");
    assert_eq!(readings, got);
}

// ---------------------------------------------------------------------------
// Test 7: Multiple SensorEvents via write_all / read_all
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_events_write_all_read_all() {
    let events = make_mixed_events(12);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(events.clone().into_iter())
        .await
        .expect("write_all");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorEvent> = dec.read_all().await.expect("read_all");
    assert_eq!(events, got);
}

// ---------------------------------------------------------------------------
// Test 8: Empty collection — write_all(empty) then read_all returns []
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_empty_collection_write_all_read_all() {
    let empty: Vec<SensorReading> = vec![];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(empty.clone().into_iter())
        .await
        .expect("write_all empty");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorReading> = dec.read_all().await.expect("read_all empty");
    assert!(
        got.is_empty(),
        "expected empty vec after write_all of 0 items"
    );
    assert!(dec.is_finished());
}

// ---------------------------------------------------------------------------
// Test 9: Large collection (100+ SensorReadings) roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_large_collection_roundtrip() {
    let readings = make_readings(150);
    assert_eq!(readings.len(), 150);

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(readings.clone().into_iter())
        .await
        .expect("write_all 150 readings");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorReading> = dec.read_all().await.expect("read_all 150 readings");
    assert_eq!(readings.len(), got.len(), "count mismatch");
    assert_eq!(readings, got, "data mismatch for large collection");
}

// ---------------------------------------------------------------------------
// Test 10: Progress tracking — items_processed equals number written
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_progress_items_processed() {
    const N: u64 = 20;
    let events = make_mixed_events(N as usize);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.set_estimated_total(N);
    enc.write_all(events.clone().into_iter())
        .await
        .expect("write_all");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let _: Vec<SensorEvent> = dec.read_all().await.expect("read_all");
    assert_eq!(
        dec.progress().items_processed,
        N,
        "items_processed must equal {N}"
    );
    assert!(
        dec.progress().bytes_processed > 0,
        "bytes_processed must be > 0"
    );
}

// ---------------------------------------------------------------------------
// Test 11: With config (small chunk size) — data integrity preserved
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_with_small_chunk_size_config() {
    let config = StreamingConfig::new().with_chunk_size(64);
    let readings = make_readings(30);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for r in &readings {
        enc.write_item(r).await.expect("write_item with config");
    }
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorReading> = dec.read_all().await.expect("read_all");
    assert_eq!(readings, got, "data integrity failed with small chunk size");
    assert!(dec.progress().items_processed > 0);
}

// ---------------------------------------------------------------------------
// Test 12: Mixed SensorEvent variants in one stream
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_mixed_event_variants_stream() {
    let events = vec![
        SensorEvent::Reading(make_reading(1, 22.3, "Celsius", 1_000_001)),
        SensorEvent::Alarm {
            sensor_id: 2,
            threshold: 75.0,
        },
        SensorEvent::Calibrate { sensor_id: 3 },
        SensorEvent::Offline {
            sensor_id: 4,
            reason: "hardware_fault".to_string(),
        },
        SensorEvent::Reading(make_reading(5, -10.0, "Celsius", 1_000_005)),
        SensorEvent::Alarm {
            sensor_id: 6,
            threshold: 0.001,
        },
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    for e in &events {
        enc.write_item(e).await.expect("write_item mixed");
    }
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorEvent> = dec.read_all().await.expect("read_all mixed");
    assert_eq!(events, got);
}

// ---------------------------------------------------------------------------
// Test 13: finish() then read_all() — stream exhausted cleanly
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_finish_then_read_all_exhausts_cleanly() {
    let readings = make_readings(5);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(readings.clone().into_iter())
        .await
        .expect("write_all");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorReading> = dec.read_all().await.expect("read_all after finish");
    assert_eq!(readings, got);

    // After read_all the stream must be marked finished and subsequent reads return None
    let extra = dec
        .read_item::<SensorReading>()
        .await
        .expect("read after exhaustion");
    assert_eq!(extra, None, "must return None after stream exhausted");
    assert!(dec.is_finished());
}

// ---------------------------------------------------------------------------
// Test 14: Sequential writes with multiple write_item calls
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_sequential_write_item_calls() {
    let r1 = make_reading(10, 1.0, "V", 1_000_010);
    let r2 = make_reading(11, 2.0, "V", 1_000_011);
    let r3 = make_reading(12, 3.0, "V", 1_000_012);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&r1).await.expect("write r1");
    enc.write_item(&r2).await.expect("write r2");
    enc.write_item(&r3).await.expect("write r3");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got1: SensorReading = dec.read_item().await.expect("read r1").expect("Some r1");
    let got2: SensorReading = dec.read_item().await.expect("read r2").expect("Some r2");
    let got3: SensorReading = dec.read_item().await.expect("read r3").expect("Some r3");
    let eof: Option<SensorReading> = dec.read_item().await.expect("read eof");

    assert_eq!(r1, got1, "sequential read mismatch at position 1");
    assert_eq!(r2, got2, "sequential read mismatch at position 2");
    assert_eq!(r3, got3, "sequential read mismatch at position 3");
    assert_eq!(eof, None, "expected None after all items read");
}

// ---------------------------------------------------------------------------
// Test 15: bytes_processed grows after each read_item
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_bytes_processed_grows_per_item() {
    let readings = make_readings(5);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(readings.clone().into_iter())
        .await
        .expect("write_all");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);

    let _first: SensorReading = dec
        .read_item()
        .await
        .expect("read first")
        .expect("Some first");
    let bytes_after_first = dec.progress().bytes_processed;
    assert!(
        bytes_after_first > 0,
        "bytes_processed must be > 0 after first item"
    );

    let _second: SensorReading = dec
        .read_item()
        .await
        .expect("read second")
        .expect("Some second");
    let bytes_after_second = dec.progress().bytes_processed;
    assert!(
        bytes_after_second > bytes_after_first,
        "bytes_processed must grow after second item (was {bytes_after_first}, now {bytes_after_second})"
    );
}

// ---------------------------------------------------------------------------
// Test 16: SensorReading with boundary values
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_reading_boundary_values() {
    let reading = SensorReading {
        sensor_id: u32::MAX,
        value: f64::MAX / 2.0,
        unit: "unit-boundary".to_string(),
        timestamp: u64::MAX,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&reading)
        .await
        .expect("write boundary reading");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: SensorReading = dec
        .read_item()
        .await
        .expect("read boundary reading")
        .expect("Some boundary reading");
    assert_eq!(reading.sensor_id, got.sensor_id);
    assert_eq!(
        reading.value.to_bits(),
        got.value.to_bits(),
        "f64 boundary value mismatch"
    );
    assert_eq!(reading.unit, got.unit);
    assert_eq!(reading.timestamp, got.timestamp);
}

// ---------------------------------------------------------------------------
// Test 17: SensorEvent::Alarm threshold boundary values
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_alarm_threshold_boundary_values() {
    let alarms = vec![
        SensorEvent::Alarm {
            sensor_id: 0,
            threshold: f64::MIN_POSITIVE,
        },
        SensorEvent::Alarm {
            sensor_id: 1,
            threshold: 0.0,
        },
        SensorEvent::Alarm {
            sensor_id: 2,
            threshold: -1.0,
        },
        SensorEvent::Alarm {
            sensor_id: 3,
            threshold: f64::MAX / 2.0,
        },
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    for a in &alarms {
        enc.write_item(a).await.expect("write alarm boundary");
    }
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorEvent> = dec.read_all().await.expect("read_all alarms");
    assert_eq!(alarms.len(), got.len());
    for (orig, decoded) in alarms.iter().zip(got.iter()) {
        if let (
            SensorEvent::Alarm {
                sensor_id: sid_o,
                threshold: thr_o,
            },
            SensorEvent::Alarm {
                sensor_id: sid_d,
                threshold: thr_d,
            },
        ) = (orig, decoded)
        {
            assert_eq!(sid_o, sid_d, "sensor_id mismatch");
            assert_eq!(
                thr_o.to_bits(),
                thr_d.to_bits(),
                "threshold f64 bits mismatch"
            );
        } else {
            panic!("expected Alarm variant");
        }
    }
}

// ---------------------------------------------------------------------------
// Test 18: write_all preserving original via clone
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_write_all_clone_preserves_original() {
    let readings = make_readings(6);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    // Clone so we can compare after consuming into iterator
    enc.write_all(readings.clone().into_iter())
        .await
        .expect("write_all cloned");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorReading> = dec.read_all().await.expect("read_all");

    // Original is still available for comparison
    assert_eq!(readings, got, "cloned write_all roundtrip mismatch");
    assert_eq!(readings.len(), 6, "original was preserved");
}

// ---------------------------------------------------------------------------
// Test 19: read_all after interleaved write_all on same pipe
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_interleaved_write_all_read_all() {
    let batch_a: Vec<SensorReading> = make_readings(4);
    let batch_b: Vec<SensorEvent> = make_mixed_events(4);

    let (ca, sa) = tokio::io::duplex(65536);
    let (cb, sb) = tokio::io::duplex(65536);

    let mut enc_a = AsyncEncoder::new(ca);
    let mut enc_b = AsyncEncoder::new(cb);

    enc_a
        .write_all(batch_a.clone().into_iter())
        .await
        .expect("write_all batch_a");
    enc_b
        .write_all(batch_b.clone().into_iter())
        .await
        .expect("write_all batch_b");

    enc_a.finish().await.expect("finish a");
    enc_b.finish().await.expect("finish b");

    let mut dec_a = AsyncDecoder::new(sa);
    let mut dec_b = AsyncDecoder::new(sb);

    let got_a: Vec<SensorReading> = dec_a.read_all().await.expect("read_all a");
    let got_b: Vec<SensorEvent> = dec_b.read_all().await.expect("read_all b");

    assert_eq!(batch_a, got_a, "batch_a mismatch");
    assert_eq!(batch_b, got_b, "batch_b mismatch");
}

// ---------------------------------------------------------------------------
// Test 20: Error handling — decoding wrong type returns Err
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_wrong_type_decode_returns_err() {
    // Encode a SensorReading (which has sensor_id: u32, value: f64, unit: String, timestamp: u64)
    // then try to decode it as a plain u8 — that decode will likely succeed trivially because
    // a single byte can always be read.  Instead encode a SensorEvent::Alarm and then decode
    // as SensorReading; the length/variant discriminant will mismatch and must return Err.
    let event = SensorEvent::Alarm {
        sensor_id: 255,
        threshold: 9_999.0,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event).await.expect("write_item alarm");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    // Attempt to decode as SensorReading; the binary layout is different so this must Err.
    let result = dec.read_item::<SensorReading>().await;
    assert!(
        result.is_err(),
        "decoding SensorEvent as SensorReading must return Err, got Ok({result:?})"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Flush-per-item config — chunks_processed matches item count
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_flush_per_item_chunks_match_item_count() {
    let config = StreamingConfig::new().with_flush_per_item(true);
    let readings = make_readings(5);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for r in &readings {
        enc.write_item(r).await.expect("write_item flush-per-item");
    }
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorReading> = dec.read_all().await.expect("read_all flush-per-item");
    assert_eq!(readings, got, "data mismatch with flush-per-item config");
    assert!(
        dec.progress().chunks_processed >= readings.len() as u64,
        "chunks_processed must be >= item count with flush_per_item (got {})",
        dec.progress().chunks_processed
    );
}

// ---------------------------------------------------------------------------
// Test 22: Concurrent encode/decode via tokio::join!
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor21_concurrent_encode_decode_join() {
    let readings = make_readings(10);
    let readings_for_enc = readings.clone();

    let (client, server) = tokio::io::duplex(65536);

    let (_, got) = tokio::join!(
        async move {
            let mut enc = AsyncEncoder::new(client);
            enc.write_all(readings_for_enc.into_iter())
                .await
                .expect("concurrent write_all");
            enc.finish().await.expect("concurrent finish");
        },
        async move {
            let mut dec = AsyncDecoder::new(server);
            dec.read_all::<SensorReading>()
                .await
                .expect("concurrent read_all")
        }
    );

    assert_eq!(readings, got, "concurrent encode/decode mismatch");
    assert_eq!(readings.len(), 10);
}
