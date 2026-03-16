//! Advanced async streaming tests (40th set) — telemedicine / remote patient monitoring domain.
//!
//! All 22 tests are top-level `#[test]` functions (no module wrapper, no async fn).
//! Each test drives a tokio Runtime via `block_on`.
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file:
//!   `VitalSign`, `AlertLevel`, `RemoteReading`, `ConsultationNote`
//!
//! Coverage matrix:
//!    1:  Single RemoteReading duplex roundtrip
//!   2-7: Each VitalSign variant async roundtrip
//!  8-11: Each AlertLevel variant async roundtrip
//!   12:  ConsultationNote full roundtrip
//!   13:  Batch of 10 readings write_all / read_all
//!   14:  Empty stream returns None
//!   15:  Large batch 50 readings
//!   16:  Progress tracking
//!   17:  All vital signs in one batch
//!   18:  Critical alert readings
//!   19:  Concurrent write/read (encoder in spawned task)
//!   20:  Heart rate normal range
//!   21:  SpO2 low saturation
//!   22:  Blood glucose spike
//!   23:  Temperature fever
//!   24:  Respiratory distress
//!   25:  Consultation with 10 readings
//!   26:  Doctor review stream
//!   27:  Patient history (20 readings)
//!   28:  Sequential read item by item
//!   29:  Sync vs async consistency
//!   30:  Multi-device patient monitoring

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types — telemedicine / remote patient monitoring
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VitalSign {
    HeartRate,
    BloodPressure,
    SpO2,
    Temperature,
    RespiratoryRate,
    BloodGlucose,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AlertLevel {
    Normal,
    Borderline,
    Elevated,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RemoteReading {
    device_id: u32,
    patient_id: u64,
    vital: VitalSign,
    value_micro: i64,
    alert: AlertLevel,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConsultationNote {
    note_id: u64,
    doctor_id: u32,
    patient_id: u64,
    summary: String,
    readings: Vec<RemoteReading>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_reading(
    device_id: u32,
    patient_id: u64,
    vital: VitalSign,
    value_micro: i64,
    alert: AlertLevel,
    timestamp_ms: u64,
) -> RemoteReading {
    RemoteReading {
        device_id,
        patient_id,
        vital,
        value_micro,
        alert,
        timestamp_ms,
    }
}

fn make_heart_rate(
    device_id: u32,
    patient_id: u64,
    bpm_micro: i64,
    alert: AlertLevel,
) -> RemoteReading {
    make_reading(
        device_id,
        patient_id,
        VitalSign::HeartRate,
        bpm_micro,
        alert,
        1_700_000_000_000 + device_id as u64 * 1000,
    )
}

// ---------------------------------------------------------------------------
// Test 1: Single RemoteReading duplex roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_single_remote_reading_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let reading = make_reading(
            42,
            100_000,
            VitalSign::HeartRate,
            72_000_000,
            AlertLevel::Normal,
            1_700_000_000_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded, reading);
    });
}

// ---------------------------------------------------------------------------
// Test 2: VitalSign::HeartRate variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_vital_heart_rate_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let reading = make_reading(
            1,
            200_001,
            VitalSign::HeartRate,
            65_000_000,
            AlertLevel::Normal,
            1_700_000_001_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.vital, VitalSign::HeartRate);
        assert_eq!(decoded.value_micro, 65_000_000);
        assert_eq!(decoded.patient_id, 200_001);
    });
}

// ---------------------------------------------------------------------------
// Test 3: VitalSign::BloodPressure variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_vital_blood_pressure_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let reading = make_reading(
            2,
            200_002,
            VitalSign::BloodPressure,
            120_000_000,
            AlertLevel::Normal,
            1_700_000_002_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.vital, VitalSign::BloodPressure);
        assert_eq!(decoded.value_micro, 120_000_000);
    });
}

// ---------------------------------------------------------------------------
// Test 4: VitalSign::SpO2 variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_vital_spo2_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let reading = make_reading(
            3,
            200_003,
            VitalSign::SpO2,
            98_000_000,
            AlertLevel::Normal,
            1_700_000_003_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.vital, VitalSign::SpO2);
        assert_eq!(decoded.device_id, 3);
        assert_eq!(decoded.value_micro, 98_000_000);
    });
}

// ---------------------------------------------------------------------------
// Test 5: VitalSign::Temperature variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_vital_temperature_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let reading = make_reading(
            4,
            200_004,
            VitalSign::Temperature,
            36_700_000,
            AlertLevel::Normal,
            1_700_000_004_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.vital, VitalSign::Temperature);
        assert_eq!(decoded.value_micro, 36_700_000);
    });
}

// ---------------------------------------------------------------------------
// Test 6: VitalSign::RespiratoryRate variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_vital_respiratory_rate_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let reading = make_reading(
            5,
            200_005,
            VitalSign::RespiratoryRate,
            16_000_000,
            AlertLevel::Normal,
            1_700_000_005_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.vital, VitalSign::RespiratoryRate);
        assert_eq!(decoded.value_micro, 16_000_000);
    });
}

// ---------------------------------------------------------------------------
// Test 7: VitalSign::BloodGlucose variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_vital_blood_glucose_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let reading = make_reading(
            6,
            200_006,
            VitalSign::BloodGlucose,
            5_500_000,
            AlertLevel::Normal,
            1_700_000_006_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.vital, VitalSign::BloodGlucose);
        assert_eq!(decoded.value_micro, 5_500_000);
    });
}

// ---------------------------------------------------------------------------
// Test 8: AlertLevel::Normal variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_alert_normal_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let reading = make_reading(
            10,
            300_001,
            VitalSign::HeartRate,
            70_000_000,
            AlertLevel::Normal,
            1_700_000_010_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.alert, AlertLevel::Normal);
        assert_eq!(decoded.device_id, 10);
    });
}

// ---------------------------------------------------------------------------
// Test 9: AlertLevel::Borderline variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_alert_borderline_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let reading = make_reading(
            11,
            300_002,
            VitalSign::BloodPressure,
            135_000_000,
            AlertLevel::Borderline,
            1_700_000_011_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.alert, AlertLevel::Borderline);
        assert_eq!(decoded.value_micro, 135_000_000);
    });
}

// ---------------------------------------------------------------------------
// Test 10: AlertLevel::Elevated variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_alert_elevated_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let reading = make_reading(
            12,
            300_003,
            VitalSign::HeartRate,
            105_000_000,
            AlertLevel::Elevated,
            1_700_000_012_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.alert, AlertLevel::Elevated);
        assert_eq!(decoded.value_micro, 105_000_000);
    });
}

// ---------------------------------------------------------------------------
// Test 11: AlertLevel::Critical variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_alert_critical_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let reading = make_reading(
            13,
            300_004,
            VitalSign::HeartRate,
            155_000_000,
            AlertLevel::Critical,
            1_700_000_013_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded.alert, AlertLevel::Critical);
        assert_eq!(decoded.value_micro, 155_000_000);
        assert_eq!(decoded.patient_id, 300_004);
    });
}

// ---------------------------------------------------------------------------
// Test 12: ConsultationNote full roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_consultation_note_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let readings = vec![
            make_reading(
                20,
                400_001,
                VitalSign::HeartRate,
                78_000_000,
                AlertLevel::Normal,
                1_700_000_020_000,
            ),
            make_reading(
                21,
                400_001,
                VitalSign::BloodPressure,
                125_000_000,
                AlertLevel::Borderline,
                1_700_000_021_000,
            ),
            make_reading(
                22,
                400_001,
                VitalSign::SpO2,
                97_000_000,
                AlertLevel::Normal,
                1_700_000_022_000,
            ),
        ];
        let note = ConsultationNote {
            note_id: 0xDEAD_0001,
            doctor_id: 9001,
            patient_id: 400_001,
            summary: String::from("Patient stable, blood pressure slightly elevated. Monitor."),
            readings,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&note).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: ConsultationNote =
            decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded, note);
        assert_eq!(decoded.doctor_id, 9001);
        assert_eq!(decoded.readings.len(), 3);
        assert_eq!(
            decoded.summary,
            "Patient stable, blood pressure slightly elevated. Monitor."
        );
    });
}

// ---------------------------------------------------------------------------
// Test 13: Batch of 10 readings — write_all / read_all
// ---------------------------------------------------------------------------
#[test]
fn test_tele_batch_10_readings_write_all_read_all() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let vitals = [
            VitalSign::HeartRate,
            VitalSign::BloodPressure,
            VitalSign::SpO2,
            VitalSign::Temperature,
            VitalSign::RespiratoryRate,
        ];
        let readings: Vec<RemoteReading> = (0u32..10)
            .map(|i| {
                make_reading(
                    500 + i,
                    600_000 + i as u64,
                    vitals[(i as usize) % vitals.len()].clone(),
                    60_000_000 + i as i64 * 1_000_000,
                    AlertLevel::Normal,
                    1_700_000_100_000 + i as u64 * 1000,
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
        let decoded: Vec<RemoteReading> = decoder.read_all().await.expect("read_all");
        assert_eq!(decoded.len(), 10);
        assert_eq!(decoded, readings);
    });
}

// ---------------------------------------------------------------------------
// Test 14: Empty stream returns None
// ---------------------------------------------------------------------------
#[test]
fn test_tele_empty_stream_returns_none() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let encoder = AsyncEncoder::new(writer);
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let result: Option<RemoteReading> = decoder.read_item().await.expect("read_item");
        assert_eq!(result, None);
        assert!(decoder.is_finished());
    });
}

// ---------------------------------------------------------------------------
// Test 15: Large batch 50 readings roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tele_large_batch_50_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let alerts = [
            AlertLevel::Normal,
            AlertLevel::Borderline,
            AlertLevel::Elevated,
            AlertLevel::Critical,
        ];
        let vitals = [
            VitalSign::HeartRate,
            VitalSign::BloodPressure,
            VitalSign::SpO2,
            VitalSign::Temperature,
            VitalSign::RespiratoryRate,
            VitalSign::BloodGlucose,
        ];
        let readings: Vec<RemoteReading> = (0u32..50)
            .map(|i| {
                make_reading(
                    700 + i,
                    800_000 + i as u64,
                    vitals[(i as usize) % vitals.len()].clone(),
                    50_000_000 + i as i64 * 500_000,
                    alerts[(i as usize) % alerts.len()].clone(),
                    1_700_000_200_000 + i as u64 * 1000,
                )
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for r in &readings {
            encoder.write_item(r).await.expect("write_item");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder
            .read_item::<RemoteReading>()
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
// Test 16: Progress tracking — items_processed and bytes_processed
// ---------------------------------------------------------------------------
#[test]
fn test_tele_progress_tracking() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let readings: Vec<RemoteReading> = (0u32..15)
            .map(|i| {
                make_reading(
                    900 + i,
                    1_000_000 + i as u64,
                    VitalSign::HeartRate,
                    70_000_000 + i as i64 * 100_000,
                    AlertLevel::Normal,
                    1_700_000_300_000 + i as u64 * 1000,
                )
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for r in &readings {
            encoder.write_item(r).await.expect("write_item");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        while let Some(_) = decoder
            .read_item::<RemoteReading>()
            .await
            .expect("read_item")
        {}

        let progress = decoder.progress();
        assert_eq!(progress.items_processed, 15);
        assert!(progress.bytes_processed > 0, "bytes_processed must be > 0");
        assert!(decoder.is_finished());
    });
}

// ---------------------------------------------------------------------------
// Test 17: All vital signs in one batch — verify order preserved
// ---------------------------------------------------------------------------
#[test]
fn test_tele_all_vital_signs_one_batch() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let readings = vec![
            make_reading(
                101,
                1_001,
                VitalSign::HeartRate,
                72_000_000,
                AlertLevel::Normal,
                1_700_000_400_000,
            ),
            make_reading(
                102,
                1_001,
                VitalSign::BloodPressure,
                118_000_000,
                AlertLevel::Normal,
                1_700_000_401_000,
            ),
            make_reading(
                103,
                1_001,
                VitalSign::SpO2,
                99_000_000,
                AlertLevel::Normal,
                1_700_000_402_000,
            ),
            make_reading(
                104,
                1_001,
                VitalSign::Temperature,
                37_000_000,
                AlertLevel::Normal,
                1_700_000_403_000,
            ),
            make_reading(
                105,
                1_001,
                VitalSign::RespiratoryRate,
                14_000_000,
                AlertLevel::Normal,
                1_700_000_404_000,
            ),
            make_reading(
                106,
                1_001,
                VitalSign::BloodGlucose,
                5_200_000,
                AlertLevel::Normal,
                1_700_000_405_000,
            ),
        ];

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for r in &readings {
            encoder.write_item(r).await.expect("write_item");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder
            .read_item::<RemoteReading>()
            .await
            .expect("read_item")
        {
            decoded.push(item);
        }
        assert_eq!(decoded.len(), 6);
        assert_eq!(decoded[0].vital, VitalSign::HeartRate);
        assert_eq!(decoded[1].vital, VitalSign::BloodPressure);
        assert_eq!(decoded[2].vital, VitalSign::SpO2);
        assert_eq!(decoded[3].vital, VitalSign::Temperature);
        assert_eq!(decoded[4].vital, VitalSign::RespiratoryRate);
        assert_eq!(decoded[5].vital, VitalSign::BloodGlucose);
        assert_eq!(decoded, readings);
    });
}

// ---------------------------------------------------------------------------
// Test 18: Critical alert readings — all items carry AlertLevel::Critical
// ---------------------------------------------------------------------------
#[test]
fn test_tele_critical_alert_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let critical_readings: Vec<RemoteReading> = (1u32..=6)
            .map(|i| {
                make_reading(
                    2_000 + i,
                    5_000_000 + i as u64,
                    VitalSign::HeartRate,
                    160_000_000 + i as i64 * 5_000_000,
                    AlertLevel::Critical,
                    1_700_000_500_000 + i as u64 * 500,
                )
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for r in &critical_readings {
            encoder.write_item(r).await.expect("write_item");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: Vec<RemoteReading> = decoder.read_all().await.expect("read_all");

        assert_eq!(decoded.len(), 6);
        for r in &decoded {
            assert_eq!(
                r.alert,
                AlertLevel::Critical,
                "expected all Critical alerts"
            );
        }
        assert_eq!(decoded, critical_readings);
    });
}

// ---------------------------------------------------------------------------
// Test 19: Concurrent write/read — encoder in spawned task
// ---------------------------------------------------------------------------
#[test]
fn test_tele_concurrent_write_read() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let readings: Vec<RemoteReading> = (0u32..20)
            .map(|i| {
                make_reading(
                    3_000 + i,
                    7_000_000 + i as u64,
                    VitalSign::SpO2,
                    95_000_000 + i as i64 * 100_000,
                    AlertLevel::Normal,
                    1_700_000_600_000 + i as u64 * 1000,
                )
            })
            .collect();
        let expected = readings.clone();

        let (writer, reader) = tokio::io::duplex(65536);

        let encode_handle = tokio::spawn(async move {
            let mut encoder = AsyncEncoder::new(writer);
            for r in &readings {
                encoder.write_item(r).await.expect("write_item in task");
            }
            encoder.finish().await.expect("finish in task");
        });

        let mut decoder = AsyncDecoder::new(reader);
        let mut decoded = Vec::new();
        while let Some(item) = decoder
            .read_item::<RemoteReading>()
            .await
            .expect("read_item")
        {
            decoded.push(item);
        }

        encode_handle.await.expect("encoder task panicked");

        assert_eq!(decoded.len(), 20);
        assert_eq!(decoded, expected);
        assert!(decoder.is_finished());
    });
}

// ---------------------------------------------------------------------------
// Test 20: Heart rate normal range — values in [60, 100] bpm
// ---------------------------------------------------------------------------
#[test]
fn test_tele_heart_rate_normal_range() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let readings: Vec<RemoteReading> = (60u32..=100)
            .step_by(5)
            .map(|bpm| make_heart_rate(bpm, 9_000_001, bpm as i64 * 1_000_000, AlertLevel::Normal))
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_all(readings.clone())
            .await
            .expect("write_all");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: Vec<RemoteReading> = decoder.read_all().await.expect("read_all");

        assert_eq!(decoded.len(), readings.len());
        for r in &decoded {
            assert_eq!(r.vital, VitalSign::HeartRate);
            assert_eq!(r.alert, AlertLevel::Normal);
            let bpm = r.value_micro / 1_000_000;
            assert!(bpm >= 60 && bpm <= 100, "bpm {bpm} out of normal range");
        }
        assert_eq!(decoded, readings);
    });
}

// ---------------------------------------------------------------------------
// Test 21: SpO2 low saturation — hypoxia alert scenario
// ---------------------------------------------------------------------------
#[test]
fn test_tele_spo2_low_saturation() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        // SpO2 values below 90% are critical (in micro-percent units)
        let low_spo2 = make_reading(
            4_001,
            10_000_001,
            VitalSign::SpO2,
            88_000_000, // 88% — hypoxia
            AlertLevel::Critical,
            1_700_000_700_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&low_spo2).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded, low_spo2);
        assert_eq!(decoded.vital, VitalSign::SpO2);
        assert_eq!(decoded.alert, AlertLevel::Critical);
        assert!(decoded.value_micro < 90_000_000, "expected SpO2 below 90%");
    });
}

// ---------------------------------------------------------------------------
// Test 22: Blood glucose spike — hyperglycemia scenario
// ---------------------------------------------------------------------------
#[test]
fn test_tele_blood_glucose_spike() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        // Glucose spike: 25.0 mmol/L in micro units = 25_000_000
        let spike = make_reading(
            5_001,
            11_000_001,
            VitalSign::BloodGlucose,
            25_000_000,
            AlertLevel::Critical,
            1_700_000_800_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&spike).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded, spike);
        assert_eq!(decoded.vital, VitalSign::BloodGlucose);
        assert_eq!(decoded.alert, AlertLevel::Critical);
        assert_eq!(decoded.value_micro, 25_000_000);
    });
}

// ---------------------------------------------------------------------------
// Test 23: Temperature fever — body temperature above 38.5 °C
// ---------------------------------------------------------------------------
#[test]
fn test_tele_temperature_fever() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let fever = make_reading(
            6_001,
            12_000_001,
            VitalSign::Temperature,
            39_200_000, // 39.2 °C in micro units
            AlertLevel::Elevated,
            1_700_000_900_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&fever).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: RemoteReading = decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded, fever);
        assert_eq!(decoded.vital, VitalSign::Temperature);
        assert!(
            decoded.value_micro > 38_500_000,
            "expected temperature above 38.5 C, got {}",
            decoded.value_micro
        );
        assert_eq!(decoded.alert, AlertLevel::Elevated);
    });
}

// ---------------------------------------------------------------------------
// Test 24: Respiratory distress — elevated respiratory rate
// ---------------------------------------------------------------------------
#[test]
fn test_tele_respiratory_distress() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        // Normal resp rate: 12-20 breaths/min; >25 = elevated distress
        let distress_readings: Vec<RemoteReading> = (25u32..=30)
            .map(|rate| {
                make_reading(
                    7_000 + rate,
                    13_000_001,
                    VitalSign::RespiratoryRate,
                    rate as i64 * 1_000_000,
                    AlertLevel::Elevated,
                    1_700_001_000_000 + rate as u64 * 1000,
                )
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder
            .write_all(distress_readings.clone())
            .await
            .expect("write_all");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: Vec<RemoteReading> = decoder.read_all().await.expect("read_all");

        assert_eq!(decoded.len(), distress_readings.len());
        for r in &decoded {
            assert_eq!(r.vital, VitalSign::RespiratoryRate);
            assert_eq!(r.alert, AlertLevel::Elevated);
            let rate = r.value_micro / 1_000_000;
            assert!(rate >= 25, "expected elevated resp rate >= 25, got {rate}");
        }
        assert_eq!(decoded, distress_readings);
    });
}

// ---------------------------------------------------------------------------
// Test 25: Consultation with 10 readings
// ---------------------------------------------------------------------------
#[test]
fn test_tele_consultation_with_10_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let vitals = [
            VitalSign::HeartRate,
            VitalSign::BloodPressure,
            VitalSign::SpO2,
            VitalSign::Temperature,
            VitalSign::RespiratoryRate,
        ];
        let readings: Vec<RemoteReading> = (0u32..10)
            .map(|i| {
                make_reading(
                    8_000 + i,
                    14_000_001,
                    vitals[(i as usize) % vitals.len()].clone(),
                    70_000_000 + i as i64 * 1_000_000,
                    AlertLevel::Normal,
                    1_700_001_100_000 + i as u64 * 1000,
                )
            })
            .collect();
        let note = ConsultationNote {
            note_id: 0xBEEF_0002,
            doctor_id: 9_002,
            patient_id: 14_000_001,
            summary: String::from("Routine checkup. All readings within acceptable range."),
            readings,
        };

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&note).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: ConsultationNote =
            decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(decoded, note);
        assert_eq!(decoded.readings.len(), 10);
        assert_eq!(decoded.doctor_id, 9_002);
    });
}

// ---------------------------------------------------------------------------
// Test 26: Doctor review stream — multiple ConsultationNotes streamed
// ---------------------------------------------------------------------------
#[test]
fn test_tele_doctor_review_stream() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let notes: Vec<ConsultationNote> = (1u64..=5)
            .map(|i| ConsultationNote {
                note_id: 1_000 + i,
                doctor_id: 9_003,
                patient_id: 15_000_000 + i,
                summary: format!("Review note {} — vitals reviewed", i),
                readings: vec![make_reading(
                    9_000 + i as u32,
                    15_000_000 + i,
                    VitalSign::HeartRate,
                    68_000_000 + i as i64 * 500_000,
                    AlertLevel::Normal,
                    1_700_001_200_000 + i * 1000,
                )],
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        for n in &notes {
            encoder.write_item(n).await.expect("write_item");
        }
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: Vec<ConsultationNote> = decoder.read_all().await.expect("read_all");

        assert_eq!(decoded.len(), 5);
        for (i, note) in decoded.iter().enumerate() {
            assert_eq!(note.doctor_id, 9_003);
            assert_eq!(note.readings.len(), 1);
            assert_eq!(note.note_id, 1_001 + i as u64);
        }
        assert_eq!(decoded, notes);
    });
}

// ---------------------------------------------------------------------------
// Test 27: Patient history — 20 readings across multiple vital signs
// ---------------------------------------------------------------------------
#[test]
fn test_tele_patient_history_20_readings() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let vitals = [
            VitalSign::HeartRate,
            VitalSign::BloodPressure,
            VitalSign::SpO2,
            VitalSign::Temperature,
            VitalSign::RespiratoryRate,
            VitalSign::BloodGlucose,
        ];
        let history: Vec<RemoteReading> = (0u32..20)
            .map(|i| {
                make_reading(
                    10_000 + i,
                    16_000_001,
                    vitals[(i as usize) % vitals.len()].clone(),
                    60_000_000 + i as i64 * 2_000_000,
                    AlertLevel::Normal,
                    1_700_001_300_000 + i as u64 * 3600_000,
                )
            })
            .collect();

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_all(history.clone()).await.expect("write_all");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: Vec<RemoteReading> = decoder.read_all().await.expect("read_all");

        assert_eq!(decoded.len(), 20);
        assert_eq!(decoded[0].vital, VitalSign::HeartRate);
        assert_eq!(decoded[5].vital, VitalSign::BloodGlucose);
        assert_eq!(decoded, history);
    });
}

// ---------------------------------------------------------------------------
// Test 28: Sequential read item by item — verify ordering and EOF
// ---------------------------------------------------------------------------
#[test]
fn test_tele_sequential_read_item_by_item() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let r1 = make_reading(
            11_001,
            17_000_001,
            VitalSign::HeartRate,
            75_000_000,
            AlertLevel::Normal,
            1_700_001_400_000,
        );
        let r2 = make_reading(
            11_002,
            17_000_001,
            VitalSign::SpO2,
            97_000_000,
            AlertLevel::Normal,
            1_700_001_401_000,
        );
        let r3 = make_reading(
            11_003,
            17_000_001,
            VitalSign::Temperature,
            37_200_000,
            AlertLevel::Normal,
            1_700_001_402_000,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&r1).await.expect("write r1");
        encoder.write_item(&r2).await.expect("write r2");
        encoder.write_item(&r3).await.expect("write r3");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);

        let d1: RemoteReading = decoder
            .read_item()
            .await
            .expect("read r1")
            .expect("some r1");
        assert_eq!(d1, r1, "first reading mismatch");

        let d2: RemoteReading = decoder
            .read_item()
            .await
            .expect("read r2")
            .expect("some r2");
        assert_eq!(d2, r2, "second reading mismatch");

        let d3: RemoteReading = decoder
            .read_item()
            .await
            .expect("read r3")
            .expect("some r3");
        assert_eq!(d3, r3, "third reading mismatch");

        let eof: Option<RemoteReading> = decoder.read_item().await.expect("eof read");
        assert_eq!(eof, None, "expected None after all readings");
        assert!(decoder.is_finished());
    });
}

// ---------------------------------------------------------------------------
// Test 29: Sync vs async consistency — same reading encodes identically
// ---------------------------------------------------------------------------
#[test]
fn test_tele_sync_vs_async_consistency() {
    let reading = make_reading(
        12_001,
        18_000_001,
        VitalSign::BloodPressure,
        130_000_000,
        AlertLevel::Borderline,
        1_700_001_500_000,
    );

    let sync_encoded = encode_to_vec(&reading).expect("encode_to_vec");
    let (sync_decoded, sync_consumed): (RemoteReading, _) =
        decode_from_slice(&sync_encoded).expect("decode_from_slice");
    assert_eq!(sync_decoded, reading);
    assert_eq!(sync_consumed, sync_encoded.len());

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&reading).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let async_decoded: RemoteReading =
            decoder.read_item().await.expect("read_item").expect("some");
        assert_eq!(async_decoded, reading);
        assert_eq!(
            async_decoded, sync_decoded,
            "async and sync decode must match"
        );
    });
}

// ---------------------------------------------------------------------------
// Test 30: Multi-device patient monitoring — 5 devices, 1 patient
// ---------------------------------------------------------------------------
#[test]
fn test_tele_multi_device_patient_monitoring() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let patient_id = 19_000_001u64;
        let devices = [
            (
                13_001u32,
                VitalSign::HeartRate,
                74_000_000i64,
                AlertLevel::Normal,
            ),
            (
                13_002u32,
                VitalSign::BloodPressure,
                122_000_000i64,
                AlertLevel::Normal,
            ),
            (
                13_003u32,
                VitalSign::SpO2,
                96_000_000i64,
                AlertLevel::Normal,
            ),
            (
                13_004u32,
                VitalSign::Temperature,
                37_100_000i64,
                AlertLevel::Normal,
            ),
            (
                13_005u32,
                VitalSign::BloodGlucose,
                5_800_000i64,
                AlertLevel::Borderline,
            ),
        ];

        let readings: Vec<RemoteReading> = devices
            .iter()
            .map(|(dev_id, vital, val, alert)| {
                make_reading(
                    *dev_id,
                    patient_id,
                    vital.clone(),
                    *val,
                    alert.clone(),
                    1_700_001_600_000 + *dev_id as u64 * 500,
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
        let decoded: Vec<RemoteReading> = decoder.read_all().await.expect("read_all");

        assert_eq!(decoded.len(), 5);
        for r in &decoded {
            assert_eq!(
                r.patient_id, patient_id,
                "all readings must belong to same patient"
            );
        }
        assert_eq!(decoded[0].vital, VitalSign::HeartRate);
        assert_eq!(decoded[4].vital, VitalSign::BloodGlucose);
        assert_eq!(decoded[4].alert, AlertLevel::Borderline);
        assert_eq!(decoded, readings);
    });
}
