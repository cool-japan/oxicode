//! Advanced async streaming tests (25th set) for OxiCode.
//!
//! Theme: Log streaming / audit events.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types: `LogLevel`, `AuditEvent`, `EventBatch`.
//!
//! Coverage matrix:
//!   1:  LogLevel::Info single roundtrip via duplex
//!   2:  LogLevel::Warn single roundtrip via duplex
//!   3:  LogLevel::Error single roundtrip via duplex
//!   4:  LogLevel::Fatal single roundtrip via duplex
//!   5:  LogLevel::Trace single roundtrip via duplex
//!   6:  LogLevel::Debug single roundtrip via duplex
//!   7:  AuditEvent with None user_id roundtrip
//!   8:  AuditEvent with Some user_id roundtrip
//!   9:  Five AuditEvents in order via write_item / read_item
//!  10:  EventBatch with empty events roundtrip
//!  11:  EventBatch with multiple events roundtrip
//!  12:  write_all / read_all for Vec<AuditEvent> (8 items)
//!  13:  Large batch of 150 AuditEvents via write_all, verify read_all
//!  14:  progress().items_processed > 0 after reading events
//!  15:  StreamingConfig with chunk_size(512) forces multiple chunks
//!  16:  flush_per_item produces one chunk per AuditEvent
//!  17:  Empty stream returns None on first read_item
//!  18:  is_finished() true after stream exhausted
//!  19:  bytes_processed grows after reading more events
//!  20:  Sync encode / async decode interop for AuditEvent
//!  21:  Async encode / sync decode interop for EventBatch
//!  22:  tokio::join! concurrent encode/decode for audit log replay

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
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AuditEvent {
    event_id: u64,
    level: LogLevel,
    source: String,
    message: String,
    user_id: Option<u64>,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EventBatch {
    batch_id: u64,
    events: Vec<AuditEvent>,
    processed: bool,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_event(event_id: u64, level: LogLevel, user_id: Option<u64>) -> AuditEvent {
    AuditEvent {
        event_id,
        level,
        source: format!("service-{}", event_id % 10),
        message: format!("audit event {}", event_id),
        user_id,
        timestamp_ms: 1_700_000_000_000 + event_id * 1000,
    }
}

fn make_batch_events(count: usize) -> Vec<AuditEvent> {
    (0..count)
        .map(|i| {
            let level = match i % 6 {
                0 => LogLevel::Trace,
                1 => LogLevel::Debug,
                2 => LogLevel::Info,
                3 => LogLevel::Warn,
                4 => LogLevel::Error,
                _ => LogLevel::Fatal,
            };
            let user_id = if i % 3 == 0 {
                None
            } else {
                Some(i as u64 * 100)
            };
            make_event(i as u64, level, user_id)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Test 1: LogLevel::Info single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_log_level_info_roundtrip() {
    let level = LogLevel::Info;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&level)
        .await
        .expect("write_item LogLevel::Info failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: LogLevel = dec
        .read_item()
        .await
        .expect("read_item LogLevel::Info failed")
        .expect("expected Some(LogLevel::Info)");
    assert_eq!(level, got, "LogLevel::Info roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 2: LogLevel::Warn single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_log_level_warn_roundtrip() {
    let level = LogLevel::Warn;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&level)
        .await
        .expect("write_item LogLevel::Warn failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: LogLevel = dec
        .read_item()
        .await
        .expect("read_item LogLevel::Warn failed")
        .expect("expected Some(LogLevel::Warn)");
    assert_eq!(level, got, "LogLevel::Warn roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 3: LogLevel::Error single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_log_level_error_roundtrip() {
    let level = LogLevel::Error;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&level)
        .await
        .expect("write_item LogLevel::Error failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: LogLevel = dec
        .read_item()
        .await
        .expect("read_item LogLevel::Error failed")
        .expect("expected Some(LogLevel::Error)");
    assert_eq!(level, got, "LogLevel::Error roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 4: LogLevel::Fatal single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_log_level_fatal_roundtrip() {
    let level = LogLevel::Fatal;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&level)
        .await
        .expect("write_item LogLevel::Fatal failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: LogLevel = dec
        .read_item()
        .await
        .expect("read_item LogLevel::Fatal failed")
        .expect("expected Some(LogLevel::Fatal)");
    assert_eq!(level, got, "LogLevel::Fatal roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 5: LogLevel::Trace single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_log_level_trace_roundtrip() {
    let level = LogLevel::Trace;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&level)
        .await
        .expect("write_item LogLevel::Trace failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: LogLevel = dec
        .read_item()
        .await
        .expect("read_item LogLevel::Trace failed")
        .expect("expected Some(LogLevel::Trace)");
    assert_eq!(level, got, "LogLevel::Trace roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 6: LogLevel::Debug single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_log_level_debug_roundtrip() {
    let level = LogLevel::Debug;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&level)
        .await
        .expect("write_item LogLevel::Debug failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: LogLevel = dec
        .read_item()
        .await
        .expect("read_item LogLevel::Debug failed")
        .expect("expected Some(LogLevel::Debug)");
    assert_eq!(level, got, "LogLevel::Debug roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 7: AuditEvent with None user_id roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_event_none_user_id_roundtrip() {
    let event = AuditEvent {
        event_id: 1001,
        level: LogLevel::Info,
        source: "auth-service".to_string(),
        message: "anonymous access attempt".to_string(),
        user_id: None,
        timestamp_ms: 1_700_000_001_000,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event)
        .await
        .expect("write_item AuditEvent(None user_id) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: AuditEvent = dec
        .read_item()
        .await
        .expect("read_item AuditEvent(None user_id) failed")
        .expect("expected Some(AuditEvent)");
    assert_eq!(
        event, got,
        "AuditEvent with None user_id roundtrip mismatch"
    );
    assert_eq!(got.user_id, None, "user_id must be None");
}

// ---------------------------------------------------------------------------
// Test 8: AuditEvent with Some user_id roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_event_some_user_id_roundtrip() {
    let event = AuditEvent {
        event_id: 2002,
        level: LogLevel::Warn,
        source: "billing-service".to_string(),
        message: "suspicious transaction detected".to_string(),
        user_id: Some(99_999),
        timestamp_ms: 1_700_000_002_000,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event)
        .await
        .expect("write_item AuditEvent(Some user_id) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: AuditEvent = dec
        .read_item()
        .await
        .expect("read_item AuditEvent(Some user_id) failed")
        .expect("expected Some(AuditEvent)");
    assert_eq!(
        event, got,
        "AuditEvent with Some user_id roundtrip mismatch"
    );
    assert_eq!(got.user_id, Some(99_999), "user_id must be Some(99_999)");
}

// ---------------------------------------------------------------------------
// Test 9: Five AuditEvents in order via write_item / read_item
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_five_events_in_order() {
    let events = vec![
        make_event(10, LogLevel::Trace, None),
        make_event(11, LogLevel::Debug, Some(1)),
        make_event(12, LogLevel::Info, Some(2)),
        make_event(13, LogLevel::Warn, None),
        make_event(14, LogLevel::Error, Some(3)),
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    for ev in &events {
        enc.write_item(ev)
            .await
            .expect("write_item in 5-event sequence failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    for expected in &events {
        let got: AuditEvent = dec
            .read_item()
            .await
            .expect("read_item in 5-event sequence failed")
            .expect("expected Some(AuditEvent)");
        assert_eq!(
            *expected, got,
            "AuditEvent mismatch at event_id {}",
            expected.event_id
        );
    }

    let eof: Option<AuditEvent> = dec.read_item().await.expect("eof read_item failed");
    assert_eq!(eof, None, "expected None after all five events");
}

// ---------------------------------------------------------------------------
// Test 10: EventBatch with empty events roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_event_batch_empty_events_roundtrip() {
    let batch = EventBatch {
        batch_id: 0,
        events: vec![],
        processed: false,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&batch)
        .await
        .expect("write_item EventBatch(empty) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: EventBatch = dec
        .read_item()
        .await
        .expect("read_item EventBatch(empty) failed")
        .expect("expected Some(EventBatch)");
    assert_eq!(
        batch, got,
        "EventBatch with empty events roundtrip mismatch"
    );
    assert!(got.events.is_empty(), "events must be empty");
}

// ---------------------------------------------------------------------------
// Test 11: EventBatch with multiple events roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_event_batch_multiple_events_roundtrip() {
    let batch = EventBatch {
        batch_id: 42,
        events: vec![
            make_event(100, LogLevel::Info, Some(1000)),
            make_event(101, LogLevel::Error, None),
            make_event(102, LogLevel::Fatal, Some(2000)),
        ],
        processed: true,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&batch)
        .await
        .expect("write_item EventBatch(multiple) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: EventBatch = dec
        .read_item()
        .await
        .expect("read_item EventBatch(multiple) failed")
        .expect("expected Some(EventBatch)");
    assert_eq!(
        batch, got,
        "EventBatch with multiple events roundtrip mismatch"
    );
    assert_eq!(got.events.len(), 3, "decoded batch must have 3 events");
    assert!(got.processed, "processed flag must be true");
}

// ---------------------------------------------------------------------------
// Test 12: write_all / read_all for Vec<AuditEvent> (8 items)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_write_all_read_all_8_events() {
    let events: Vec<AuditEvent> = (0u64..8)
        .map(|i| {
            let level = match i % 3 {
                0 => LogLevel::Info,
                1 => LogLevel::Warn,
                _ => LogLevel::Error,
            };
            make_event(i, level, if i % 2 == 0 { None } else { Some(i * 10) })
        })
        .collect();

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(events.clone().into_iter())
        .await
        .expect("write_all 8 AuditEvents failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<AuditEvent> = dec.read_all().await.expect("read_all 8 AuditEvents failed");
    assert_eq!(events, got, "write_all/read_all 8-item roundtrip mismatch");
    assert_eq!(got.len(), 8, "must decode exactly 8 AuditEvents");
}

// ---------------------------------------------------------------------------
// Test 13: Large batch of 150 AuditEvents via write_all, verify read_all
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_large_batch_150_events_write_all_read_all() {
    let events = make_batch_events(150);
    assert_eq!(events.len(), 150, "must generate exactly 150 events");

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(events.clone().into_iter())
        .await
        .expect("write_all 150 AuditEvents failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<AuditEvent> = dec
        .read_all()
        .await
        .expect("read_all 150 AuditEvents failed");
    assert_eq!(got.len(), 150, "expected 150 decoded AuditEvents");
    assert_eq!(events, got, "large batch 150-event roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 14: progress().items_processed > 0 after reading events
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_progress_items_processed_after_reading() {
    const N: u64 = 9;
    let events = make_batch_events(N as usize);

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.set_estimated_total(N);
    enc.write_all(events.clone().into_iter())
        .await
        .expect("write_all for progress test failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let _: Vec<AuditEvent> = dec
        .read_all()
        .await
        .expect("read_all for progress test failed");

    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after reading events"
    );
    assert_eq!(
        dec.progress().items_processed,
        N,
        "items_processed must equal N={N} after reading all events"
    );
}

// ---------------------------------------------------------------------------
// Test 15: StreamingConfig with chunk_size(512) forces multiple chunks
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_streaming_config_small_chunk_forces_multiple_chunks() {
    let config = StreamingConfig::new().with_chunk_size(512);
    // Each AuditEvent is ~40–80 bytes; 60 events ~2400–4800 bytes → multiple 512-byte chunks
    let events = make_batch_events(60);

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for ev in &events {
        enc.write_item(ev)
            .await
            .expect("write_item with chunk_size 512 failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<AuditEvent> = dec
        .read_all()
        .await
        .expect("read_all with chunk_size 512 failed");

    assert_eq!(got.len(), 60, "must decode 60 AuditEvents");
    assert_eq!(events, got, "small-chunk roundtrip mismatch");
    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after reading with small chunk size"
    );
}

// ---------------------------------------------------------------------------
// Test 16: flush_per_item produces one chunk per AuditEvent
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_flush_per_item_one_chunk_per_event() {
    let config = StreamingConfig::new().with_flush_per_item(true);
    let events: Vec<AuditEvent> = (0u64..6)
        .map(|i| make_event(i, LogLevel::Info, Some(i * 7)))
        .collect();

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for ev in &events {
        enc.write_item(ev)
            .await
            .expect("write_item flush_per_item failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<AuditEvent> = dec
        .read_all()
        .await
        .expect("read_all flush_per_item failed");

    assert_eq!(got, events, "flush_per_item roundtrip mismatch");
    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after flush_per_item read"
    );
    assert_eq!(
        dec.progress().items_processed,
        6,
        "items_processed must equal 6 after reading 6 flush_per_item events"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Empty stream returns None on first read_item
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_empty_stream_returns_none() {
    let (client, server) = tokio::io::duplex(65536);

    let enc = AsyncEncoder::new(client);
    enc.finish().await.expect("finish empty stream failed");

    let mut dec = AsyncDecoder::new(server);
    let item: Option<AuditEvent> = dec
        .read_item()
        .await
        .expect("read_item from empty stream failed");
    assert_eq!(
        item, None,
        "empty stream must return None on first read_item"
    );
}

// ---------------------------------------------------------------------------
// Test 18: is_finished() true after stream exhausted
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_is_finished_after_stream_exhausted() {
    let events = vec![
        make_event(1, LogLevel::Info, None),
        make_event(2, LogLevel::Error, Some(42)),
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    for ev in &events {
        enc.write_item(ev).await.expect("write_item failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);

    assert!(
        !dec.is_finished(),
        "decoder must not be finished before reading"
    );

    let _: Option<AuditEvent> = dec.read_item().await.expect("read item 1 failed");
    let _: Option<AuditEvent> = dec.read_item().await.expect("read item 2 failed");

    let eof: Option<AuditEvent> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None at end of stream");
    assert!(
        dec.is_finished(),
        "decoder must report is_finished() after stream is exhausted"
    );
}

// ---------------------------------------------------------------------------
// Test 19: bytes_processed grows after reading more events
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_bytes_processed_grows_with_more_events() {
    let events = make_batch_events(12);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(events.clone().into_iter())
        .await
        .expect("write_all for bytes_processed test failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);

    let first: AuditEvent = dec
        .read_item()
        .await
        .expect("read first AuditEvent failed")
        .expect("expected Some(AuditEvent) for first event");
    assert_eq!(first, events[0], "first decoded AuditEvent mismatch");

    let bytes_after_one = dec.progress().bytes_processed;
    assert!(
        bytes_after_one > 0,
        "bytes_processed must be > 0 after reading first event"
    );

    let rest: Vec<AuditEvent> = dec
        .read_all()
        .await
        .expect("read_all remaining events failed");
    assert_eq!(rest.len(), 11, "must decode 11 remaining events");

    let bytes_after_all = dec.progress().bytes_processed;
    assert!(
        bytes_after_all > bytes_after_one,
        "bytes_processed must grow: was {bytes_after_one}, now {bytes_after_all}"
    );
    assert!(
        dec.progress().items_processed >= 12,
        "items_processed must be >= 12 after reading all events"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Sync encode / async decode interop for AuditEvent
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_sync_encode_async_decode_interop_audit_event() {
    let event = AuditEvent {
        event_id: 5555,
        level: LogLevel::Fatal,
        source: "kernel".to_string(),
        message: "critical fault detected".to_string(),
        user_id: Some(u64::MAX / 2),
        timestamp_ms: u64::MAX / 4,
    };

    // Sync encode, then async decode via Cursor
    let sync_bytes = encode_to_vec(&event).expect("sync encode AuditEvent failed");

    // Wrap in async encoder/decoder pipeline using duplex for consistency
    let (client, server) = tokio::io::duplex(65536);
    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event)
        .await
        .expect("async write_item for interop test failed");
    enc.finish().await.expect("finish for interop test failed");

    let mut dec = AsyncDecoder::new(server);
    let async_decoded: AuditEvent = dec
        .read_item()
        .await
        .expect("async read_item for interop test failed")
        .expect("expected Some(AuditEvent) in interop test");
    assert_eq!(event, async_decoded, "async encode/decode interop mismatch");

    // Also verify sync roundtrip is consistent
    let (sync_decoded, _): (AuditEvent, _) =
        decode_from_slice(&sync_bytes).expect("sync decode AuditEvent failed");
    assert_eq!(
        sync_decoded, event,
        "sync AuditEvent roundtrip consistency check failed"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Async encode / sync decode interop for EventBatch
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_async_encode_sync_decode_interop_event_batch() {
    let batch = EventBatch {
        batch_id: 999,
        events: vec![
            make_event(200, LogLevel::Debug, Some(8888)),
            make_event(201, LogLevel::Trace, None),
            make_event(202, LogLevel::Info, Some(9999)),
        ],
        processed: false,
    };

    // Sync encode then sync decode for consistency baseline
    let sync_bytes = encode_to_vec(&batch).expect("sync encode EventBatch failed");
    let (sync_decoded, _): (EventBatch, _) =
        decode_from_slice(&sync_bytes).expect("sync decode EventBatch failed");
    assert_eq!(batch, sync_decoded, "sync EventBatch roundtrip mismatch");

    // Async encode then async decode
    let (client, server) = tokio::io::duplex(65536);
    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&batch)
        .await
        .expect("async write_item EventBatch failed");
    enc.finish().await.expect("finish EventBatch failed");

    let mut dec = AsyncDecoder::new(server);
    let async_decoded: EventBatch = dec
        .read_item()
        .await
        .expect("async read_item EventBatch failed")
        .expect("expected Some(EventBatch)");
    assert_eq!(
        batch, async_decoded,
        "async encode/decode EventBatch interop mismatch"
    );
    assert_eq!(
        async_decoded.events.len(),
        3,
        "decoded EventBatch must contain 3 events"
    );
}

// ---------------------------------------------------------------------------
// Test 22: tokio::join! concurrent encode/decode for audit log replay
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_audit25_concurrent_encode_decode_audit_log_replay() {
    let events = make_batch_events(22);
    let events_for_enc = events.clone();

    let (client, server) = tokio::io::duplex(65536);

    let (_, got) = tokio::join!(
        async move {
            let mut enc = AsyncEncoder::new(client);
            enc.write_all(events_for_enc.into_iter())
                .await
                .expect("concurrent write_all audit events failed");
            enc.finish().await.expect("concurrent finish failed");
        },
        async move {
            let mut dec = AsyncDecoder::new(server);
            let decoded: Vec<AuditEvent> = dec
                .read_all()
                .await
                .expect("concurrent read_all audit events failed");
            decoded
        }
    );

    assert_eq!(
        got.len(),
        22,
        "must decode 22 events from concurrent stream"
    );
    assert_eq!(
        events, got,
        "concurrent audit log replay roundtrip mismatch"
    );
}
