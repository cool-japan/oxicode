//! Advanced file I/O tests for OxiCode — cryptocurrency exchange / trading / order book domain.

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

fn tmp(name: impl AsRef<str>) -> std::path::PathBuf {
    temp_dir().join(format!("{}_{}", name.as_ref(), std::process::id()))
}

// ---------------------------------------------------------------------------
// Domain model
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLimit,
    TrailingStop,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OrderStatus {
    Pending,
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AssetClass {
    Spot,
    Futures,
    Options,
    Perpetual,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TradingPair {
    base: String,
    quote: String,
    asset_class: AssetClass,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Order {
    order_id: u64,
    trader_id: u64,
    pair: TradingPair,
    side: OrderSide,
    order_type: OrderType,
    price_sat: u64,
    quantity_sat: u64,
    status: OrderStatus,
    created_at: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Trade {
    trade_id: u64,
    buy_order_id: u64,
    sell_order_id: u64,
    price_sat: u64,
    quantity_sat: u64,
    executed_at: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrderBook {
    pair: TradingPair,
    bids: Vec<Order>,
    asks: Vec<Order>,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AccountBalance {
    trader_id: u64,
    asset: String,
    available_sat: u64,
    locked_sat: u64,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn btc_usdt_spot() -> TradingPair {
    TradingPair {
        base: "BTC".to_string(),
        quote: "USDT".to_string(),
        asset_class: AssetClass::Spot,
    }
}

fn eth_usdt_perp() -> TradingPair {
    TradingPair {
        base: "ETH".to_string(),
        quote: "USDT".to_string(),
        asset_class: AssetClass::Perpetual,
    }
}

fn make_order(
    order_id: u64,
    trader_id: u64,
    pair: TradingPair,
    side: OrderSide,
    order_type: OrderType,
    price_sat: u64,
    quantity_sat: u64,
    status: OrderStatus,
) -> Order {
    Order {
        order_id,
        trader_id,
        pair,
        side,
        order_type,
        price_sat,
        quantity_sat,
        status,
        created_at: 1_700_000_000 + order_id,
    }
}

fn make_trade(
    trade_id: u64,
    buy_order_id: u64,
    sell_order_id: u64,
    price_sat: u64,
    quantity_sat: u64,
) -> Trade {
    Trade {
        trade_id,
        buy_order_id,
        sell_order_id,
        price_sat,
        quantity_sat,
        executed_at: 1_700_100_000 + trade_id,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_order_file_roundtrip() {
    let order = make_order(
        1,
        42,
        btc_usdt_spot(),
        OrderSide::Buy,
        OrderType::Limit,
        6_500_000_000,
        100_000_000,
        OrderStatus::Open,
    );
    let path = tmp("oxicode_crypto_order_roundtrip.bin");

    encode_to_file(&order, &path).expect("encode_to_file Order failed");
    let decoded: Order = decode_from_file(&path).expect("decode_from_file Order failed");

    assert_eq!(order, decoded);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_trade_file_roundtrip() {
    let trade = make_trade(101, 1, 2, 6_500_000_000, 50_000_000);
    let path = tmp("oxicode_crypto_trade_roundtrip.bin");

    encode_to_file(&trade, &path).expect("encode_to_file Trade failed");
    let decoded: Trade = decode_from_file(&path).expect("decode_from_file Trade failed");

    assert_eq!(trade, decoded);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_account_balance_file_roundtrip() {
    let balance = AccountBalance {
        trader_id: 99,
        asset: "BTC".to_string(),
        available_sat: 500_000_000,
        locked_sat: 100_000_000,
    };
    let path = tmp("oxicode_crypto_balance_roundtrip.bin");

    encode_to_file(&balance, &path).expect("encode AccountBalance failed");
    let decoded: AccountBalance = decode_from_file(&path).expect("decode AccountBalance failed");

    assert_eq!(balance, decoded);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_order_book_file_roundtrip() {
    let bids: Vec<Order> = (0..10)
        .map(|i| {
            make_order(
                i,
                10 + i,
                btc_usdt_spot(),
                OrderSide::Buy,
                OrderType::Limit,
                6_450_000_000 - i * 1_000_000,
                100_000_000,
                OrderStatus::Open,
            )
        })
        .collect();
    let asks: Vec<Order> = (0..10)
        .map(|i| {
            make_order(
                100 + i,
                20 + i,
                btc_usdt_spot(),
                OrderSide::Sell,
                OrderType::Limit,
                6_500_000_000 + i * 1_000_000,
                100_000_000,
                OrderStatus::Open,
            )
        })
        .collect();
    let book = OrderBook {
        pair: btc_usdt_spot(),
        bids,
        asks,
        timestamp: 1_700_050_000,
    };
    let path = tmp("oxicode_crypto_orderbook_roundtrip.bin");

    encode_to_file(&book, &path).expect("encode OrderBook failed");
    let decoded: OrderBook = decode_from_file(&path).expect("decode OrderBook failed");

    assert_eq!(book, decoded);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_large_order_book_500_orders() {
    let bids: Vec<Order> = (0..300)
        .map(|i| {
            make_order(
                i,
                1000 + i,
                btc_usdt_spot(),
                OrderSide::Buy,
                OrderType::Limit,
                6_400_000_000 - i * 10_000,
                50_000_000,
                OrderStatus::Open,
            )
        })
        .collect();
    let asks: Vec<Order> = (0..250)
        .map(|i| {
            make_order(
                300 + i,
                2000 + i,
                btc_usdt_spot(),
                OrderSide::Sell,
                OrderType::Limit,
                6_500_000_000 + i * 10_000,
                50_000_000,
                OrderStatus::Open,
            )
        })
        .collect();
    assert!(bids.len() + asks.len() >= 500);

    let book = OrderBook {
        pair: btc_usdt_spot(),
        bids,
        asks,
        timestamp: 1_700_060_000,
    };
    let path = tmp("oxicode_crypto_large_orderbook.bin");

    encode_to_file(&book, &path).expect("encode large OrderBook failed");
    let decoded: OrderBook = decode_from_file(&path).expect("decode large OrderBook failed");

    assert_eq!(book.bids.len(), decoded.bids.len());
    assert_eq!(book.asks.len(), decoded.asks.len());
    assert_eq!(book, decoded);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_vec_of_trades_roundtrip() {
    let trades: Vec<Trade> = (0..200)
        .map(|i| make_trade(i, i * 2, i * 2 + 1, 6_500_000_000 + i * 500, 10_000_000))
        .collect();
    let path = tmp("oxicode_crypto_vec_trades.bin");

    encode_to_file(&trades, &path).expect("encode Vec<Trade> failed");
    let decoded: Vec<Trade> = decode_from_file(&path).expect("decode Vec<Trade> failed");

    assert_eq!(trades, decoded);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_bytes_match_encode_to_vec_for_order() {
    let order = make_order(
        77,
        88,
        eth_usdt_perp(),
        OrderSide::Sell,
        OrderType::Market,
        3_000_000_000,
        200_000_000,
        OrderStatus::Filled,
    );
    let path = tmp("oxicode_crypto_bytes_match_order.bin");

    encode_to_file(&order, &path).expect("encode_to_file for byte-match test failed");
    let file_bytes = std::fs::read(&path).expect("read file bytes failed");
    let vec_bytes = encode_to_vec(&order).expect("encode_to_vec for byte-match test failed");

    assert_eq!(
        file_bytes, vec_bytes,
        "file bytes must equal encode_to_vec bytes"
    );
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_bytes_match_encode_to_vec_for_order_book() {
    let bids: Vec<Order> = (0..5)
        .map(|i| {
            make_order(
                i,
                i,
                btc_usdt_spot(),
                OrderSide::Buy,
                OrderType::StopLoss,
                6_300_000_000 - i * 5_000,
                10_000_000,
                OrderStatus::Pending,
            )
        })
        .collect();
    let book = OrderBook {
        pair: btc_usdt_spot(),
        bids,
        asks: Vec::new(),
        timestamp: 1_700_070_000,
    };
    let path = tmp("oxicode_crypto_bytes_match_book.bin");

    encode_to_file(&book, &path).expect("encode_to_file OrderBook for byte match failed");
    let file_bytes = std::fs::read(&path).expect("read file bytes for OrderBook failed");
    let vec_bytes = encode_to_vec(&book).expect("encode_to_vec OrderBook for byte match failed");

    assert_eq!(file_bytes, vec_bytes);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_overwrite_order_file() {
    let path = tmp("oxicode_crypto_overwrite_order.bin");

    let first = make_order(
        1,
        10,
        btc_usdt_spot(),
        OrderSide::Buy,
        OrderType::Limit,
        6_000_000_000,
        100_000_000,
        OrderStatus::Open,
    );
    encode_to_file(&first, &path).expect("first encode_to_file failed");

    let second = make_order(
        2,
        20,
        eth_usdt_perp(),
        OrderSide::Sell,
        OrderType::Market,
        3_200_000_000,
        50_000_000,
        OrderStatus::Filled,
    );
    encode_to_file(&second, &path).expect("second encode_to_file (overwrite) failed");

    let decoded: Order = decode_from_file(&path).expect("decode after overwrite failed");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_error_on_missing_file() {
    let path = tmp("oxicode_crypto_nonexistent_order_xyzabc.bin");
    let result = decode_from_file::<Order>(&path);
    assert!(result.is_err(), "expected Err for missing file, got Ok");
}

#[test]
fn test_all_order_sides_roundtrip() {
    for (idx, side) in [OrderSide::Buy, OrderSide::Sell].iter().enumerate() {
        let path = tmp(format!("oxicode_crypto_side_{idx}.bin"));
        encode_to_file(side, &path).expect("encode OrderSide failed");
        let decoded: OrderSide = decode_from_file(&path).expect("decode OrderSide failed");
        assert_eq!(side, &decoded);
        std::fs::remove_file(&path).expect("cleanup failed");
    }
}

#[test]
fn test_all_order_types_roundtrip() {
    let types = [
        OrderType::Market,
        OrderType::Limit,
        OrderType::StopLoss,
        OrderType::StopLimit,
        OrderType::TrailingStop,
    ];
    for (idx, ot) in types.iter().enumerate() {
        let path = tmp(format!("oxicode_crypto_order_type_{idx}.bin"));
        encode_to_file(ot, &path).expect("encode OrderType failed");
        let decoded: OrderType = decode_from_file(&path).expect("decode OrderType failed");
        assert_eq!(ot, &decoded);
        std::fs::remove_file(&path).expect("cleanup failed");
    }
}

#[test]
fn test_all_order_statuses_roundtrip() {
    let statuses = [
        OrderStatus::Pending,
        OrderStatus::Open,
        OrderStatus::PartiallyFilled,
        OrderStatus::Filled,
        OrderStatus::Cancelled,
        OrderStatus::Rejected,
    ];
    for (idx, status) in statuses.iter().enumerate() {
        let path = tmp(format!("oxicode_crypto_status_{idx}.bin"));
        encode_to_file(status, &path).expect("encode OrderStatus failed");
        let decoded: OrderStatus = decode_from_file(&path).expect("decode OrderStatus failed");
        assert_eq!(status, &decoded);
        std::fs::remove_file(&path).expect("cleanup failed");
    }
}

#[test]
fn test_all_asset_classes_roundtrip() {
    let classes = [
        AssetClass::Spot,
        AssetClass::Futures,
        AssetClass::Options,
        AssetClass::Perpetual,
    ];
    for (idx, cls) in classes.iter().enumerate() {
        let path = tmp(format!("oxicode_crypto_asset_class_{idx}.bin"));
        encode_to_file(cls, &path).expect("encode AssetClass failed");
        let decoded: AssetClass = decode_from_file(&path).expect("decode AssetClass failed");
        assert_eq!(cls, &decoded);
        std::fs::remove_file(&path).expect("cleanup failed");
    }
}

#[test]
fn test_trading_pair_all_asset_classes() {
    let pairs = [
        TradingPair {
            base: "BTC".to_string(),
            quote: "USDT".to_string(),
            asset_class: AssetClass::Spot,
        },
        TradingPair {
            base: "ETH".to_string(),
            quote: "USDT".to_string(),
            asset_class: AssetClass::Futures,
        },
        TradingPair {
            base: "SOL".to_string(),
            quote: "USDT".to_string(),
            asset_class: AssetClass::Options,
        },
        TradingPair {
            base: "BNB".to_string(),
            quote: "USDT".to_string(),
            asset_class: AssetClass::Perpetual,
        },
    ];
    for (idx, pair) in pairs.iter().enumerate() {
        let path = tmp(format!("oxicode_crypto_trading_pair_{idx}.bin"));
        encode_to_file(pair, &path).expect("encode TradingPair failed");
        let decoded: TradingPair = decode_from_file(&path).expect("decode TradingPair failed");
        assert_eq!(pair, &decoded);
        std::fs::remove_file(&path).expect("cleanup failed");
    }
}

#[test]
fn test_partially_filled_order_roundtrip() {
    let order = make_order(
        500,
        999,
        TradingPair {
            base: "SOL".to_string(),
            quote: "BTC".to_string(),
            asset_class: AssetClass::Spot,
        },
        OrderSide::Buy,
        OrderType::Limit,
        200_000_000,
        1_000_000_000,
        OrderStatus::PartiallyFilled,
    );
    let path = tmp("oxicode_crypto_partial_fill.bin");

    encode_to_file(&order, &path).expect("encode partially filled order failed");
    let decoded: Order = decode_from_file(&path).expect("decode partially filled order failed");

    assert_eq!(order, decoded);
    assert_eq!(decoded.status, OrderStatus::PartiallyFilled);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_stop_limit_order_roundtrip() {
    let order = make_order(
        777,
        333,
        eth_usdt_perp(),
        OrderSide::Sell,
        OrderType::StopLimit,
        2_900_000_000,
        300_000_000,
        OrderStatus::Pending,
    );
    let path = tmp("oxicode_crypto_stop_limit_order.bin");

    encode_to_file(&order, &path).expect("encode StopLimit order failed");
    let decoded: Order = decode_from_file(&path).expect("decode StopLimit order failed");

    assert_eq!(order, decoded);
    assert_eq!(decoded.order_type, OrderType::StopLimit);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_trailing_stop_order_roundtrip() {
    let order = make_order(
        888,
        444,
        btc_usdt_spot(),
        OrderSide::Sell,
        OrderType::TrailingStop,
        0,
        200_000_000,
        OrderStatus::Open,
    );
    let path = tmp("oxicode_crypto_trailing_stop.bin");

    encode_to_file(&order, &path).expect("encode TrailingStop order failed");
    let decoded: Order = decode_from_file(&path).expect("decode TrailingStop order failed");

    assert_eq!(order, decoded);
    assert_eq!(decoded.order_type, OrderType::TrailingStop);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_vec_of_balances_roundtrip() {
    let assets = ["BTC", "ETH", "SOL", "BNB", "USDT", "USDC", "XRP", "ADA"];
    let balances: Vec<AccountBalance> = assets
        .iter()
        .enumerate()
        .map(|(i, asset)| AccountBalance {
            trader_id: 1,
            asset: asset.to_string(),
            available_sat: (i as u64 + 1) * 1_000_000,
            locked_sat: (i as u64) * 100_000,
        })
        .collect();
    let path = tmp("oxicode_crypto_vec_balances.bin");

    encode_to_file(&balances, &path).expect("encode Vec<AccountBalance> failed");
    let decoded: Vec<AccountBalance> =
        decode_from_file(&path).expect("decode Vec<AccountBalance> failed");

    assert_eq!(balances, decoded);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_empty_order_book_roundtrip() {
    let book = OrderBook {
        pair: btc_usdt_spot(),
        bids: Vec::new(),
        asks: Vec::new(),
        timestamp: 1_700_080_000,
    };
    let path = tmp("oxicode_crypto_empty_orderbook.bin");

    encode_to_file(&book, &path).expect("encode empty OrderBook failed");
    let decoded: OrderBook = decode_from_file(&path).expect("decode empty OrderBook failed");

    assert_eq!(book, decoded);
    assert!(decoded.bids.is_empty());
    assert!(decoded.asks.is_empty());
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_encode_to_vec_then_decode_from_slice_order() {
    let order = make_order(
        9999,
        1111,
        btc_usdt_spot(),
        OrderSide::Buy,
        OrderType::Market,
        0,
        500_000_000,
        OrderStatus::Filled,
    );

    let bytes = encode_to_vec(&order).expect("encode_to_vec Order failed");
    let (decoded, consumed): (Order, usize) =
        decode_from_slice(&bytes).expect("decode_from_slice Order failed");

    assert_eq!(order, decoded);
    assert_eq!(consumed, bytes.len(), "all bytes should be consumed");
}

#[test]
fn test_encode_to_vec_then_decode_from_slice_trade() {
    let trade = make_trade(5000, 100, 200, 6_600_000_000, 25_000_000);

    let bytes = encode_to_vec(&trade).expect("encode_to_vec Trade failed");
    let (decoded, consumed): (Trade, usize) =
        decode_from_slice(&bytes).expect("decode_from_slice Trade failed");

    assert_eq!(trade, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_options_futures_order_book() {
    let bids: Vec<Order> = (0..20)
        .map(|i| {
            make_order(
                i,
                i + 100,
                TradingPair {
                    base: "ETH".to_string(),
                    quote: "USDT".to_string(),
                    asset_class: AssetClass::Options,
                },
                OrderSide::Buy,
                OrderType::Limit,
                2_000_000_000 - i * 1_000_000,
                30_000_000,
                OrderStatus::Open,
            )
        })
        .collect();
    let asks: Vec<Order> = (0..20)
        .map(|i| {
            make_order(
                i + 1000,
                i + 200,
                TradingPair {
                    base: "ETH".to_string(),
                    quote: "USDT".to_string(),
                    asset_class: AssetClass::Futures,
                },
                OrderSide::Sell,
                OrderType::Limit,
                2_100_000_000 + i * 1_000_000,
                30_000_000,
                OrderStatus::Open,
            )
        })
        .collect();
    let book = OrderBook {
        pair: TradingPair {
            base: "ETH".to_string(),
            quote: "USDT".to_string(),
            asset_class: AssetClass::Futures,
        },
        bids,
        asks,
        timestamp: 1_700_090_000,
    };
    let path = tmp("oxicode_crypto_options_futures_book.bin");

    encode_to_file(&book, &path).expect("encode options/futures OrderBook failed");
    let decoded: OrderBook =
        decode_from_file(&path).expect("decode options/futures OrderBook failed");

    assert_eq!(book, decoded);
    assert_eq!(decoded.bids.len(), 20);
    assert_eq!(decoded.asks.len(), 20);
    std::fs::remove_file(&path).expect("cleanup failed");
}

#[test]
fn test_cancelled_rejected_orders_roundtrip() {
    let cancelled = make_order(
        2001,
        7001,
        btc_usdt_spot(),
        OrderSide::Buy,
        OrderType::Limit,
        5_900_000_000,
        100_000_000,
        OrderStatus::Cancelled,
    );
    let rejected = make_order(
        2002,
        7002,
        btc_usdt_spot(),
        OrderSide::Sell,
        OrderType::StopLoss,
        6_700_000_000,
        200_000_000,
        OrderStatus::Rejected,
    );

    let path_c = tmp("oxicode_crypto_cancelled_order.bin");
    let path_r = tmp("oxicode_crypto_rejected_order.bin");

    encode_to_file(&cancelled, &path_c).expect("encode Cancelled order failed");
    encode_to_file(&rejected, &path_r).expect("encode Rejected order failed");

    let decoded_c: Order = decode_from_file(&path_c).expect("decode Cancelled order failed");
    let decoded_r: Order = decode_from_file(&path_r).expect("decode Rejected order failed");

    assert_eq!(cancelled, decoded_c);
    assert_eq!(rejected, decoded_r);
    assert_eq!(decoded_c.status, OrderStatus::Cancelled);
    assert_eq!(decoded_r.status, OrderStatus::Rejected);

    std::fs::remove_file(&path_c).expect("cleanup cancelled failed");
    std::fs::remove_file(&path_r).expect("cleanup rejected failed");
}
