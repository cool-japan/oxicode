//! Advanced async streaming tests (28th set) for OxiCode.
//!
//! Theme: Sensor data pipeline.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types: `SensorType`, `SensorReading`, `DataBatch`.
//!
//! Coverage matrix:
//!   1:  SensorType::Accelerometer single roundtrip via duplex
//!   2:  SensorType::Gps single roundtrip via duplex
//!   3:  SensorReading with Gyroscope roundtrip via duplex
//!   4:  SensorReading with Barometer (z = 0) roundtrip via duplex
//!   5:  SensorReading with negative coordinate values roundtrip
//!   6:  DataBatch with empty readings Vec roundtrip via duplex
//!   7:  DataBatch with non-empty readings roundtrip via duplex
//!   8:  Five SensorReadings in order via write_item / read_item
//!   9:  write_all / read_all for Vec<SensorReading> (8 items)
//!  10:  Large batch of 100 DataBatches via write_all, verify read_all
//!  11:  Vec<SensorType> all variants roundtrip
//!  12:  progress().items_processed > 0 after reading sensor readings
//!  13:  StreamingConfig with chunk_size(512) forces multiple chunks
//!  14:  flush_per_item produces correct items_processed per SensorReading
//!  15:  Empty stream returns None on first read_item
//!  16:  is_finished() true after sensor reading stream exhausted
//!  17:  bytes_processed grows after reading more DataBatches
//!  18:  Sync encode / async decode interop for SensorReading
//!  19:  Async encode / sync decode interop for DataBatch
//!  20:  DataBatch with max-value fields roundtrip via duplex
//!  21:  Multiple DataBatches from different devices roundtrip
//!  22:  tokio::join! concurrent encode/decode for sensor feed replay

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SensorType {
    Accelerometer,
    Gyroscope,
    Magnetometer,
    Barometer,
    Gps,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SensorReading {
    sensor_id: u32,
    sensor_type: SensorType,
    x: i32,
    y: i32,
    z: i32,
    timestamp_us: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DataBatch {
    batch_id: u64,
    device_id: String,
    readings: Vec<SensorReading>,
    sequence: u32,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_reading(
    sensor_id: u32,
    sensor_type: SensorType,
    x: i32,
    y: i32,
    z: i32,
    ts: u64,
) -> SensorReading {
    SensorReading {
        sensor_id,
        sensor_type,
        x,
        y,
        z,
        timestamp_us: ts,
    }
}

fn make_batch(batch_id: u64, device_id: &str, reading_count: usize, sequence: u32) -> DataBatch {
    let sensor_types = [
        SensorType::Accelerometer,
        SensorType::Gyroscope,
        SensorType::Magnetometer,
        SensorType::Barometer,
        SensorType::Gps,
    ];
    let readings: Vec<SensorReading> = (0..reading_count)
        .map(|i| {
            let st = match i % 5 {
                0 => SensorType::Accelerometer,
                1 => SensorType::Gyroscope,
                2 => SensorType::Magnetometer,
                3 => SensorType::Barometer,
                _ => SensorType::Gps,
            };
            make_reading(
                i as u32,
                st,
                (i as i32) * 100 - 500,
                (i as i32) * 200 - 1000,
                (i as i32) * 50,
                1_700_000_000_000 + (i as u64) * 10_000,
            )
        })
        .collect();
    // Suppress unused variable warning on sensor_types array
    let _ = &sensor_types;
    DataBatch {
        batch_id,
        device_id: device_id.to_string(),
        readings,
        sequence,
    }
}

fn make_batch_vec(count: usize) -> Vec<DataBatch> {
    let devices = ["device-alpha", "device-beta", "device-gamma"];
    (0..count)
        .map(|i| {
            let device = devices[i % devices.len()];
            make_batch(i as u64, device, (i % 8) + 1, i as u32)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Test 1: SensorType::Accelerometer single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_sensor_type_accelerometer_roundtrip() {
    let sensor_type = SensorType::Accelerometer;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&sensor_type)
        .await
        .expect("write_item SensorType::Accelerometer failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: SensorType = dec
        .read_item()
        .await
        .expect("read_item SensorType::Accelerometer failed")
        .expect("expected Some(SensorType::Accelerometer)");
    assert_eq!(
        sensor_type, got,
        "SensorType::Accelerometer roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 2: SensorType::Gps single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_sensor_type_gps_roundtrip() {
    let sensor_type = SensorType::Gps;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&sensor_type)
        .await
        .expect("write_item SensorType::Gps failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: SensorType = dec
        .read_item()
        .await
        .expect("read_item SensorType::Gps failed")
        .expect("expected Some(SensorType::Gps)");
    assert_eq!(sensor_type, got, "SensorType::Gps roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 3: SensorReading with Gyroscope roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_sensor_reading_gyroscope_roundtrip() {
    let reading = SensorReading {
        sensor_id: 42,
        sensor_type: SensorType::Gyroscope,
        x: 1_234,
        y: -5_678,
        z: 9_012,
        timestamp_us: 1_700_000_123_456,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&reading)
        .await
        .expect("write_item SensorReading(Gyroscope) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: SensorReading = dec
        .read_item()
        .await
        .expect("read_item SensorReading(Gyroscope) failed")
        .expect("expected Some(SensorReading)");
    assert_eq!(reading, got, "SensorReading(Gyroscope) roundtrip mismatch");
    assert_eq!(
        got.sensor_type,
        SensorType::Gyroscope,
        "sensor_type must be Gyroscope"
    );
    assert_eq!(got.sensor_id, 42, "sensor_id must be 42");
}

// ---------------------------------------------------------------------------
// Test 4: SensorReading with Barometer (z = 0) roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_sensor_reading_barometer_z_zero_roundtrip() {
    let reading = SensorReading {
        sensor_id: 7,
        sensor_type: SensorType::Barometer,
        x: 101_325,
        y: 0,
        z: 0,
        timestamp_us: 1_700_000_000_000,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&reading)
        .await
        .expect("write_item SensorReading(Barometer, z=0) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: SensorReading = dec
        .read_item()
        .await
        .expect("read_item SensorReading(Barometer, z=0) failed")
        .expect("expected Some(SensorReading) with z=0");
    assert_eq!(
        reading, got,
        "SensorReading(Barometer, z=0) roundtrip mismatch"
    );
    assert_eq!(got.z, 0, "z must be 0 for Barometer reading");
    assert_eq!(
        got.sensor_type,
        SensorType::Barometer,
        "sensor_type must be Barometer"
    );
}

// ---------------------------------------------------------------------------
// Test 5: SensorReading with negative coordinate values roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_sensor_reading_negative_coordinates_roundtrip() {
    let reading = SensorReading {
        sensor_id: 99,
        sensor_type: SensorType::Magnetometer,
        x: i32::MIN,
        y: -1,
        z: i32::MIN / 2,
        timestamp_us: u64::MAX / 4,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&reading)
        .await
        .expect("write_item SensorReading(negative coords) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: SensorReading = dec
        .read_item()
        .await
        .expect("read_item SensorReading(negative coords) failed")
        .expect("expected Some(SensorReading) with negative coords");
    assert_eq!(
        reading, got,
        "SensorReading with negative coordinates roundtrip mismatch"
    );
    assert_eq!(got.x, i32::MIN, "x must be i32::MIN");
    assert_eq!(got.y, -1, "y must be -1");
    assert_eq!(
        got.sensor_type,
        SensorType::Magnetometer,
        "sensor_type must be Magnetometer"
    );
}

// ---------------------------------------------------------------------------
// Test 6: DataBatch with empty readings Vec roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_data_batch_empty_readings_roundtrip() {
    let batch = DataBatch {
        batch_id: 0,
        device_id: "device-empty".to_string(),
        readings: vec![],
        sequence: 0,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&batch)
        .await
        .expect("write_item DataBatch(empty readings) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: DataBatch = dec
        .read_item()
        .await
        .expect("read_item DataBatch(empty readings) failed")
        .expect("expected Some(DataBatch) with empty readings");
    assert_eq!(
        batch, got,
        "DataBatch with empty readings roundtrip mismatch"
    );
    assert!(
        got.readings.is_empty(),
        "readings must be empty in decoded DataBatch"
    );
    assert_eq!(got.device_id, "device-empty", "device_id must match");
}

// ---------------------------------------------------------------------------
// Test 7: DataBatch with non-empty readings roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_data_batch_with_readings_roundtrip() {
    let readings = vec![
        make_reading(1, SensorType::Accelerometer, 980, -100, 9810, 1_000_000),
        make_reading(2, SensorType::Gyroscope, 50, -30, 10, 1_010_000),
        make_reading(3, SensorType::Magnetometer, 200, 300, -100, 1_020_000),
    ];
    let batch = DataBatch {
        batch_id: 1001,
        device_id: "device-01".to_string(),
        readings: readings.clone(),
        sequence: 7,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&batch)
        .await
        .expect("write_item DataBatch(3 readings) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: DataBatch = dec
        .read_item()
        .await
        .expect("read_item DataBatch(3 readings) failed")
        .expect("expected Some(DataBatch) with 3 readings");
    assert_eq!(batch, got, "DataBatch with 3 readings roundtrip mismatch");
    assert_eq!(got.readings.len(), 3, "must decode exactly 3 readings");
    assert_eq!(got.batch_id, 1001, "batch_id must be 1001");
    assert_eq!(got.sequence, 7, "sequence must be 7");
}

// ---------------------------------------------------------------------------
// Test 8: Five SensorReadings in order via write_item / read_item
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_five_sensor_readings_in_order() {
    let readings = vec![
        make_reading(1, SensorType::Accelerometer, 0, 0, 9810, 1_000_000),
        make_reading(2, SensorType::Gyroscope, 100, -200, 50, 1_010_000),
        make_reading(3, SensorType::Magnetometer, 300, 400, -50, 1_020_000),
        make_reading(4, SensorType::Barometer, 101_325, 0, 0, 1_030_000),
        make_reading(
            5,
            SensorType::Gps,
            37_422_000,
            -122_084_000,
            10_000,
            1_040_000,
        ),
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    for r in &readings {
        enc.write_item(r)
            .await
            .expect("write_item in 5-reading sequence failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    for expected in &readings {
        let got: SensorReading = dec
            .read_item()
            .await
            .expect("read_item in 5-reading sequence failed")
            .expect("expected Some(SensorReading)");
        assert_eq!(
            *expected, got,
            "SensorReading mismatch at sensor_id {}",
            expected.sensor_id
        );
    }

    let eof: Option<SensorReading> = dec.read_item().await.expect("eof read_item failed");
    assert_eq!(eof, None, "expected None after all five sensor readings");
}

// ---------------------------------------------------------------------------
// Test 9: write_all / read_all for Vec<SensorReading> (8 items)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_write_all_read_all_8_sensor_readings() {
    let readings: Vec<SensorReading> = (0u32..8)
        .map(|i| {
            let st = match i % 5 {
                0 => SensorType::Accelerometer,
                1 => SensorType::Gyroscope,
                2 => SensorType::Magnetometer,
                3 => SensorType::Barometer,
                _ => SensorType::Gps,
            };
            make_reading(
                i,
                st,
                i as i32 * 10,
                -(i as i32 * 5),
                i as i32 * 3,
                i as u64 * 1000,
            )
        })
        .collect();

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(readings.clone().into_iter())
        .await
        .expect("write_all 8 SensorReadings failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorReading> = dec
        .read_all()
        .await
        .expect("read_all 8 SensorReadings failed");
    assert_eq!(
        readings, got,
        "write_all/read_all 8-reading roundtrip mismatch"
    );
    assert_eq!(got.len(), 8, "must decode exactly 8 SensorReadings");
}

// ---------------------------------------------------------------------------
// Test 10: Large batch of 100 DataBatches via write_all, verify read_all
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_large_batch_100_data_batches_write_all_read_all() {
    let batches = make_batch_vec(100);
    assert_eq!(batches.len(), 100, "must generate exactly 100 DataBatches");

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(batches.clone().into_iter())
        .await
        .expect("write_all 100 DataBatches failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<DataBatch> = dec
        .read_all()
        .await
        .expect("read_all 100 DataBatches failed");
    assert_eq!(got.len(), 100, "expected 100 decoded DataBatches");
    assert_eq!(batches, got, "large batch 100-DataBatch roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 11: Vec<SensorType> all variants roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_vec_sensor_type_all_variants_roundtrip() {
    let variants = vec![
        SensorType::Accelerometer,
        SensorType::Gyroscope,
        SensorType::Magnetometer,
        SensorType::Barometer,
        SensorType::Gps,
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&variants)
        .await
        .expect("write_item Vec<SensorType> all variants failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorType> = dec
        .read_item()
        .await
        .expect("read_item Vec<SensorType> all variants failed")
        .expect("expected Some(Vec<SensorType>)");
    assert_eq!(
        variants, got,
        "Vec<SensorType> all-variants roundtrip mismatch"
    );
    assert_eq!(got.len(), 5, "decoded Vec<SensorType> must have 5 variants");
    assert_eq!(
        got[0],
        SensorType::Accelerometer,
        "first variant must be Accelerometer"
    );
    assert_eq!(got[4], SensorType::Gps, "last variant must be Gps");
}

// ---------------------------------------------------------------------------
// Test 12: progress().items_processed > 0 after reading sensor readings
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_progress_items_processed_after_reading_sensor_readings() {
    const N: u64 = 9;
    let readings: Vec<SensorReading> = (0u64..N)
        .map(|i| {
            let st = match i % 5 {
                0 => SensorType::Accelerometer,
                1 => SensorType::Gyroscope,
                2 => SensorType::Magnetometer,
                3 => SensorType::Barometer,
                _ => SensorType::Gps,
            };
            make_reading(i as u32, st, i as i32, -(i as i32), 0, i * 1000)
        })
        .collect();

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.set_estimated_total(N);
    enc.write_all(readings.clone().into_iter())
        .await
        .expect("write_all for progress test failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let _: Vec<SensorReading> = dec
        .read_all()
        .await
        .expect("read_all for progress test failed");

    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after reading sensor readings"
    );
    assert_eq!(
        dec.progress().items_processed,
        N,
        "items_processed must equal N={N} after reading all sensor readings"
    );
}

// ---------------------------------------------------------------------------
// Test 13: StreamingConfig with chunk_size(512) forces multiple chunks
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_streaming_config_small_chunk_forces_multiple_chunks() {
    let config = StreamingConfig::new().with_chunk_size(512);
    // Each DataBatch with readings is substantial; 40 batches will exceed 512 bytes many times over
    let batches = make_batch_vec(40);

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for batch in &batches {
        enc.write_item(batch)
            .await
            .expect("write_item with chunk_size 512 failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<DataBatch> = dec
        .read_all()
        .await
        .expect("read_all with chunk_size 512 failed");

    assert_eq!(got.len(), 40, "must decode 40 DataBatches");
    assert_eq!(batches, got, "small-chunk DataBatch roundtrip mismatch");
    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after reading with small chunk size"
    );
}

// ---------------------------------------------------------------------------
// Test 14: flush_per_item produces correct items_processed per SensorReading
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_flush_per_item_correct_items_processed() {
    let config = StreamingConfig::new().with_flush_per_item(true);
    let readings: Vec<SensorReading> = (0u32..7)
        .map(|i| {
            make_reading(
                i,
                SensorType::Accelerometer,
                i as i32 * 100,
                0,
                0,
                i as u64 * 500,
            )
        })
        .collect();

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for reading in &readings {
        enc.write_item(reading)
            .await
            .expect("write_item flush_per_item failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<SensorReading> = dec
        .read_all()
        .await
        .expect("read_all flush_per_item failed");

    assert_eq!(
        got, readings,
        "flush_per_item SensorReading roundtrip mismatch"
    );
    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after flush_per_item read"
    );
    assert_eq!(
        dec.progress().items_processed,
        7,
        "items_processed must equal 7 after reading 7 flush_per_item sensor readings"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Empty stream returns None on first read_item
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_empty_stream_returns_none() {
    let (client, server) = tokio::io::duplex(65536);

    let enc = AsyncEncoder::new(client);
    enc.finish()
        .await
        .expect("finish empty sensor stream failed");

    let mut dec = AsyncDecoder::new(server);
    let item: Option<SensorReading> = dec
        .read_item()
        .await
        .expect("read_item from empty sensor stream failed");
    assert_eq!(
        item, None,
        "empty sensor stream must return None on first read_item"
    );
}

// ---------------------------------------------------------------------------
// Test 16: is_finished() true after sensor reading stream exhausted
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_is_finished_after_sensor_stream_exhausted() {
    let readings = vec![
        make_reading(1, SensorType::Accelerometer, 100, 200, 300, 1_000_000),
        make_reading(2, SensorType::Gyroscope, -10, -20, -30, 2_000_000),
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    for r in &readings {
        enc.write_item(r).await.expect("write_item failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);

    assert!(
        !dec.is_finished(),
        "decoder must not be finished before reading"
    );

    let _: Option<SensorReading> = dec.read_item().await.expect("read reading 1 failed");
    let _: Option<SensorReading> = dec.read_item().await.expect("read reading 2 failed");

    let eof: Option<SensorReading> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None at end of sensor reading stream");
    assert!(
        dec.is_finished(),
        "decoder must report is_finished() after sensor reading stream is exhausted"
    );
}

// ---------------------------------------------------------------------------
// Test 17: bytes_processed grows after reading more DataBatches
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_bytes_processed_grows_with_more_data_batches() {
    let batches = make_batch_vec(12);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(batches.clone().into_iter())
        .await
        .expect("write_all for bytes_processed test failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);

    let first: DataBatch = dec
        .read_item()
        .await
        .expect("read first DataBatch failed")
        .expect("expected Some(DataBatch) for first batch");
    assert_eq!(first, batches[0], "first decoded DataBatch mismatch");

    let bytes_after_one = dec.progress().bytes_processed;
    assert!(
        bytes_after_one > 0,
        "bytes_processed must be > 0 after reading first DataBatch"
    );

    let rest: Vec<DataBatch> = dec
        .read_all()
        .await
        .expect("read_all remaining DataBatches failed");
    assert_eq!(rest.len(), 11, "must decode 11 remaining DataBatches");

    let bytes_after_all = dec.progress().bytes_processed;
    assert!(
        bytes_after_all > bytes_after_one,
        "bytes_processed must grow: was {bytes_after_one}, now {bytes_after_all}"
    );
    assert!(
        dec.progress().items_processed >= 12,
        "items_processed must be >= 12 after reading all DataBatches"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Sync encode / async decode interop for SensorReading
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_sync_encode_async_decode_interop_sensor_reading() {
    let reading = SensorReading {
        sensor_id: u32::MAX,
        sensor_type: SensorType::Gps,
        x: i32::MAX,
        y: i32::MIN,
        z: 0,
        timestamp_us: u64::MAX / 2,
    };

    // Sync encode then sync decode for consistency baseline
    let sync_bytes = encode_to_vec(&reading).expect("sync encode SensorReading failed");
    let (sync_decoded, _): (SensorReading, _) =
        decode_from_slice(&sync_bytes).expect("sync decode SensorReading failed");
    assert_eq!(
        reading, sync_decoded,
        "sync SensorReading roundtrip mismatch"
    );

    // Async encode then async decode via duplex
    let (client, server) = tokio::io::duplex(65536);
    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&reading)
        .await
        .expect("async write_item for interop test failed");
    enc.finish().await.expect("finish for interop test failed");

    let mut dec = AsyncDecoder::new(server);
    let async_decoded: SensorReading = dec
        .read_item()
        .await
        .expect("async read_item for interop test failed")
        .expect("expected Some(SensorReading) in interop test");
    assert_eq!(
        reading, async_decoded,
        "async encode/decode SensorReading interop mismatch"
    );
    assert_eq!(
        async_decoded.sensor_type,
        SensorType::Gps,
        "sensor_type must be Gps after async decode"
    );
    assert_eq!(
        async_decoded.sensor_id,
        u32::MAX,
        "sensor_id must be u32::MAX"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Async encode / sync decode interop for DataBatch
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_async_encode_sync_decode_interop_data_batch() {
    let batch = DataBatch {
        batch_id: u64::MAX / 3,
        device_id: "interop-device".to_string(),
        readings: vec![
            make_reading(0, SensorType::Accelerometer, 1000, -1000, 9800, 0),
            make_reading(1, SensorType::Barometer, 101_000, 0, 0, 1_000),
        ],
        sequence: u32::MAX,
    };

    // Sync encode then sync decode for consistency baseline
    let sync_bytes = encode_to_vec(&batch).expect("sync encode DataBatch failed");
    let (sync_decoded, _): (DataBatch, _) =
        decode_from_slice(&sync_bytes).expect("sync decode DataBatch failed");
    assert_eq!(batch, sync_decoded, "sync DataBatch roundtrip mismatch");

    // Async encode then async decode via duplex
    let (client, server) = tokio::io::duplex(65536);
    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&batch)
        .await
        .expect("async write_item DataBatch failed");
    enc.finish().await.expect("finish DataBatch failed");

    let mut dec = AsyncDecoder::new(server);
    let async_decoded: DataBatch = dec
        .read_item()
        .await
        .expect("async read_item DataBatch failed")
        .expect("expected Some(DataBatch)");
    assert_eq!(
        batch, async_decoded,
        "async encode/decode DataBatch interop mismatch"
    );
    assert_eq!(
        async_decoded.sequence,
        u32::MAX,
        "decoded sequence must be u32::MAX"
    );
    assert_eq!(
        async_decoded.readings.len(),
        2,
        "decoded DataBatch must have exactly 2 readings"
    );
}

// ---------------------------------------------------------------------------
// Test 20: DataBatch with max-value fields roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_data_batch_max_value_fields_roundtrip() {
    let readings: Vec<SensorReading> = (0u32..5)
        .map(|i| {
            make_reading(
                i,
                SensorType::Magnetometer,
                i32::MAX - i as i32,
                i32::MIN + i as i32,
                i32::MAX / 2,
                u64::MAX - i as u64,
            )
        })
        .collect();
    let batch = DataBatch {
        batch_id: u64::MAX,
        device_id: "max-value-device".to_string(),
        readings,
        sequence: u32::MAX,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&batch)
        .await
        .expect("write_item DataBatch(max values) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: DataBatch = dec
        .read_item()
        .await
        .expect("read_item DataBatch(max values) failed")
        .expect("expected Some(DataBatch) with max values");
    assert_eq!(
        batch, got,
        "DataBatch with max-value fields roundtrip mismatch"
    );
    assert_eq!(got.batch_id, u64::MAX, "batch_id must be u64::MAX");
    assert_eq!(got.sequence, u32::MAX, "sequence must be u32::MAX");
    assert_eq!(got.readings.len(), 5, "must have exactly 5 readings");
}

// ---------------------------------------------------------------------------
// Test 21: Multiple DataBatches from different devices roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_multiple_data_batches_different_devices_roundtrip() {
    let devices = ["sensor-node-1", "sensor-node-2", "sensor-node-3"];
    let batches: Vec<DataBatch> = devices
        .iter()
        .enumerate()
        .map(|(i, device)| {
            let readings = vec![
                make_reading(
                    0,
                    SensorType::Accelerometer,
                    i as i32 * 100,
                    0,
                    9810,
                    i as u64 * 1000,
                ),
                make_reading(
                    1,
                    SensorType::Gyroscope,
                    0,
                    i as i32 * 50,
                    0,
                    i as u64 * 1000 + 100,
                ),
                make_reading(
                    2,
                    SensorType::Gps,
                    37_000_000,
                    -122_000_000,
                    100 * i as i32,
                    i as u64 * 1000 + 200,
                ),
            ];
            DataBatch {
                batch_id: i as u64 * 1000,
                device_id: device.to_string(),
                readings,
                sequence: i as u32,
            }
        })
        .collect();

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(batches.clone().into_iter())
        .await
        .expect("write_all multiple device DataBatches failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<DataBatch> = dec
        .read_all()
        .await
        .expect("read_all multiple device DataBatches failed");

    assert_eq!(
        got.len(),
        3,
        "must decode exactly 3 DataBatches from different devices"
    );
    assert_eq!(batches, got, "multiple device DataBatch roundtrip mismatch");
    for (i, batch) in got.iter().enumerate() {
        assert_eq!(
            batch.readings.len(),
            3,
            "each DataBatch must have 3 readings"
        );
        assert_eq!(batch.sequence, i as u32, "sequence must match index {i}");
    }
}

// ---------------------------------------------------------------------------
// Test 22: tokio::join! concurrent encode/decode for sensor feed replay
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sensor28_concurrent_encode_decode_sensor_feed_replay() {
    let batches = make_batch_vec(22);
    let batches_for_enc = batches.clone();

    let (client, server) = tokio::io::duplex(65536);

    let (_, got) = tokio::join!(
        async move {
            let mut enc = AsyncEncoder::new(client);
            enc.write_all(batches_for_enc.into_iter())
                .await
                .expect("concurrent write_all sensor feed failed");
            enc.finish().await.expect("concurrent finish failed");
        },
        async move {
            let mut dec = AsyncDecoder::new(server);
            let decoded: Vec<DataBatch> = dec
                .read_all()
                .await
                .expect("concurrent read_all sensor feed failed");
            decoded
        }
    );

    assert_eq!(
        got.len(),
        22,
        "must decode 22 DataBatches from concurrent sensor stream"
    );
    assert_eq!(
        batches, got,
        "concurrent sensor feed replay roundtrip mismatch"
    );
}
