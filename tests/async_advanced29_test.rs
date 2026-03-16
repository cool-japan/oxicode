//! Advanced async streaming tests (29th set) for OxiCode.
//!
//! Domain: Live Sports Analytics.
//! 22 top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.

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
use std::io::Cursor;
use tokio::io::BufReader;

// ---------------------------------------------------------------------------
// Domain types: Live Sports Analytics
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SportType {
    Football,
    Basketball,
    Soccer,
    Tennis,
    Baseball,
    Hockey,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EventType {
    Goal,
    Foul,
    Timeout,
    Substitution,
    Injury,
    CardYellow,
    CardRed,
    GameStart,
    GameEnd,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GameEvent {
    event_id: u64,
    sport: SportType,
    event_type: EventType,
    timestamp_ms: u64,
    team_id: u32,
    player_id: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TeamStats {
    team_id: u32,
    score: u32,
    fouls: u32,
    timeouts_remaining: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GameSnapshot {
    game_id: u64,
    sport: SportType,
    home: TeamStats,
    away: TeamStats,
    period: u8,
    clock_ms: u64,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_goal_event(event_id: u64, team_id: u32, player_id: u32) -> GameEvent {
    GameEvent {
        event_id,
        sport: SportType::Soccer,
        event_type: EventType::Goal,
        timestamp_ms: event_id * 1_000,
        team_id,
        player_id: Some(player_id),
    }
}

fn make_snapshot(game_id: u64, sport: SportType, period: u8) -> GameSnapshot {
    GameSnapshot {
        game_id,
        sport,
        home: TeamStats {
            team_id: 1,
            score: period as u32,
            fouls: 2,
            timeouts_remaining: 3,
        },
        away: TeamStats {
            team_id: 2,
            score: 0,
            fouls: 1,
            timeouts_remaining: 3,
        },
        period,
        clock_ms: 720_000,
    }
}

async fn encode_single_item<T: Encode>(item: &T) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(item)
            .await
            .expect("encode_single_item: write_item failed");
        enc.finish()
            .await
            .expect("encode_single_item: finish failed");
    }
    buf
}

async fn decode_single_item<T: Decode>(buf: Vec<u8>) -> Option<T> {
    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);
    dec.read_item::<T>()
        .await
        .expect("decode_single_item: read_item failed")
}

// ---------------------------------------------------------------------------
// Test 1: Single GameEvent write and read roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_single_game_event_roundtrip() {
    let event = make_goal_event(1, 10, 42);
    let buf = encode_single_item(&event).await;
    let decoded = decode_single_item::<GameEvent>(buf).await;
    assert_eq!(
        decoded,
        Some(event),
        "single GameEvent async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 2: Batch of GameEvents written and read back in order
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_batch_game_events_in_order() {
    let events: Vec<GameEvent> = (1u64..=5)
        .map(|i| make_goal_event(i, (i % 2 + 10) as u32, (i * 7) as u32))
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for ev in &events {
            enc.write_item(ev).await.expect("batch write failed");
        }
        enc.finish().await.expect("batch finish failed");
    }

    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);

    for expected in &events {
        let item: Option<GameEvent> = dec.read_item().await.expect("batch read failed");
        assert_eq!(
            item.as_ref(),
            Some(expected),
            "event_id={} mismatch",
            expected.event_id
        );
    }
    let eof: Option<GameEvent> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None at stream end");
}

// ---------------------------------------------------------------------------
// Test 3: GameSnapshot write and read roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_game_snapshot_roundtrip() {
    let snapshot = make_snapshot(100, SportType::Basketball, 2);
    let buf = encode_single_item(&snapshot).await;
    let decoded = decode_single_item::<GameSnapshot>(buf).await;
    assert_eq!(
        decoded,
        Some(snapshot),
        "GameSnapshot async roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Empty stream — finish immediately, no items
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_empty_stream_no_items() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let enc = AsyncEncoder::new(cursor);
        enc.finish().await.expect("empty stream finish failed");
    }

    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);
    let item: Option<GameEvent> = dec.read_item().await.expect("empty stream read failed");
    assert_eq!(item, None, "empty stream must return None immediately");
    assert!(
        dec.is_finished(),
        "decoder must report finished on empty stream"
    );
}

// ---------------------------------------------------------------------------
// Test 5: Large event series (200 events)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_large_event_series() {
    let events: Vec<GameEvent> = (0u64..200)
        .map(|i| GameEvent {
            event_id: i,
            sport: if i % 2 == 0 {
                SportType::Football
            } else {
                SportType::Hockey
            },
            event_type: EventType::Foul,
            timestamp_ms: i * 500,
            team_id: (i % 4 + 1) as u32,
            player_id: Some((i % 22) as u32),
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for ev in &events {
            enc.write_item(ev).await.expect("large series write failed");
        }
        enc.finish().await.expect("large series finish failed");
    }

    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);
    let decoded: Vec<GameEvent> = dec.read_all().await.expect("large series read_all failed");

    assert_eq!(decoded.len(), 200, "must decode 200 events");
    assert_eq!(decoded, events, "large event series content mismatch");
}

// ---------------------------------------------------------------------------
// Test 6: Progress tracking — items_processed > 0 after flush
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_progress_items_processed() {
    let config = StreamingConfig::default().with_flush_per_item(true);
    let event = make_goal_event(99, 5, 17);

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::with_config(cursor, config);
        enc.write_item(&event)
            .await
            .expect("progress test write failed");

        // After flush_per_item the item is flushed immediately
        assert!(
            enc.progress().items_processed > 0,
            "items_processed must be > 0 after flush_per_item write"
        );

        enc.finish().await.expect("progress test finish failed");
    }

    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);
    let decoded: Option<GameEvent> = dec.read_item().await.expect("progress read failed");
    assert_eq!(decoded, Some(event));
}

// ---------------------------------------------------------------------------
// Test 7: Multiple sports — one event per sport type
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_multiple_sports_one_event_each() {
    let sports = [
        SportType::Football,
        SportType::Basketball,
        SportType::Soccer,
        SportType::Tennis,
        SportType::Baseball,
        SportType::Hockey,
    ];
    let events: Vec<GameEvent> = sports
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, sport)| GameEvent {
            event_id: i as u64,
            sport,
            event_type: EventType::GameStart,
            timestamp_ms: 0,
            team_id: i as u32 + 1,
            player_id: None,
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for ev in &events {
            enc.write_item(ev).await.expect("multi-sport write failed");
        }
        enc.finish().await.expect("multi-sport finish failed");
    }

    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);
    let decoded: Vec<GameEvent> = dec.read_all().await.expect("multi-sport read_all failed");

    assert_eq!(decoded.len(), 6, "must have one event per sport");
    assert_eq!(decoded, events, "multi-sport event content mismatch");
}

// ---------------------------------------------------------------------------
// Test 8: GameEvent with no player_id (None variant)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_event_without_player_id() {
    let event = GameEvent {
        event_id: 1000,
        sport: SportType::Football,
        event_type: EventType::Timeout,
        timestamp_ms: 45_000,
        team_id: 7,
        player_id: None,
    };

    let buf = encode_single_item(&event).await;
    let decoded = decode_single_item::<GameEvent>(buf).await;
    assert_eq!(
        decoded,
        Some(event),
        "GameEvent with None player_id mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Multiple GameSnapshots in sequence
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_multiple_snapshots_sequence() {
    let snapshots: Vec<GameSnapshot> = (1u8..=4)
        .map(|p| make_snapshot(200, SportType::Basketball, p))
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for snap in &snapshots {
            enc.write_item(snap).await.expect("snapshot write failed");
        }
        enc.finish().await.expect("snapshot finish failed");
    }

    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);
    for expected in &snapshots {
        let item: Option<GameSnapshot> = dec.read_item().await.expect("snapshot read failed");
        assert_eq!(
            item.as_ref(),
            Some(expected),
            "snapshot period={} mismatch",
            expected.period
        );
    }
}

// ---------------------------------------------------------------------------
// Test 10: Concurrent read — two decoders on independent copies of encoded data
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_concurrent_independent_reads() {
    let event_a = make_goal_event(10, 1, 99);
    let event_b = make_goal_event(20, 2, 88);

    let buf_a = encode_single_item(&event_a).await;
    let buf_b = encode_single_item(&event_b).await;

    // Spawn two independent decode tasks
    let task_a = tokio::spawn(async move {
        let cursor = Cursor::new(buf_a);
        let br = BufReader::new(cursor);
        let mut dec = AsyncDecoder::new(br);
        dec.read_item::<GameEvent>()
            .await
            .expect("concurrent decode_a failed")
    });

    let task_b = tokio::spawn(async move {
        let cursor = Cursor::new(buf_b);
        let br = BufReader::new(cursor);
        let mut dec = AsyncDecoder::new(br);
        dec.read_item::<GameEvent>()
            .await
            .expect("concurrent decode_b failed")
    });

    let result_a = task_a.await.expect("task_a panicked");
    let result_b = task_b.await.expect("task_b panicked");

    assert_eq!(result_a, Some(event_a), "concurrent decode_a mismatch");
    assert_eq!(result_b, Some(event_b), "concurrent decode_b mismatch");
}

// ---------------------------------------------------------------------------
// Test 11: Substitution event roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_substitution_event_roundtrip() {
    let event = GameEvent {
        event_id: 500,
        sport: SportType::Soccer,
        event_type: EventType::Substitution,
        timestamp_ms: 60_000,
        team_id: 3,
        player_id: Some(11),
    };

    let buf = encode_single_item(&event).await;
    let decoded = decode_single_item::<GameEvent>(buf).await;
    assert_eq!(decoded, Some(event), "Substitution event roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 12: Injury event roundtrip (with player_id)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_injury_event_roundtrip() {
    let event = GameEvent {
        event_id: 700,
        sport: SportType::Football,
        event_type: EventType::Injury,
        timestamp_ms: 75_000,
        team_id: 8,
        player_id: Some(23),
    };

    let buf = encode_single_item(&event).await;
    let decoded = decode_single_item::<GameEvent>(buf).await;
    assert_eq!(decoded, Some(event), "Injury event roundtrip failed");
}

// ---------------------------------------------------------------------------
// Test 13: Card events (yellow and red) in sequence
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_card_events_in_sequence() {
    let yellow = GameEvent {
        event_id: 301,
        sport: SportType::Soccer,
        event_type: EventType::CardYellow,
        timestamp_ms: 30_000,
        team_id: 2,
        player_id: Some(5),
    };
    let red = GameEvent {
        event_id: 302,
        sport: SportType::Soccer,
        event_type: EventType::CardRed,
        timestamp_ms: 32_000,
        team_id: 2,
        player_id: Some(5),
    };

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&yellow)
            .await
            .expect("yellow card write failed");
        enc.write_item(&red).await.expect("red card write failed");
        enc.finish().await.expect("card events finish failed");
    }

    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);

    let d_yellow: Option<GameEvent> = dec.read_item().await.expect("yellow card read failed");
    let d_red: Option<GameEvent> = dec.read_item().await.expect("red card read failed");

    assert_eq!(d_yellow, Some(yellow), "yellow card mismatch");
    assert_eq!(d_red, Some(red), "red card mismatch");
}

// ---------------------------------------------------------------------------
// Test 14: GameStart and GameEnd events roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_game_start_end_events() {
    let start = GameEvent {
        event_id: 1,
        sport: SportType::Baseball,
        event_type: EventType::GameStart,
        timestamp_ms: 0,
        team_id: 0,
        player_id: None,
    };
    let end = GameEvent {
        event_id: 9999,
        sport: SportType::Baseball,
        event_type: EventType::GameEnd,
        timestamp_ms: 10_800_000,
        team_id: 0,
        player_id: None,
    };

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&start)
            .await
            .expect("game start write failed");
        enc.write_item(&end).await.expect("game end write failed");
        enc.finish().await.expect("start/end finish failed");
    }

    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);

    let d_start: Option<GameEvent> = dec.read_item().await.expect("game start read failed");
    let d_end: Option<GameEvent> = dec.read_item().await.expect("game end read failed");

    assert_eq!(d_start, Some(start), "GameStart event mismatch");
    assert_eq!(d_end, Some(end), "GameEnd event mismatch");
}

// ---------------------------------------------------------------------------
// Test 15: TeamStats roundtrip via GameSnapshot
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_team_stats_via_snapshot() {
    let snapshot = GameSnapshot {
        game_id: 555,
        sport: SportType::Hockey,
        home: TeamStats {
            team_id: 10,
            score: 3,
            fouls: 5,
            timeouts_remaining: 1,
        },
        away: TeamStats {
            team_id: 11,
            score: 2,
            fouls: 4,
            timeouts_remaining: 2,
        },
        period: 3,
        clock_ms: 1_200_000,
    };

    let buf = encode_single_item(&snapshot).await;
    let decoded = decode_single_item::<GameSnapshot>(buf).await;
    assert_eq!(decoded, Some(snapshot), "TeamStats in snapshot mismatch");
}

// ---------------------------------------------------------------------------
// Test 16: write_all with cloned vec of events
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_write_all_cloned_vec() {
    let events: Vec<GameEvent> = (0u64..8)
        .map(|i| GameEvent {
            event_id: i,
            sport: SportType::Tennis,
            event_type: EventType::Foul,
            timestamp_ms: i * 300,
            team_id: 1,
            player_id: Some(i as u32),
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        // write_all takes owned IntoIterator<Item=T>; clone before passing
        enc.write_all(events.clone())
            .await
            .expect("write_all failed");
        enc.finish().await.expect("write_all finish failed");
    }

    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);
    let decoded: Vec<GameEvent> = dec.read_all().await.expect("write_all decode failed");

    assert_eq!(decoded.len(), 8, "write_all must produce 8 events");
    assert_eq!(decoded, events, "write_all decoded content mismatch");
}

// ---------------------------------------------------------------------------
// Test 17: Async encode then sync decode verification
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_async_encode_sync_decode() {
    let event = make_goal_event(888, 4, 9);

    let mut stream_buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut stream_buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(&event)
            .await
            .expect("async encode write failed");
        enc.finish().await.expect("async encode finish failed");
    }

    // Also verify sync roundtrip independently
    let sync_bytes = encode_to_vec(&event).expect("sync encode_to_vec failed");
    let (sync_decoded, _): (GameEvent, _) =
        decode_from_slice(&sync_bytes).expect("sync decode failed");
    assert_eq!(sync_decoded, event, "sync roundtrip of same event mismatch");

    // Async decode the stream-encoded data
    let cursor = Cursor::new(stream_buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);
    let async_decoded: Option<GameEvent> = dec.read_item().await.expect("async decode failed");
    assert_eq!(
        async_decoded,
        Some(event),
        "async-encode then async-decode mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Small chunk size forces multiple chunks for large event batch
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_small_chunk_size_multiple_chunks() {
    let config = StreamingConfig::new().with_chunk_size(1024);
    let events: Vec<GameEvent> = (0u64..50)
        .map(|i| GameEvent {
            event_id: i,
            sport: SportType::Football,
            event_type: EventType::Foul,
            timestamp_ms: i * 1000,
            team_id: (i % 2 + 1) as u32,
            player_id: Some((i % 11) as u32),
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::with_config(cursor, config);
        for ev in &events {
            enc.write_item(ev).await.expect("chunk test write failed");
        }
        enc.finish().await.expect("chunk test finish failed");
    }

    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);
    let decoded: Vec<GameEvent> = dec.read_all().await.expect("chunk test read_all failed");

    assert_eq!(decoded.len(), 50, "must decode 50 events with small chunks");
    assert_eq!(
        decoded, events,
        "event content mismatch with small chunk size"
    );
    assert!(
        dec.progress().chunks_processed >= 1,
        "must have processed at least one chunk"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Decoder progress items_processed tracks count correctly
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_decoder_items_processed_count() {
    let events: Vec<GameEvent> = (0u64..10)
        .map(|i| make_goal_event(i, 1, i as u32))
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for ev in &events {
            enc.write_item(ev)
                .await
                .expect("decoder progress write failed");
        }
        enc.finish().await.expect("decoder progress finish failed");
    }

    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);

    // Read first 5
    for _ in 0..5 {
        let _: Option<GameEvent> = dec.read_item().await.expect("decoder progress read failed");
    }
    assert_eq!(
        dec.progress().items_processed,
        5,
        "items_processed must be 5 after reading 5 events"
    );

    // Read remaining 5
    let _rest: Vec<GameEvent> = dec
        .read_all()
        .await
        .expect("decoder progress read_all failed");
    assert_eq!(
        dec.progress().items_processed,
        10,
        "items_processed must be 10 after reading all events"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Tennis match events stream — all event types for one sport
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_tennis_match_all_event_types() {
    let tennis_events = vec![
        EventType::GameStart,
        EventType::Foul,
        EventType::Timeout,
        EventType::Injury,
        EventType::Substitution,
        EventType::CardYellow,
        EventType::CardRed,
        EventType::Goal,
        EventType::GameEnd,
    ];

    let events: Vec<GameEvent> = tennis_events
        .into_iter()
        .enumerate()
        .map(|(i, event_type)| GameEvent {
            event_id: i as u64,
            sport: SportType::Tennis,
            event_type,
            timestamp_ms: i as u64 * 600_000,
            team_id: (i % 2 + 1) as u32,
            player_id: if i % 2 == 0 { Some(i as u32) } else { None },
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for ev in &events {
            enc.write_item(ev)
                .await
                .expect("tennis stream write failed");
        }
        enc.finish().await.expect("tennis stream finish failed");
    }

    let cursor = Cursor::new(buf);
    let br = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(br);
    let decoded: Vec<GameEvent> = dec.read_all().await.expect("tennis stream read_all failed");

    assert_eq!(
        decoded.len(),
        9,
        "must have 9 events covering all event types"
    );
    assert_eq!(decoded, events, "tennis event stream content mismatch");
}

// ---------------------------------------------------------------------------
// Test 21: Mixed GameEvent and GameSnapshot interleaved — separate streams
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_mixed_events_and_snapshots_separate_streams() {
    let event = make_goal_event(1, 10, 7);
    let snapshot = make_snapshot(42, SportType::Soccer, 1);

    // Encode event
    let buf_event = encode_single_item(&event).await;

    // Encode snapshot
    let buf_snapshot = encode_single_item(&snapshot).await;

    // Decode independently
    let decoded_event = decode_single_item::<GameEvent>(buf_event).await;
    let decoded_snapshot = decode_single_item::<GameSnapshot>(buf_snapshot).await;

    assert_eq!(
        decoded_event,
        Some(event),
        "interleaved test: GameEvent mismatch"
    );
    assert_eq!(
        decoded_snapshot,
        Some(snapshot),
        "interleaved test: GameSnapshot mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 22: High-volume concurrent reads — 4 tasks, 50 events each
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sports29_high_volume_concurrent_reads() {
    // Build 4 independent encoded buffers of 50 events each
    let mut encoded_buffers: Vec<Vec<u8>> = Vec::with_capacity(4);
    for task_idx in 0u64..4 {
        let events: Vec<GameEvent> = (0u64..50)
            .map(|i| GameEvent {
                event_id: task_idx * 50 + i,
                sport: SportType::Basketball,
                event_type: EventType::Foul,
                timestamp_ms: i * 200,
                team_id: (task_idx + 1) as u32,
                player_id: Some(i as u32),
            })
            .collect();

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut enc = AsyncEncoder::new(cursor);
            for ev in &events {
                enc.write_item(ev)
                    .await
                    .expect("high-volume encode write failed");
            }
            enc.finish()
                .await
                .expect("high-volume encode finish failed");
        }
        encoded_buffers.push(buf);
    }

    // Spawn 4 concurrent decode tasks
    let mut handles = Vec::with_capacity(4);
    for (task_idx, buf) in encoded_buffers.into_iter().enumerate() {
        let handle = tokio::spawn(async move {
            let cursor = Cursor::new(buf);
            let br = BufReader::new(cursor);
            let mut dec = AsyncDecoder::new(br);
            let decoded: Vec<GameEvent> = dec
                .read_all()
                .await
                .expect("high-volume concurrent decode failed");
            (task_idx, decoded.len(), dec.progress().items_processed)
        });
        handles.push(handle);
    }

    for handle in handles {
        let (task_idx, count, items_processed) = handle.await.expect("concurrent task panicked");
        assert_eq!(
            count, 50,
            "task {task_idx}: expected 50 decoded events, got {count}"
        );
        assert_eq!(
            items_processed, 50,
            "task {task_idx}: items_processed must be 50"
        );
    }
}
