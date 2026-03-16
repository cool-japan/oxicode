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
// Domain types: Climate science / atmospheric measurements
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AtmosphericLayer {
    Troposphere,
    Stratosphere,
    Mesosphere,
    Thermosphere,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MeasurementParameter {
    Temperature,
    Humidity,
    Pressure,
    Co2Ppm,
    WindSpeed,
    Precipitation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AtmosphericReading {
    station_id: u32,
    layer: AtmosphericLayer,
    parameter: MeasurementParameter,
    value_micro: i64,
    altitude_m: u32,
    timestamp_s: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClimateSnapshot {
    snapshot_id: u64,
    readings: Vec<AtmosphericReading>,
    region: String,
    year: u32,
}

// ---------------------------------------------------------------------------
// Helper: build a representative AtmosphericReading
// ---------------------------------------------------------------------------
fn make_reading(
    station_id: u32,
    layer: AtmosphericLayer,
    parameter: MeasurementParameter,
) -> AtmosphericReading {
    AtmosphericReading {
        station_id,
        layer,
        parameter,
        value_micro: station_id as i64 * 1_000,
        altitude_m: station_id * 100,
        timestamp_s: 1_700_000_000 + station_id as u64 * 60,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Single AtmosphericReading roundtrip via duplex
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_single_reading_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let original = make_reading(
            1,
            AtmosphericLayer::Troposphere,
            MeasurementParameter::Temperature,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&original).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::with_config(reader, StreamingConfig::default());
        let decoded: Option<AtmosphericReading> = decoder.read_item().await.expect("read_item");
        assert_eq!(decoded, Some(original));
    });
}

// ---------------------------------------------------------------------------
// Test 2: AtmosphericLayer::Troposphere variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_layer_troposphere_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(
            10,
            AtmosphericLayer::Troposphere,
            MeasurementParameter::Humidity,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: AtmosphericReading =
            decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.layer, AtmosphericLayer::Troposphere);
        assert_eq!(decoded.station_id, 10);
    });
}

// ---------------------------------------------------------------------------
// Test 3: AtmosphericLayer::Stratosphere variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_layer_stratosphere_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(
            20,
            AtmosphericLayer::Stratosphere,
            MeasurementParameter::Pressure,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: AtmosphericReading =
            decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.layer, AtmosphericLayer::Stratosphere);
    });
}

// ---------------------------------------------------------------------------
// Test 4: AtmosphericLayer::Mesosphere variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_layer_mesosphere_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(
            30,
            AtmosphericLayer::Mesosphere,
            MeasurementParameter::Co2Ppm,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: AtmosphericReading =
            decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.layer, AtmosphericLayer::Mesosphere);
    });
}

// ---------------------------------------------------------------------------
// Test 5: AtmosphericLayer::Thermosphere variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_layer_thermosphere_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(
            40,
            AtmosphericLayer::Thermosphere,
            MeasurementParameter::WindSpeed,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: AtmosphericReading =
            decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.layer, AtmosphericLayer::Thermosphere);
    });
}

// ---------------------------------------------------------------------------
// Test 6: MeasurementParameter::Temperature variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_param_temperature_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(
            11,
            AtmosphericLayer::Troposphere,
            MeasurementParameter::Temperature,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: AtmosphericReading =
            decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.parameter, MeasurementParameter::Temperature);
    });
}

// ---------------------------------------------------------------------------
// Test 7: MeasurementParameter::Co2Ppm variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_param_co2ppm_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let reading = make_reading(
            22,
            AtmosphericLayer::Stratosphere,
            MeasurementParameter::Co2Ppm,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: AtmosphericReading =
            decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.parameter, MeasurementParameter::Co2Ppm);
    });
}

// ---------------------------------------------------------------------------
// Test 8: ClimateSnapshot roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_climate_snapshot_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let snapshot = ClimateSnapshot {
            snapshot_id: 0xC0DE_C0DE_0000_0001,
            readings: vec![
                make_reading(
                    100,
                    AtmosphericLayer::Troposphere,
                    MeasurementParameter::Temperature,
                ),
                make_reading(
                    101,
                    AtmosphericLayer::Stratosphere,
                    MeasurementParameter::Co2Ppm,
                ),
                make_reading(
                    102,
                    AtmosphericLayer::Mesosphere,
                    MeasurementParameter::Pressure,
                ),
            ],
            region: String::from("North Atlantic"),
            year: 2024,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&snapshot).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: ClimateSnapshot = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.snapshot_id, snapshot.snapshot_id);
        assert_eq!(decoded.readings.len(), 3);
        assert_eq!(decoded.region, "North Atlantic");
        assert_eq!(decoded.year, 2024);
    });
}

// ---------------------------------------------------------------------------
// Test 9: Batch of 10 readings write_all / read_all
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_batch_10_write_all_read_all() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let layers = [
            AtmosphericLayer::Troposphere,
            AtmosphericLayer::Stratosphere,
        ];
        let params = [
            MeasurementParameter::Temperature,
            MeasurementParameter::Humidity,
            MeasurementParameter::Co2Ppm,
        ];
        let readings: Vec<AtmosphericReading> = (0u32..10)
            .map(|i| {
                make_reading(
                    i,
                    layers[(i as usize) % layers.len()].clone(),
                    params[(i as usize) % params.len()].clone(),
                )
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_all(readings.clone())
            .await
            .expect("write_all");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder
            .read_item::<AtmosphericReading>()
            .await
            .expect("read_item")
        {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 10);
        assert_eq!(decoded, readings);
    });
}

// ---------------------------------------------------------------------------
// Test 10: Empty stream returns None
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_empty_stream_returns_none() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let encoder = AsyncEncoder::new(writer);
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<AtmosphericReading> = decoder.read_item().await.expect("read_item");
        assert_eq!(result, None);
        assert!(decoder.is_finished());
    });
}

// ---------------------------------------------------------------------------
// Test 11: Large batch of 50 readings roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_large_batch_50_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let layers = [
            AtmosphericLayer::Troposphere,
            AtmosphericLayer::Stratosphere,
            AtmosphericLayer::Mesosphere,
            AtmosphericLayer::Thermosphere,
        ];
        let params = [
            MeasurementParameter::Temperature,
            MeasurementParameter::Humidity,
            MeasurementParameter::Pressure,
            MeasurementParameter::Co2Ppm,
            MeasurementParameter::WindSpeed,
            MeasurementParameter::Precipitation,
        ];
        let readings: Vec<AtmosphericReading> = (0u32..50)
            .map(|i| {
                make_reading(
                    i,
                    layers[(i as usize) % layers.len()].clone(),
                    params[(i as usize) % params.len()].clone(),
                )
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder
            .read_item::<AtmosphericReading>()
            .await
            .expect("read_item")
        {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 50);
        assert_eq!(decoded, readings);
    });
}

// ---------------------------------------------------------------------------
// Test 12: Progress tracking — items_processed > 0 after encoding
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_progress_tracking() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings: Vec<AtmosphericReading> = (0u32..12)
            .map(|i| {
                make_reading(
                    i,
                    AtmosphericLayer::Troposphere,
                    MeasurementParameter::Temperature,
                )
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        while let Some(_) = decoder
            .read_item::<AtmosphericReading>()
            .await
            .expect("read_item")
        {}
        assert!(decoder.progress().items_processed > 0);
        assert_eq!(decoder.progress().items_processed, 12);
    });
}

// ---------------------------------------------------------------------------
// Test 13: All AtmosphericLayer variants in one batch
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_all_layer_variants_one_batch() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings = vec![
            make_reading(
                1,
                AtmosphericLayer::Troposphere,
                MeasurementParameter::Temperature,
            ),
            make_reading(
                2,
                AtmosphericLayer::Stratosphere,
                MeasurementParameter::Temperature,
            ),
            make_reading(
                3,
                AtmosphericLayer::Mesosphere,
                MeasurementParameter::Temperature,
            ),
            make_reading(
                4,
                AtmosphericLayer::Thermosphere,
                MeasurementParameter::Temperature,
            ),
        ];

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder
            .read_item::<AtmosphericReading>()
            .await
            .expect("read_item")
        {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 4);
        assert_eq!(decoded[0].layer, AtmosphericLayer::Troposphere);
        assert_eq!(decoded[1].layer, AtmosphericLayer::Stratosphere);
        assert_eq!(decoded[2].layer, AtmosphericLayer::Mesosphere);
        assert_eq!(decoded[3].layer, AtmosphericLayer::Thermosphere);
    });
}

// ---------------------------------------------------------------------------
// Test 14: All MeasurementParameter variants in one batch
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_all_parameter_variants_one_batch() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings = vec![
            make_reading(
                1,
                AtmosphericLayer::Troposphere,
                MeasurementParameter::Temperature,
            ),
            make_reading(
                2,
                AtmosphericLayer::Troposphere,
                MeasurementParameter::Humidity,
            ),
            make_reading(
                3,
                AtmosphericLayer::Troposphere,
                MeasurementParameter::Pressure,
            ),
            make_reading(
                4,
                AtmosphericLayer::Troposphere,
                MeasurementParameter::Co2Ppm,
            ),
            make_reading(
                5,
                AtmosphericLayer::Troposphere,
                MeasurementParameter::WindSpeed,
            ),
            make_reading(
                6,
                AtmosphericLayer::Troposphere,
                MeasurementParameter::Precipitation,
            ),
        ];

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder
            .read_item::<AtmosphericReading>()
            .await
            .expect("read_item")
        {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 6);
        assert_eq!(decoded[0].parameter, MeasurementParameter::Temperature);
        assert_eq!(decoded[1].parameter, MeasurementParameter::Humidity);
        assert_eq!(decoded[2].parameter, MeasurementParameter::Pressure);
        assert_eq!(decoded[3].parameter, MeasurementParameter::Co2Ppm);
        assert_eq!(decoded[4].parameter, MeasurementParameter::WindSpeed);
        assert_eq!(decoded[5].parameter, MeasurementParameter::Precipitation);
    });
}

// ---------------------------------------------------------------------------
// Test 15: Concurrent write/read via tokio::spawn
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_concurrent_write_read() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings: Vec<AtmosphericReading> = (0u32..40)
            .map(|i| {
                make_reading(
                    i,
                    AtmosphericLayer::Stratosphere,
                    MeasurementParameter::Co2Ppm,
                )
            })
            .collect();
        let expected = readings.clone();

        let (writer, reader) = tokio::io::duplex(65536);

        let encode_handle = tokio::spawn(async move {
            let mut encoder = AsyncEncoder::new(writer);
            for r in &readings {
                encoder.write_item(r).await.expect("write");
            }
            encoder.finish().await.expect("finish");
        });

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder
            .read_item::<AtmosphericReading>()
            .await
            .expect("read_item")
        {
            decoded.push(item);
        }

        encode_handle.await.expect("encoder task");

        assert_eq!(decoded.len(), 40);
        assert_eq!(decoded, expected);
        assert!(decoder.progress().items_processed > 0);
        assert!(decoder.is_finished());
    });
}

// ---------------------------------------------------------------------------
// Test 16: Max value readings (i64::MAX for value_micro)
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_max_value_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let max_reading = AtmosphericReading {
            station_id: u32::MAX,
            layer: AtmosphericLayer::Thermosphere,
            parameter: MeasurementParameter::Temperature,
            value_micro: i64::MAX,
            altitude_m: u32::MAX,
            timestamp_s: u64::MAX,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&max_reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: AtmosphericReading =
            decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.value_micro, i64::MAX);
        assert_eq!(decoded.station_id, u32::MAX);
        assert_eq!(decoded.altitude_m, u32::MAX);
        assert_eq!(decoded.timestamp_s, u64::MAX);
    });
}

// ---------------------------------------------------------------------------
// Test 17: Negative temperature readings (negative value_micro)
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_negative_temperature_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let cold_readings: Vec<AtmosphericReading> = (1u32..=5)
            .map(|i| AtmosphericReading {
                station_id: i,
                layer: AtmosphericLayer::Mesosphere,
                parameter: MeasurementParameter::Temperature,
                value_micro: -(i as i64 * 50_000_000),
                altitude_m: 60_000 + i * 1_000,
                timestamp_s: 1_710_000_000 + i as u64 * 3600,
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for r in &cold_readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder
            .read_item::<AtmosphericReading>()
            .await
            .expect("read_item")
        {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 5);
        for r in &decoded {
            assert!(r.value_micro < 0, "expected negative value_micro");
            assert_eq!(r.layer, AtmosphericLayer::Mesosphere);
            assert_eq!(r.parameter, MeasurementParameter::Temperature);
        }
    });
}

// ---------------------------------------------------------------------------
// Test 18: High altitude readings
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_high_altitude_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let high_alt_readings: Vec<AtmosphericReading> =
            [80_000u32, 100_000, 200_000, 500_000, 1_000_000]
                .iter()
                .enumerate()
                .map(|(i, &alt)| AtmosphericReading {
                    station_id: i as u32 + 500,
                    layer: AtmosphericLayer::Thermosphere,
                    parameter: MeasurementParameter::Pressure,
                    value_micro: alt as i64 * 10,
                    altitude_m: alt,
                    timestamp_s: 1_720_000_000 + i as u64 * 600,
                })
                .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for r in &high_alt_readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder
            .read_item::<AtmosphericReading>()
            .await
            .expect("read_item")
        {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 5);
        assert_eq!(decoded[4].altitude_m, 1_000_000);
        assert_eq!(decoded[0].altitude_m, 80_000);
        assert_eq!(decoded, high_alt_readings);
    });
}

// ---------------------------------------------------------------------------
// Test 19: ClimateSnapshot with 20 readings
// ---------------------------------------------------------------------------
#[test]
fn test_climate_snapshot_with_20_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let layers = [
            AtmosphericLayer::Troposphere,
            AtmosphericLayer::Stratosphere,
            AtmosphericLayer::Mesosphere,
            AtmosphericLayer::Thermosphere,
        ];
        let params = [
            MeasurementParameter::Temperature,
            MeasurementParameter::Humidity,
            MeasurementParameter::Pressure,
            MeasurementParameter::Co2Ppm,
            MeasurementParameter::WindSpeed,
        ];
        let snapshot = ClimateSnapshot {
            snapshot_id: 20_240_315_000_000_u64,
            readings: (0u32..20)
                .map(|i| {
                    make_reading(
                        i + 200,
                        layers[(i as usize) % layers.len()].clone(),
                        params[(i as usize) % params.len()].clone(),
                    )
                })
                .collect(),
            region: String::from("Pacific Ocean Basin"),
            year: 2024,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&snapshot).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: ClimateSnapshot = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.snapshot_id, snapshot.snapshot_id);
        assert_eq!(decoded.readings.len(), 20);
        assert_eq!(decoded.region, "Pacific Ocean Basin");
        assert_eq!(decoded.year, 2024);
        assert_eq!(decoded, snapshot);
    });
}

// ---------------------------------------------------------------------------
// Test 20: Multiple ClimateSnapshots chain
// ---------------------------------------------------------------------------
#[test]
fn test_climate_multiple_snapshots_chain() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let snapshots: Vec<ClimateSnapshot> = (0u64..6)
            .map(|s| ClimateSnapshot {
                snapshot_id: s * 10_000,
                readings: (0u32..4)
                    .map(|i| {
                        make_reading(
                            i + s as u32 * 4,
                            AtmosphericLayer::Stratosphere,
                            MeasurementParameter::Co2Ppm,
                        )
                    })
                    .collect(),
                region: format!("Region-{}", s),
                year: 2020 + s as u32,
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for snap in &snapshots {
            encoder.write_item(snap).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder
            .read_item::<ClimateSnapshot>()
            .await
            .expect("read_item")
        {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 6);
        for (i, snap) in decoded.iter().enumerate() {
            assert_eq!(snap.snapshot_id, i as u64 * 10_000);
            assert_eq!(snap.year, 2020 + i as u32);
            assert_eq!(snap.region, format!("Region-{}", i));
            assert_eq!(snap.readings.len(), 4);
        }
    });
}

// ---------------------------------------------------------------------------
// Test 21: Sequential read item by item verification
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_sequential_read_item_by_item() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let readings = vec![
            AtmosphericReading {
                station_id: 1001,
                layer: AtmosphericLayer::Troposphere,
                parameter: MeasurementParameter::Temperature,
                value_micro: 293_150_000,
                altitude_m: 1_500,
                timestamp_s: 1_700_100_000,
            },
            AtmosphericReading {
                station_id: 1002,
                layer: AtmosphericLayer::Stratosphere,
                parameter: MeasurementParameter::Co2Ppm,
                value_micro: 421_000_000,
                altitude_m: 25_000,
                timestamp_s: 1_700_100_060,
            },
            AtmosphericReading {
                station_id: 1003,
                layer: AtmosphericLayer::Mesosphere,
                parameter: MeasurementParameter::WindSpeed,
                value_micro: 150_000_000,
                altitude_m: 75_000,
                timestamp_s: 1_700_100_120,
            },
        ];

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for r in &readings {
            encoder.write_item(r).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);

        let first: AtmosphericReading = decoder
            .read_item()
            .await
            .expect("read_item 1")
            .expect("first");
        assert_eq!(first.station_id, 1001);
        assert_eq!(first.value_micro, 293_150_000);

        let second: AtmosphericReading = decoder
            .read_item()
            .await
            .expect("read_item 2")
            .expect("second");
        assert_eq!(second.station_id, 1002);
        assert_eq!(second.parameter, MeasurementParameter::Co2Ppm);

        let third: AtmosphericReading = decoder
            .read_item()
            .await
            .expect("read_item 3")
            .expect("third");
        assert_eq!(third.station_id, 1003);
        assert_eq!(third.layer, AtmosphericLayer::Mesosphere);

        let eof: Option<AtmosphericReading> = decoder.read_item().await.expect("read_item eof");
        assert_eq!(eof, None);
    });
}

// ---------------------------------------------------------------------------
// Test 22: Sync encode_to_vec / decode_from_slice consistency vs async
// ---------------------------------------------------------------------------
#[test]
fn test_atmos_sync_vs_async_consistency() {
    let reading = AtmosphericReading {
        station_id: 9999,
        layer: AtmosphericLayer::Stratosphere,
        parameter: MeasurementParameter::Precipitation,
        value_micro: 12_345_678,
        altitude_m: 30_000,
        timestamp_s: 1_730_000_000,
    };

    // Sync path
    let sync_encoded = encode_to_vec(&reading).expect("encode_to_vec");
    let (sync_decoded, sync_consumed): (AtmosphericReading, _) =
        decode_from_slice(&sync_encoded).expect("decode_from_slice");
    assert_eq!(sync_decoded, reading);
    assert_eq!(sync_consumed, sync_encoded.len());

    // Async path
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let async_decoded: AtmosphericReading =
            decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(async_decoded, reading);
        assert_eq!(async_decoded, sync_decoded);
    });
}
