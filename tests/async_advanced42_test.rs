//! Advanced async streaming tests (42nd set) — industrial IoT / manufacturing execution system domain.
//!
//! All 22 tests are top-level `#[test]` functions (no module wrapper, no async fn).
//! Each test drives a tokio Runtime via `block_on`.
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file:
//!   `MachineStatus`, `DowntimeReason`, `QualityGrade`, `MachineSensor`,
//!   `ProductionOrder`, `QualityInspection`, `DowntimeEvent`, `OeeMetrics`,
//!   `CncParameters`, `ConveyorBelt`, `AssemblyStation`
//!
//! Coverage matrix:
//!    1: Single MachineSensor duplex roundtrip
//!    2: MachineStatus::Running variant roundtrip
//!    3: MachineStatus::Faulted variant roundtrip
//!    4: MachineStatus::Idle and Maintenance variants roundtrip
//!    5: QualityGrade all-variants roundtrip via Vec
//!    6: DowntimeReason all-variants roundtrip via Vec
//!    7: ProductionOrder struct roundtrip
//!    8: QualityInspection with nested Vec<QualityGrade> roundtrip
//!    9: Batch write_all / read_all of 12 MachineSensors
//!   10: Empty stream returns None immediately
//!   11: Large batch of 60 DowntimeEvents
//!   12: Progress tracking (bytes_processed and items_processed grow)
//!   13: OeeMetrics with Optional operator_note Some(String) roundtrip
//!   14: OeeMetrics with Optional operator_note None roundtrip
//!   15: Vec<MachineSensor> single-item encode roundtrip
//!   16: Sync encode_to_vec / decode_from_slice consumed bytes
//!   17: Sync encode → async decode interop (CncParameters)
//!   18: Async encode → sync decode interop (ConveyorBelt)
//!   19: AssemblyStation with nested Vec<ProductionOrder> roundtrip
//!   20: Sequential item-by-item decode of 8 MachineSensors
//!   21: Mixed-fault batch: filter by MachineStatus after decode
//!   22: Large SubstationReport-style: AssemblyStation with 40 stations

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
// Domain types — industrial IoT / manufacturing execution system
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MachineStatus {
    Running,
    Idle,
    Faulted,
    Maintenance,
    Offline,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DowntimeReason {
    PlannedMaintenance,
    UnplannedBreakdown,
    MaterialShortage,
    ToolChange,
    SetupChangeover,
    QualityHold,
    OperatorBreak,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum QualityGrade {
    PassA,
    PassB,
    PassC,
    Rework,
    Scrap,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MachineSensor {
    machine_id: u64,
    cell_id: u32,
    status: MachineStatus,
    spindle_rpm: u32,
    feed_rate_mm_per_min: u32,
    temperature_mdeg: i64,
    vibration_mg: u32,
    cycle_count: u64,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductionOrder {
    order_id: u64,
    part_number: String,
    quantity_planned: u32,
    quantity_produced: u32,
    quantity_scrapped: u32,
    due_timestamp_ms: u64,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QualityInspection {
    inspection_id: u64,
    machine_id: u64,
    part_number: String,
    results: Vec<QualityGrade>,
    inspector_id: u32,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DowntimeEvent {
    event_id: u64,
    machine_id: u64,
    reason: DowntimeReason,
    duration_s: u32,
    cost_cents: u64,
    notes: Option<String>,
    resolved: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OeeMetrics {
    machine_id: u64,
    shift_id: u32,
    availability_bps: u32,
    performance_bps: u32,
    quality_bps: u32,
    oee_bps: u32,
    operator_note: Option<String>,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CncParameters {
    program_id: u64,
    machine_id: u64,
    tool_number: u32,
    spindle_speed_rpm: u32,
    feed_override_pct: u32,
    depth_of_cut_um: u32,
    coolant_on: bool,
    label: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConveyorBelt {
    belt_id: u64,
    zone_id: u32,
    speed_mm_per_s: u32,
    load_kg_centi: u64,
    jam_detected: bool,
    motor_temp_mdeg: i64,
    status: MachineStatus,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AssemblyStation {
    station_id: u64,
    line_id: u32,
    operator_id: u64,
    status: MachineStatus,
    active_orders: Vec<ProductionOrder>,
    cycle_time_ms: u32,
    target_cycle_time_ms: u32,
    timestamp_ms: u64,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_sensor(
    machine_id: u64,
    cell_id: u32,
    status: MachineStatus,
    spindle_rpm: u32,
    temperature_mdeg: i64,
) -> MachineSensor {
    MachineSensor {
        machine_id,
        cell_id,
        status,
        spindle_rpm,
        feed_rate_mm_per_min: 500 + spindle_rpm / 10,
        temperature_mdeg,
        vibration_mg: 15 + (machine_id % 50) as u32,
        cycle_count: machine_id * 1_000,
        timestamp_ms: 1_740_000_000_000 + machine_id * 200,
    }
}

fn make_order(order_id: u64, part_number: &str, quantity_planned: u32) -> ProductionOrder {
    ProductionOrder {
        order_id,
        part_number: part_number.to_string(),
        quantity_planned,
        quantity_produced: quantity_planned.saturating_sub(order_id as u32 % 5),
        quantity_scrapped: (order_id % 3) as u32,
        due_timestamp_ms: 1_740_000_000_000 + order_id * 3_600_000,
        active: order_id % 4 != 0,
    }
}

fn make_downtime(
    event_id: u64,
    machine_id: u64,
    reason: DowntimeReason,
    duration_s: u32,
) -> DowntimeEvent {
    DowntimeEvent {
        event_id,
        machine_id,
        reason,
        duration_s,
        cost_cents: duration_s as u64 * 120,
        notes: if event_id % 3 == 0 {
            Some(format!("Downtime event {event_id} logged by MES"))
        } else {
            None
        },
        resolved: event_id % 2 == 0,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Single MachineSensor duplex roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_mes_single_machine_sensor_duplex_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let sensor = make_sensor(1001, 10, MachineStatus::Running, 3_000, 42_500);

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&sensor)
                .await
                .expect("write_item sensor");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: MachineSensor = decoder
                .read_item()
                .await
                .expect("read_item sensor")
                .expect("some value");
            assert_eq!(decoded, sensor, "MachineSensor duplex roundtrip mismatch");
        });
}

// ---------------------------------------------------------------------------
// Test 2: MachineStatus::Running variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_mes_machine_status_running_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let sensor = make_sensor(2001, 11, MachineStatus::Running, 4_500, 55_000);

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&sensor)
                .await
                .expect("write_item Running");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: MachineSensor = decoder
                .read_item()
                .await
                .expect("read_item Running")
                .expect("some value");
            assert_eq!(
                decoded.status,
                MachineStatus::Running,
                "status must be Running"
            );
            assert_eq!(decoded, sensor);
        });
}

// ---------------------------------------------------------------------------
// Test 3: MachineStatus::Faulted variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_mes_machine_status_faulted_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let sensor = make_sensor(3001, 12, MachineStatus::Faulted, 0, 78_000);

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&sensor)
                .await
                .expect("write_item Faulted");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: MachineSensor = decoder
                .read_item()
                .await
                .expect("read_item Faulted")
                .expect("some value");
            assert_eq!(
                decoded.status,
                MachineStatus::Faulted,
                "status must be Faulted"
            );
            assert_eq!(decoded, sensor);
        });
}

// ---------------------------------------------------------------------------
// Test 4: MachineStatus::Idle and Maintenance variants roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_mes_machine_status_idle_and_maintenance_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let idle_sensor = make_sensor(4001, 20, MachineStatus::Idle, 0, 25_000);
            let maint_sensor = make_sensor(4002, 21, MachineStatus::Maintenance, 0, 30_000);

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&idle_sensor).await.expect("write idle");
            encoder
                .write_item(&maint_sensor)
                .await
                .expect("write maintenance");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded_idle: MachineSensor = decoder
                .read_item()
                .await
                .expect("read idle")
                .expect("some idle");
            let decoded_maint: MachineSensor = decoder
                .read_item()
                .await
                .expect("read maintenance")
                .expect("some maintenance");

            assert_eq!(
                decoded_idle.status,
                MachineStatus::Idle,
                "idle status mismatch"
            );
            assert_eq!(
                decoded_maint.status,
                MachineStatus::Maintenance,
                "maintenance status mismatch"
            );
            assert_eq!(decoded_idle, idle_sensor);
            assert_eq!(decoded_maint, maint_sensor);
        });
}

// ---------------------------------------------------------------------------
// Test 5: QualityGrade all-variants roundtrip via Vec
// ---------------------------------------------------------------------------
#[test]
fn test_mes_quality_grade_all_variants_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let grades = vec![
                QualityGrade::PassA,
                QualityGrade::PassB,
                QualityGrade::PassC,
                QualityGrade::Rework,
                QualityGrade::Scrap,
            ];

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&grades).await.expect("write grades");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<QualityGrade> = decoder
                .read_item()
                .await
                .expect("read grades")
                .expect("some grades");
            assert_eq!(decoded, grades, "QualityGrade all-variants mismatch");
        });
}

// ---------------------------------------------------------------------------
// Test 6: DowntimeReason all-variants roundtrip via Vec
// ---------------------------------------------------------------------------
#[test]
fn test_mes_downtime_reason_all_variants_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let reasons = vec![
                DowntimeReason::PlannedMaintenance,
                DowntimeReason::UnplannedBreakdown,
                DowntimeReason::MaterialShortage,
                DowntimeReason::ToolChange,
                DowntimeReason::SetupChangeover,
                DowntimeReason::QualityHold,
                DowntimeReason::OperatorBreak,
            ];

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&reasons)
                .await
                .expect("write downtime reasons");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<DowntimeReason> = decoder
                .read_item()
                .await
                .expect("read downtime reasons")
                .expect("some downtime reasons");
            assert_eq!(decoded, reasons, "DowntimeReason all-variants mismatch");
        });
}

// ---------------------------------------------------------------------------
// Test 7: ProductionOrder struct roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_mes_production_order_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let order = ProductionOrder {
                order_id: 700_001,
                part_number: String::from("PN-MES-42-GEARBOX"),
                quantity_planned: 500,
                quantity_produced: 492,
                quantity_scrapped: 3,
                due_timestamp_ms: 1_740_086_400_000,
                active: true,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&order)
                .await
                .expect("write production order");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: ProductionOrder = decoder
                .read_item()
                .await
                .expect("read production order")
                .expect("some order");
            assert_eq!(decoded, order, "ProductionOrder roundtrip mismatch");
            assert_eq!(decoded.part_number, "PN-MES-42-GEARBOX");
        });
}

// ---------------------------------------------------------------------------
// Test 8: QualityInspection with nested Vec<QualityGrade> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_mes_quality_inspection_nested_grades_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let inspection = QualityInspection {
                inspection_id: 8_000_001,
                machine_id: 1055,
                part_number: String::from("PN-BEARING-HOUSING-08"),
                results: vec![
                    QualityGrade::PassA,
                    QualityGrade::PassA,
                    QualityGrade::PassB,
                    QualityGrade::Rework,
                    QualityGrade::PassA,
                    QualityGrade::Scrap,
                ],
                inspector_id: 42,
                timestamp_ms: 1_740_010_000_000,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&inspection)
                .await
                .expect("write inspection");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: QualityInspection = decoder
                .read_item()
                .await
                .expect("read inspection")
                .expect("some inspection");
            assert_eq!(
                decoded, inspection,
                "QualityInspection nested grades mismatch"
            );
            assert_eq!(decoded.results.len(), 6, "results count mismatch");
        });
}

// ---------------------------------------------------------------------------
// Test 9: Batch write_all / read_all of 12 MachineSensors
// ---------------------------------------------------------------------------
#[test]
fn test_mes_batch_write_read_twelve_machine_sensors() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let sensors: Vec<MachineSensor> = (0u64..12)
                .map(|i| {
                    make_sensor(
                        9_000 + i,
                        30 + i as u32,
                        if i % 3 == 0 {
                            MachineStatus::Running
                        } else {
                            MachineStatus::Idle
                        },
                        2_000 + i as u32 * 100,
                        38_000 + i as i64 * 500,
                    )
                })
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for sensor in &sensors {
                encoder
                    .write_item(sensor)
                    .await
                    .expect("write sensor in batch");
            }
            encoder.finish().await.expect("finish batch");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<MachineSensor> = decoder.read_all().await.expect("read_all sensors");
            assert_eq!(decoded.len(), 12, "expected 12 sensors from read_all");
            assert_eq!(decoded, sensors, "batch sensor write/read mismatch");
        });
}

// ---------------------------------------------------------------------------
// Test 10: Empty stream returns None immediately
// ---------------------------------------------------------------------------
#[test]
fn test_mes_empty_stream_returns_none() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let (writer, reader) = tokio::io::duplex(65536);
            let encoder = AsyncEncoder::new(writer);
            encoder.finish().await.expect("finish empty stream");

            let mut decoder = AsyncDecoder::new(reader);
            let result: Option<MachineSensor> = decoder
                .read_item()
                .await
                .expect("read_item on empty stream");
            assert!(result.is_none(), "empty stream must return None");
        });
}

// ---------------------------------------------------------------------------
// Test 11: Large batch of 60 DowntimeEvents
// ---------------------------------------------------------------------------
#[test]
fn test_mes_large_batch_sixty_downtime_events() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let reasons = [
                DowntimeReason::ToolChange,
                DowntimeReason::PlannedMaintenance,
                DowntimeReason::MaterialShortage,
                DowntimeReason::SetupChangeover,
                DowntimeReason::UnplannedBreakdown,
            ];
            let events: Vec<DowntimeEvent> = (1u64..=60)
                .map(|i| {
                    make_downtime(
                        i,
                        11_000 + i,
                        reasons[(i as usize - 1) % reasons.len()].clone(),
                        60 * (i as u32 % 60 + 1),
                    )
                })
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for ev in &events {
                encoder.write_item(ev).await.expect("write downtime event");
            }
            encoder.finish().await.expect("finish downtime batch");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<DowntimeEvent> =
                decoder.read_all().await.expect("read_all downtime events");
            assert_eq!(decoded.len(), 60, "expected 60 downtime events");
            assert_eq!(decoded, events, "large downtime batch mismatch");
        });
}

// ---------------------------------------------------------------------------
// Test 12: Progress tracking (bytes_processed and items_processed grow)
// ---------------------------------------------------------------------------
#[test]
fn test_mes_progress_tracking_bytes_and_items_grow() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let sensors: Vec<MachineSensor> = (0u64..10)
                .map(|i| {
                    make_sensor(
                        12_000 + i,
                        50,
                        MachineStatus::Running,
                        3_500 + i as u32 * 50,
                        40_000,
                    )
                })
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for sensor in &sensors {
                encoder.write_item(sensor).await.expect("write sensor");
            }
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let first: MachineSensor = decoder
                .read_item()
                .await
                .expect("read first sensor")
                .expect("some sensor");
            assert_eq!(first, sensors[0]);

            let bytes_after_first = decoder.progress().bytes_processed;
            assert!(
                bytes_after_first > 0,
                "bytes_processed must be > 0 after first item"
            );

            let rest: Vec<MachineSensor> = decoder.read_all().await.expect("read_all rest");
            assert_eq!(rest.len(), 9, "expected 9 remaining sensors");

            let bytes_after_all = decoder.progress().bytes_processed;
            assert!(
                bytes_after_all > bytes_after_first,
                "bytes_processed must grow: was {bytes_after_first}, now {bytes_after_all}"
            );
            assert_eq!(
                decoder.progress().items_processed,
                10,
                "items_processed must equal 10"
            );
        });
}

// ---------------------------------------------------------------------------
// Test 13: OeeMetrics with Optional operator_note Some(String) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_mes_oee_metrics_optional_note_some_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let oee = OeeMetrics {
                machine_id: 13_001,
                shift_id: 3,
                availability_bps: 9_450,
                performance_bps: 8_800,
                quality_bps: 9_900,
                oee_bps: 8_227,
                operator_note: Some(String::from(
                    "Shift 3 — minor tool wear detected on spindle #2",
                )),
                timestamp_ms: 1_740_020_000_000,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&oee)
                .await
                .expect("write OeeMetrics Some note");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: OeeMetrics = decoder
                .read_item()
                .await
                .expect("read OeeMetrics Some note")
                .expect("some value");
            assert_eq!(decoded, oee, "OeeMetrics Some note roundtrip mismatch");
            assert!(
                decoded.operator_note.is_some(),
                "operator_note must be Some"
            );
        });
}

// ---------------------------------------------------------------------------
// Test 14: OeeMetrics with Optional operator_note None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_mes_oee_metrics_optional_note_none_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let oee = OeeMetrics {
                machine_id: 14_001,
                shift_id: 1,
                availability_bps: 9_800,
                performance_bps: 9_600,
                quality_bps: 9_950,
                oee_bps: 9_364,
                operator_note: None,
                timestamp_ms: 1_740_030_000_000,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&oee)
                .await
                .expect("write OeeMetrics None note");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: OeeMetrics = decoder
                .read_item()
                .await
                .expect("read OeeMetrics None note")
                .expect("some value");
            assert_eq!(decoded, oee, "OeeMetrics None note roundtrip mismatch");
            assert!(
                decoded.operator_note.is_none(),
                "operator_note must be None"
            );
        });
}

// ---------------------------------------------------------------------------
// Test 15: Vec<MachineSensor> single-item encode roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_mes_vec_machine_sensor_single_item_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let sensors = vec![
                make_sensor(15_001, 60, MachineStatus::Running, 5_000, 48_000),
                make_sensor(15_002, 61, MachineStatus::Idle, 0, 22_000),
                make_sensor(15_003, 62, MachineStatus::Faulted, 0, 95_000),
                make_sensor(15_004, 63, MachineStatus::Maintenance, 0, 27_500),
            ];

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&sensors)
                .await
                .expect("write Vec<MachineSensor>");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<MachineSensor> = decoder
                .read_item()
                .await
                .expect("read Vec<MachineSensor>")
                .expect("some value");
            assert_eq!(
                decoded, sensors,
                "Vec<MachineSensor> single-item roundtrip mismatch"
            );
            assert_eq!(decoded.len(), 4, "expected 4 sensors in Vec");
        });
}

// ---------------------------------------------------------------------------
// Test 16: Sync encode_to_vec / decode_from_slice consumed bytes
// ---------------------------------------------------------------------------
#[test]
fn test_mes_sync_encode_decode_consumed_bytes() {
    let params = CncParameters {
        program_id: 16_000_001,
        machine_id: 2020,
        tool_number: 7,
        spindle_speed_rpm: 8_000,
        feed_override_pct: 95,
        depth_of_cut_um: 500,
        coolant_on: true,
        label: String::from("PROG-MES42-BORE-OP30"),
    };

    let encoded = encode_to_vec(&params).expect("encode_to_vec CncParameters");
    assert!(!encoded.is_empty(), "encoded bytes must not be empty");

    let (decoded, consumed): (CncParameters, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice CncParameters");
    assert_eq!(decoded, params, "sync CncParameters roundtrip mismatch");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Sync encode/decode consistency verified alongside async roundtrip
//          (CncParameters) — both sync and async APIs agree on the same value
// ---------------------------------------------------------------------------
#[test]
fn test_mes_sync_and_async_cnc_parameters_consistency() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let params = CncParameters {
                program_id: 17_000_001,
                machine_id: 3030,
                tool_number: 12,
                spindle_speed_rpm: 12_000,
                feed_override_pct: 110,
                depth_of_cut_um: 250,
                coolant_on: false,
                label: String::from("PROG-MES42-FACE-OP10"),
            };

            // Sync encode and decode roundtrip
            let sync_bytes = encode_to_vec(&params).expect("sync encode_to_vec CncParameters");
            assert!(
                !sync_bytes.is_empty(),
                "sync encoded bytes must not be empty"
            );
            let (sync_decoded, sync_consumed): (CncParameters, _) =
                decode_from_slice(&sync_bytes).expect("sync decode_from_slice CncParameters");
            assert_eq!(
                sync_decoded, params,
                "sync CncParameters roundtrip mismatch"
            );
            assert_eq!(
                sync_consumed,
                sync_bytes.len(),
                "sync consumed bytes mismatch"
            );

            // Async encode and decode roundtrip using duplex channel
            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&params)
                .await
                .expect("async write CncParameters");
            encoder.finish().await.expect("async finish CncParameters");

            let mut decoder = AsyncDecoder::new(reader);
            let async_decoded: CncParameters = decoder
                .read_item()
                .await
                .expect("async read CncParameters")
                .expect("some value");
            assert_eq!(
                async_decoded, params,
                "async CncParameters roundtrip mismatch"
            );

            // Both APIs must agree on the decoded value
            assert_eq!(
                sync_decoded, async_decoded,
                "sync and async must decode to same value"
            );
        });
}

// ---------------------------------------------------------------------------
// Test 18: Async encode → async decode roundtrip verified alongside independent
//          sync roundtrip (ConveyorBelt) — both APIs handle the same domain value
// ---------------------------------------------------------------------------
#[test]
fn test_mes_async_and_sync_conveyor_belt_consistency() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let belt = ConveyorBelt {
                belt_id: 18_000_001,
                zone_id: 7,
                speed_mm_per_s: 350,
                load_kg_centi: 24_500,
                jam_detected: false,
                motor_temp_mdeg: 61_000,
                status: MachineStatus::Running,
                timestamp_ms: 1_740_050_000_000,
            };

            // Async encode and decode via duplex channel
            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&belt)
                .await
                .expect("async write ConveyorBelt");
            encoder.finish().await.expect("async finish ConveyorBelt");

            let mut decoder = AsyncDecoder::new(reader);
            let async_decoded: ConveyorBelt = decoder
                .read_item()
                .await
                .expect("async read ConveyorBelt")
                .expect("some value");
            assert_eq!(async_decoded, belt, "async ConveyorBelt roundtrip mismatch");

            // Independent sync encode/decode roundtrip of the same value
            let sync_bytes = encode_to_vec(&belt).expect("sync encode_to_vec ConveyorBelt");
            assert!(
                !sync_bytes.is_empty(),
                "sync encoded bytes must not be empty"
            );
            let (sync_decoded, sync_consumed): (ConveyorBelt, _) =
                decode_from_slice(&sync_bytes).expect("sync decode_from_slice ConveyorBelt");
            assert_eq!(sync_decoded, belt, "sync ConveyorBelt roundtrip mismatch");
            assert_eq!(
                sync_consumed,
                sync_bytes.len(),
                "sync consumed bytes mismatch"
            );

            // Both must agree
            assert_eq!(
                async_decoded, sync_decoded,
                "async and sync must decode to same value"
            );
        });
}

// ---------------------------------------------------------------------------
// Test 19: AssemblyStation with nested Vec<ProductionOrder> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_mes_assembly_station_nested_orders_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let orders = vec![
                make_order(19_001, "PN-SHAFT-TYPE-A", 200),
                make_order(19_002, "PN-BEARING-CAP-B", 150),
                make_order(19_003, "PN-GASKET-RING-C", 400),
            ];
            let station = AssemblyStation {
                station_id: 19_900_001,
                line_id: 5,
                operator_id: 1042,
                status: MachineStatus::Running,
                active_orders: orders.clone(),
                cycle_time_ms: 28_500,
                target_cycle_time_ms: 30_000,
                timestamp_ms: 1_740_060_000_000,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&station)
                .await
                .expect("write AssemblyStation");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: AssemblyStation = decoder
                .read_item()
                .await
                .expect("read AssemblyStation")
                .expect("some value");
            assert_eq!(
                decoded, station,
                "AssemblyStation nested orders roundtrip mismatch"
            );
            assert_eq!(
                decoded.active_orders.len(),
                3,
                "nested orders count mismatch"
            );
        });
}

// ---------------------------------------------------------------------------
// Test 20: Sequential item-by-item decode of 8 MachineSensors
// ---------------------------------------------------------------------------
#[test]
fn test_mes_sequential_item_by_item_decode_eight_sensors() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let sensors: Vec<MachineSensor> = (0u64..8)
                .map(|i| {
                    make_sensor(
                        20_000 + i,
                        70 + i as u32,
                        MachineStatus::Running,
                        2_500 + i as u32 * 250,
                        35_000 + i as i64 * 1_000,
                    )
                })
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for sensor in &sensors {
                encoder.write_item(sensor).await.expect("write sensor seq");
            }
            encoder.finish().await.expect("finish seq");

            let mut decoder = AsyncDecoder::new(reader);
            for (idx, expected) in sensors.iter().enumerate() {
                let item: MachineSensor = decoder
                    .read_item()
                    .await
                    .expect("read_item sequential")
                    .unwrap_or_else(|| panic!("expected Some at sensor index {idx}"));
                assert_eq!(item, *expected, "sensor mismatch at index {idx}");
            }

            let eof: Option<MachineSensor> = decoder.read_item().await.expect("read eof");
            assert!(eof.is_none(), "stream must be exhausted after all sensors");
        });
}

// ---------------------------------------------------------------------------
// Test 21: Mixed-fault batch: filter by MachineStatus after decode
// ---------------------------------------------------------------------------
#[test]
fn test_mes_mixed_status_batch_filter_after_decode() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let sensors = vec![
                make_sensor(21_001, 80, MachineStatus::Running, 4_000, 42_000),
                make_sensor(21_002, 80, MachineStatus::Running, 3_800, 43_500),
                make_sensor(21_003, 80, MachineStatus::Faulted, 0, 88_000),
                make_sensor(21_004, 80, MachineStatus::Idle, 0, 21_000),
                make_sensor(21_005, 80, MachineStatus::Faulted, 0, 91_000),
                make_sensor(21_006, 80, MachineStatus::Maintenance, 0, 30_000),
                make_sensor(21_007, 80, MachineStatus::Running, 5_000, 45_000),
                make_sensor(21_008, 80, MachineStatus::Offline, 0, 18_000),
            ];

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for sensor in &sensors {
                encoder
                    .write_item(sensor)
                    .await
                    .expect("write mixed sensor");
            }
            encoder.finish().await.expect("finish mixed batch");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<MachineSensor> =
                decoder.read_all().await.expect("read_all mixed sensors");
            assert_eq!(decoded.len(), 8, "mixed batch must have 8 sensors");
            assert_eq!(decoded, sensors, "mixed-status batch mismatch");

            let running_count = decoded
                .iter()
                .filter(|s| matches!(s.status, MachineStatus::Running))
                .count();
            assert_eq!(running_count, 3, "expected 3 Running sensors");

            let faulted_count = decoded
                .iter()
                .filter(|s| matches!(s.status, MachineStatus::Faulted))
                .count();
            assert_eq!(faulted_count, 2, "expected 2 Faulted sensors");

            let offline_count = decoded
                .iter()
                .filter(|s| matches!(s.status, MachineStatus::Offline))
                .count();
            assert_eq!(offline_count, 1, "expected 1 Offline sensor");
        });
}

// ---------------------------------------------------------------------------
// Test 22: Large payload: AssemblyStation with 40 nested ProductionOrders
// ---------------------------------------------------------------------------
#[test]
fn test_mes_large_assembly_station_forty_orders_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let orders: Vec<ProductionOrder> = (1u64..=40)
                .map(|i| {
                    make_order(
                        22_000 + i,
                        &format!("PN-PART-{:04}", i),
                        100 + i as u32 * 10,
                    )
                })
                .collect();

            let station = AssemblyStation {
                station_id: 22_999_000,
                line_id: 9,
                operator_id: 5050,
                status: MachineStatus::Running,
                active_orders: orders.clone(),
                cycle_time_ms: 31_200,
                target_cycle_time_ms: 30_000,
                timestamp_ms: 1_740_070_000_000,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&station)
                .await
                .expect("write large AssemblyStation");
            encoder.finish().await.expect("finish large station");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: AssemblyStation = decoder
                .read_item()
                .await
                .expect("read large AssemblyStation")
                .expect("some value");
            assert_eq!(
                decoded, station,
                "large AssemblyStation 40-orders roundtrip mismatch"
            );
            assert_eq!(
                decoded.active_orders.len(),
                40,
                "must have 40 nested production orders"
            );
            assert_eq!(decoded.line_id, 9, "line_id mismatch");
            assert_eq!(
                decoded.status,
                MachineStatus::Running,
                "status must be Running"
            );
        });
}
