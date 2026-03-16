//! Advanced async streaming tests (23rd set) for OxiCode — IoT device telemetry theme.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Domain types unique to this file:
//!   `DeviceStatus`   — enum { Online, Offline, Maintenance, Error { code: u32 } }
//!   `DeviceTelemetry` — struct { device_id: u64, status: DeviceStatus,
//!                                temperature_c: f32, humidity_pct: f32,
//!                                battery_pct: u8, uptime_s: u64, readings: Vec<f32> }
//!
//! Coverage matrix:
//!   1:   DeviceStatus::Online roundtrip
//!   2:   DeviceStatus::Offline roundtrip
//!   3:   DeviceStatus::Maintenance roundtrip
//!   4:   DeviceStatus::Error { code } roundtrip
//!   5:   DeviceTelemetry basic roundtrip
//!   6:   DeviceTelemetry with empty readings
//!   7:   DeviceTelemetry with many readings (20 readings)
//!   8:   write_all / read_all for Vec<DeviceTelemetry> (5 items)
//!   9:   Empty Vec write_all / read_all
//!  10:   Large batch (100 DeviceTelemetry items)
//!  11:   Progress tracking items_processed
//!  12:   bytes_processed grows with each item
//!  13:   StreamingConfig chunk_size(64)
//!  14:   finish() then read_all()
//!  15:   Sequential write_item + read_item
//!  16:   Online status with extreme values (f32::MAX temp, 100% battery)
//!  17:   Error status with u32::MAX code
//!  18:   Device with u64::MAX uptime roundtrip
//!  19:   Mixed status types in one batch
//!  20:   Clone for write_all: original unchanged after write_all
//!  21:   Wrong-type decode returns Err
//!  22:   tokio::join! concurrent encode/decode

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
enum DeviceStatus {
    Online,
    Offline,
    Maintenance,
    Error { code: u32 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DeviceTelemetry {
    device_id: u64,
    status: DeviceStatus,
    temperature_c: f32,
    humidity_pct: f32,
    battery_pct: u8,
    uptime_s: u64,
    readings: Vec<f32>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_telemetry(device_id: u64, status: DeviceStatus, readings: Vec<f32>) -> DeviceTelemetry {
    DeviceTelemetry {
        device_id,
        status,
        temperature_c: 22.5,
        humidity_pct: 55.0,
        battery_pct: 80,
        uptime_s: 3600,
        readings,
    }
}

fn make_batch(count: usize) -> Vec<DeviceTelemetry> {
    (0..count)
        .map(|i| {
            let status = match i % 4 {
                0 => DeviceStatus::Online,
                1 => DeviceStatus::Offline,
                2 => DeviceStatus::Maintenance,
                _ => DeviceStatus::Error { code: i as u32 },
            };
            DeviceTelemetry {
                device_id: i as u64,
                status,
                temperature_c: 20.0 + (i as f32) * 0.1,
                humidity_pct: 50.0 + (i as f32) * 0.2,
                battery_pct: (i % 101) as u8,
                uptime_s: (i as u64) * 60,
                readings: (0..5).map(|j| (i * 10 + j) as f32).collect(),
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Test 1: DeviceStatus::Online roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_device_status_online_roundtrip() {
    let status = DeviceStatus::Online;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&status)
        .await
        .expect("write_item DeviceStatus::Online");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: DeviceStatus = dec
        .read_item()
        .await
        .expect("read_item DeviceStatus::Online")
        .expect("expected Some(DeviceStatus::Online)");
    assert_eq!(status, got, "DeviceStatus::Online roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 2: DeviceStatus::Offline roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_device_status_offline_roundtrip() {
    let status = DeviceStatus::Offline;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&status)
        .await
        .expect("write_item DeviceStatus::Offline");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: DeviceStatus = dec
        .read_item()
        .await
        .expect("read_item DeviceStatus::Offline")
        .expect("expected Some(DeviceStatus::Offline)");
    assert_eq!(status, got, "DeviceStatus::Offline roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 3: DeviceStatus::Maintenance roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_device_status_maintenance_roundtrip() {
    let status = DeviceStatus::Maintenance;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&status)
        .await
        .expect("write_item DeviceStatus::Maintenance");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: DeviceStatus = dec
        .read_item()
        .await
        .expect("read_item DeviceStatus::Maintenance")
        .expect("expected Some(DeviceStatus::Maintenance)");
    assert_eq!(status, got, "DeviceStatus::Maintenance roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 4: DeviceStatus::Error { code } roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_device_status_error_roundtrip() {
    let status = DeviceStatus::Error { code: 0xDEAD_BEEF };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&status)
        .await
        .expect("write_item DeviceStatus::Error");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: DeviceStatus = dec
        .read_item()
        .await
        .expect("read_item DeviceStatus::Error")
        .expect("expected Some(DeviceStatus::Error)");
    assert_eq!(status, got, "DeviceStatus::Error roundtrip mismatch");
    if let DeviceStatus::Error { code } = &got {
        assert_eq!(*code, 0xDEAD_BEEF, "Error code mismatch");
    } else {
        panic!("expected DeviceStatus::Error variant");
    }
}

// ---------------------------------------------------------------------------
// Test 5: DeviceTelemetry basic roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_device_telemetry_basic_roundtrip() {
    let telemetry = DeviceTelemetry {
        device_id: 42,
        status: DeviceStatus::Online,
        temperature_c: 23.7,
        humidity_pct: 60.5,
        battery_pct: 95,
        uptime_s: 86400,
        readings: vec![1.1, 2.2, 3.3],
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&telemetry)
        .await
        .expect("write_item DeviceTelemetry basic");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: DeviceTelemetry = dec
        .read_item()
        .await
        .expect("read_item DeviceTelemetry basic")
        .expect("expected Some(DeviceTelemetry)");
    assert_eq!(telemetry, got, "DeviceTelemetry basic roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 6: DeviceTelemetry with empty readings
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_device_telemetry_empty_readings() {
    let telemetry = make_telemetry(100, DeviceStatus::Offline, vec![]);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&telemetry)
        .await
        .expect("write_item DeviceTelemetry empty readings");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: DeviceTelemetry = dec
        .read_item()
        .await
        .expect("read_item DeviceTelemetry empty readings")
        .expect("expected Some(DeviceTelemetry)");
    assert_eq!(
        telemetry, got,
        "DeviceTelemetry empty readings roundtrip mismatch"
    );
    assert!(got.readings.is_empty(), "readings must be empty");
}

// ---------------------------------------------------------------------------
// Test 7: DeviceTelemetry with many readings (20 readings)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_device_telemetry_many_readings() {
    let readings: Vec<f32> = (0..20).map(|i| i as f32 * 0.5).collect();
    let telemetry = make_telemetry(200, DeviceStatus::Online, readings.clone());
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&telemetry)
        .await
        .expect("write_item DeviceTelemetry many readings");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: DeviceTelemetry = dec
        .read_item()
        .await
        .expect("read_item DeviceTelemetry many readings")
        .expect("expected Some(DeviceTelemetry)");
    assert_eq!(
        telemetry, got,
        "DeviceTelemetry many readings roundtrip mismatch"
    );
    assert_eq!(got.readings.len(), 20, "must have exactly 20 readings");
    for (orig, decoded) in readings.iter().zip(got.readings.iter()) {
        assert_eq!(
            orig.to_bits(),
            decoded.to_bits(),
            "f32 reading bit mismatch: {orig} vs {decoded}"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 8: write_all / read_all for Vec<DeviceTelemetry> (5 items)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_vec_telemetry_write_all_read_all_5_items() {
    let items: Vec<DeviceTelemetry> = vec![
        make_telemetry(1, DeviceStatus::Online, vec![1.0, 2.0]),
        make_telemetry(2, DeviceStatus::Offline, vec![]),
        make_telemetry(3, DeviceStatus::Maintenance, vec![3.0]),
        make_telemetry(4, DeviceStatus::Error { code: 404 }, vec![4.0, 5.0, 6.0]),
        make_telemetry(5, DeviceStatus::Online, vec![7.0, 8.0, 9.0, 10.0]),
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(items.clone().into_iter())
        .await
        .expect("write_all 5 telemetry items");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<DeviceTelemetry> = dec.read_all().await.expect("read_all 5 telemetry items");
    assert_eq!(items, got, "write_all/read_all 5-item roundtrip mismatch");
    assert_eq!(got.len(), 5, "must have exactly 5 items");
}

// ---------------------------------------------------------------------------
// Test 9: Empty Vec write_all / read_all
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_empty_vec_write_all_read_all() {
    let empty: Vec<DeviceTelemetry> = vec![];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(empty.clone().into_iter())
        .await
        .expect("write_all empty telemetry");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<DeviceTelemetry> = dec.read_all().await.expect("read_all empty telemetry");
    assert!(
        got.is_empty(),
        "expected empty Vec from write_all of 0 items"
    );
    assert!(
        dec.is_finished(),
        "decoder must be finished after empty stream"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Large batch (100 DeviceTelemetry items)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_large_batch_100_items() {
    let batch = make_batch(100);
    assert_eq!(batch.len(), 100, "must generate exactly 100 items");

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(batch.clone().into_iter())
        .await
        .expect("write_all 100 telemetry items");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<DeviceTelemetry> = dec.read_all().await.expect("read_all 100 telemetry items");
    assert_eq!(batch.len(), got.len(), "count mismatch for 100-item batch");
    assert_eq!(batch, got, "data mismatch for 100-item large batch");
}

// ---------------------------------------------------------------------------
// Test 11: Progress tracking items_processed
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_progress_items_processed() {
    const N: u64 = 17;
    let batch = make_batch(N as usize);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.set_estimated_total(N);
    enc.write_all(batch.clone().into_iter())
        .await
        .expect("write_all for progress tracking");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let _: Vec<DeviceTelemetry> = dec
        .read_all()
        .await
        .expect("read_all for progress tracking");
    assert_eq!(
        dec.progress().items_processed,
        N,
        "items_processed must equal {N} after reading all items"
    );
    assert!(
        dec.progress().bytes_processed > 0,
        "bytes_processed must be > 0 after reading {N} telemetry items"
    );
}

// ---------------------------------------------------------------------------
// Test 12: bytes_processed grows with each item
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_bytes_processed_grows_with_each_item() {
    let items = make_batch(5);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(items.clone().into_iter())
        .await
        .expect("write_all for bytes_processed check");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);

    let _first: DeviceTelemetry = dec
        .read_item()
        .await
        .expect("read first item for bytes check")
        .expect("expected Some(DeviceTelemetry) for first item");
    let bytes_after_first = dec.progress().bytes_processed;
    assert!(
        bytes_after_first > 0,
        "bytes_processed must be > 0 after reading first item"
    );

    let _second: DeviceTelemetry = dec
        .read_item()
        .await
        .expect("read second item for bytes check")
        .expect("expected Some(DeviceTelemetry) for second item");
    let bytes_after_second = dec.progress().bytes_processed;
    assert!(
        bytes_after_second > bytes_after_first,
        "bytes_processed must grow after reading second item (was {bytes_after_first}, now {bytes_after_second})"
    );

    // Drain remaining and confirm further growth
    let rest: Vec<DeviceTelemetry> = dec.read_all().await.expect("read_all remaining items");
    assert_eq!(rest.len(), 3, "must have 3 remaining items after reading 2");
    assert!(
        dec.progress().bytes_processed > bytes_after_second,
        "bytes_processed must grow further after reading all remaining items"
    );
}

// ---------------------------------------------------------------------------
// Test 13: StreamingConfig chunk_size(64)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_streaming_config_chunk_size_64() {
    let config = StreamingConfig::new().with_chunk_size(64);
    let batch = make_batch(15);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for item in &batch {
        enc.write_item(item)
            .await
            .expect("write_item with chunk_size 64");
    }
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<DeviceTelemetry> = dec.read_all().await.expect("read_all with chunk_size 64");
    assert_eq!(
        batch, got,
        "data integrity with chunk_size(64) config mismatch"
    );
    assert!(
        dec.progress().chunks_processed > 0,
        "chunks_processed must be > 0 with small chunk_size 64"
    );
}

// ---------------------------------------------------------------------------
// Test 14: finish() then read_all()
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_finish_then_read_all() {
    let batch = make_batch(8);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(batch.clone().into_iter())
        .await
        .expect("write_all before explicit finish");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<DeviceTelemetry> = dec.read_all().await.expect("read_all after finish");
    assert_eq!(batch, got, "finish then read_all roundtrip mismatch");

    // Stream must be exhausted — further reads must return None
    let extra: Option<DeviceTelemetry> = dec
        .read_item()
        .await
        .expect("read after exhaustion must not error");
    assert_eq!(extra, None, "must return None after stream is exhausted");
    assert!(
        dec.is_finished(),
        "decoder must report is_finished after stream exhausted"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Sequential write_item + read_item
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_sequential_write_item_read_item() {
    let a = make_telemetry(10, DeviceStatus::Online, vec![1.0]);
    let b = make_telemetry(11, DeviceStatus::Maintenance, vec![2.0, 3.0]);
    let c = make_telemetry(12, DeviceStatus::Error { code: 500 }, vec![]);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&a).await.expect("write_item a");
    enc.write_item(&b).await.expect("write_item b");
    enc.write_item(&c).await.expect("write_item c");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got_a: DeviceTelemetry = dec
        .read_item()
        .await
        .expect("read_item a")
        .expect("expected Some(DeviceTelemetry) for a");
    let got_b: DeviceTelemetry = dec
        .read_item()
        .await
        .expect("read_item b")
        .expect("expected Some(DeviceTelemetry) for b");
    let got_c: DeviceTelemetry = dec
        .read_item()
        .await
        .expect("read_item c")
        .expect("expected Some(DeviceTelemetry) for c");
    let eof: Option<DeviceTelemetry> = dec.read_item().await.expect("read eof must not error");

    assert_eq!(a, got_a, "sequential item a mismatch");
    assert_eq!(b, got_b, "sequential item b mismatch");
    assert_eq!(c, got_c, "sequential item c mismatch");
    assert_eq!(eof, None, "must return None after all items read");
}

// ---------------------------------------------------------------------------
// Test 16: Online status with extreme values (f32::MAX temp, 100% battery)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_online_status_extreme_values() {
    let telemetry = DeviceTelemetry {
        device_id: u64::MAX / 2,
        status: DeviceStatus::Online,
        temperature_c: f32::MAX,
        humidity_pct: 100.0,
        battery_pct: 100,
        uptime_s: u64::MAX / 2,
        readings: vec![f32::MAX, f32::MIN_POSITIVE, 0.0],
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&telemetry)
        .await
        .expect("write_item extreme values telemetry");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: DeviceTelemetry = dec
        .read_item()
        .await
        .expect("read_item extreme values telemetry")
        .expect("expected Some(DeviceTelemetry) with extreme values");
    assert_eq!(telemetry.device_id, got.device_id, "device_id mismatch");
    assert_eq!(telemetry.status, got.status, "status mismatch");
    assert_eq!(
        telemetry.temperature_c.to_bits(),
        got.temperature_c.to_bits(),
        "f32::MAX temperature bit mismatch"
    );
    assert_eq!(
        telemetry.battery_pct, got.battery_pct,
        "battery_pct mismatch"
    );
    assert_eq!(
        telemetry.readings.len(),
        got.readings.len(),
        "readings length mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Error status with u32::MAX code
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_error_status_u32_max_code() {
    let telemetry = DeviceTelemetry {
        device_id: 9999,
        status: DeviceStatus::Error { code: u32::MAX },
        temperature_c: -40.0,
        humidity_pct: 0.0,
        battery_pct: 0,
        uptime_s: 0,
        readings: vec![],
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&telemetry)
        .await
        .expect("write_item Error u32::MAX");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: DeviceTelemetry = dec
        .read_item()
        .await
        .expect("read_item Error u32::MAX")
        .expect("expected Some(DeviceTelemetry) with u32::MAX error code");
    assert_eq!(telemetry, got, "Error u32::MAX roundtrip mismatch");
    if let DeviceStatus::Error { code } = &got.status {
        assert_eq!(
            *code,
            u32::MAX,
            "u32::MAX error code must survive roundtrip"
        );
    } else {
        panic!("expected DeviceStatus::Error variant with u32::MAX code");
    }
}

// ---------------------------------------------------------------------------
// Test 18: Device with u64::MAX uptime roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_device_u64_max_uptime_roundtrip() {
    let telemetry = DeviceTelemetry {
        device_id: u64::MAX,
        status: DeviceStatus::Online,
        temperature_c: 25.0,
        humidity_pct: 50.0,
        battery_pct: 50,
        uptime_s: u64::MAX,
        readings: vec![0.0, 1.0],
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&telemetry)
        .await
        .expect("write_item u64::MAX uptime");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: DeviceTelemetry = dec
        .read_item()
        .await
        .expect("read_item u64::MAX uptime")
        .expect("expected Some(DeviceTelemetry) with u64::MAX uptime");
    assert_eq!(telemetry, got, "u64::MAX uptime roundtrip mismatch");
    assert_eq!(got.device_id, u64::MAX, "device_id u64::MAX mismatch");
    assert_eq!(got.uptime_s, u64::MAX, "uptime_s u64::MAX mismatch");
}

// ---------------------------------------------------------------------------
// Test 19: Mixed status types in one batch
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_mixed_status_types_in_one_batch() {
    let items = vec![
        make_telemetry(1, DeviceStatus::Online, vec![1.0]),
        make_telemetry(2, DeviceStatus::Offline, vec![2.0, 3.0]),
        make_telemetry(3, DeviceStatus::Maintenance, vec![]),
        make_telemetry(4, DeviceStatus::Error { code: 1001 }, vec![4.0]),
        make_telemetry(5, DeviceStatus::Error { code: 0 }, vec![5.0, 6.0]),
        make_telemetry(6, DeviceStatus::Online, vec![7.0, 8.0, 9.0]),
        make_telemetry(7, DeviceStatus::Maintenance, vec![10.0]),
        make_telemetry(8, DeviceStatus::Offline, vec![]),
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(items.clone().into_iter())
        .await
        .expect("write_all mixed status batch");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<DeviceTelemetry> = dec.read_all().await.expect("read_all mixed status batch");
    assert_eq!(items, got, "mixed status batch roundtrip mismatch");
    assert_eq!(got.len(), 8, "must have 8 items in mixed status batch");

    // Spot-check each status variant
    assert_eq!(got[0].status, DeviceStatus::Online, "item 0 must be Online");
    assert_eq!(
        got[1].status,
        DeviceStatus::Offline,
        "item 1 must be Offline"
    );
    assert_eq!(
        got[2].status,
        DeviceStatus::Maintenance,
        "item 2 must be Maintenance"
    );
    assert!(
        matches!(got[3].status, DeviceStatus::Error { code: 1001 }),
        "item 3 must be Error {{ code: 1001 }}"
    );
    assert!(
        matches!(got[4].status, DeviceStatus::Error { code: 0 }),
        "item 4 must be Error {{ code: 0 }}"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Clone for write_all: original unchanged after write_all
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_clone_for_write_all_original_unchanged() {
    let original: Vec<DeviceTelemetry> = make_batch(10);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    // Pass clone into write_all so original is not moved
    enc.write_all(original.clone().into_iter())
        .await
        .expect("write_all with clone");
    enc.finish().await.expect("finish");

    // Original must still be accessible and unchanged
    assert_eq!(
        original.len(),
        10,
        "original must still have 10 items after write_all with clone"
    );

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<DeviceTelemetry> = dec
        .read_all()
        .await
        .expect("read_all after clone write_all");
    assert_eq!(original, got, "clone write_all roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 21: Wrong-type decode returns Err
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_wrong_type_decode_returns_err() {
    // Encode a DeviceTelemetry (complex struct with nested enum + Vec<f32>)
    // then attempt to decode as DeviceStatus — the binary layouts are incompatible
    // and must return Err rather than silently succeeding.
    let telemetry = DeviceTelemetry {
        device_id: 7777,
        status: DeviceStatus::Error { code: 12345 },
        temperature_c: 30.5,
        humidity_pct: 75.0,
        battery_pct: 20,
        uptime_s: 123456789,
        readings: vec![1.1, 2.2, 3.3, 4.4, 5.5],
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&telemetry)
        .await
        .expect("write_item DeviceTelemetry for wrong-type test");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    // Attempt to decode DeviceTelemetry as DeviceStatus — must fail with Err.
    let result = dec.read_item::<DeviceStatus>().await;
    assert!(
        result.is_err(),
        "decoding DeviceTelemetry as DeviceStatus must return Err, got Ok({result:?})"
    );
}

// ---------------------------------------------------------------------------
// Test 22: tokio::join! concurrent encode/decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_iot23_concurrent_encode_decode_join() {
    let batch = make_batch(22);
    let batch_for_enc = batch.clone();

    let (client, server) = tokio::io::duplex(65536);

    let (_, got) = tokio::join!(
        async move {
            let mut enc = AsyncEncoder::new(client);
            enc.write_all(batch_for_enc.into_iter())
                .await
                .expect("concurrent write_all");
            enc.finish().await.expect("concurrent finish");
        },
        async move {
            let mut dec = AsyncDecoder::new(server);
            dec.read_all::<DeviceTelemetry>()
                .await
                .expect("concurrent read_all")
        }
    );

    assert_eq!(batch, got, "concurrent encode/decode roundtrip mismatch");
    assert_eq!(
        got.len(),
        22,
        "must have decoded all 22 telemetry items concurrently"
    );
}
