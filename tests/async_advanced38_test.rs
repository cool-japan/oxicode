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
use oxicode::{encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types: ocean monitoring / marine biology
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OceanLayer {
    Surface,
    Mesopelagic,
    Bathypelagic,
    Abyssopelagic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MarineEvent {
    WaveHeightAlert,
    TidalAnomaly,
    PollutionDetected,
    SpeciesTagged,
    CurrentShift,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BuoySensor {
    buoy_id: u32,
    layer: OceanLayer,
    temp_mc: i32,
    salinity_ppm: u32,
    depth_cm: u32,
    timestamp_s: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MarineAlert {
    alert_id: u64,
    buoy_id: u32,
    event: MarineEvent,
    severity: u8,
    lat_micro: i32,
    lon_micro: i32,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_buoy_sensor(buoy_id: u32, layer: OceanLayer, temp_mc: i32, depth_cm: u32) -> BuoySensor {
    BuoySensor {
        buoy_id,
        layer,
        temp_mc,
        salinity_ppm: 35_000 + buoy_id * 10,
        depth_cm,
        timestamp_s: 1_700_000_000 + buoy_id as u64 * 60,
    }
}

fn make_marine_alert(
    alert_id: u64,
    buoy_id: u32,
    event: MarineEvent,
    severity: u8,
    lat_micro: i32,
    lon_micro: i32,
) -> MarineAlert {
    MarineAlert {
        alert_id,
        buoy_id,
        event,
        severity,
        lat_micro,
        lon_micro,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Single BuoySensor duplex roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_single_buoy_sensor_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let sensor = make_buoy_sensor(1001, OceanLayer::Surface, 2850, 50);

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&sensor).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<BuoySensor>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded, sensor);
    });
}

// ---------------------------------------------------------------------------
// Test 2: OceanLayer::Surface variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_layer_surface_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let sensor = make_buoy_sensor(1, OceanLayer::Surface, 2200, 10);

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&sensor).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<BuoySensor>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded.layer, OceanLayer::Surface);
        assert_eq!(decoded.buoy_id, 1);
    });
}

// ---------------------------------------------------------------------------
// Test 3: OceanLayer::Mesopelagic variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_layer_mesopelagic_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let sensor = make_buoy_sensor(2, OceanLayer::Mesopelagic, 800, 50000);

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&sensor).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<BuoySensor>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded.layer, OceanLayer::Mesopelagic);
        assert_eq!(decoded.depth_cm, 50000);
    });
}

// ---------------------------------------------------------------------------
// Test 4: OceanLayer::Bathypelagic variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_layer_bathypelagic_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let sensor = make_buoy_sensor(3, OceanLayer::Bathypelagic, -100, 200000);

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&sensor).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<BuoySensor>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded.layer, OceanLayer::Bathypelagic);
        assert_eq!(decoded.depth_cm, 200000);
        assert_eq!(decoded.temp_mc, -100);
    });
}

// ---------------------------------------------------------------------------
// Test 5: OceanLayer::Abyssopelagic variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_layer_abyssopelagic_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let sensor = make_buoy_sensor(4, OceanLayer::Abyssopelagic, -200, 400000);

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&sensor).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<BuoySensor>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded.layer, OceanLayer::Abyssopelagic);
        assert_eq!(decoded.depth_cm, 400000);
    });
}

// ---------------------------------------------------------------------------
// Test 6: MarineEvent::WaveHeightAlert variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_event_wave_height_alert_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let alert = make_marine_alert(
            1001,
            42,
            MarineEvent::WaveHeightAlert,
            8,
            48_123456,
            -125_987654,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&alert).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<MarineAlert>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded.event, MarineEvent::WaveHeightAlert);
        assert_eq!(decoded.severity, 8);
    });
}

// ---------------------------------------------------------------------------
// Test 7: MarineEvent::TidalAnomaly variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_event_tidal_anomaly_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let alert = make_marine_alert(
            2002,
            17,
            MarineEvent::TidalAnomaly,
            5,
            35_500000,
            139_750000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&alert).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<MarineAlert>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded.event, MarineEvent::TidalAnomaly);
        assert_eq!(decoded.buoy_id, 17);
        assert_eq!(decoded.lat_micro, 35_500000);
    });
}

// ---------------------------------------------------------------------------
// Test 8: MarineEvent::PollutionDetected variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_event_pollution_detected_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let alert = make_marine_alert(
            3003,
            55,
            MarineEvent::PollutionDetected,
            9,
            51_477928,
            -0_001274,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&alert).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<MarineAlert>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded.event, MarineEvent::PollutionDetected);
        assert_eq!(decoded.severity, 9);
        assert_eq!(decoded.alert_id, 3003);
    });
}

// ---------------------------------------------------------------------------
// Test 9: MarineEvent::SpeciesTagged variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_event_species_tagged_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let alert = make_marine_alert(
            4004,
            88,
            MarineEvent::SpeciesTagged,
            2,
            -33_868820,
            151_209290,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&alert).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<MarineAlert>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded.event, MarineEvent::SpeciesTagged);
        assert_eq!(decoded.buoy_id, 88);
        assert_eq!(decoded.lat_micro, -33_868820);
    });
}

// ---------------------------------------------------------------------------
// Test 10: MarineEvent::CurrentShift variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_event_current_shift_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let alert = make_marine_alert(
            5005,
            99,
            MarineEvent::CurrentShift,
            4,
            -54_432000,
            -3_567000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&alert).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<MarineAlert>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded.event, MarineEvent::CurrentShift);
        assert_eq!(decoded.lon_micro, -3_567000);
    });
}

// ---------------------------------------------------------------------------
// Test 11: MarineAlert full roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_marine_alert_full_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let alert = MarineAlert {
            alert_id: 0xDEAD_BEEF_0001,
            buoy_id: 777,
            event: MarineEvent::WaveHeightAlert,
            severity: 10,
            lat_micro: -23_550520,
            lon_micro: -46_633309,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&alert).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<MarineAlert>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded, alert);
    });
}

// ---------------------------------------------------------------------------
// Test 12: Batch of 10 sensor readings — write_all / read_all
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_batch_10_sensors_write_all_read_all() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let layers = [
            OceanLayer::Surface,
            OceanLayer::Mesopelagic,
            OceanLayer::Bathypelagic,
            OceanLayer::Abyssopelagic,
        ];
        let sensors: Vec<BuoySensor> = (0u32..10)
            .map(|i| {
                make_buoy_sensor(
                    100 + i,
                    layers[(i as usize) % layers.len()].clone(),
                    1500 + i as i32 * 100,
                    5000 + i * 10000,
                )
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_all(sensors.clone()).await.expect("write_all");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: Vec<BuoySensor> = decoder.read_all().await.expect("read_all");
        assert_eq!(decoded.len(), 10);
        assert_eq!(decoded, sensors);
    });
}

// ---------------------------------------------------------------------------
// Test 13: Empty stream returns None
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_empty_stream_returns_none() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let result = decoder.read_item::<BuoySensor>().await.expect("read_item");
        assert_eq!(result, None);
        assert!(decoder.is_finished());
    });
}

// ---------------------------------------------------------------------------
// Test 14: Large batch of 50 sensor readings roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_large_batch_50_sensors() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let layers = [
            OceanLayer::Surface,
            OceanLayer::Mesopelagic,
            OceanLayer::Bathypelagic,
            OceanLayer::Abyssopelagic,
        ];
        let sensors: Vec<BuoySensor> = (0u32..50)
            .map(|i| {
                make_buoy_sensor(
                    200 + i,
                    layers[(i as usize) % layers.len()].clone(),
                    1000 + i as i32 * 50,
                    1000 + i * 8000,
                )
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for s in &sensors {
            encoder.write_item(s).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder.read_item::<BuoySensor>().await.expect("read_item") {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 50);
        assert_eq!(decoded, sensors);
    });
}

// ---------------------------------------------------------------------------
// Test 15: Progress tracking — items_processed after decoding batch
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_progress_tracking() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let sensors: Vec<BuoySensor> = (0u32..12)
            .map(|i| make_buoy_sensor(300 + i, OceanLayer::Surface, 2000, 500 + i * 100))
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for s in &sensors {
            encoder.write_item(s).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        while let Some(_) = decoder.read_item::<BuoySensor>().await.expect("read_item") {}

        let progress = decoder.progress();
        assert_eq!(progress.items_processed, 12);
        assert!(progress.bytes_processed > 0);
        assert!(decoder.is_finished());
    });
}

// ---------------------------------------------------------------------------
// Test 16: All ocean layers in one batch — verify order preserved
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_all_layers_one_batch() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let sensors = vec![
            make_buoy_sensor(10, OceanLayer::Surface, 2500, 100),
            make_buoy_sensor(11, OceanLayer::Mesopelagic, 1000, 75000),
            make_buoy_sensor(12, OceanLayer::Bathypelagic, 200, 200000),
            make_buoy_sensor(13, OceanLayer::Abyssopelagic, -150, 400000),
        ];

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for s in &sensors {
            encoder.write_item(s).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder.read_item::<BuoySensor>().await.expect("read_item") {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 4);
        assert_eq!(decoded[0].layer, OceanLayer::Surface);
        assert_eq!(decoded[1].layer, OceanLayer::Mesopelagic);
        assert_eq!(decoded[2].layer, OceanLayer::Bathypelagic);
        assert_eq!(decoded[3].layer, OceanLayer::Abyssopelagic);
    });
}

// ---------------------------------------------------------------------------
// Test 17: All marine events in one batch — verify event types preserved
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_all_marine_events_one_batch() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let alerts = vec![
            make_marine_alert(1, 1, MarineEvent::WaveHeightAlert, 7, 0, 0),
            make_marine_alert(2, 2, MarineEvent::TidalAnomaly, 5, 10_000000, 20_000000),
            make_marine_alert(
                3,
                3,
                MarineEvent::PollutionDetected,
                9,
                30_000000,
                40_000000,
            ),
            make_marine_alert(4, 4, MarineEvent::SpeciesTagged, 2, -10_000000, 50_000000),
            make_marine_alert(5, 5, MarineEvent::CurrentShift, 4, -20_000000, -30_000000),
        ];

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        for a in &alerts {
            encoder.write_item(a).await.expect("write");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder.read_item::<MarineAlert>().await.expect("read_item") {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 5);
        assert_eq!(decoded[0].event, MarineEvent::WaveHeightAlert);
        assert_eq!(decoded[1].event, MarineEvent::TidalAnomaly);
        assert_eq!(decoded[2].event, MarineEvent::PollutionDetected);
        assert_eq!(decoded[3].event, MarineEvent::SpeciesTagged);
        assert_eq!(decoded[4].event, MarineEvent::CurrentShift);
    });
}

// ---------------------------------------------------------------------------
// Test 18: Concurrent write/read — encoder in spawned task
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_concurrent_write_read() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let sensors: Vec<BuoySensor> = (0u32..25)
            .map(|i| make_buoy_sensor(500 + i, OceanLayer::Surface, 2100 + i as i32, 300 + i * 50))
            .collect();
        let expected = sensors.clone();

        let (writer, reader) = tokio::io::duplex(65536);

        let encode_handle = tokio::spawn(async move {
            let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
            for s in &sensors {
                encoder.write_item(s).await.expect("write");
            }
            encoder.finish().await.expect("finish");
        });

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder.read_item::<BuoySensor>().await.expect("read_item") {
            decoded.push(item);
        }

        encode_handle.await.expect("encoder task");

        assert_eq!(decoded.len(), 25);
        assert_eq!(decoded, expected);
        assert!(decoder.is_finished());
    });
}

// ---------------------------------------------------------------------------
// Test 19: Deep ocean sensor (Abyssopelagic) — boundary values
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_deep_ocean_sensor_abyssopelagic() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let sensor = BuoySensor {
            buoy_id: u32::MAX,
            layer: OceanLayer::Abyssopelagic,
            temp_mc: i32::MIN,
            salinity_ppm: u32::MAX,
            depth_cm: 1_100_000,
            timestamp_s: u64::MAX,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&sensor).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<BuoySensor>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded, sensor);
        assert_eq!(decoded.layer, OceanLayer::Abyssopelagic);
        assert_eq!(decoded.depth_cm, 1_100_000);
        assert_eq!(decoded.temp_mc, i32::MIN);
    });
}

// ---------------------------------------------------------------------------
// Test 20: Surface vs Bathypelagic distinct bytes — encoding differs by layer
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_surface_vs_bathypelagic_distinct_bytes() {
    let surface = BuoySensor {
        buoy_id: 1,
        layer: OceanLayer::Surface,
        temp_mc: 2000,
        salinity_ppm: 35000,
        depth_cm: 100,
        timestamp_s: 1_700_000_000,
    };
    let bathypelagic = BuoySensor {
        buoy_id: 1,
        layer: OceanLayer::Bathypelagic,
        temp_mc: 2000,
        salinity_ppm: 35000,
        depth_cm: 100,
        timestamp_s: 1_700_000_000,
    };

    let bytes_surface = encode_to_vec(&surface).expect("encode surface");
    let bytes_bathy = encode_to_vec(&bathypelagic).expect("encode bathypelagic");
    assert_ne!(
        bytes_surface, bytes_bathy,
        "surface and bathypelagic must encode differently"
    );

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&surface).await.expect("write surface");
        encoder
            .write_item(&bathypelagic)
            .await
            .expect("write bathy");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);

        let dec_surface = decoder
            .read_item::<BuoySensor>()
            .await
            .expect("read surface")
            .expect("some");
        assert_eq!(dec_surface.layer, OceanLayer::Surface);

        let dec_bathy = decoder
            .read_item::<BuoySensor>()
            .await
            .expect("read bathy")
            .expect("some");
        assert_eq!(dec_bathy.layer, OceanLayer::Bathypelagic);
    });
}

// ---------------------------------------------------------------------------
// Test 21: Pollution detection alert — verify full alert fields
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_pollution_detection_alert() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let alert = MarineAlert {
            alert_id: 0xCAFE_BABE_0042,
            buoy_id: 333,
            event: MarineEvent::PollutionDetected,
            severity: 10,
            lat_micro: 51_509865,
            lon_micro: -0_118092,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&alert).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<MarineAlert>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded, alert);
        assert_eq!(decoded.event, MarineEvent::PollutionDetected);
        assert_eq!(decoded.severity, 10);
        assert_eq!(decoded.alert_id, 0xCAFE_BABE_0042);
    });
}

// ---------------------------------------------------------------------------
// Test 22: Species tagging event — verify lat/lon micro-degree precision
// ---------------------------------------------------------------------------
#[test]
fn test_ocean_species_tagging_event() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let alert = MarineAlert {
            alert_id: 0x7A6_0000_BEEF,
            buoy_id: 212,
            event: MarineEvent::SpeciesTagged,
            severity: 3,
            lat_micro: -8_090000,
            lon_micro: -14_355000,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::with_config(writer, StreamingConfig::default());
        encoder.write_item(&alert).await.expect("write");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded = decoder
            .read_item::<MarineAlert>()
            .await
            .expect("read_item")
            .expect("some");
        assert_eq!(decoded, alert);
        assert_eq!(decoded.event, MarineEvent::SpeciesTagged);
        assert_eq!(decoded.lat_micro, -8_090000);
        assert_eq!(decoded.lon_micro, -14_355000);
    });
}
