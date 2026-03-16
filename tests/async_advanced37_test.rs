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
// Domain types: 5G / telecommunications network
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NetworkGeneration {
    G3,
    G4Lte,
    G5Nr,
    G5NrSa,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HandoverReason {
    SignalQuality,
    LoadBalance,
    Coverage,
    UserRequest,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CellSignal {
    cell_id: u32,
    generation: NetworkGeneration,
    rsrp_dbm: i16,
    sinr_db: i16,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HandoverEvent {
    ue_id: u64,
    source_cell: u32,
    target_cell: u32,
    reason: HandoverReason,
    duration_ms: u32,
    success: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BearerSession {
    session_id: u64,
    ue_id: u64,
    qos_class: u8,
    ul_kbps: u32,
    dl_kbps: u32,
    active: bool,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_cell_signal(cell_id: u32, generation: NetworkGeneration) -> CellSignal {
    CellSignal {
        cell_id,
        generation,
        rsrp_dbm: -80 - (cell_id as i16 % 40),
        sinr_db: 10 + (cell_id as i16 % 20),
        timestamp_ms: 1_700_000_000_000 + cell_id as u64 * 1_000,
    }
}

fn make_handover(
    ue_id: u64,
    source_cell: u32,
    target_cell: u32,
    reason: HandoverReason,
    success: bool,
) -> HandoverEvent {
    HandoverEvent {
        ue_id,
        source_cell,
        target_cell,
        reason,
        duration_ms: 30 + (ue_id % 50) as u32,
        success,
    }
}

#[allow(dead_code)]
fn make_bearer(session_id: u64, ue_id: u64, active: bool) -> BearerSession {
    BearerSession {
        session_id,
        ue_id,
        qos_class: (session_id % 9 + 1) as u8,
        ul_kbps: 1_000 * (session_id % 100 + 1) as u32,
        dl_kbps: 5_000 * (session_id % 100 + 1) as u32,
        active,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Single CellSignal duplex roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_single_cell_signal_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let signal = make_cell_signal(101, NetworkGeneration::G5Nr);

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&signal).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Option<CellSignal> = decoder.read_item().await.expect("read_item");
            assert_eq!(decoded, Some(signal));
        });
}

// ---------------------------------------------------------------------------
// Test 2: NetworkGeneration::G3 variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_network_generation_g3_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let signal = make_cell_signal(1, NetworkGeneration::G3);

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&signal).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Option<CellSignal> = decoder.read_item().await.expect("read_item");
            let decoded = decoded.expect("some");
            assert_eq!(decoded.generation, NetworkGeneration::G3);
            assert_eq!(decoded.cell_id, 1);
        });
}

// ---------------------------------------------------------------------------
// Test 3: NetworkGeneration::G4Lte variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_network_generation_g4lte_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let signal = make_cell_signal(2, NetworkGeneration::G4Lte);

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&signal).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Option<CellSignal> = decoder.read_item().await.expect("read_item");
            assert_eq!(decoded.expect("some").generation, NetworkGeneration::G4Lte);
        });
}

// ---------------------------------------------------------------------------
// Test 4: NetworkGeneration::G5NrSa variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_network_generation_g5nrsa_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let signal = make_cell_signal(3, NetworkGeneration::G5NrSa);

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&signal).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Option<CellSignal> = decoder.read_item().await.expect("read_item");
            assert_eq!(decoded.expect("some").generation, NetworkGeneration::G5NrSa);
        });
}

// ---------------------------------------------------------------------------
// Test 5: HandoverReason::SignalQuality variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_handover_reason_signal_quality_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let event = make_handover(1001, 10, 20, HandoverReason::SignalQuality, true);

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&event).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Option<HandoverEvent> = decoder.read_item().await.expect("read_item");
            let decoded = decoded.expect("some");
            assert_eq!(decoded.reason, HandoverReason::SignalQuality);
            assert!(decoded.success);
        });
}

// ---------------------------------------------------------------------------
// Test 6: HandoverReason::LoadBalance variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_handover_reason_load_balance_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let event = make_handover(2002, 30, 40, HandoverReason::LoadBalance, true);

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&event).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Option<HandoverEvent> = decoder.read_item().await.expect("read_item");
            let decoded = decoded.expect("some");
            assert_eq!(decoded.reason, HandoverReason::LoadBalance);
            assert_eq!(decoded.source_cell, 30);
            assert_eq!(decoded.target_cell, 40);
        });
}

// ---------------------------------------------------------------------------
// Test 7: HandoverReason::Coverage variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_handover_reason_coverage_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let event = make_handover(3003, 50, 60, HandoverReason::Coverage, false);

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&event).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Option<HandoverEvent> = decoder.read_item().await.expect("read_item");
            let decoded = decoded.expect("some");
            assert_eq!(decoded.reason, HandoverReason::Coverage);
            assert!(!decoded.success);
        });
}

// ---------------------------------------------------------------------------
// Test 8: HandoverReason::UserRequest variant async roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_handover_reason_user_request_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let event = make_handover(4004, 70, 80, HandoverReason::UserRequest, true);

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&event).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Option<HandoverEvent> = decoder.read_item().await.expect("read_item");
            let decoded = decoded.expect("some");
            assert_eq!(decoded.reason, HandoverReason::UserRequest);
            assert_eq!(decoded.ue_id, 4004);
        });
}

// ---------------------------------------------------------------------------
// Test 9: HandoverEvent success roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_handover_event_success_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let event = HandoverEvent {
                ue_id: 0xFFFF_0001,
                source_cell: 100,
                target_cell: 200,
                reason: HandoverReason::SignalQuality,
                duration_ms: 45,
                success: true,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&event).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Option<HandoverEvent> = decoder.read_item().await.expect("read_item");
            let decoded = decoded.expect("some");
            assert_eq!(decoded, event);
            assert!(decoded.success);
            assert_eq!(decoded.duration_ms, 45);
        });
}

// ---------------------------------------------------------------------------
// Test 10: HandoverEvent failure roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_handover_event_failure_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let event = HandoverEvent {
                ue_id: 0xFFFF_0002,
                source_cell: 300,
                target_cell: 400,
                reason: HandoverReason::Coverage,
                duration_ms: 120,
                success: false,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&event).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Option<HandoverEvent> = decoder.read_item().await.expect("read_item");
            let decoded = decoded.expect("some");
            assert_eq!(decoded, event);
            assert!(!decoded.success);
            assert_eq!(decoded.duration_ms, 120);
        });
}

// ---------------------------------------------------------------------------
// Test 11: BearerSession roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_bearer_session_roundtrip() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let session = BearerSession {
                session_id: 0xABCD_0001,
                ue_id: 0x1234_5678,
                qos_class: 5,
                ul_kbps: 50_000,
                dl_kbps: 200_000,
                active: true,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&session).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Option<BearerSession> = decoder.read_item().await.expect("read_item");
            let decoded = decoded.expect("some");
            assert_eq!(decoded, session);
            assert!(decoded.active);
            assert_eq!(decoded.qos_class, 5);
        });
}

// ---------------------------------------------------------------------------
// Test 12: Batch of 10 CellSignals via write_all / read_item loop
// ---------------------------------------------------------------------------
#[test]
fn test_5g_batch_10_cell_signals() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let generations = [
                NetworkGeneration::G3,
                NetworkGeneration::G4Lte,
                NetworkGeneration::G5Nr,
                NetworkGeneration::G5NrSa,
            ];
            let signals: Vec<CellSignal> = (0u32..10)
                .map(|i| {
                    make_cell_signal(i + 1, generations[(i as usize) % generations.len()].clone())
                })
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_all(signals.clone()).await.expect("write_all");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let mut decoded: Vec<CellSignal> = Vec::new();
            while let Some(item) = decoder.read_item().await.expect("read_item") {
                decoded.push(item);
            }
            assert_eq!(decoded.len(), 10);
            assert_eq!(decoded, signals);
        });
}

// ---------------------------------------------------------------------------
// Test 13: Empty stream returns None
// ---------------------------------------------------------------------------
#[test]
fn test_5g_empty_stream_returns_none() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let (writer, reader) = tokio::io::duplex(65536);
            let encoder = AsyncEncoder::new(writer);
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let result: Option<CellSignal> = decoder.read_item().await.expect("read_item");
            assert_eq!(result, None);
            assert!(decoder.is_finished());
        });
}

// ---------------------------------------------------------------------------
// Test 14: Large batch of 50 CellSignals roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_large_batch_50_signals() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let generations = [
                NetworkGeneration::G3,
                NetworkGeneration::G4Lte,
                NetworkGeneration::G5Nr,
                NetworkGeneration::G5NrSa,
            ];
            let signals: Vec<CellSignal> = (0u32..50)
                .map(|i| {
                    make_cell_signal(i + 1, generations[(i as usize) % generations.len()].clone())
                })
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for s in &signals {
                encoder.write_item(s).await.expect("write");
            }
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let mut decoded: Vec<CellSignal> = Vec::new();
            while let Some(item) = decoder.read_item().await.expect("read_item") {
                decoded.push(item);
            }
            assert_eq!(decoded.len(), 50);
            assert_eq!(decoded, signals);
        });
}

// ---------------------------------------------------------------------------
// Test 15: Progress tracking after decoding multiple signals
// ---------------------------------------------------------------------------
#[test]
fn test_5g_progress_tracking() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let signals: Vec<CellSignal> = (0u32..15)
                .map(|i| make_cell_signal(i + 1, NetworkGeneration::G5Nr))
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for s in &signals {
                encoder.write_item(s).await.expect("write");
            }
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            while let Some(_) = decoder.read_item::<CellSignal>().await.expect("read_item") {}
            let progress = decoder.progress();
            assert_eq!(progress.items_processed, 15);
            assert!(decoder.is_finished());
        });
}

// ---------------------------------------------------------------------------
// Test 16: All NetworkGeneration variants in one batch
// ---------------------------------------------------------------------------
#[test]
fn test_5g_all_generations_one_batch() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let signals = vec![
                make_cell_signal(10, NetworkGeneration::G3),
                make_cell_signal(11, NetworkGeneration::G4Lte),
                make_cell_signal(12, NetworkGeneration::G5Nr),
                make_cell_signal(13, NetworkGeneration::G5NrSa),
            ];

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for s in &signals {
                encoder.write_item(s).await.expect("write");
            }
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let mut decoded: Vec<CellSignal> = Vec::new();
            while let Some(item) = decoder.read_item().await.expect("read_item") {
                decoded.push(item);
            }
            assert_eq!(decoded.len(), 4);
            assert_eq!(decoded[0].generation, NetworkGeneration::G3);
            assert_eq!(decoded[1].generation, NetworkGeneration::G4Lte);
            assert_eq!(decoded[2].generation, NetworkGeneration::G5Nr);
            assert_eq!(decoded[3].generation, NetworkGeneration::G5NrSa);
        });
}

// ---------------------------------------------------------------------------
// Test 17: Concurrent write/read for CellSignals via tokio::spawn
// ---------------------------------------------------------------------------
#[test]
fn test_5g_concurrent_write_read() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let signals: Vec<CellSignal> = (0u32..30)
                .map(|i| make_cell_signal(i + 1, NetworkGeneration::G5NrSa))
                .collect();
            let expected = signals.clone();

            let (writer, reader) = tokio::io::duplex(65536);

            let encode_handle = tokio::spawn(async move {
                let mut encoder = AsyncEncoder::new(writer);
                for s in &signals {
                    encoder.write_item(s).await.expect("write");
                }
                encoder.finish().await.expect("finish");
            });

            let mut decoder = AsyncDecoder::new(reader);
            let mut decoded: Vec<CellSignal> = Vec::new();
            while let Some(item) = decoder.read_item().await.expect("read_item") {
                decoded.push(item);
            }

            encode_handle.await.expect("encoder task");

            assert_eq!(decoded.len(), 30);
            assert_eq!(decoded, expected);
            assert!(decoder.is_finished());
        });
}

// ---------------------------------------------------------------------------
// Test 18: Max RSRP value roundtrip (i16::MAX for rsrp_dbm, u64::MAX timestamp)
// ---------------------------------------------------------------------------
#[test]
fn test_5g_max_rsrp_value() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let signal = CellSignal {
                cell_id: u32::MAX,
                generation: NetworkGeneration::G5Nr,
                rsrp_dbm: i16::MAX,
                sinr_db: i16::MAX,
                timestamp_ms: u64::MAX,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&signal).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let decoded: Option<CellSignal> = decoder.read_item().await.expect("read_item");
            let decoded = decoded.expect("some");
            assert_eq!(decoded.rsrp_dbm, i16::MAX);
            assert_eq!(decoded.sinr_db, i16::MAX);
            assert_eq!(decoded.cell_id, u32::MAX);
            assert_eq!(decoded.timestamp_ms, u64::MAX);
        });
}

// ---------------------------------------------------------------------------
// Test 19: Negative SINR value roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_negative_sinr_value() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let poor_signals: Vec<CellSignal> = (1u32..=5)
                .map(|i| CellSignal {
                    cell_id: i,
                    generation: NetworkGeneration::G4Lte,
                    rsrp_dbm: -140 + (i as i16),
                    sinr_db: -(i as i16 * 3),
                    timestamp_ms: 1_700_000_000_000 + i as u64 * 500,
                })
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for s in &poor_signals {
                encoder.write_item(s).await.expect("write");
            }
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let mut decoded: Vec<CellSignal> = Vec::new();
            while let Some(item) = decoder.read_item().await.expect("read_item") {
                decoded.push(item);
            }
            assert_eq!(decoded.len(), 5);
            for s in &decoded {
                assert!(s.sinr_db < 0, "expected negative SINR");
                assert!(s.rsrp_dbm < 0, "expected negative RSRP");
            }
            assert_eq!(decoded, poor_signals);
        });
}

// ---------------------------------------------------------------------------
// Test 20: High-speed handover sequence (5 rapid handovers for one UE)
// ---------------------------------------------------------------------------
#[test]
fn test_5g_high_speed_handover_sequence() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let ue_id = 0xDEAD_BEEF_0001u64;
            let cell_path: Vec<u32> = vec![100, 101, 102, 103, 104, 105];
            let reasons = [
                HandoverReason::SignalQuality,
                HandoverReason::LoadBalance,
                HandoverReason::Coverage,
                HandoverReason::UserRequest,
                HandoverReason::SignalQuality,
            ];
            let events: Vec<HandoverEvent> = cell_path
                .windows(2)
                .enumerate()
                .map(|(i, w)| HandoverEvent {
                    ue_id,
                    source_cell: w[0],
                    target_cell: w[1],
                    reason: reasons[i].clone(),
                    duration_ms: 20 + i as u32 * 5,
                    success: i % 5 != 3,
                })
                .collect();

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            for e in &events {
                encoder.write_item(e).await.expect("write");
            }
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let mut decoded: Vec<HandoverEvent> = Vec::new();
            while let Some(item) = decoder.read_item().await.expect("read_item") {
                decoded.push(item);
            }
            assert_eq!(decoded.len(), 5);
            assert_eq!(decoded, events);
            assert_eq!(decoded[0].source_cell, 100);
            assert_eq!(decoded[4].target_cell, 105);
            for e in &decoded {
                assert_eq!(e.ue_id, ue_id);
            }
        });
}

// ---------------------------------------------------------------------------
// Test 21: Session activation/deactivation roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_5g_session_activation_deactivation() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let active_session = BearerSession {
                session_id: 0x0001_0001,
                ue_id: 0xAAAA_0001,
                qos_class: 7,
                ul_kbps: 25_000,
                dl_kbps: 100_000,
                active: true,
            };
            let inactive_session = BearerSession {
                session_id: 0x0001_0001,
                ue_id: 0xAAAA_0001,
                qos_class: 7,
                ul_kbps: 0,
                dl_kbps: 0,
                active: false,
            };

            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_item(&active_session)
                .await
                .expect("write active");
            encoder
                .write_item(&inactive_session)
                .await
                .expect("write inactive");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);

            let dec_active: Option<BearerSession> = decoder.read_item().await.expect("read active");
            let dec_active = dec_active.expect("some active");
            assert!(dec_active.active);
            assert_eq!(dec_active.ul_kbps, 25_000);

            let dec_inactive: Option<BearerSession> =
                decoder.read_item().await.expect("read inactive");
            let dec_inactive = dec_inactive.expect("some inactive");
            assert!(!dec_inactive.active);
            assert_eq!(dec_inactive.ul_kbps, 0);
            assert_eq!(dec_inactive.dl_kbps, 0);
        });
}

// ---------------------------------------------------------------------------
// Test 22: Sync encode_to_vec / decode_from_slice consistency vs async
// ---------------------------------------------------------------------------
#[test]
fn test_5g_sync_vs_async_consistency() {
    // Suppress unused variable warning for StreamingConfig — it is imported per spec
    let _cfg = StreamingConfig::default();

    let signal = CellSignal {
        cell_id: 42,
        generation: NetworkGeneration::G5NrSa,
        rsrp_dbm: -75,
        sinr_db: 18,
        timestamp_ms: 1_720_000_000_000,
    };

    // Sync path
    let sync_encoded = encode_to_vec(&signal).expect("encode_to_vec");
    let (sync_decoded, sync_consumed): (CellSignal, _) =
        decode_from_slice(&sync_encoded).expect("decode_from_slice");
    assert_eq!(sync_decoded, signal);
    assert_eq!(sync_consumed, sync_encoded.len());

    // Async path
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt")
        .block_on(async {
            let (writer, reader) = tokio::io::duplex(65536);
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&signal).await.expect("write");
            encoder.finish().await.expect("finish");

            let mut decoder = AsyncDecoder::new(reader);
            let async_decoded: Option<CellSignal> = decoder.read_item().await.expect("read_item");
            let async_decoded = async_decoded.expect("some");
            assert_eq!(async_decoded, signal);
            assert_eq!(async_decoded, sync_decoded);
        });
}
