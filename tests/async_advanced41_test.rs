//! Advanced async streaming tests (41st set) — smart grid / power distribution domain.
//!
//! All 22 tests are top-level `#[test]` functions (no module wrapper, no async fn).
//! Each test drives a tokio Runtime via `block_on`.
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file:
//!   `GridNodeType`, `FaultSeverity`, `GridNode`, `TransformerRecord`,
//!   `SubstationReport`, `MeterReading`, `DemandResponseEvent`
//!
//! Coverage matrix:
//!    1: Single GridNode duplex roundtrip
//!    2: GridNodeType::Substation variant roundtrip
//!    3: GridNodeType::Feeder variant roundtrip
//!    4: GridNodeType::Transformer variant roundtrip
//!    5: GridNodeType::RenewableSource variant roundtrip
//!    6: FaultSeverity all-variants roundtrip via Vec
//!    7: TransformerRecord struct roundtrip
//!    8: SubstationReport with nested Vec<GridNode> roundtrip
//!    9: Batch write_all / read_all of 10 GridNodes
//!   10: Empty stream returns None immediately
//!   11: Large batch of 50 MeterReadings
//!   12: Progress tracking (bytes_processed grows)
//!   13: Optional field Some(String) roundtrip on DemandResponseEvent
//!   14: Optional field None roundtrip on DemandResponseEvent
//!   15: Vec<MeterReading> roundtrip
//!   16: Consumed bytes (sync encode_to_vec / decode_from_slice)
//!   17: Sync encode → sync decode roundtrip (GridNode)
//!   18: Async encode → async decode roundtrip (TransformerRecord)
//!   19: SCADA command batch with mixed FaultSeverity
//!   20: Load-balancing batch: sequential item-by-item decode
//!   21: Fault detection: consecutive high-severity readings
//!   22: Renewable integration: large payload roundtrip

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
// Domain types — smart grid / power distribution
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GridNodeType {
    Substation,
    Feeder,
    Transformer,
    RenewableSource,
    LoadCenter,
    StorageUnit,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FaultSeverity {
    None,
    Minor,
    Moderate,
    Critical,
    Catastrophic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GridNode {
    node_id: u64,
    region_code: u32,
    node_type: GridNodeType,
    voltage_mv: i64,
    current_ma: i64,
    active_power_mw: i64,
    fault: FaultSeverity,
    online: bool,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TransformerRecord {
    transformer_id: u64,
    primary_voltage_mv: i64,
    secondary_voltage_mv: i64,
    load_percent: u32,
    efficiency_bps: u32,
    fault: FaultSeverity,
    label: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SubstationReport {
    substation_id: u64,
    operator: String,
    nodes: Vec<GridNode>,
    total_load_mw: i64,
    fault: FaultSeverity,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MeterReading {
    meter_id: u64,
    consumer_id: u64,
    kwh_micro: i64,
    reactive_kvar_micro: i64,
    power_factor_bps: u32,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DemandResponseEvent {
    event_id: u64,
    target_reduction_mw: i64,
    duration_s: u32,
    operator_note: Option<String>,
    active: bool,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_node(
    node_id: u64,
    region_code: u32,
    node_type: GridNodeType,
    voltage_mv: i64,
    current_ma: i64,
    fault: FaultSeverity,
    online: bool,
) -> GridNode {
    GridNode {
        node_id,
        region_code,
        node_type,
        voltage_mv,
        current_ma,
        active_power_mw: voltage_mv * current_ma / 1_000_000,
        fault,
        online,
        timestamp_ms: 1_700_000_000_000 + node_id * 100,
    }
}

fn make_meter(meter_id: u64, consumer_id: u64, kwh_micro: i64) -> MeterReading {
    MeterReading {
        meter_id,
        consumer_id,
        kwh_micro,
        reactive_kvar_micro: kwh_micro / 10,
        power_factor_bps: 9_500,
        timestamp_ms: 1_700_000_000_000 + meter_id * 50,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Single GridNode duplex roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_single_node_duplex_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let node = make_node(
            1001,
            42,
            GridNodeType::Substation,
            110_000,
            500_000,
            FaultSeverity::None,
            true,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&node).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: GridNode = decoder
            .read_item()
            .await
            .expect("read_item")
            .expect("some value");
        assert_eq!(decoded, node, "GridNode duplex roundtrip mismatch");
    });
}

// ---------------------------------------------------------------------------
// Test 2: GridNodeType::Substation variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_node_type_substation_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let node = make_node(
            2001,
            10,
            GridNodeType::Substation,
            230_000,
            200_000,
            FaultSeverity::None,
            true,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&node).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: GridNode = decoder
            .read_item()
            .await
            .expect("read_item")
            .expect("some value");
        assert_eq!(decoded.node_type, GridNodeType::Substation);
        assert_eq!(decoded, node);
    });
}

// ---------------------------------------------------------------------------
// Test 3: GridNodeType::Feeder variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_node_type_feeder_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let node = make_node(
            3001,
            11,
            GridNodeType::Feeder,
            11_000,
            1_500_000,
            FaultSeverity::Minor,
            true,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&node).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: GridNode = decoder
            .read_item()
            .await
            .expect("read_item")
            .expect("some value");
        assert_eq!(decoded.node_type, GridNodeType::Feeder);
        assert_eq!(decoded, node);
    });
}

// ---------------------------------------------------------------------------
// Test 4: GridNodeType::Transformer variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_node_type_transformer_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let node = make_node(
            4001,
            12,
            GridNodeType::Transformer,
            33_000,
            800_000,
            FaultSeverity::None,
            true,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&node).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: GridNode = decoder
            .read_item()
            .await
            .expect("read_item")
            .expect("some value");
        assert_eq!(decoded.node_type, GridNodeType::Transformer);
        assert_eq!(decoded, node);
    });
}

// ---------------------------------------------------------------------------
// Test 5: GridNodeType::RenewableSource variant roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_node_type_renewable_source_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build rt");
    rt.block_on(async {
        let node = make_node(
            5001,
            20,
            GridNodeType::RenewableSource,
            690,
            2_200_000,
            FaultSeverity::None,
            true,
        );

        let (writer, reader) = tokio::io::duplex(65536);
        let mut encoder = AsyncEncoder::new(writer);
        encoder.write_item(&node).await.expect("write_item");
        encoder.finish().await.expect("finish");

        let mut decoder = AsyncDecoder::new(reader);
        let decoded: GridNode = decoder
            .read_item()
            .await
            .expect("read_item")
            .expect("some value");
        assert_eq!(decoded.node_type, GridNodeType::RenewableSource);
        assert_eq!(decoded, node);
    });
}

// ---------------------------------------------------------------------------
// Test 6: FaultSeverity all-variants roundtrip via Vec
// ---------------------------------------------------------------------------
#[test]
fn test_grid_fault_severity_all_variants_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let severities = vec![
                FaultSeverity::None,
                FaultSeverity::Minor,
                FaultSeverity::Moderate,
                FaultSeverity::Critical,
                FaultSeverity::Catastrophic,
            ];

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&severities)
                .await
                .expect("write_item severities");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<FaultSeverity> = decoder
                .read_item()
                .await
                .expect("read_item")
                .expect("some value");
            assert_eq!(decoded, severities, "FaultSeverity all-variants mismatch");
        });
}

// ---------------------------------------------------------------------------
// Test 7: TransformerRecord struct roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_transformer_record_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let record = TransformerRecord {
                transformer_id: 7_000_001,
                primary_voltage_mv: 110_000,
                secondary_voltage_mv: 33_000,
                load_percent: 78,
                efficiency_bps: 9_850,
                fault: FaultSeverity::None,
                label: String::from("TX-NORTH-GRID-07"),
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&record).await.expect("write_item");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: TransformerRecord = decoder
                .read_item()
                .await
                .expect("read_item")
                .expect("some value");
            assert_eq!(decoded, record, "TransformerRecord roundtrip mismatch");
        });
}

// ---------------------------------------------------------------------------
// Test 8: SubstationReport with nested Vec<GridNode> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_substation_report_with_nested_nodes_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let nodes = vec![
                make_node(
                    8001,
                    5,
                    GridNodeType::Feeder,
                    11_000,
                    900_000,
                    FaultSeverity::None,
                    true,
                ),
                make_node(
                    8002,
                    5,
                    GridNodeType::Transformer,
                    33_000,
                    400_000,
                    FaultSeverity::Minor,
                    true,
                ),
                make_node(
                    8003,
                    5,
                    GridNodeType::LoadCenter,
                    415,
                    5_000_000,
                    FaultSeverity::None,
                    true,
                ),
            ];
            let report = SubstationReport {
                substation_id: 80_000,
                operator: String::from("GRID-OPS-EAST"),
                nodes: nodes.clone(),
                total_load_mw: 2_450,
                fault: FaultSeverity::None,
                timestamp_ms: 1_700_100_000_000,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&report).await.expect("write_item");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: SubstationReport = decoder
                .read_item()
                .await
                .expect("read_item")
                .expect("some value");
            assert_eq!(
                decoded, report,
                "SubstationReport nested roundtrip mismatch"
            );
            assert_eq!(decoded.nodes.len(), 3, "nested nodes count mismatch");
        });
}

// ---------------------------------------------------------------------------
// Test 9: Batch write_all / read_all of 10 GridNodes
// ---------------------------------------------------------------------------
#[test]
fn test_grid_batch_write_all_read_all_ten_nodes() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let nodes: Vec<GridNode> = (0u64..10)
                .map(|i| {
                    make_node(
                        9000 + i,
                        30 + i as u32,
                        GridNodeType::Feeder,
                        11_000 + i as i64 * 100,
                        500_000 + i as i64 * 10_000,
                        FaultSeverity::None,
                        true,
                    )
                })
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for node in &nodes {
                encoder.write_item(node).await.expect("write_item in batch");
            }
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<GridNode> = decoder.read_all().await.expect("read_all");
            assert_eq!(decoded.len(), 10, "expected 10 nodes from read_all");
            assert_eq!(decoded, nodes, "batch write_all/read_all mismatch");
        });
}

// ---------------------------------------------------------------------------
// Test 10: Empty stream returns None immediately
// ---------------------------------------------------------------------------
#[test]
fn test_grid_empty_stream_returns_none() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let (writer, reader) = tokio::io::duplex(65536);
            let encoder = AsyncEncoder::new(writer);
            encoder.finish().await.expect("finish empty");

            let mut decoder = AsyncDecoder::new(reader);
            let result: Option<GridNode> = decoder.read_item().await.expect("read_item on empty");
            assert!(result.is_none(), "empty stream must return None");
        });
}

// ---------------------------------------------------------------------------
// Test 11: Large batch of 50 MeterReadings
// ---------------------------------------------------------------------------
#[test]
fn test_grid_large_batch_fifty_meter_readings() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let readings: Vec<MeterReading> = (1u64..=50)
                .map(|i| make_meter(i, i * 100, i as i64 * 1_000_000))
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for r in &readings {
                encoder.write_item(r).await.expect("write meter reading");
            }
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<MeterReading> = decoder.read_all().await.expect("read_all meters");
            assert_eq!(decoded.len(), 50, "expected 50 meter readings");
            assert_eq!(decoded, readings, "large batch meter readings mismatch");
        });
}

// ---------------------------------------------------------------------------
// Test 12: Progress tracking (bytes_processed grows)
// ---------------------------------------------------------------------------
#[test]
fn test_grid_progress_tracking_bytes_processed_grows() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let nodes: Vec<GridNode> = (0u64..8)
                .map(|i| {
                    make_node(
                        12_000 + i,
                        50,
                        GridNodeType::StorageUnit,
                        48_000,
                        750_000 + i as i64 * 5_000,
                        FaultSeverity::None,
                        true,
                    )
                })
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for node in &nodes {
                encoder.write_item(node).await.expect("write_item");
            }
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let first: GridNode = decoder
                .read_item()
                .await
                .expect("read first item")
                .expect("some value");
            assert_eq!(first, nodes[0]);

            let bytes_after_first = decoder.progress().bytes_processed;
            assert!(
                bytes_after_first > 0,
                "bytes_processed must be > 0 after reading first node"
            );

            let rest: Vec<GridNode> = decoder.read_all().await.expect("read_all rest");
            assert_eq!(rest.len(), 7, "expected 7 remaining nodes");

            let bytes_after_all = decoder.progress().bytes_processed;
            assert!(
                bytes_after_all > bytes_after_first,
                "bytes_processed must grow: was {bytes_after_first}, now {bytes_after_all}"
            );
            assert_eq!(
                decoder.progress().items_processed,
                8,
                "items_processed must equal 8"
            );
        });
}

// ---------------------------------------------------------------------------
// Test 13: Optional field Some(String) roundtrip on DemandResponseEvent
// ---------------------------------------------------------------------------
#[test]
fn test_grid_demand_response_optional_some_string_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let event = DemandResponseEvent {
                event_id: 13_000_001,
                target_reduction_mw: 250,
                duration_s: 3600,
                operator_note: Some(String::from("Peak demand reduction — evening ramp")),
                active: true,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&event).await.expect("write_item");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: DemandResponseEvent = decoder
                .read_item()
                .await
                .expect("read_item")
                .expect("some value");
            assert_eq!(decoded, event);
            assert!(
                decoded.operator_note.is_some(),
                "operator_note must be Some"
            );
        });
}

// ---------------------------------------------------------------------------
// Test 14: Optional field None roundtrip on DemandResponseEvent
// ---------------------------------------------------------------------------
#[test]
fn test_grid_demand_response_optional_none_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let event = DemandResponseEvent {
                event_id: 14_000_002,
                target_reduction_mw: 100,
                duration_s: 1800,
                operator_note: None,
                active: false,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&event).await.expect("write_item");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: DemandResponseEvent = decoder
                .read_item()
                .await
                .expect("read_item")
                .expect("some value");
            assert_eq!(decoded, event);
            assert!(
                decoded.operator_note.is_none(),
                "operator_note must be None"
            );
        });
}

// ---------------------------------------------------------------------------
// Test 15: Vec<MeterReading> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_vec_meter_reading_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let readings = vec![
                make_meter(15_001, 200_001, 45_000_000),
                make_meter(15_002, 200_002, 67_500_000),
                make_meter(15_003, 200_003, 12_300_000),
                make_meter(15_004, 200_004, 99_000_000),
            ];

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&readings).await.expect("write_item");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<MeterReading> = decoder
                .read_item()
                .await
                .expect("read_item")
                .expect("some value");
            assert_eq!(decoded, readings, "Vec<MeterReading> roundtrip mismatch");
        });
}

// ---------------------------------------------------------------------------
// Test 16: Consumed bytes (sync encode_to_vec / decode_from_slice)
// ---------------------------------------------------------------------------
#[test]
fn test_grid_consumed_bytes_sync_encode_decode() {
    let node = make_node(
        16_001,
        60,
        GridNodeType::LoadCenter,
        415,
        10_000_000,
        FaultSeverity::Minor,
        true,
    );

    let encoded = encode_to_vec(&node).expect("encode_to_vec");
    assert!(!encoded.is_empty(), "encoded bytes must be non-empty");

    let (decoded, consumed): (GridNode, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice");
    assert_eq!(decoded, node, "sync roundtrip mismatch");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Sync encode → sync decode roundtrip (GridNode)
// ---------------------------------------------------------------------------
#[test]
fn test_grid_sync_encode_async_decode_interop() {
    let node = make_node(
        17_001,
        70,
        GridNodeType::RenewableSource,
        690,
        3_000_000,
        FaultSeverity::None,
        true,
    );

    // Sync encode
    let sync_bytes = encode_to_vec(&node).expect("sync encode_to_vec");
    assert!(!sync_bytes.is_empty(), "encoded bytes must be non-empty");

    // Sync decode
    let (decoded, consumed): (GridNode, usize) =
        decode_from_slice(&sync_bytes).expect("sync decode_from_slice");
    assert_eq!(decoded, node, "sync roundtrip mismatch for GridNode");
    assert_eq!(
        consumed,
        sync_bytes.len(),
        "consumed bytes must equal encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Async encode → async decode roundtrip (TransformerRecord)
// ---------------------------------------------------------------------------
#[test]
fn test_grid_async_encode_sync_decode_interop() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let record = TransformerRecord {
                transformer_id: 18_000_001,
                primary_voltage_mv: 230_000,
                secondary_voltage_mv: 110_000,
                load_percent: 62,
                efficiency_bps: 9_920,
                fault: FaultSeverity::None,
                label: String::from("TX-SOUTH-SUBSTATION-18"),
            };

            // Async encode into a Vec<u8>
            let mut async_bytes = Vec::<u8>::new();
            {
                let cursor = std::io::Cursor::new(&mut async_bytes);
                let mut encoder = AsyncEncoder::new(cursor);
                encoder.write_item(&record).await.expect("async write_item");
                encoder.finish().await.expect("async finish");
            }

            assert!(
                !async_bytes.is_empty(),
                "async encoded bytes must not be empty"
            );

            // Async decode from the same bytes
            let cursor = std::io::Cursor::new(async_bytes);
            let buf_reader = tokio::io::BufReader::new(cursor);
            let mut decoder = AsyncDecoder::new(buf_reader);
            let decoded: TransformerRecord = decoder
                .read_item()
                .await
                .expect("async read_item")
                .expect("expected a TransformerRecord from async decode");
            assert_eq!(
                decoded, record,
                "async roundtrip mismatch for TransformerRecord"
            );
        });
}

// ---------------------------------------------------------------------------
// Test 19: SCADA command batch with mixed FaultSeverity
// ---------------------------------------------------------------------------
#[test]
fn test_grid_scada_command_batch_mixed_fault_severity() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let nodes = vec![
                make_node(
                    19_001,
                    80,
                    GridNodeType::Substation,
                    110_000,
                    300_000,
                    FaultSeverity::None,
                    true,
                ),
                make_node(
                    19_002,
                    80,
                    GridNodeType::Feeder,
                    11_000,
                    900_000,
                    FaultSeverity::Minor,
                    true,
                ),
                make_node(
                    19_003,
                    80,
                    GridNodeType::Transformer,
                    33_000,
                    500_000,
                    FaultSeverity::Moderate,
                    true,
                ),
                make_node(
                    19_004,
                    80,
                    GridNodeType::LoadCenter,
                    415,
                    8_000_000,
                    FaultSeverity::Critical,
                    false,
                ),
                make_node(
                    19_005,
                    80,
                    GridNodeType::StorageUnit,
                    48_000,
                    200_000,
                    FaultSeverity::Catastrophic,
                    false,
                ),
            ];

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for node in &nodes {
                encoder.write_item(node).await.expect("write SCADA node");
            }
            encoder.finish().await.expect("finish SCADA batch");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<GridNode> = decoder.read_all().await.expect("read_all SCADA");
            assert_eq!(decoded.len(), 5, "SCADA batch must have 5 nodes");
            assert_eq!(decoded, nodes, "SCADA batch mismatch");
            assert_eq!(decoded[3].fault, FaultSeverity::Critical);
            assert_eq!(decoded[4].fault, FaultSeverity::Catastrophic);
        });
}

// ---------------------------------------------------------------------------
// Test 20: Load-balancing batch: sequential item-by-item decode
// ---------------------------------------------------------------------------
#[test]
fn test_grid_load_balancing_sequential_item_by_item_decode() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let nodes: Vec<GridNode> = (0u64..6)
                .map(|i| {
                    make_node(
                        20_000 + i,
                        90 + i as u32,
                        GridNodeType::LoadCenter,
                        415,
                        1_000_000 * (i as i64 + 1),
                        FaultSeverity::None,
                        true,
                    )
                })
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for node in &nodes {
                encoder
                    .write_item(node)
                    .await
                    .expect("write_item load node");
            }
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            for (idx, expected) in nodes.iter().enumerate() {
                let item: GridNode = decoder
                    .read_item()
                    .await
                    .expect("read_item load node")
                    .unwrap_or_else(|| panic!("expected Some at index {idx}"));
                assert_eq!(item, *expected, "load-balance node mismatch at index {idx}");
            }

            let eof: Option<GridNode> = decoder.read_item().await.expect("read_item eof");
            assert!(eof.is_none(), "stream must be exhausted after all nodes");
        });
}

// ---------------------------------------------------------------------------
// Test 21: Fault detection: consecutive high-severity readings
// ---------------------------------------------------------------------------
#[test]
fn test_grid_fault_detection_consecutive_high_severity_readings() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let fault_nodes = vec![
                make_node(
                    21_001,
                    100,
                    GridNodeType::Feeder,
                    10_500,
                    1_100_000,
                    FaultSeverity::Moderate,
                    true,
                ),
                make_node(
                    21_002,
                    100,
                    GridNodeType::Feeder,
                    10_200,
                    1_300_000,
                    FaultSeverity::Critical,
                    true,
                ),
                make_node(
                    21_003,
                    100,
                    GridNodeType::Transformer,
                    32_500,
                    600_000,
                    FaultSeverity::Critical,
                    false,
                ),
                make_node(
                    21_004,
                    100,
                    GridNodeType::Substation,
                    108_000,
                    350_000,
                    FaultSeverity::Catastrophic,
                    false,
                ),
            ];

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for node in &fault_nodes {
                encoder.write_item(node).await.expect("write fault node");
            }
            encoder.finish().await.expect("finish fault batch");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Vec<GridNode> = decoder.read_all().await.expect("read_all fault nodes");
            assert_eq!(decoded.len(), 4, "fault batch must have 4 nodes");

            let critical_count = decoded
                .iter()
                .filter(|n| {
                    matches!(
                        n.fault,
                        FaultSeverity::Critical | FaultSeverity::Catastrophic
                    )
                })
                .count();
            assert_eq!(critical_count, 3, "expected 3 critical/catastrophic faults");

            let offline_count = decoded.iter().filter(|n| !n.online).count();
            assert_eq!(offline_count, 2, "expected 2 offline nodes");
        });
}

// ---------------------------------------------------------------------------
// Test 22: Renewable integration: large payload roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_grid_renewable_integration_large_payload_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let renewable_nodes: Vec<GridNode> = (0u64..30)
                .map(|i| {
                    make_node(
                        22_000 + i,
                        200 + i as u32 / 5,
                        GridNodeType::RenewableSource,
                        690 + i as i64 * 10,
                        2_000_000 + i as i64 * 50_000,
                        if i % 10 == 0 {
                            FaultSeverity::Minor
                        } else {
                            FaultSeverity::None
                        },
                        true,
                    )
                })
                .collect();

            let report = SubstationReport {
                substation_id: 22_900_000,
                operator: String::from("RENEWABLE-OPS-NORTH"),
                nodes: renewable_nodes.clone(),
                total_load_mw: 15_000,
                fault: FaultSeverity::None,
                timestamp_ms: 1_700_200_000_000,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&report)
                .await
                .expect("write large SubstationReport");
            encoder.finish().await.expect("finish large payload");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: SubstationReport = decoder
                .read_item()
                .await
                .expect("read large SubstationReport")
                .expect("some value");
            assert_eq!(
                decoded, report,
                "renewable large payload roundtrip mismatch"
            );
            assert_eq!(decoded.nodes.len(), 30, "must have 30 renewable nodes");
            assert_eq!(
                decoded.operator, "RENEWABLE-OPS-NORTH",
                "operator label mismatch"
            );
        });
}
