//! Advanced async streaming tests (26th set) for OxiCode.
//!
//! Theme: Stock market / trading data.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types: `TradeType`, `StockTrade`, `MarketSnapshot`.
//!
//! Coverage matrix:
//!   1:  TradeType::Buy single roundtrip via duplex
//!   2:  TradeType::Sell single roundtrip via duplex
//!   3:  TradeType::ShortSell single roundtrip via duplex
//!   4:  TradeType::Cover single roundtrip via duplex
//!   5:  StockTrade with Buy type roundtrip via duplex
//!   6:  StockTrade with ShortSell type roundtrip via duplex
//!   7:  MarketSnapshot roundtrip via duplex
//!   8:  MarketSnapshot with zero volume roundtrip
//!   9:  Five StockTrades in order via write_item / read_item
//!  10:  write_all / read_all for Vec<StockTrade> (8 items)
//!  11:  Large batch of 120 StockTrades via write_all, verify read_all
//!  12:  Mixed stream: StockTrades and MarketSnapshots separately
//!  13:  progress().items_processed > 0 after reading trades
//!  14:  StreamingConfig with chunk_size(256) forces multiple chunks
//!  15:  flush_per_item produces one chunk per StockTrade
//!  16:  Empty stream returns None on first read_item
//!  17:  is_finished() true after stream exhausted
//!  18:  bytes_processed grows after reading more trades
//!  19:  Sync encode / async decode interop for StockTrade
//!  20:  Async encode / sync decode interop for MarketSnapshot
//!  21:  Vec<TradeType> all variants roundtrip
//!  22:  tokio::join! concurrent encode/decode for trade feed replay

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
enum TradeType {
    Buy,
    Sell,
    ShortSell,
    Cover,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StockTrade {
    trade_id: u64,
    symbol: String,
    trade_type: TradeType,
    shares: u32,
    price_cents: u64,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MarketSnapshot {
    symbol: String,
    bid_cents: u64,
    ask_cents: u64,
    last_price_cents: u64,
    volume: u64,
    timestamp_ms: u64,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_trade(trade_id: u64, symbol: &str, trade_type: TradeType) -> StockTrade {
    StockTrade {
        trade_id,
        symbol: symbol.to_string(),
        trade_type,
        shares: ((trade_id % 500) as u32 + 1) * 10,
        price_cents: 100_00 + trade_id * 37,
        timestamp_ms: 1_700_000_000_000 + trade_id * 250,
    }
}

fn make_snapshot(symbol: &str, seq: u64) -> MarketSnapshot {
    MarketSnapshot {
        symbol: symbol.to_string(),
        bid_cents: 99_50 + seq * 10,
        ask_cents: 100_00 + seq * 10,
        last_price_cents: 99_75 + seq * 10,
        volume: 500_000 + seq * 1_000,
        timestamp_ms: 1_700_000_000_000 + seq * 500,
    }
}

fn make_trade_batch(count: usize) -> Vec<StockTrade> {
    let symbols = ["AAPL", "MSFT", "GOOG", "AMZN", "TSLA"];
    (0..count)
        .map(|i| {
            let trade_type = match i % 4 {
                0 => TradeType::Buy,
                1 => TradeType::Sell,
                2 => TradeType::ShortSell,
                _ => TradeType::Cover,
            };
            let symbol = symbols[i % symbols.len()];
            make_trade(i as u64, symbol, trade_type)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Test 1: TradeType::Buy single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_trade_type_buy_roundtrip() {
    let trade_type = TradeType::Buy;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&trade_type)
        .await
        .expect("write_item TradeType::Buy failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: TradeType = dec
        .read_item()
        .await
        .expect("read_item TradeType::Buy failed")
        .expect("expected Some(TradeType::Buy)");
    assert_eq!(trade_type, got, "TradeType::Buy roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 2: TradeType::Sell single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_trade_type_sell_roundtrip() {
    let trade_type = TradeType::Sell;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&trade_type)
        .await
        .expect("write_item TradeType::Sell failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: TradeType = dec
        .read_item()
        .await
        .expect("read_item TradeType::Sell failed")
        .expect("expected Some(TradeType::Sell)");
    assert_eq!(trade_type, got, "TradeType::Sell roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 3: TradeType::ShortSell single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_trade_type_short_sell_roundtrip() {
    let trade_type = TradeType::ShortSell;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&trade_type)
        .await
        .expect("write_item TradeType::ShortSell failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: TradeType = dec
        .read_item()
        .await
        .expect("read_item TradeType::ShortSell failed")
        .expect("expected Some(TradeType::ShortSell)");
    assert_eq!(trade_type, got, "TradeType::ShortSell roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 4: TradeType::Cover single roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_trade_type_cover_roundtrip() {
    let trade_type = TradeType::Cover;
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&trade_type)
        .await
        .expect("write_item TradeType::Cover failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: TradeType = dec
        .read_item()
        .await
        .expect("read_item TradeType::Cover failed")
        .expect("expected Some(TradeType::Cover)");
    assert_eq!(trade_type, got, "TradeType::Cover roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 5: StockTrade with Buy type roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_stock_trade_buy_roundtrip() {
    let trade = StockTrade {
        trade_id: 1001,
        symbol: "AAPL".to_string(),
        trade_type: TradeType::Buy,
        shares: 500,
        price_cents: 18_250,
        timestamp_ms: 1_700_000_001_000,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&trade)
        .await
        .expect("write_item StockTrade(Buy) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: StockTrade = dec
        .read_item()
        .await
        .expect("read_item StockTrade(Buy) failed")
        .expect("expected Some(StockTrade)");
    assert_eq!(trade, got, "StockTrade with Buy type roundtrip mismatch");
    assert_eq!(got.trade_type, TradeType::Buy, "trade_type must be Buy");
    assert_eq!(got.shares, 500, "shares must be 500");
}

// ---------------------------------------------------------------------------
// Test 6: StockTrade with ShortSell type roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_stock_trade_short_sell_roundtrip() {
    let trade = StockTrade {
        trade_id: 2002,
        symbol: "TSLA".to_string(),
        trade_type: TradeType::ShortSell,
        shares: 200,
        price_cents: 24_500,
        timestamp_ms: 1_700_000_002_000,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&trade)
        .await
        .expect("write_item StockTrade(ShortSell) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: StockTrade = dec
        .read_item()
        .await
        .expect("read_item StockTrade(ShortSell) failed")
        .expect("expected Some(StockTrade)");
    assert_eq!(
        trade, got,
        "StockTrade with ShortSell type roundtrip mismatch"
    );
    assert_eq!(
        got.trade_type,
        TradeType::ShortSell,
        "trade_type must be ShortSell"
    );
}

// ---------------------------------------------------------------------------
// Test 7: MarketSnapshot roundtrip via duplex
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_market_snapshot_roundtrip() {
    let snapshot = MarketSnapshot {
        symbol: "MSFT".to_string(),
        bid_cents: 37_450,
        ask_cents: 37_455,
        last_price_cents: 37_452,
        volume: 8_500_000,
        timestamp_ms: 1_700_000_003_000,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&snapshot)
        .await
        .expect("write_item MarketSnapshot failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: MarketSnapshot = dec
        .read_item()
        .await
        .expect("read_item MarketSnapshot failed")
        .expect("expected Some(MarketSnapshot)");
    assert_eq!(snapshot, got, "MarketSnapshot roundtrip mismatch");
    assert_eq!(got.symbol, "MSFT", "symbol must be MSFT");
    assert!(
        got.ask_cents > got.bid_cents,
        "ask must be greater than bid"
    );
}

// ---------------------------------------------------------------------------
// Test 8: MarketSnapshot with zero volume roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_market_snapshot_zero_volume_roundtrip() {
    let snapshot = MarketSnapshot {
        symbol: "GOOG".to_string(),
        bid_cents: 0,
        ask_cents: 0,
        last_price_cents: 0,
        volume: 0,
        timestamp_ms: 0,
    };
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&snapshot)
        .await
        .expect("write_item MarketSnapshot(zero volume) failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: MarketSnapshot = dec
        .read_item()
        .await
        .expect("read_item MarketSnapshot(zero volume) failed")
        .expect("expected Some(MarketSnapshot) with zero volume");
    assert_eq!(
        snapshot, got,
        "MarketSnapshot with zero volume roundtrip mismatch"
    );
    assert_eq!(got.volume, 0, "volume must be zero");
    assert_eq!(got.bid_cents, 0, "bid_cents must be zero");
}

// ---------------------------------------------------------------------------
// Test 9: Five StockTrades in order via write_item / read_item
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_five_trades_in_order() {
    let trades = vec![
        make_trade(10, "AAPL", TradeType::Buy),
        make_trade(11, "MSFT", TradeType::Sell),
        make_trade(12, "GOOG", TradeType::ShortSell),
        make_trade(13, "AMZN", TradeType::Cover),
        make_trade(14, "TSLA", TradeType::Buy),
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    for trade in &trades {
        enc.write_item(trade)
            .await
            .expect("write_item in 5-trade sequence failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    for expected in &trades {
        let got: StockTrade = dec
            .read_item()
            .await
            .expect("read_item in 5-trade sequence failed")
            .expect("expected Some(StockTrade)");
        assert_eq!(
            *expected, got,
            "StockTrade mismatch at trade_id {}",
            expected.trade_id
        );
    }

    let eof: Option<StockTrade> = dec.read_item().await.expect("eof read_item failed");
    assert_eq!(eof, None, "expected None after all five trades");
}

// ---------------------------------------------------------------------------
// Test 10: write_all / read_all for Vec<StockTrade> (8 items)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_write_all_read_all_8_trades() {
    let trades: Vec<StockTrade> = (0u64..8)
        .map(|i| {
            let trade_type = match i % 4 {
                0 => TradeType::Buy,
                1 => TradeType::Sell,
                2 => TradeType::ShortSell,
                _ => TradeType::Cover,
            };
            make_trade(i, "AAPL", trade_type)
        })
        .collect();

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(trades.clone().into_iter())
        .await
        .expect("write_all 8 StockTrades failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<StockTrade> = dec.read_all().await.expect("read_all 8 StockTrades failed");
    assert_eq!(trades, got, "write_all/read_all 8-trade roundtrip mismatch");
    assert_eq!(got.len(), 8, "must decode exactly 8 StockTrades");
}

// ---------------------------------------------------------------------------
// Test 11: Large batch of 120 StockTrades via write_all, verify read_all
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_large_batch_120_trades_write_all_read_all() {
    let trades = make_trade_batch(120);
    assert_eq!(trades.len(), 120, "must generate exactly 120 trades");

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(trades.clone().into_iter())
        .await
        .expect("write_all 120 StockTrades failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<StockTrade> = dec
        .read_all()
        .await
        .expect("read_all 120 StockTrades failed");
    assert_eq!(got.len(), 120, "expected 120 decoded StockTrades");
    assert_eq!(trades, got, "large batch 120-trade roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 12: Mixed stream: StockTrades and MarketSnapshots separately
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_market_snapshot_stream_roundtrip() {
    let snapshots: Vec<MarketSnapshot> = (0u64..5).map(|i| make_snapshot("AMZN", i)).collect();

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(snapshots.clone().into_iter())
        .await
        .expect("write_all MarketSnapshots failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<MarketSnapshot> = dec
        .read_all()
        .await
        .expect("read_all MarketSnapshots failed");
    assert_eq!(got.len(), 5, "must decode exactly 5 MarketSnapshots");
    assert_eq!(snapshots, got, "MarketSnapshot stream roundtrip mismatch");

    // Verify ordering and spread invariants
    for snap in &got {
        assert!(
            snap.ask_cents >= snap.bid_cents,
            "ask must be >= bid in snapshot for symbol {}",
            snap.symbol
        );
    }
}

// ---------------------------------------------------------------------------
// Test 13: progress().items_processed > 0 after reading trades
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_progress_items_processed_after_reading_trades() {
    const N: u64 = 9;
    let trades = make_trade_batch(N as usize);

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.set_estimated_total(N);
    enc.write_all(trades.clone().into_iter())
        .await
        .expect("write_all for progress test failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let _: Vec<StockTrade> = dec
        .read_all()
        .await
        .expect("read_all for progress test failed");

    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after reading trades"
    );
    assert_eq!(
        dec.progress().items_processed,
        N,
        "items_processed must equal N={N} after reading all trades"
    );
}

// ---------------------------------------------------------------------------
// Test 14: StreamingConfig with chunk_size(256) forces multiple chunks
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_streaming_config_small_chunk_forces_multiple_chunks() {
    let config = StreamingConfig::new().with_chunk_size(256);
    // Each StockTrade is ~40-60 bytes; 50 trades ~2000-3000 bytes → multiple 256-byte chunks
    let trades = make_trade_batch(50);

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for trade in &trades {
        enc.write_item(trade)
            .await
            .expect("write_item with chunk_size 256 failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<StockTrade> = dec
        .read_all()
        .await
        .expect("read_all with chunk_size 256 failed");

    assert_eq!(got.len(), 50, "must decode 50 StockTrades");
    assert_eq!(trades, got, "small-chunk roundtrip mismatch");
    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after reading with small chunk size"
    );
}

// ---------------------------------------------------------------------------
// Test 15: flush_per_item produces one chunk per StockTrade
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_flush_per_item_one_chunk_per_trade() {
    let config = StreamingConfig::new().with_flush_per_item(true);
    let trades: Vec<StockTrade> = (0u64..6)
        .map(|i| make_trade(i, "GOOG", TradeType::Buy))
        .collect();

    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::with_config(client, config);
    for trade in &trades {
        enc.write_item(trade)
            .await
            .expect("write_item flush_per_item failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<StockTrade> = dec
        .read_all()
        .await
        .expect("read_all flush_per_item failed");

    assert_eq!(got, trades, "flush_per_item roundtrip mismatch");
    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after flush_per_item read"
    );
    assert_eq!(
        dec.progress().items_processed,
        6,
        "items_processed must equal 6 after reading 6 flush_per_item trades"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Empty stream returns None on first read_item
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_empty_stream_returns_none() {
    let (client, server) = tokio::io::duplex(65536);

    let enc = AsyncEncoder::new(client);
    enc.finish().await.expect("finish empty stream failed");

    let mut dec = AsyncDecoder::new(server);
    let item: Option<StockTrade> = dec
        .read_item()
        .await
        .expect("read_item from empty stream failed");
    assert_eq!(
        item, None,
        "empty stream must return None on first read_item"
    );
}

// ---------------------------------------------------------------------------
// Test 17: is_finished() true after stream exhausted
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_is_finished_after_stream_exhausted() {
    let trades = vec![
        make_trade(1, "AAPL", TradeType::Buy),
        make_trade(2, "MSFT", TradeType::Sell),
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    for trade in &trades {
        enc.write_item(trade).await.expect("write_item failed");
    }
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);

    assert!(
        !dec.is_finished(),
        "decoder must not be finished before reading"
    );

    let _: Option<StockTrade> = dec.read_item().await.expect("read item 1 failed");
    let _: Option<StockTrade> = dec.read_item().await.expect("read item 2 failed");

    let eof: Option<StockTrade> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None at end of stream");
    assert!(
        dec.is_finished(),
        "decoder must report is_finished() after stream is exhausted"
    );
}

// ---------------------------------------------------------------------------
// Test 18: bytes_processed grows after reading more trades
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_bytes_processed_grows_with_more_trades() {
    let trades = make_trade_batch(12);
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_all(trades.clone().into_iter())
        .await
        .expect("write_all for bytes_processed test failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);

    let first: StockTrade = dec
        .read_item()
        .await
        .expect("read first StockTrade failed")
        .expect("expected Some(StockTrade) for first trade");
    assert_eq!(first, trades[0], "first decoded StockTrade mismatch");

    let bytes_after_one = dec.progress().bytes_processed;
    assert!(
        bytes_after_one > 0,
        "bytes_processed must be > 0 after reading first trade"
    );

    let rest: Vec<StockTrade> = dec
        .read_all()
        .await
        .expect("read_all remaining trades failed");
    assert_eq!(rest.len(), 11, "must decode 11 remaining trades");

    let bytes_after_all = dec.progress().bytes_processed;
    assert!(
        bytes_after_all > bytes_after_one,
        "bytes_processed must grow: was {bytes_after_one}, now {bytes_after_all}"
    );
    assert!(
        dec.progress().items_processed >= 12,
        "items_processed must be >= 12 after reading all trades"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Sync encode / async decode interop for StockTrade
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_sync_encode_async_decode_interop_stock_trade() {
    let trade = StockTrade {
        trade_id: 9_999_999,
        symbol: "TSLA".to_string(),
        trade_type: TradeType::Cover,
        shares: 10_000,
        price_cents: 27_500,
        timestamp_ms: u64::MAX / 8,
    };

    // Sync encode for consistency baseline
    let sync_bytes = encode_to_vec(&trade).expect("sync encode StockTrade failed");
    let (sync_decoded, _): (StockTrade, _) =
        decode_from_slice(&sync_bytes).expect("sync decode StockTrade failed");
    assert_eq!(trade, sync_decoded, "sync StockTrade roundtrip mismatch");

    // Async encode then async decode via duplex
    let (client, server) = tokio::io::duplex(65536);
    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&trade)
        .await
        .expect("async write_item for interop test failed");
    enc.finish().await.expect("finish for interop test failed");

    let mut dec = AsyncDecoder::new(server);
    let async_decoded: StockTrade = dec
        .read_item()
        .await
        .expect("async read_item for interop test failed")
        .expect("expected Some(StockTrade) in interop test");
    assert_eq!(trade, async_decoded, "async encode/decode interop mismatch");
    assert_eq!(
        async_decoded.trade_type,
        TradeType::Cover,
        "trade_type must be Cover after async decode"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Async encode / sync decode interop for MarketSnapshot
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_async_encode_sync_decode_interop_market_snapshot() {
    let snapshot = MarketSnapshot {
        symbol: "AMZN".to_string(),
        bid_cents: 185_00,
        ask_cents: 185_05,
        last_price_cents: 185_02,
        volume: 12_345_678,
        timestamp_ms: 1_700_999_999_000,
    };

    // Sync encode then sync decode for consistency baseline
    let sync_bytes = encode_to_vec(&snapshot).expect("sync encode MarketSnapshot failed");
    let (sync_decoded, _): (MarketSnapshot, _) =
        decode_from_slice(&sync_bytes).expect("sync decode MarketSnapshot failed");
    assert_eq!(
        snapshot, sync_decoded,
        "sync MarketSnapshot roundtrip mismatch"
    );

    // Async encode then async decode via duplex
    let (client, server) = tokio::io::duplex(65536);
    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&snapshot)
        .await
        .expect("async write_item MarketSnapshot failed");
    enc.finish().await.expect("finish MarketSnapshot failed");

    let mut dec = AsyncDecoder::new(server);
    let async_decoded: MarketSnapshot = dec
        .read_item()
        .await
        .expect("async read_item MarketSnapshot failed")
        .expect("expected Some(MarketSnapshot)");
    assert_eq!(
        snapshot, async_decoded,
        "async encode/decode MarketSnapshot interop mismatch"
    );
    assert_eq!(
        async_decoded.volume, 12_345_678,
        "decoded volume must be 12_345_678"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Vec<TradeType> all variants roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_vec_trade_type_all_variants_roundtrip() {
    let variants = vec![
        TradeType::Buy,
        TradeType::Sell,
        TradeType::ShortSell,
        TradeType::Cover,
    ];
    let (client, server) = tokio::io::duplex(65536);

    let mut enc = AsyncEncoder::new(client);
    enc.write_item(&variants)
        .await
        .expect("write_item Vec<TradeType> all variants failed");
    enc.finish().await.expect("finish failed");

    let mut dec = AsyncDecoder::new(server);
    let got: Vec<TradeType> = dec
        .read_item()
        .await
        .expect("read_item Vec<TradeType> all variants failed")
        .expect("expected Some(Vec<TradeType>)");
    assert_eq!(
        variants, got,
        "Vec<TradeType> all-variants roundtrip mismatch"
    );
    assert_eq!(got.len(), 4, "decoded Vec<TradeType> must have 4 variants");
    assert_eq!(got[0], TradeType::Buy, "first variant must be Buy");
    assert_eq!(got[3], TradeType::Cover, "last variant must be Cover");
}

// ---------------------------------------------------------------------------
// Test 22: tokio::join! concurrent encode/decode for trade feed replay
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_trade26_concurrent_encode_decode_trade_feed_replay() {
    let trades = make_trade_batch(22);
    let trades_for_enc = trades.clone();

    let (client, server) = tokio::io::duplex(65536);

    let (_, got) = tokio::join!(
        async move {
            let mut enc = AsyncEncoder::new(client);
            enc.write_all(trades_for_enc.into_iter())
                .await
                .expect("concurrent write_all trade feed failed");
            enc.finish().await.expect("concurrent finish failed");
        },
        async move {
            let mut dec = AsyncDecoder::new(server);
            let decoded: Vec<StockTrade> = dec
                .read_all()
                .await
                .expect("concurrent read_all trade feed failed");
            decoded
        }
    );

    assert_eq!(
        got.len(),
        22,
        "must decode 22 trades from concurrent stream"
    );
    assert_eq!(
        trades, got,
        "concurrent trade feed replay roundtrip mismatch"
    );
}
