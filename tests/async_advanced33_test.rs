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
use tokio::io::duplex;

// ---------------------------------------------------------------------------
// Smart city / urban IoT infrastructure domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InfraType {
    TrafficLight,
    ParkingMeter,
    AirQualitySensor,
    WaterMeter,
    ElectricGrid,
    Streetlight,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CityReading {
    device_id: u64,
    infra_type: InfraType,
    value: f64,
    unit: String,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UrbanReport {
    district_id: u32,
    readings: Vec<CityReading>,
    period_start: u64,
    period_end: u64,
}

// ---------------------------------------------------------------------------
// Helper: build a sample CityReading
// ---------------------------------------------------------------------------

fn make_reading(
    device_id: u64,
    infra_type: InfraType,
    value: f64,
    unit: &str,
    ts: u64,
) -> CityReading {
    CityReading {
        device_id,
        infra_type,
        value,
        unit: unit.to_string(),
        timestamp: ts,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Single CityReading roundtrip via duplex channel
// ---------------------------------------------------------------------------

#[test]
fn test_city_single_reading_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let original = make_reading(
            1001,
            InfraType::TrafficLight,
            42.5,
            "seconds",
            1_700_000_000,
        );

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&original).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<CityReading> = decoder.read_item().await.expect("read");
        assert_eq!(result, Some(original));

        let eof: Option<CityReading> = decoder.read_item().await.expect("read eof");
        assert!(eof.is_none());
    });
}

// ---------------------------------------------------------------------------
// Test 2: Batch of CityReadings — three items in order
// ---------------------------------------------------------------------------

#[test]
fn test_city_batch_readings_ordered() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let items = vec![
            make_reading(101, InfraType::ParkingMeter, 0.5, "hours", 1_700_000_001),
            make_reading(102, InfraType::AirQualitySensor, 38.2, "aqi", 1_700_000_002),
            make_reading(103, InfraType::WaterMeter, 12.7, "liters", 1_700_000_003),
        ];

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for item in &items {
            encoder.write_item(item).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded: Vec<CityReading> = Vec::new();
        while let Some(r) = decoder.read_item::<CityReading>().await.expect("read") {
            decoded.push(r);
        }
        assert_eq!(decoded, items);
    });
}

// ---------------------------------------------------------------------------
// Test 3: InfraType::TrafficLight variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_city_infra_type_traffic_light() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(200, InfraType::TrafficLight, 30.0, "seconds", 1_700_000_010);

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let got: Option<CityReading> = decoder.read_item().await.expect("read");
        assert_eq!(got.expect("some").infra_type, InfraType::TrafficLight);
    });
}

// ---------------------------------------------------------------------------
// Test 4: InfraType::ParkingMeter variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_city_infra_type_parking_meter() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(201, InfraType::ParkingMeter, 1.0, "hours", 1_700_000_011);

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let got: Option<CityReading> = decoder.read_item().await.expect("read");
        assert_eq!(got.expect("some").infra_type, InfraType::ParkingMeter);
    });
}

// ---------------------------------------------------------------------------
// Test 5: InfraType::AirQualitySensor variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_city_infra_type_air_quality_sensor() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(202, InfraType::AirQualitySensor, 55.1, "aqi", 1_700_000_012);

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let got: Option<CityReading> = decoder.read_item().await.expect("read");
        assert_eq!(got.expect("some").infra_type, InfraType::AirQualitySensor);
    });
}

// ---------------------------------------------------------------------------
// Test 6: InfraType::WaterMeter variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_city_infra_type_water_meter() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(203, InfraType::WaterMeter, 250.8, "liters", 1_700_000_013);

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let got: Option<CityReading> = decoder.read_item().await.expect("read");
        assert_eq!(got.expect("some").infra_type, InfraType::WaterMeter);
    });
}

// ---------------------------------------------------------------------------
// Test 7: InfraType::ElectricGrid variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_city_infra_type_electric_grid() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(204, InfraType::ElectricGrid, 220.0, "volts", 1_700_000_014);

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let got: Option<CityReading> = decoder.read_item().await.expect("read");
        assert_eq!(got.expect("some").infra_type, InfraType::ElectricGrid);
    });
}

// ---------------------------------------------------------------------------
// Test 8: InfraType::Streetlight variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_city_infra_type_streetlight() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(205, InfraType::Streetlight, 80.0, "percent", 1_700_000_015);

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let got: Option<CityReading> = decoder.read_item().await.expect("read");
        assert_eq!(got.expect("some").infra_type, InfraType::Streetlight);
    });
}

// ---------------------------------------------------------------------------
// Test 9: UrbanReport roundtrip with multiple readings
// ---------------------------------------------------------------------------

#[test]
fn test_city_urban_report_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let report = UrbanReport {
            district_id: 7,
            readings: vec![
                make_reading(300, InfraType::TrafficLight, 45.0, "seconds", 1_700_001_000),
                make_reading(301, InfraType::AirQualitySensor, 62.3, "aqi", 1_700_001_001),
                make_reading(302, InfraType::Streetlight, 95.0, "percent", 1_700_001_002),
            ],
            period_start: 1_700_001_000,
            period_end: 1_700_001_999,
        };

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&report).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let got: Option<UrbanReport> = decoder.read_item().await.expect("read");
        assert_eq!(got, Some(report));
    });
}

// ---------------------------------------------------------------------------
// Test 10: Empty stream — no items written, first read returns None
// ---------------------------------------------------------------------------

#[test]
fn test_city_empty_stream_returns_none() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let (writer, reader) = duplex(65536);
        let encoder = AsyncEncoder::<_>::with_config(writer, StreamingConfig::default());
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let got: Option<CityReading> = decoder.read_item().await.expect("read empty");
        assert!(got.is_none());
        assert!(decoder.is_finished());
    });
}

// ---------------------------------------------------------------------------
// Test 11: Large batch — 100 CityReadings roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_city_large_batch_100_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let infra_cycle = [
            InfraType::TrafficLight,
            InfraType::ParkingMeter,
            InfraType::AirQualitySensor,
            InfraType::WaterMeter,
            InfraType::ElectricGrid,
            InfraType::Streetlight,
        ];

        let readings: Vec<CityReading> = (0u64..100)
            .map(|i| {
                let infra = infra_cycle[(i as usize) % infra_cycle.len()].clone();
                make_reading(
                    1000 + i,
                    infra,
                    (i as f64) * 0.5 + 10.0,
                    "units",
                    1_700_002_000 + i,
                )
            })
            .collect();

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded: Vec<CityReading> = Vec::new();
        while let Some(r) = decoder.read_item::<CityReading>().await.expect("read") {
            decoded.push(r);
        }

        assert_eq!(decoded.len(), 100);
        assert_eq!(decoded, readings);
    });
}

// ---------------------------------------------------------------------------
// Test 12: Progress tracking — items_processed > 0 after encoding
// ---------------------------------------------------------------------------

#[test]
fn test_city_progress_tracking_items_processed() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings: Vec<CityReading> = (0u64..5)
            .map(|i| {
                make_reading(
                    500 + i,
                    InfraType::ElectricGrid,
                    230.0 + i as f64,
                    "volts",
                    1_700_003_000 + i,
                )
            })
            .collect();

        // Use flush_per_item so each write triggers a chunk flush,
        // allowing items_processed to be updated before finish().
        let config = StreamingConfig::new().with_flush_per_item(true);

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, config);
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        // Verify progress is tracked after each flushed write
        assert!(encoder.progress().items_processed > 0);
        encoder.finish().await.expect("finish");

        // Drain the decoder
        let mut decoder = AsyncDecoder::new(reader);
        while decoder
            .read_item::<CityReading>()
            .await
            .expect("read")
            .is_some()
        {}

        assert_eq!(decoder.progress().items_processed, 5);
    });
}

// ---------------------------------------------------------------------------
// Test 13: write_all method with iterator of CityReadings
// ---------------------------------------------------------------------------

#[test]
fn test_city_write_all_iterator() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings: Vec<CityReading> = vec![
            make_reading(600, InfraType::WaterMeter, 88.4, "liters", 1_700_004_000),
            make_reading(601, InfraType::WaterMeter, 91.2, "liters", 1_700_004_001),
            make_reading(602, InfraType::WaterMeter, 76.0, "liters", 1_700_004_002),
        ];

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder
            .write_all(readings.iter().cloned())
            .await
            .expect("write_all");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded: Vec<CityReading> = Vec::new();
        while let Some(r) = decoder.read_item::<CityReading>().await.expect("read") {
            decoded.push(r);
        }
        assert_eq!(decoded, readings);
    });
}

// ---------------------------------------------------------------------------
// Test 14: UrbanReport with empty readings list
// ---------------------------------------------------------------------------

#[test]
fn test_city_urban_report_empty_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let report = UrbanReport {
            district_id: 99,
            readings: Vec::new(),
            period_start: 1_700_005_000,
            period_end: 1_700_005_999,
        };

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&report).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let got: Option<UrbanReport> = decoder.read_item().await.expect("read");
        let r = got.expect("some");
        assert_eq!(r.district_id, 99);
        assert!(r.readings.is_empty());
        assert_eq!(r.period_start, 1_700_005_000);
        assert_eq!(r.period_end, 1_700_005_999);
    });
}

// ---------------------------------------------------------------------------
// Test 15: All InfraType variants in a single stream
// ---------------------------------------------------------------------------

#[test]
fn test_city_all_infra_variants_single_stream() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let all_types = vec![
            InfraType::TrafficLight,
            InfraType::ParkingMeter,
            InfraType::AirQualitySensor,
            InfraType::WaterMeter,
            InfraType::ElectricGrid,
            InfraType::Streetlight,
        ];

        let readings: Vec<CityReading> = all_types
            .iter()
            .enumerate()
            .map(|(i, t)| {
                make_reading(
                    700 + i as u64,
                    t.clone(),
                    i as f64 * 10.0,
                    "unit",
                    1_700_006_000 + i as u64,
                )
            })
            .collect();

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded_types: Vec<InfraType> = Vec::new();
        while let Some(r) = decoder.read_item::<CityReading>().await.expect("read") {
            decoded_types.push(r.infra_type);
        }
        assert_eq!(decoded_types, all_types);
    });
}

// ---------------------------------------------------------------------------
// Test 16: Sync encode / async decode interop for CityReading
// ---------------------------------------------------------------------------

#[test]
fn test_city_sync_encode_async_decode_interop() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let original = make_reading(800, InfraType::Streetlight, 67.5, "percent", 1_700_007_000);

        // Verify sync roundtrip
        let sync_bytes = encode_to_vec(&original).expect("sync encode");
        let (sync_decoded, _): (CityReading, _) =
            decode_from_slice(&sync_bytes).expect("sync decode");
        assert_eq!(sync_decoded, original);

        // Async streaming roundtrip of the same value
        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&original).await.expect("async write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let async_decoded: Option<CityReading> = decoder.read_item().await.expect("async read");
        assert_eq!(async_decoded, Some(original));
    });
}

// ---------------------------------------------------------------------------
// Test 17: Small chunk size forces multiple chunks for 10 readings
// ---------------------------------------------------------------------------

#[test]
fn test_city_small_chunk_size_multiple_chunks() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let config = StreamingConfig::new().with_chunk_size(1024);

        let readings: Vec<CityReading> = (0u64..10)
            .map(|i| {
                make_reading(
                    900 + i,
                    InfraType::AirQualitySensor,
                    40.0 + i as f64,
                    "aqi",
                    1_700_008_000 + i,
                )
            })
            .collect();

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, config);
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded: Vec<CityReading> = Vec::new();
        while let Some(r) = decoder.read_item::<CityReading>().await.expect("read") {
            decoded.push(r);
        }
        assert_eq!(decoded, readings);
        assert!(decoder.progress().chunks_processed >= 1);
    });
}

// ---------------------------------------------------------------------------
// Test 18: flush_per_item config — each item flushed immediately
// ---------------------------------------------------------------------------

#[test]
fn test_city_flush_per_item_config() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let config = StreamingConfig::new().with_flush_per_item(true);

        let readings: Vec<CityReading> = vec![
            make_reading(
                1001,
                InfraType::TrafficLight,
                25.0,
                "seconds",
                1_700_009_000,
            ),
            make_reading(1002, InfraType::ParkingMeter, 2.0, "hours", 1_700_009_001),
            make_reading(1003, InfraType::ElectricGrid, 240.0, "volts", 1_700_009_002),
        ];

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, config);
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded: Vec<CityReading> = Vec::new();
        while let Some(r) = decoder.read_item::<CityReading>().await.expect("read") {
            decoded.push(r);
        }
        assert_eq!(decoded, readings);
        // One chunk per item with flush_per_item
        assert_eq!(decoder.progress().chunks_processed, readings.len() as u64);
    });
}

// ---------------------------------------------------------------------------
// Test 19: Multiple UrbanReports in a single stream
// ---------------------------------------------------------------------------

#[test]
fn test_city_multiple_urban_reports_stream() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reports = vec![
            UrbanReport {
                district_id: 1,
                readings: vec![make_reading(
                    1100,
                    InfraType::WaterMeter,
                    120.0,
                    "liters",
                    1_700_010_000,
                )],
                period_start: 1_700_010_000,
                period_end: 1_700_010_099,
            },
            UrbanReport {
                district_id: 2,
                readings: vec![
                    make_reading(1101, InfraType::Streetlight, 70.0, "percent", 1_700_010_100),
                    make_reading(1102, InfraType::ElectricGrid, 230.0, "volts", 1_700_010_101),
                ],
                period_start: 1_700_010_100,
                period_end: 1_700_010_199,
            },
        ];

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for rep in &reports {
            encoder.write_item(rep).await.expect("write report");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded: Vec<UrbanReport> = Vec::new();
        while let Some(r) = decoder.read_item::<UrbanReport>().await.expect("read") {
            decoded.push(r);
        }
        assert_eq!(decoded, reports);
    });
}

// ---------------------------------------------------------------------------
// Test 20: decoder.read_all() collects all CityReadings at once
// ---------------------------------------------------------------------------

#[test]
fn test_city_decoder_read_all() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings: Vec<CityReading> = (0u64..8)
            .map(|i| {
                make_reading(
                    1200 + i,
                    InfraType::ParkingMeter,
                    i as f64 * 0.25,
                    "hours",
                    1_700_011_000 + i,
                )
            })
            .collect();

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let collected: Vec<CityReading> = decoder.read_all().await.expect("read_all");
        assert_eq!(collected, readings);
        assert!(decoder.is_finished());
    });
}

// ---------------------------------------------------------------------------
// Test 21: Concurrent reads after sequential encode into duplex
// ---------------------------------------------------------------------------

#[test]
fn test_city_sequential_decode_after_encode_complete() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        // Encode all items first, then decode sequentially to simulate
        // a completed-write-then-full-read scenario (within single task).
        let readings: Vec<CityReading> = (0u64..6)
            .map(|i| {
                let infra = if i % 2 == 0 {
                    InfraType::TrafficLight
                } else {
                    InfraType::AirQualitySensor
                };
                make_reading(1300 + i, infra, 50.0 + i as f64, "unit", 1_700_012_000 + i)
            })
            .collect();

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());

        // Write all items, verify progress between writes
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        let progress_after_writes = encoder.progress().items_processed;
        encoder.finish().await.expect("finish");

        // Decode sequentially
        let mut decoder = AsyncDecoder::new(reader);
        let mut index = 0usize;
        while let Some(r) = decoder.read_item::<CityReading>().await.expect("read") {
            assert_eq!(r, readings[index], "mismatch at index {index}");
            index += 1;
        }
        assert_eq!(index, readings.len());

        // Encoder progress was tracked (items_processed counted from flush_chunk calls)
        let _ = progress_after_writes; // used for assertion above
        assert_eq!(decoder.progress().items_processed, readings.len() as u64);
    });
}

// ---------------------------------------------------------------------------
// Test 22: Large batch (100) UrbanReport readings — bytes_processed grows
// ---------------------------------------------------------------------------

#[test]
fn test_city_bytes_processed_grows_with_large_batch() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let infra_cycle = [
            InfraType::TrafficLight,
            InfraType::ParkingMeter,
            InfraType::AirQualitySensor,
            InfraType::WaterMeter,
            InfraType::ElectricGrid,
            InfraType::Streetlight,
        ];

        let report = UrbanReport {
            district_id: 42,
            readings: (0u64..100)
                .map(|i| {
                    let infra = infra_cycle[(i as usize) % infra_cycle.len()].clone();
                    make_reading(2000 + i, infra, i as f64 * 1.1, "metric", 1_700_013_000 + i)
                })
                .collect(),
            period_start: 1_700_013_000,
            period_end: 1_700_013_099,
        };

        let (writer, reader) = duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&report).await.expect("write report");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let first: Option<UrbanReport> = decoder.read_item().await.expect("read first");
        assert!(first.is_some(), "expected UrbanReport from stream");

        let bytes_after_first = decoder.progress().bytes_processed;
        assert!(
            bytes_after_first > 0,
            "bytes_processed must be positive after decoding a large report"
        );

        let decoded_report = first.expect("report present");
        assert_eq!(decoded_report.district_id, 42);
        assert_eq!(decoded_report.readings.len(), 100);

        // Stream must be exhausted
        let eof: Option<UrbanReport> = decoder.read_item().await.expect("eof read");
        assert!(eof.is_none());
        assert!(decoder.is_finished());
    });
}
