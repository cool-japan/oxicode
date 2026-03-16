//! Advanced file I/O tests for financial trading / market data domain.
//!
//! Covers Quote, OHLCV, MarketSummary structs across all AssetClass variants,
//! round-trip consistency, overwrite semantics, error handling, large data, etc.

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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AssetClass {
    Equity,
    Bond,
    Commodity,
    Forex,
    Crypto,
    Derivative,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Quote {
    symbol: String,
    asset_class: AssetClass,
    bid: f64,
    ask: f64,
    volume: u64,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OHLCV {
    symbol: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u64,
    period_start: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MarketSummary {
    exchange_id: u32,
    quotes: Vec<Quote>,
    ohlcv_bars: Vec<OHLCV>,
    session_time: u64,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_quote(symbol: &str, asset_class: AssetClass, bid: f64, ask: f64) -> Quote {
    Quote {
        symbol: symbol.to_string(),
        asset_class,
        bid,
        ask,
        volume: 100_000,
        timestamp: 1_700_000_000,
    }
}

fn make_ohlcv(symbol: &str, open: f64, close: f64) -> OHLCV {
    OHLCV {
        symbol: symbol.to_string(),
        open,
        high: open.max(close) + 0.5,
        low: open.min(close) - 0.5,
        close,
        volume: 250_000,
        period_start: 1_700_000_000,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Basic Quote write/read round-trip (Equity)
// ---------------------------------------------------------------------------
#[test]
fn test_quote_equity_roundtrip() {
    let quote = make_quote("AAPL", AssetClass::Equity, 189.50, 189.55);
    let path = temp_dir().join("oxicode_trading_001.bin");

    encode_to_file(&quote, &path).expect("encode equity quote");
    let decoded: Quote = decode_from_file(&path).expect("decode equity quote");

    assert_eq!(quote, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 2: Basic OHLCV write/read round-trip
// ---------------------------------------------------------------------------
#[test]
fn test_ohlcv_roundtrip() {
    let bar = make_ohlcv("SPY", 445.10, 448.30);
    let path = temp_dir().join("oxicode_trading_002.bin");

    encode_to_file(&bar, &path).expect("encode OHLCV bar");
    let decoded: OHLCV = decode_from_file(&path).expect("decode OHLCV bar");

    assert_eq!(bar, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 3: AssetClass::Bond variant persists correctly
// ---------------------------------------------------------------------------
#[test]
fn test_asset_class_bond() {
    let quote = make_quote("US10Y", AssetClass::Bond, 4.255, 4.260);
    let path = temp_dir().join("oxicode_trading_003.bin");

    encode_to_file(&quote, &path).expect("encode bond quote");
    let decoded: Quote = decode_from_file(&path).expect("decode bond quote");

    assert_eq!(decoded.asset_class, AssetClass::Bond);
    assert_eq!(quote, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 4: AssetClass::Commodity variant persists correctly
// ---------------------------------------------------------------------------
#[test]
fn test_asset_class_commodity() {
    let quote = make_quote("GOLD", AssetClass::Commodity, 1975.00, 1975.50);
    let path = temp_dir().join("oxicode_trading_004.bin");

    encode_to_file(&quote, &path).expect("encode commodity quote");
    let decoded: Quote = decode_from_file(&path).expect("decode commodity quote");

    assert_eq!(decoded.asset_class, AssetClass::Commodity);
    assert_eq!(quote, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 5: AssetClass::Forex variant persists correctly
// ---------------------------------------------------------------------------
#[test]
fn test_asset_class_forex() {
    let quote = make_quote("EURUSD", AssetClass::Forex, 1.08540, 1.08545);
    let path = temp_dir().join("oxicode_trading_005.bin");

    encode_to_file(&quote, &path).expect("encode forex quote");
    let decoded: Quote = decode_from_file(&path).expect("decode forex quote");

    assert_eq!(decoded.asset_class, AssetClass::Forex);
    assert_eq!(quote, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 6: AssetClass::Crypto variant persists correctly
// ---------------------------------------------------------------------------
#[test]
fn test_asset_class_crypto() {
    let quote = make_quote("BTCUSD", AssetClass::Crypto, 43_200.00, 43_205.00);
    let path = temp_dir().join("oxicode_trading_006.bin");

    encode_to_file(&quote, &path).expect("encode crypto quote");
    let decoded: Quote = decode_from_file(&path).expect("decode crypto quote");

    assert_eq!(decoded.asset_class, AssetClass::Crypto);
    assert_eq!(quote, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 7: AssetClass::Derivative variant persists correctly
// ---------------------------------------------------------------------------
#[test]
fn test_asset_class_derivative() {
    let quote = make_quote("ES_FUT", AssetClass::Derivative, 4725.25, 4725.50);
    let path = temp_dir().join("oxicode_trading_007.bin");

    encode_to_file(&quote, &path).expect("encode derivative quote");
    let decoded: Quote = decode_from_file(&path).expect("decode derivative quote");

    assert_eq!(decoded.asset_class, AssetClass::Derivative);
    assert_eq!(quote, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 8: MarketSummary with multiple quotes round-trip
// ---------------------------------------------------------------------------
#[test]
fn test_market_summary_roundtrip() {
    let summary = MarketSummary {
        exchange_id: 1,
        quotes: vec![
            make_quote("AAPL", AssetClass::Equity, 189.50, 189.55),
            make_quote("MSFT", AssetClass::Equity, 375.20, 375.25),
            make_quote("EURUSD", AssetClass::Forex, 1.0854, 1.0855),
        ],
        ohlcv_bars: vec![
            make_ohlcv("AAPL", 188.00, 190.10),
            make_ohlcv("MSFT", 374.00, 376.50),
        ],
        session_time: 1_700_010_000,
    };
    let path = temp_dir().join("oxicode_trading_008.bin");

    encode_to_file(&summary, &path).expect("encode market summary");
    let decoded: MarketSummary = decode_from_file(&path).expect("decode market summary");

    assert_eq!(summary, decoded);
    assert_eq!(decoded.quotes.len(), 3);
    assert_eq!(decoded.ohlcv_bars.len(), 2);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 9: File bytes match encode_to_vec output for Quote
// ---------------------------------------------------------------------------
#[test]
fn test_quote_file_bytes_match_vec() {
    let quote = make_quote("NVDA", AssetClass::Equity, 620.10, 620.20);
    let path = temp_dir().join("oxicode_trading_009.bin");

    encode_to_file(&quote, &path).expect("encode to file");
    let file_bytes = std::fs::read(&path).expect("read file bytes");
    let vec_bytes = encode_to_vec(&quote).expect("encode to vec");

    assert_eq!(file_bytes, vec_bytes);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 10: File bytes match encode_to_vec output for MarketSummary
// ---------------------------------------------------------------------------
#[test]
fn test_market_summary_file_bytes_match_vec() {
    let summary = MarketSummary {
        exchange_id: 42,
        quotes: vec![make_quote("GS", AssetClass::Equity, 380.0, 380.5)],
        ohlcv_bars: vec![make_ohlcv("GS", 378.0, 381.0)],
        session_time: 1_700_020_000,
    };
    let path = temp_dir().join("oxicode_trading_010.bin");

    encode_to_file(&summary, &path).expect("encode to file");
    let file_bytes = std::fs::read(&path).expect("read file bytes");
    let vec_bytes = encode_to_vec(&summary).expect("encode to vec");

    assert_eq!(file_bytes, vec_bytes);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 11: Overwrite existing file with new Quote
// ---------------------------------------------------------------------------
#[test]
fn test_overwrite_quote_file() {
    let path = temp_dir().join("oxicode_trading_011.bin");

    let first = make_quote("IBM", AssetClass::Equity, 160.00, 160.05);
    encode_to_file(&first, &path).expect("first encode");

    let second = make_quote("HPQ", AssetClass::Equity, 30.10, 30.15);
    encode_to_file(&second, &path).expect("second encode (overwrite)");

    let decoded: Quote = decode_from_file(&path).expect("decode after overwrite");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 12: Error returned when decoding from a missing file
// ---------------------------------------------------------------------------
#[test]
fn test_error_on_missing_file() {
    let path = temp_dir().join("oxicode_trading_012_nonexistent.bin");
    // ensure it does not exist
    std::fs::remove_file(&path).ok();

    let result = decode_from_file::<Quote>(&path);
    assert!(result.is_err(), "expected error for missing file");
}

// ---------------------------------------------------------------------------
// Test 13: Multiple independent files don't interfere with each other
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_independent_files() {
    let equity_path = temp_dir().join("oxicode_trading_013a.bin");
    let bond_path = temp_dir().join("oxicode_trading_013b.bin");
    let forex_path = temp_dir().join("oxicode_trading_013c.bin");

    let equity = make_quote("TSLA", AssetClass::Equity, 240.00, 240.10);
    let bond = make_quote("TLT", AssetClass::Bond, 95.00, 95.05);
    let forex = make_quote("USDJPY", AssetClass::Forex, 147.50, 147.52);

    encode_to_file(&equity, &equity_path).expect("encode equity");
    encode_to_file(&bond, &bond_path).expect("encode bond");
    encode_to_file(&forex, &forex_path).expect("encode forex");

    let d_equity: Quote = decode_from_file(&equity_path).expect("decode equity");
    let d_bond: Quote = decode_from_file(&bond_path).expect("decode bond");
    let d_forex: Quote = decode_from_file(&forex_path).expect("decode forex");

    assert_eq!(equity, d_equity);
    assert_eq!(bond, d_bond);
    assert_eq!(forex, d_forex);
    assert_eq!(d_equity.asset_class, AssetClass::Equity);
    assert_eq!(d_bond.asset_class, AssetClass::Bond);
    assert_eq!(d_forex.asset_class, AssetClass::Forex);

    std::fs::remove_file(&equity_path).ok();
    std::fs::remove_file(&bond_path).ok();
    std::fs::remove_file(&forex_path).ok();
}

// ---------------------------------------------------------------------------
// Test 14: Negative price handling (short-sell / inverted spread edge case)
// ---------------------------------------------------------------------------
#[test]
fn test_negative_price_handling() {
    // Natural gas futures can go negative; test that negative f64 survives
    let quote = Quote {
        symbol: "NG_FUT".to_string(),
        asset_class: AssetClass::Commodity,
        bid: -5.50,
        ask: -5.45,
        volume: 5_000,
        timestamp: 1_588_291_200, // Apr 30, 2020 (historic negative event)
    };
    let path = temp_dir().join("oxicode_trading_014.bin");

    encode_to_file(&quote, &path).expect("encode negative-price quote");
    let decoded: Quote = decode_from_file(&path).expect("decode negative-price quote");

    assert_eq!(quote, decoded);
    assert!(decoded.bid < 0.0);
    assert!(decoded.ask < 0.0);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 15: Large quotes Vec (500 quotes) in MarketSummary
// ---------------------------------------------------------------------------
#[test]
fn test_large_quotes_vec_500() {
    let quotes: Vec<Quote> = (0u64..500)
        .map(|i| Quote {
            symbol: format!("SYM{:04}", i),
            asset_class: match i % 6 {
                0 => AssetClass::Equity,
                1 => AssetClass::Bond,
                2 => AssetClass::Commodity,
                3 => AssetClass::Forex,
                4 => AssetClass::Crypto,
                _ => AssetClass::Derivative,
            },
            bid: 100.0 + i as f64 * 0.01,
            ask: 100.0 + i as f64 * 0.01 + 0.05,
            volume: 1_000 + i * 10,
            timestamp: 1_700_000_000 + i,
        })
        .collect();

    let summary = MarketSummary {
        exchange_id: 99,
        quotes: quotes.clone(),
        ohlcv_bars: vec![],
        session_time: 1_700_000_000,
    };
    let path = temp_dir().join("oxicode_trading_015.bin");

    encode_to_file(&summary, &path).expect("encode large quotes");
    let decoded: MarketSummary = decode_from_file(&path).expect("decode large quotes");

    assert_eq!(decoded.quotes.len(), 500);
    assert_eq!(decoded.quotes[0], quotes[0]);
    assert_eq!(decoded.quotes[499], quotes[499]);
    assert_eq!(summary, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 16: OHLCV with maximum f64 values
// ---------------------------------------------------------------------------
#[test]
fn test_ohlcv_extreme_values() {
    let bar = OHLCV {
        symbol: "EXTREME".to_string(),
        open: f64::MAX / 2.0,
        high: f64::MAX,
        low: f64::MIN_POSITIVE,
        close: 1.0e200,
        volume: u64::MAX,
        period_start: u64::MAX,
    };
    let path = temp_dir().join("oxicode_trading_016.bin");

    encode_to_file(&bar, &path).expect("encode extreme OHLCV");
    let decoded: OHLCV = decode_from_file(&path).expect("decode extreme OHLCV");

    assert_eq!(bar.open.to_bits(), decoded.open.to_bits());
    assert_eq!(bar.high.to_bits(), decoded.high.to_bits());
    assert_eq!(bar.low.to_bits(), decoded.low.to_bits());
    assert_eq!(bar.close.to_bits(), decoded.close.to_bits());
    assert_eq!(bar.volume, decoded.volume);
    assert_eq!(bar.period_start, decoded.period_start);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 17: Round-trip via encode_to_vec / decode_from_slice for Quote
// ---------------------------------------------------------------------------
#[test]
fn test_quote_vec_slice_roundtrip() {
    let quote = make_quote("META", AssetClass::Equity, 345.00, 345.10);

    let bytes = encode_to_vec(&quote).expect("encode_to_vec");
    let (decoded, consumed): (Quote, usize) = decode_from_slice(&bytes).expect("decode_from_slice");

    assert_eq!(quote, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 18: MarketSummary with zero quotes and zero OHLCV bars
// ---------------------------------------------------------------------------
#[test]
fn test_market_summary_empty_collections() {
    let summary = MarketSummary {
        exchange_id: 7,
        quotes: vec![],
        ohlcv_bars: vec![],
        session_time: 0,
    };
    let path = temp_dir().join("oxicode_trading_018.bin");

    encode_to_file(&summary, &path).expect("encode empty summary");
    let decoded: MarketSummary = decode_from_file(&path).expect("decode empty summary");

    assert_eq!(summary, decoded);
    assert!(decoded.quotes.is_empty());
    assert!(decoded.ohlcv_bars.is_empty());
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 19: All six AssetClass variants round-trip in a single Vec
// ---------------------------------------------------------------------------
#[test]
fn test_all_asset_class_variants_in_vec() {
    let quotes: Vec<Quote> = vec![
        make_quote("AAPL", AssetClass::Equity, 189.0, 189.1),
        make_quote("TLT", AssetClass::Bond, 95.0, 95.1),
        make_quote("GLD", AssetClass::Commodity, 185.0, 185.1),
        make_quote("EURUSD", AssetClass::Forex, 1.085, 1.086),
        make_quote("ETH", AssetClass::Crypto, 2_200.0, 2_200.5),
        make_quote("SPX_OPT", AssetClass::Derivative, 50.0, 50.5),
    ];

    let path = temp_dir().join("oxicode_trading_019.bin");
    encode_to_file(&quotes, &path).expect("encode all asset classes");
    let decoded: Vec<Quote> = decode_from_file(&path).expect("decode all asset classes");

    assert_eq!(decoded.len(), 6);
    assert_eq!(decoded[0].asset_class, AssetClass::Equity);
    assert_eq!(decoded[1].asset_class, AssetClass::Bond);
    assert_eq!(decoded[2].asset_class, AssetClass::Commodity);
    assert_eq!(decoded[3].asset_class, AssetClass::Forex);
    assert_eq!(decoded[4].asset_class, AssetClass::Crypto);
    assert_eq!(decoded[5].asset_class, AssetClass::Derivative);
    assert_eq!(quotes, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 20: Sequential overwrites and final state correctness
// ---------------------------------------------------------------------------
#[test]
fn test_sequential_overwrites_final_state() {
    let path = temp_dir().join("oxicode_trading_020.bin");

    let symbols = ["AMZN", "GOOG", "NFLX", "NVDA", "AMD"];
    let mut last_quote = make_quote(symbols[0], AssetClass::Equity, 100.0, 100.5);

    for (i, sym) in symbols.iter().enumerate() {
        let q = make_quote(sym, AssetClass::Equity, 100.0 + i as f64, 100.5 + i as f64);
        encode_to_file(&q, &path).expect("sequential overwrite encode");
        last_quote = q;
    }

    let decoded: Quote = decode_from_file(&path).expect("decode after sequential overwrites");
    assert_eq!(last_quote, decoded);
    assert_eq!(decoded.symbol, "AMD");
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 21: Quote with unicode symbol and special characters in string fields
// ---------------------------------------------------------------------------
#[test]
fn test_quote_unicode_symbol() {
    let quote = Quote {
        symbol: "日経平均_N225".to_string(),
        asset_class: AssetClass::Equity,
        bid: 33_500.0,
        ask: 33_505.0,
        volume: 2_000_000,
        timestamp: 1_700_050_000,
    };
    let path = temp_dir().join("oxicode_trading_021.bin");

    encode_to_file(&quote, &path).expect("encode unicode symbol");
    let decoded: Quote = decode_from_file(&path).expect("decode unicode symbol");

    assert_eq!(quote, decoded);
    assert_eq!(decoded.symbol, "日経平均_N225");
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 22: MarketSummary with large OHLCV history (100 bars) round-trip
// ---------------------------------------------------------------------------
#[test]
fn test_market_summary_large_ohlcv_history() {
    let ohlcv_bars: Vec<OHLCV> = (0u64..100)
        .map(|i| OHLCV {
            symbol: "SPY".to_string(),
            open: 440.0 + i as f64 * 0.1,
            high: 440.5 + i as f64 * 0.1,
            low: 439.5 + i as f64 * 0.1,
            close: 440.2 + i as f64 * 0.1,
            volume: 10_000_000 + i * 50_000,
            period_start: 1_699_000_000 + i * 86_400,
        })
        .collect();

    let summary = MarketSummary {
        exchange_id: 3,
        quotes: vec![make_quote("SPY", AssetClass::Equity, 449.8, 449.85)],
        ohlcv_bars: ohlcv_bars.clone(),
        session_time: 1_700_100_000,
    };
    let path = temp_dir().join("oxicode_trading_022.bin");

    encode_to_file(&summary, &path).expect("encode large OHLCV history");
    let decoded: MarketSummary = decode_from_file(&path).expect("decode large OHLCV history");

    assert_eq!(decoded.ohlcv_bars.len(), 100);
    assert_eq!(decoded.ohlcv_bars[0], ohlcv_bars[0]);
    assert_eq!(decoded.ohlcv_bars[99], ohlcv_bars[99]);
    assert_eq!(summary, decoded);
    std::fs::remove_file(&path).ok();
}
