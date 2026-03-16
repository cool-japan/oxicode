//! Advanced async streaming tests (22nd set) for OxiCode — game events theme.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Domain types unique to this file:
//!   `GameEvent`   — enum with 7 variants covering all common game event types
//!   `GameSession` — struct { session_id: u64, game_name: String, max_players: u32,
//!                           current_players: Vec<u64> }
//!
//! Coverage matrix:
//!   1:   PlayerJoined roundtrip
//!   2:   PlayerLeft roundtrip
//!   3:   ChatMessage roundtrip
//!   4:   ScoreUpdate with positive delta
//!   5:   ScoreUpdate with negative delta
//!   6:   LevelUp roundtrip
//!   7:   ItemPickup roundtrip
//!   8:   GameOver with non-empty final_scores
//!   9:   GameOver with empty final_scores
//!  10:   GameSession roundtrip
//!  11:   write_all / read_all for Vec<GameEvent>
//!  12:   Empty collection write_all / read_all
//!  13:   Large collection (50 events) roundtrip
//!  14:   Mixed event types in sequence
//!  15:   Progress tracking items_processed
//!  16:   StreamingConfig with chunk_size
//!  17:   finish() then read_all()
//!  18:   GameSession Vec roundtrip
//!  19:   Boundary values (u64::MAX score, i32::MIN delta)
//!  20:   Unicode username/message roundtrip
//!  21:   Wrong-type decode returns Err
//!  22:   tokio::join! concurrent encode/decode

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
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GameEvent {
    PlayerJoined {
        player_id: u64,
        username: String,
    },
    PlayerLeft {
        player_id: u64,
    },
    ChatMessage {
        player_id: u64,
        message: String,
    },
    ScoreUpdate {
        player_id: u64,
        score: u64,
        delta: i32,
    },
    LevelUp {
        player_id: u64,
        new_level: u32,
    },
    ItemPickup {
        player_id: u64,
        item_id: u32,
        item_name: String,
    },
    GameOver {
        winner_id: u64,
        final_scores: Vec<(u64, u64)>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GameSession {
    session_id: u64,
    game_name: String,
    max_players: u32,
    current_players: Vec<u64>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_game_session(session_id: u64, name: &str, max: u32, players: Vec<u64>) -> GameSession {
    GameSession {
        session_id,
        game_name: name.to_string(),
        max_players: max,
        current_players: players,
    }
}

fn make_mixed_events(n: usize) -> Vec<GameEvent> {
    (0..n)
        .map(|i| match i % 7 {
            0 => GameEvent::PlayerJoined {
                player_id: i as u64,
                username: format!("player_{}", i),
            },
            1 => GameEvent::PlayerLeft {
                player_id: i as u64,
            },
            2 => GameEvent::ChatMessage {
                player_id: i as u64,
                message: format!("hello from {}", i),
            },
            3 => GameEvent::ScoreUpdate {
                player_id: i as u64,
                score: (i as u64) * 100,
                delta: i as i32 * 10,
            },
            4 => GameEvent::LevelUp {
                player_id: i as u64,
                new_level: (i as u32) + 1,
            },
            5 => GameEvent::ItemPickup {
                player_id: i as u64,
                item_id: i as u32,
                item_name: format!("sword_{}", i),
            },
            _ => GameEvent::GameOver {
                winner_id: i as u64,
                final_scores: vec![(i as u64, (i as u64) * 500)],
            },
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Test 1: PlayerJoined roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_player_joined_roundtrip() {
    let event = GameEvent::PlayerJoined {
        player_id: 42,
        username: "alice".to_string(),
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event)
        .await
        .expect("write_item PlayerJoined");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: GameEvent = dec
        .read_item()
        .await
        .expect("read_item PlayerJoined")
        .expect("expected Some(GameEvent::PlayerJoined)");
    assert_eq!(event, got);
}

// ---------------------------------------------------------------------------
// Test 2: PlayerLeft roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_player_left_roundtrip() {
    let event = GameEvent::PlayerLeft { player_id: 99 };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event).await.expect("write_item PlayerLeft");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: GameEvent = dec
        .read_item()
        .await
        .expect("read_item PlayerLeft")
        .expect("expected Some(GameEvent::PlayerLeft)");
    assert_eq!(event, got);
}

// ---------------------------------------------------------------------------
// Test 3: ChatMessage roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_chat_message_roundtrip() {
    let event = GameEvent::ChatMessage {
        player_id: 7,
        message: "gg wp".to_string(),
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event)
        .await
        .expect("write_item ChatMessage");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: GameEvent = dec
        .read_item()
        .await
        .expect("read_item ChatMessage")
        .expect("expected Some(GameEvent::ChatMessage)");
    assert_eq!(event, got);
}

// ---------------------------------------------------------------------------
// Test 4: ScoreUpdate with positive delta
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_score_update_positive_delta() {
    let event = GameEvent::ScoreUpdate {
        player_id: 1,
        score: 5000,
        delta: 250,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event)
        .await
        .expect("write_item ScoreUpdate positive");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: GameEvent = dec
        .read_item()
        .await
        .expect("read_item ScoreUpdate positive")
        .expect("expected Some(GameEvent::ScoreUpdate positive)");
    assert_eq!(event, got);
    if let GameEvent::ScoreUpdate { delta, .. } = &got {
        assert!(*delta > 0, "delta must be positive");
    } else {
        panic!("expected ScoreUpdate variant");
    }
}

// ---------------------------------------------------------------------------
// Test 5: ScoreUpdate with negative delta
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_score_update_negative_delta() {
    let event = GameEvent::ScoreUpdate {
        player_id: 2,
        score: 3000,
        delta: -100,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event)
        .await
        .expect("write_item ScoreUpdate negative");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: GameEvent = dec
        .read_item()
        .await
        .expect("read_item ScoreUpdate negative")
        .expect("expected Some(GameEvent::ScoreUpdate negative)");
    assert_eq!(event, got);
    if let GameEvent::ScoreUpdate { delta, .. } = &got {
        assert!(*delta < 0, "delta must be negative");
    } else {
        panic!("expected ScoreUpdate variant");
    }
}

// ---------------------------------------------------------------------------
// Test 6: LevelUp roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_level_up_roundtrip() {
    let event = GameEvent::LevelUp {
        player_id: 15,
        new_level: 42,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event).await.expect("write_item LevelUp");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: GameEvent = dec
        .read_item()
        .await
        .expect("read_item LevelUp")
        .expect("expected Some(GameEvent::LevelUp)");
    assert_eq!(event, got);
}

// ---------------------------------------------------------------------------
// Test 7: ItemPickup roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_item_pickup_roundtrip() {
    let event = GameEvent::ItemPickup {
        player_id: 33,
        item_id: 9001,
        item_name: "legendary_axe".to_string(),
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event).await.expect("write_item ItemPickup");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: GameEvent = dec
        .read_item()
        .await
        .expect("read_item ItemPickup")
        .expect("expected Some(GameEvent::ItemPickup)");
    assert_eq!(event, got);
}

// ---------------------------------------------------------------------------
// Test 8: GameOver with non-empty final_scores
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_game_over_nonempty_scores() {
    let event = GameEvent::GameOver {
        winner_id: 10,
        final_scores: vec![(10, 9500), (20, 8100), (30, 7300)],
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event)
        .await
        .expect("write_item GameOver non-empty");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: GameEvent = dec
        .read_item()
        .await
        .expect("read_item GameOver non-empty")
        .expect("expected Some(GameEvent::GameOver)");
    assert_eq!(event, got);
    if let GameEvent::GameOver { final_scores, .. } = &got {
        assert_eq!(final_scores.len(), 3, "must have 3 final scores");
    } else {
        panic!("expected GameOver variant");
    }
}

// ---------------------------------------------------------------------------
// Test 9: GameOver with empty final_scores
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_game_over_empty_scores() {
    let event = GameEvent::GameOver {
        winner_id: 5,
        final_scores: vec![],
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event)
        .await
        .expect("write_item GameOver empty");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: GameEvent = dec
        .read_item()
        .await
        .expect("read_item GameOver empty")
        .expect("expected Some(GameEvent::GameOver)");
    assert_eq!(event, got);
    if let GameEvent::GameOver { final_scores, .. } = &got {
        assert!(final_scores.is_empty(), "final_scores must be empty");
    } else {
        panic!("expected GameOver variant");
    }
}

// ---------------------------------------------------------------------------
// Test 10: GameSession roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_game_session_roundtrip() {
    let session = make_game_session(1001, "Battle Arena", 8, vec![100, 101, 102, 103]);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&session)
        .await
        .expect("write_item GameSession");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: GameSession = dec
        .read_item()
        .await
        .expect("read_item GameSession")
        .expect("expected Some(GameSession)");
    assert_eq!(session, got);
}

// ---------------------------------------------------------------------------
// Test 11: write_all / read_all for Vec<GameEvent>
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_events_write_all_read_all() {
    let events = make_mixed_events(14);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(events.clone().into_iter())
        .await
        .expect("write_all events");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<GameEvent> = dec.read_all().await.expect("read_all events");
    assert_eq!(events, got);
    assert_eq!(got.len(), 14, "must have 14 events");
}

// ---------------------------------------------------------------------------
// Test 12: Empty collection write_all / read_all
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_empty_collection_write_all_read_all() {
    let empty: Vec<GameEvent> = vec![];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(empty.clone().into_iter())
        .await
        .expect("write_all empty events");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<GameEvent> = dec.read_all().await.expect("read_all empty events");
    assert!(
        got.is_empty(),
        "expected empty vec from write_all of 0 events"
    );
    assert!(
        dec.is_finished(),
        "decoder must be finished after empty stream"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Large collection (50 events) roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_large_collection_50_events() {
    let events = make_mixed_events(50);
    assert_eq!(events.len(), 50, "must generate exactly 50 events");

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(events.clone().into_iter())
        .await
        .expect("write_all 50 events");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<GameEvent> = dec.read_all().await.expect("read_all 50 events");
    assert_eq!(events.len(), got.len(), "count mismatch for 50 events");
    assert_eq!(events, got, "data mismatch for large collection");
}

// ---------------------------------------------------------------------------
// Test 14: Mixed event types in sequence (explicit variants)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_mixed_event_types_explicit_sequence() {
    let events = vec![
        GameEvent::PlayerJoined {
            player_id: 1,
            username: "bob".to_string(),
        },
        GameEvent::ChatMessage {
            player_id: 1,
            message: "ready!".to_string(),
        },
        GameEvent::ScoreUpdate {
            player_id: 1,
            score: 100,
            delta: 100,
        },
        GameEvent::ItemPickup {
            player_id: 1,
            item_id: 42,
            item_name: "health_potion".to_string(),
        },
        GameEvent::LevelUp {
            player_id: 1,
            new_level: 2,
        },
        GameEvent::ScoreUpdate {
            player_id: 1,
            score: 50,
            delta: -50,
        },
        GameEvent::PlayerLeft { player_id: 1 },
        GameEvent::GameOver {
            winner_id: 2,
            final_scores: vec![(1, 50), (2, 900)],
        },
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    for e in &events {
        enc.write_item(e).await.expect("write_item in sequence");
    }
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<GameEvent> = dec.read_all().await.expect("read_all sequence");
    assert_eq!(events, got, "mixed explicit sequence roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 15: Progress tracking items_processed
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_progress_items_processed() {
    const N: u64 = 21;
    let events = make_mixed_events(N as usize);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.set_estimated_total(N);
    enc.write_all(events.clone().into_iter())
        .await
        .expect("write_all for progress");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let _: Vec<GameEvent> = dec.read_all().await.expect("read_all for progress");
    assert_eq!(
        dec.progress().items_processed,
        N,
        "items_processed must equal {N}"
    );
    assert!(
        dec.progress().bytes_processed > 0,
        "bytes_processed must be > 0 after reading {N} events"
    );
}

// ---------------------------------------------------------------------------
// Test 16: StreamingConfig with chunk_size
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_streaming_config_chunk_size() {
    let config = StreamingConfig::new().with_chunk_size(128);
    let events = make_mixed_events(20);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for e in &events {
        enc.write_item(e)
            .await
            .expect("write_item with chunk_size config");
    }
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<GameEvent> = dec.read_all().await.expect("read_all chunk_size config");
    assert_eq!(events, got, "data integrity with custom chunk_size config");
    assert!(
        dec.progress().chunks_processed > 0,
        "chunks_processed must be > 0"
    );
}

// ---------------------------------------------------------------------------
// Test 17: finish() then read_all()
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_finish_then_read_all() {
    let events = make_mixed_events(7);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(events.clone().into_iter())
        .await
        .expect("write_all before finish");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<GameEvent> = dec.read_all().await.expect("read_all after finish");
    assert_eq!(events, got, "read_all after finish mismatch");

    // Stream exhausted — further reads must return None
    let extra: Option<GameEvent> = dec
        .read_item()
        .await
        .expect("read after exhaustion must not error");
    assert_eq!(extra, None, "must return None after stream exhausted");
    assert!(
        dec.is_finished(),
        "decoder must report is_finished after exhaustion"
    );
}

// ---------------------------------------------------------------------------
// Test 18: GameSession Vec roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_game_session_vec_roundtrip() {
    let sessions = vec![
        make_game_session(1, "Dungeon Quest", 4, vec![10, 11, 12, 13]),
        make_game_session(2, "Space Racer", 6, vec![20, 21]),
        make_game_session(3, "Tower Defense", 2, vec![]),
        make_game_session(4, "Battle Royale", 100, (50..60).collect()),
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(sessions.clone().into_iter())
        .await
        .expect("write_all GameSession vec");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<GameSession> = dec.read_all().await.expect("read_all GameSession vec");
    assert_eq!(sessions, got, "GameSession vec roundtrip mismatch");
    assert_eq!(got.len(), 4, "must have 4 sessions");
}

// ---------------------------------------------------------------------------
// Test 19: Boundary values (u64::MAX score, i32::MIN delta)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_boundary_values_score_and_delta() {
    let events = vec![
        GameEvent::ScoreUpdate {
            player_id: u64::MAX,
            score: u64::MAX,
            delta: i32::MIN,
        },
        GameEvent::ScoreUpdate {
            player_id: 0,
            score: 0,
            delta: i32::MAX,
        },
        GameEvent::LevelUp {
            player_id: u64::MAX,
            new_level: u32::MAX,
        },
        GameEvent::ItemPickup {
            player_id: u64::MAX,
            item_id: u32::MAX,
            item_name: "boundary_item".to_string(),
        },
        GameEvent::GameOver {
            winner_id: u64::MAX,
            final_scores: vec![(u64::MAX, u64::MAX), (0, 0)],
        },
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(events.clone().into_iter())
        .await
        .expect("write_all boundary events");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<GameEvent> = dec.read_all().await.expect("read_all boundary events");
    assert_eq!(events, got, "boundary value roundtrip mismatch");
    // Spot-check u64::MAX and i32::MIN
    if let GameEvent::ScoreUpdate {
        player_id,
        score,
        delta,
    } = &got[0]
    {
        assert_eq!(*player_id, u64::MAX, "player_id boundary mismatch");
        assert_eq!(*score, u64::MAX, "score boundary mismatch");
        assert_eq!(*delta, i32::MIN, "delta i32::MIN boundary mismatch");
    } else {
        panic!("expected ScoreUpdate at index 0");
    }
}

// ---------------------------------------------------------------------------
// Test 20: Unicode username/message roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_unicode_username_and_message() {
    let events = vec![
        GameEvent::PlayerJoined {
            player_id: 1,
            username: "勇者アリス".to_string(),
        },
        GameEvent::ChatMessage {
            player_id: 1,
            message: "こんにちは！🎮🗡️✨".to_string(),
        },
        GameEvent::PlayerJoined {
            player_id: 2,
            username: "Ελληνικά_Ήρωας".to_string(),
        },
        GameEvent::ChatMessage {
            player_id: 2,
            message: "Привет мир! Как дела? 🌍".to_string(),
        },
        GameEvent::ItemPickup {
            player_id: 1,
            item_id: 777,
            item_name: "魔法の剣🗡️".to_string(),
        },
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(events.clone().into_iter())
        .await
        .expect("write_all unicode events");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<GameEvent> = dec.read_all().await.expect("read_all unicode events");
    assert_eq!(events, got, "unicode username/message roundtrip mismatch");
    // Spot-check multibyte content
    if let GameEvent::PlayerJoined { username, .. } = &got[0] {
        assert_eq!(username, "勇者アリス", "Japanese username mismatch");
    } else {
        panic!("expected PlayerJoined at index 0");
    }
}

// ---------------------------------------------------------------------------
// Test 21: Wrong-type decode returns Err
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_wrong_type_decode_returns_err() {
    // Encode a GameEvent::GameOver (complex layout with Vec<(u64,u64)>) then
    // attempt to decode as GameSession — the binary format will not match and
    // must return Err rather than silently succeeding.
    let event = GameEvent::GameOver {
        winner_id: 888,
        final_scores: vec![(888, 99999), (777, 88888), (666, 77777)],
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&event)
        .await
        .expect("write_item GameOver for wrong-type test");
    enc.finish().await.expect("finish");

    let mut dec = AsyncDecoder::new(server);
    // Attempt to decode as GameSession — must return Err due to layout mismatch.
    let result = dec.read_item::<GameSession>().await;
    assert!(
        result.is_err(),
        "decoding GameEvent as GameSession must return Err, got Ok({result:?})"
    );
}

// ---------------------------------------------------------------------------
// Test 22: tokio::join! concurrent encode/decode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_game22_concurrent_encode_decode_join() {
    let events = make_mixed_events(22);
    let events_for_enc = events.clone();

    let (client, server) = tokio::io::duplex(65536);

    let (_, got) = tokio::join!(
        async move {
            let mut enc = AsyncEncoder::new(client);
            enc.write_all(events_for_enc.into_iter())
                .await
                .expect("concurrent write_all");
            enc.finish().await.expect("concurrent finish");
        },
        async move {
            let mut dec = AsyncDecoder::new(server);
            dec.read_all::<GameEvent>()
                .await
                .expect("concurrent read_all")
        }
    );

    assert_eq!(events, got, "concurrent encode/decode roundtrip mismatch");
    assert_eq!(
        got.len(),
        22,
        "must have decoded all 22 events concurrently"
    );
}
