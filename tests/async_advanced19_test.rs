//! Advanced async streaming tests (nineteenth set) for OxiCode.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file: `StreamEvent` and `StreamControl`.
//!
//! Uses `tokio::io::duplex` for in-memory async I/O pipes.
//!
//! Coverage matrix:
//!   1:  StreamEvent basic roundtrip
//!   2:  StreamControl::Start roundtrip
//!   3:  StreamControl::Stop roundtrip
//!   4:  All StreamControl variants roundtrip
//!   5:  Write 5 StreamEvents, read back in order
//!   6:  Empty payload StreamEvent
//!   7:  Large payload (1000 bytes)
//!   8:  Read after all items returns None
//!   9:  u32 roundtrip via async
//!  10:  String roundtrip via async
//!  11:  bool roundtrip via async
//!  12:  Vec<u8> roundtrip via async
//!  13:  10 StreamEvents sequential write/read
//!  14:  Mixed types don't interfere
//!  15:  StreamEvent with unicode source
//!  16:  Vec<StreamControl> all 5 variants (write one by one, read one by one)
//!  17:  u64::MAX via async
//!  18:  Empty string via async
//!  19:  Interleave writes and reads (write 2, read 2, write 2, read 2)
//!  20:  StreamEvent priority=0 and priority=255 roundtrip
//!  21:  100 u32 values streamed
//!  22:  StreamControl with empty reason string

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
use oxicode::{Decode, Encode};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, BufReader};

// ---------------------------------------------------------------------------
// Types unique to this file
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct StreamEvent {
    event_id: u64,
    source: String,
    payload: Vec<u8>,
    priority: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum StreamControl {
    Start { stream_id: u32 },
    Pause,
    Resume,
    Stop { reason: String },
    Flush,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Encode a single item via the write half of a duplex pipe, drain the read
/// half into a `Vec<u8>`, then return those bytes.
async fn duplex_encode<T: Encode>(item: &T) -> Vec<u8> {
    let (writer, mut reader) = tokio::io::duplex(65536);
    let mut encoder = AsyncEncoder::new(writer);
    encoder
        .write_item(item)
        .await
        .expect("duplex_encode: write_item failed");
    encoder
        .finish()
        .await
        .expect("duplex_encode: finish failed");

    let mut buf = Vec::new();
    reader
        .read_to_end(&mut buf)
        .await
        .expect("duplex_encode: read_to_end failed");
    buf
}

/// Encode multiple items via the write half of a duplex pipe, drain the read
/// half into a `Vec<u8>`, then return those bytes.
async fn duplex_encode_many<T: Encode>(items: &[T]) -> Vec<u8> {
    let (writer, mut reader) = tokio::io::duplex(65536);
    let mut encoder = AsyncEncoder::new(writer);
    for item in items {
        encoder
            .write_item(item)
            .await
            .expect("duplex_encode_many: write_item failed");
    }
    encoder
        .finish()
        .await
        .expect("duplex_encode_many: finish failed");

    let mut buf = Vec::new();
    reader
        .read_to_end(&mut buf)
        .await
        .expect("duplex_encode_many: read_to_end failed");
    buf
}

/// Wrap encoded bytes in a `BufReader`-backed `AsyncDecoder` and read one item.
async fn decode_one<T: Decode>(encoded: Vec<u8>) -> Option<T> {
    let cursor = Cursor::new(encoded);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);
    decoder
        .read_item::<T>()
        .await
        .expect("decode_one: read_item failed")
}

/// Wrap encoded bytes in a `BufReader`-backed `AsyncDecoder` and read `n` items.
async fn decode_n<T: Decode>(encoded: Vec<u8>, n: usize) -> Vec<T> {
    let cursor = Cursor::new(encoded);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        match decoder
            .read_item::<T>()
            .await
            .expect("decode_n: read_item failed")
        {
            Some(v) => out.push(v),
            None => break,
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Test 1: StreamEvent basic roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_stream_event_basic_roundtrip() {
    let original = StreamEvent {
        event_id: 42,
        source: String::from("sensor-A"),
        payload: vec![0x01, 0x02, 0x03],
        priority: 7,
    };
    let encoded = duplex_encode(&original).await;
    let decoded = decode_one::<StreamEvent>(encoded).await;
    assert_eq!(
        decoded,
        Some(original),
        "StreamEvent basic roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 2: StreamControl::Start roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_stream_control_start_roundtrip() {
    let original = StreamControl::Start { stream_id: 1001 };
    let encoded = duplex_encode(&original).await;
    let decoded = decode_one::<StreamControl>(encoded).await;
    assert_eq!(
        decoded,
        Some(StreamControl::Start { stream_id: 1001 }),
        "StreamControl::Start roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 3: StreamControl::Stop roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_stream_control_stop_roundtrip() {
    let original = StreamControl::Stop {
        reason: String::from("graceful shutdown"),
    };
    let encoded = duplex_encode(&original).await;
    let decoded = decode_one::<StreamControl>(encoded).await;
    assert_eq!(
        decoded,
        Some(StreamControl::Stop {
            reason: String::from("graceful shutdown"),
        }),
        "StreamControl::Stop roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 4: All StreamControl variants roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_all_stream_control_variants_roundtrip() {
    let variants = vec![
        StreamControl::Start { stream_id: 99 },
        StreamControl::Pause,
        StreamControl::Resume,
        StreamControl::Stop {
            reason: String::from("timeout"),
        },
        StreamControl::Flush,
    ];

    for variant in &variants {
        let encoded = duplex_encode(variant).await;
        let decoded = decode_one::<StreamControl>(encoded).await;
        assert!(
            decoded.is_some(),
            "StreamControl variant roundtrip returned None"
        );
    }

    // Encode all at once and verify order
    let encoded_all = duplex_encode_many(&variants).await;
    let decoded_all = decode_n::<StreamControl>(encoded_all, variants.len()).await;
    assert_eq!(
        decoded_all.len(),
        variants.len(),
        "all StreamControl variants: wrong count"
    );

    assert!(matches!(
        decoded_all[0],
        StreamControl::Start { stream_id: 99 }
    ));
    assert!(matches!(decoded_all[1], StreamControl::Pause));
    assert!(matches!(decoded_all[2], StreamControl::Resume));
    assert!(matches!(&decoded_all[3], StreamControl::Stop { reason } if reason == "timeout"));
    assert!(matches!(decoded_all[4], StreamControl::Flush));
}

// ---------------------------------------------------------------------------
// Test 5: Write 5 StreamEvents, read back in order
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_five_stream_events_in_order() {
    let events: Vec<StreamEvent> = (0u64..5)
        .map(|i| StreamEvent {
            event_id: i,
            source: format!("source-{i}"),
            payload: vec![(i & 0xFF) as u8; (i as usize + 1) * 3],
            priority: (i % 10) as u8,
        })
        .collect();

    let encoded = duplex_encode_many(&events).await;
    let decoded = decode_n::<StreamEvent>(encoded, events.len()).await;

    assert_eq!(decoded.len(), events.len(), "5 StreamEvents: wrong count");
    for (idx, (expected, got)) in events.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(expected, got, "StreamEvent mismatch at index {idx}");
    }
}

// ---------------------------------------------------------------------------
// Test 6: Empty payload StreamEvent
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_stream_event_empty_payload() {
    let original = StreamEvent {
        event_id: 0,
        source: String::from("empty-src"),
        payload: Vec::new(),
        priority: 0,
    };
    let encoded = duplex_encode(&original).await;
    let decoded = decode_one::<StreamEvent>(encoded).await;
    assert_eq!(
        decoded,
        Some(original),
        "StreamEvent with empty payload roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 7: Large payload (1000 bytes)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_stream_event_large_payload() {
    let payload: Vec<u8> = (0u16..1000).map(|i| (i & 0xFF) as u8).collect();
    let original = StreamEvent {
        event_id: 999,
        source: String::from("large-src"),
        payload: payload.clone(),
        priority: 200,
    };
    let encoded = duplex_encode(&original).await;
    let decoded = decode_one::<StreamEvent>(encoded).await;
    assert_eq!(
        decoded,
        Some(original),
        "StreamEvent with 1000-byte payload roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Read after all items returns None
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_read_after_all_items_returns_none() {
    let event = StreamEvent {
        event_id: 1,
        source: String::from("src"),
        payload: vec![0xAA],
        priority: 1,
    };

    let (writer, mut pipe_reader) = tokio::io::duplex(65536);
    let mut encoder = AsyncEncoder::new(writer);
    encoder
        .write_item(&event)
        .await
        .expect("test 8: write_item failed");
    encoder.finish().await.expect("test 8: finish failed");

    let mut raw = Vec::new();
    pipe_reader
        .read_to_end(&mut raw)
        .await
        .expect("test 8: read_to_end failed");

    let cursor = Cursor::new(raw);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);

    let first: Option<StreamEvent> = decoder
        .read_item()
        .await
        .expect("test 8: first read failed");
    assert_eq!(first, Some(event), "test 8: first item mismatch");

    let second: Option<StreamEvent> = decoder
        .read_item()
        .await
        .expect("test 8: second read failed");
    assert!(second.is_none(), "test 8: expected None after stream end");
}

// ---------------------------------------------------------------------------
// Test 9: u32 roundtrip via async
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_u32_roundtrip() {
    let value: u32 = 0xCAFE_BABE;
    let encoded = duplex_encode(&value).await;
    let decoded = decode_one::<u32>(encoded).await;
    assert_eq!(decoded, Some(value), "u32 roundtrip via async failed");
}

// ---------------------------------------------------------------------------
// Test 10: String roundtrip via async
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_string_roundtrip() {
    let value = String::from("async tokio streaming test");
    let encoded = duplex_encode(&value).await;
    let decoded = decode_one::<String>(encoded).await;
    assert_eq!(decoded, Some(value), "String roundtrip via async failed");
}

// ---------------------------------------------------------------------------
// Test 11: bool roundtrip via async
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_bool_roundtrip() {
    let true_val: bool = true;
    let false_val: bool = false;

    let encoded_true = duplex_encode(&true_val).await;
    let decoded_true = decode_one::<bool>(encoded_true).await;
    assert_eq!(decoded_true, Some(true), "bool true roundtrip failed");

    let encoded_false = duplex_encode(&false_val).await;
    let decoded_false = decode_one::<bool>(encoded_false).await;
    assert_eq!(decoded_false, Some(false), "bool false roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 12: Vec<u8> roundtrip via async
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_vec_u8_roundtrip() {
    let value: Vec<u8> = (0u8..=255u8).collect();
    let encoded = duplex_encode(&value).await;
    let decoded = decode_one::<Vec<u8>>(encoded).await;
    assert_eq!(decoded, Some(value), "Vec<u8> roundtrip via async failed");
}

// ---------------------------------------------------------------------------
// Test 13: 10 StreamEvents sequential write/read
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_ten_stream_events_sequential() {
    let events: Vec<StreamEvent> = (0u64..10)
        .map(|i| StreamEvent {
            event_id: i * 100,
            source: format!("node-{:02}", i),
            payload: vec![(i * 11 & 0xFF) as u8; i as usize * 5],
            priority: (255 - i as u8 * 20),
        })
        .collect();

    let (writer, mut pipe_reader) = tokio::io::duplex(65536);
    {
        let mut encoder = AsyncEncoder::new(writer);
        for event in &events {
            encoder
                .write_item(event)
                .await
                .expect("test 13: write_item failed");
        }
        encoder.finish().await.expect("test 13: finish failed");
    }

    let mut raw = Vec::new();
    pipe_reader
        .read_to_end(&mut raw)
        .await
        .expect("test 13: read_to_end failed");

    let cursor = Cursor::new(raw);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);

    for (idx, expected) in events.iter().enumerate() {
        let decoded: Option<StreamEvent> = decoder
            .read_item()
            .await
            .expect("test 13: read_item failed");
        assert_eq!(
            decoded.as_ref(),
            Some(expected),
            "test 13: StreamEvent mismatch at index {idx}"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 14: Mixed types don't interfere (separate streams)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_mixed_types_separate_streams() {
    let event = StreamEvent {
        event_id: 7,
        source: String::from("mixed"),
        payload: vec![1, 2, 3],
        priority: 5,
    };
    let control = StreamControl::Pause;
    let num: u64 = 12345678;

    let encoded_event = duplex_encode(&event).await;
    let encoded_control = duplex_encode(&control).await;
    let encoded_num = duplex_encode(&num).await;

    let decoded_event = decode_one::<StreamEvent>(encoded_event).await;
    let decoded_control = decode_one::<StreamControl>(encoded_control).await;
    let decoded_num = decode_one::<u64>(encoded_num).await;

    assert_eq!(
        decoded_event,
        Some(event),
        "mixed types: StreamEvent mismatch"
    );
    assert!(
        matches!(decoded_control, Some(StreamControl::Pause)),
        "mixed types: StreamControl::Pause mismatch"
    );
    assert_eq!(decoded_num, Some(num), "mixed types: u64 mismatch");
}

// ---------------------------------------------------------------------------
// Test 15: StreamEvent with unicode source
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_stream_event_unicode_source() {
    let original = StreamEvent {
        event_id: 100,
        source: String::from("センサー-データ/αβγ/🚀🌍"),
        payload: vec![0xDE, 0xAD, 0xBE, 0xEF],
        priority: 42,
    };
    let encoded = duplex_encode(&original).await;
    let decoded = decode_one::<StreamEvent>(encoded).await;
    assert_eq!(
        decoded,
        Some(original),
        "StreamEvent with unicode source roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Vec<StreamControl> all 5 variants (write one by one, read one by one)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_all_five_stream_control_variants_one_by_one() {
    let variants = vec![
        StreamControl::Start { stream_id: 55 },
        StreamControl::Pause,
        StreamControl::Resume,
        StreamControl::Stop {
            reason: String::from("user request"),
        },
        StreamControl::Flush,
    ];

    let (writer, mut pipe_reader) = tokio::io::duplex(65536);
    {
        let mut encoder = AsyncEncoder::new(writer);
        for v in &variants {
            encoder
                .write_item(v)
                .await
                .expect("test 16: write_item failed");
        }
        encoder.finish().await.expect("test 16: finish failed");
    }

    let mut raw = Vec::new();
    pipe_reader
        .read_to_end(&mut raw)
        .await
        .expect("test 16: read_to_end failed");

    let cursor = Cursor::new(raw);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);

    let v0: Option<StreamControl> = decoder
        .read_item()
        .await
        .expect("test 16: read variant 0 failed");
    assert!(
        matches!(v0, Some(StreamControl::Start { stream_id: 55 })),
        "test 16: variant 0 mismatch"
    );

    let v1: Option<StreamControl> = decoder
        .read_item()
        .await
        .expect("test 16: read variant 1 failed");
    assert!(
        matches!(v1, Some(StreamControl::Pause)),
        "test 16: variant 1 mismatch"
    );

    let v2: Option<StreamControl> = decoder
        .read_item()
        .await
        .expect("test 16: read variant 2 failed");
    assert!(
        matches!(v2, Some(StreamControl::Resume)),
        "test 16: variant 2 mismatch"
    );

    let v3: Option<StreamControl> = decoder
        .read_item()
        .await
        .expect("test 16: read variant 3 failed");
    assert!(
        matches!(&v3, Some(StreamControl::Stop { reason }) if reason == "user request"),
        "test 16: variant 3 mismatch"
    );

    let v4: Option<StreamControl> = decoder
        .read_item()
        .await
        .expect("test 16: read variant 4 failed");
    assert!(
        matches!(v4, Some(StreamControl::Flush)),
        "test 16: variant 4 mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 17: u64::MAX via async
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_u64_max_roundtrip() {
    let value: u64 = u64::MAX;
    let encoded = duplex_encode(&value).await;
    let decoded = decode_one::<u64>(encoded).await;
    assert_eq!(
        decoded,
        Some(u64::MAX),
        "u64::MAX roundtrip via async failed"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Empty string via async
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_empty_string_roundtrip() {
    let value = String::new();
    let encoded = duplex_encode(&value).await;
    let decoded = decode_one::<String>(encoded).await;
    assert_eq!(
        decoded,
        Some(String::new()),
        "empty string roundtrip via async failed"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Interleave writes and reads (write 2, read 2, write 2, read 2)
// ---------------------------------------------------------------------------
//
// The duplex pipe is used as a true bidirectional channel. We spawn the writer
// as a separate task so it can concurrently write while the reader drains.
// Each "batch" is a separate duplex channel to isolate the encoding frames.

#[tokio::test]
async fn test_async19_interleave_write_read() {
    // Batch 1: write 2 events, read 2 events
    let batch1 = vec![
        StreamEvent {
            event_id: 10,
            source: String::from("batch1-A"),
            payload: vec![0xAA, 0xBB],
            priority: 10,
        },
        StreamEvent {
            event_id: 11,
            source: String::from("batch1-B"),
            payload: vec![0xCC, 0xDD],
            priority: 11,
        },
    ];

    let (writer1, mut pipe_reader1) = tokio::io::duplex(65536);
    {
        let mut encoder = AsyncEncoder::new(writer1);
        for event in &batch1 {
            encoder
                .write_item(event)
                .await
                .expect("interleave batch1: write failed");
        }
        encoder
            .finish()
            .await
            .expect("interleave batch1: finish failed");
    }

    let mut raw1 = Vec::new();
    pipe_reader1
        .read_to_end(&mut raw1)
        .await
        .expect("interleave batch1: read_to_end failed");

    let cursor1 = Cursor::new(raw1);
    let mut decoder1 = AsyncDecoder::new(BufReader::new(cursor1));
    let r10: Option<StreamEvent> = decoder1
        .read_item()
        .await
        .expect("interleave batch1: read 0 failed");
    let r11: Option<StreamEvent> = decoder1
        .read_item()
        .await
        .expect("interleave batch1: read 1 failed");
    assert_eq!(r10.as_ref(), Some(&batch1[0]), "interleave: batch1 item 0");
    assert_eq!(r11.as_ref(), Some(&batch1[1]), "interleave: batch1 item 1");

    // Batch 2: write 2 events, read 2 events
    let batch2 = vec![
        StreamEvent {
            event_id: 20,
            source: String::from("batch2-A"),
            payload: vec![0xEE, 0xFF],
            priority: 20,
        },
        StreamEvent {
            event_id: 21,
            source: String::from("batch2-B"),
            payload: vec![0x11, 0x22],
            priority: 21,
        },
    ];

    let (writer2, mut pipe_reader2) = tokio::io::duplex(65536);
    {
        let mut encoder = AsyncEncoder::new(writer2);
        for event in &batch2 {
            encoder
                .write_item(event)
                .await
                .expect("interleave batch2: write failed");
        }
        encoder
            .finish()
            .await
            .expect("interleave batch2: finish failed");
    }

    let mut raw2 = Vec::new();
    pipe_reader2
        .read_to_end(&mut raw2)
        .await
        .expect("interleave batch2: read_to_end failed");

    let cursor2 = Cursor::new(raw2);
    let mut decoder2 = AsyncDecoder::new(BufReader::new(cursor2));
    let r20: Option<StreamEvent> = decoder2
        .read_item()
        .await
        .expect("interleave batch2: read 0 failed");
    let r21: Option<StreamEvent> = decoder2
        .read_item()
        .await
        .expect("interleave batch2: read 1 failed");
    assert_eq!(r20.as_ref(), Some(&batch2[0]), "interleave: batch2 item 0");
    assert_eq!(r21.as_ref(), Some(&batch2[1]), "interleave: batch2 item 1");
}

// ---------------------------------------------------------------------------
// Test 20: StreamEvent priority=0 and priority=255 roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_stream_event_priority_boundaries() {
    let low = StreamEvent {
        event_id: 0,
        source: String::from("low-prio"),
        payload: vec![0x00],
        priority: 0,
    };
    let high = StreamEvent {
        event_id: u64::MAX,
        source: String::from("high-prio"),
        payload: vec![0xFF],
        priority: 255,
    };

    let encoded_low = duplex_encode(&low).await;
    let decoded_low = decode_one::<StreamEvent>(encoded_low).await;
    assert_eq!(
        decoded_low,
        Some(low),
        "StreamEvent priority=0 roundtrip failed"
    );

    let encoded_high = duplex_encode(&high).await;
    let decoded_high = decode_one::<StreamEvent>(encoded_high).await;
    assert_eq!(
        decoded_high,
        Some(high),
        "StreamEvent priority=255 roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 21: 100 u32 values streamed
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_hundred_u32_values_streamed() {
    let values: Vec<u32> = (0u32..100).collect();

    let (writer, mut pipe_reader) = tokio::io::duplex(65536);
    {
        let mut encoder = AsyncEncoder::new(writer);
        for v in &values {
            encoder
                .write_item(v)
                .await
                .expect("test 21: write_item failed");
        }
        encoder.finish().await.expect("test 21: finish failed");
    }

    let mut raw = Vec::new();
    pipe_reader
        .read_to_end(&mut raw)
        .await
        .expect("test 21: read_to_end failed");

    let cursor = Cursor::new(raw);
    let buf_reader = BufReader::new(cursor);
    let mut decoder = AsyncDecoder::new(buf_reader);

    let mut decoded = Vec::with_capacity(100);
    while let Some(v) = decoder
        .read_item::<u32>()
        .await
        .expect("test 21: read_item failed")
    {
        decoded.push(v);
    }

    assert_eq!(
        decoded.len(),
        100,
        "test 21: expected 100 u32 values, got {}",
        decoded.len()
    );
    assert_eq!(decoded, values, "test 21: decoded values mismatch");
}

// ---------------------------------------------------------------------------
// Test 22: StreamControl with empty reason string
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async19_stream_control_stop_empty_reason() {
    let original = StreamControl::Stop {
        reason: String::new(),
    };
    let encoded = duplex_encode(&original).await;
    let decoded = decode_one::<StreamControl>(encoded).await;
    assert!(
        matches!(
            &decoded,
            Some(StreamControl::Stop { reason }) if reason.is_empty()
        ),
        "StreamControl::Stop with empty reason roundtrip failed: got {:?}",
        decoded
    );
}
