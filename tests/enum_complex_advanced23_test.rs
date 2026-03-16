//! Advanced tests for cryptocurrency exchange / DeFi protocol domain types.
//! 22 test functions covering enums, structs, configs, and edge cases.

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

#[derive(Debug, PartialEq, Encode, Decode)]
enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
    TrailingStop,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TxState {
    Pending,
    Confirmed,
    Failed,
    Reverted,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TradingPair {
    base: String,
    quote: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CryptoOrder {
    order_id: u64,
    pair: TradingPair,
    side: OrderSide,
    order_type: OrderType,
    amount_satoshi: u64,
    price_satoshi: u64,
    filled_satoshi: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LiquidityPool {
    pool_id: u64,
    pair: TradingPair,
    reserve_a: u64,
    reserve_b: u64,
    lp_tokens: u64,
    fee_bps: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DefiTx {
    tx_hash: String,
    state: TxState,
    gas_used: u64,
    block_number: u64,
    orders: Vec<CryptoOrder>,
}

// --- Test 1: OrderSide::Buy variant roundtrip ---
#[test]
fn test_order_side_buy_roundtrip() {
    let val = OrderSide::Buy;
    let bytes = encode_to_vec(&val).expect("encode OrderSide::Buy");
    let (decoded, _): (OrderSide, usize) =
        decode_from_slice(&bytes).expect("decode OrderSide::Buy");
    assert_eq!(val, decoded);
}

// --- Test 2: OrderSide::Sell variant roundtrip ---
#[test]
fn test_order_side_sell_roundtrip() {
    let val = OrderSide::Sell;
    let bytes = encode_to_vec(&val).expect("encode OrderSide::Sell");
    let (decoded, _): (OrderSide, usize) =
        decode_from_slice(&bytes).expect("decode OrderSide::Sell");
    assert_eq!(val, decoded);
}

// --- Test 3: OrderType::Market variant roundtrip ---
#[test]
fn test_order_type_market_roundtrip() {
    let val = OrderType::Market;
    let bytes = encode_to_vec(&val).expect("encode OrderType::Market");
    let (decoded, _): (OrderType, usize) =
        decode_from_slice(&bytes).expect("decode OrderType::Market");
    assert_eq!(val, decoded);
}

// --- Test 4: OrderType::Limit variant roundtrip ---
#[test]
fn test_order_type_limit_roundtrip() {
    let val = OrderType::Limit;
    let bytes = encode_to_vec(&val).expect("encode OrderType::Limit");
    let (decoded, _): (OrderType, usize) =
        decode_from_slice(&bytes).expect("decode OrderType::Limit");
    assert_eq!(val, decoded);
}

// --- Test 5: OrderType::StopLoss variant roundtrip ---
#[test]
fn test_order_type_stop_loss_roundtrip() {
    let val = OrderType::StopLoss;
    let bytes = encode_to_vec(&val).expect("encode OrderType::StopLoss");
    let (decoded, _): (OrderType, usize) =
        decode_from_slice(&bytes).expect("decode OrderType::StopLoss");
    assert_eq!(val, decoded);
}

// --- Test 6: OrderType::TakeProfit variant roundtrip ---
#[test]
fn test_order_type_take_profit_roundtrip() {
    let val = OrderType::TakeProfit;
    let bytes = encode_to_vec(&val).expect("encode OrderType::TakeProfit");
    let (decoded, _): (OrderType, usize) =
        decode_from_slice(&bytes).expect("decode OrderType::TakeProfit");
    assert_eq!(val, decoded);
}

// --- Test 7: OrderType::TrailingStop variant roundtrip ---
#[test]
fn test_order_type_trailing_stop_roundtrip() {
    let val = OrderType::TrailingStop;
    let bytes = encode_to_vec(&val).expect("encode OrderType::TrailingStop");
    let (decoded, _): (OrderType, usize) =
        decode_from_slice(&bytes).expect("decode OrderType::TrailingStop");
    assert_eq!(val, decoded);
}

// --- Test 8: TxState::Pending variant roundtrip ---
#[test]
fn test_tx_state_pending_roundtrip() {
    let val = TxState::Pending;
    let bytes = encode_to_vec(&val).expect("encode TxState::Pending");
    let (decoded, _): (TxState, usize) =
        decode_from_slice(&bytes).expect("decode TxState::Pending");
    assert_eq!(val, decoded);
}

// --- Test 9: TxState::Confirmed variant roundtrip ---
#[test]
fn test_tx_state_confirmed_roundtrip() {
    let val = TxState::Confirmed;
    let bytes = encode_to_vec(&val).expect("encode TxState::Confirmed");
    let (decoded, _): (TxState, usize) =
        decode_from_slice(&bytes).expect("decode TxState::Confirmed");
    assert_eq!(val, decoded);
}

// --- Test 10: TxState::Failed variant roundtrip ---
#[test]
fn test_tx_state_failed_roundtrip() {
    let val = TxState::Failed;
    let bytes = encode_to_vec(&val).expect("encode TxState::Failed");
    let (decoded, _): (TxState, usize) = decode_from_slice(&bytes).expect("decode TxState::Failed");
    assert_eq!(val, decoded);
}

// --- Test 11: TxState::Reverted variant roundtrip ---
#[test]
fn test_tx_state_reverted_roundtrip() {
    let val = TxState::Reverted;
    let bytes = encode_to_vec(&val).expect("encode TxState::Reverted");
    let (decoded, _): (TxState, usize) =
        decode_from_slice(&bytes).expect("decode TxState::Reverted");
    assert_eq!(val, decoded);
}

// --- Test 12: TradingPair BTC/USDT roundtrip ---
#[test]
fn test_trading_pair_btc_usdt_roundtrip() {
    let val = TradingPair {
        base: String::from("BTC"),
        quote: String::from("USDT"),
    };
    let bytes = encode_to_vec(&val).expect("encode TradingPair BTC/USDT");
    let (decoded, _): (TradingPair, usize) =
        decode_from_slice(&bytes).expect("decode TradingPair BTC/USDT");
    assert_eq!(val, decoded);
    assert_eq!(decoded.base, "BTC");
    assert_eq!(decoded.quote, "USDT");
}

// --- Test 13: CryptoOrder buy limit roundtrip ---
#[test]
fn test_crypto_order_buy_limit_roundtrip() {
    let val = CryptoOrder {
        order_id: 100_001,
        pair: TradingPair {
            base: String::from("ETH"),
            quote: String::from("USDT"),
        },
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        amount_satoshi: 500_000_000,
        price_satoshi: 3_000_00_000_000,
        filled_satoshi: 0,
    };
    let bytes = encode_to_vec(&val).expect("encode CryptoOrder buy limit");
    let (decoded, _): (CryptoOrder, usize) =
        decode_from_slice(&bytes).expect("decode CryptoOrder buy limit");
    assert_eq!(val, decoded);
    assert_eq!(decoded.side, OrderSide::Buy);
    assert_eq!(decoded.order_type, OrderType::Limit);
}

// --- Test 14: CryptoOrder sell market roundtrip ---
#[test]
fn test_crypto_order_sell_market_roundtrip() {
    let val = CryptoOrder {
        order_id: 200_002,
        pair: TradingPair {
            base: String::from("BTC"),
            quote: String::from("USDC"),
        },
        side: OrderSide::Sell,
        order_type: OrderType::Market,
        amount_satoshi: 1_000_000,
        price_satoshi: 0,
        filled_satoshi: 0,
    };
    let bytes = encode_to_vec(&val).expect("encode CryptoOrder sell market");
    let (decoded, _): (CryptoOrder, usize) =
        decode_from_slice(&bytes).expect("decode CryptoOrder sell market");
    assert_eq!(val, decoded);
    assert_eq!(decoded.side, OrderSide::Sell);
    assert_eq!(decoded.order_type, OrderType::Market);
}

// --- Test 15: LiquidityPool roundtrip ---
#[test]
fn test_liquidity_pool_roundtrip() {
    let val = LiquidityPool {
        pool_id: 42,
        pair: TradingPair {
            base: String::from("WETH"),
            quote: String::from("DAI"),
        },
        reserve_a: 10_000_000_000_000_000_000,
        reserve_b: 30_000_000_000,
        lp_tokens: 17_320_508_000,
        fee_bps: 30,
    };
    let bytes = encode_to_vec(&val).expect("encode LiquidityPool");
    let (decoded, _): (LiquidityPool, usize) =
        decode_from_slice(&bytes).expect("decode LiquidityPool");
    assert_eq!(val, decoded);
    assert_eq!(decoded.fee_bps, 30);
}

// --- Test 16: DefiTx with empty orders roundtrip ---
#[test]
fn test_defi_tx_empty_orders_roundtrip() {
    let val = DefiTx {
        tx_hash: String::from("0xabc123def456abc123def456abc123def456abc123def456abc123def456abcd"),
        state: TxState::Pending,
        gas_used: 21_000,
        block_number: 0,
        orders: vec![],
    };
    let bytes = encode_to_vec(&val).expect("encode DefiTx empty orders");
    let (decoded, _): (DefiTx, usize) =
        decode_from_slice(&bytes).expect("decode DefiTx empty orders");
    assert_eq!(val, decoded);
    assert!(decoded.orders.is_empty());
}

// --- Test 17: DefiTx with 3 orders roundtrip ---
#[test]
fn test_defi_tx_three_orders_roundtrip() {
    let make_order = |id: u64, side: OrderSide, ot: OrderType| CryptoOrder {
        order_id: id,
        pair: TradingPair {
            base: String::from("SOL"),
            quote: String::from("USDT"),
        },
        side,
        order_type: ot,
        amount_satoshi: id * 1_000_000,
        price_satoshi: 150_000_000,
        filled_satoshi: 0,
    };
    let val = DefiTx {
        tx_hash: String::from("0x111222333444555666777888999aaabbbccc"),
        state: TxState::Confirmed,
        gas_used: 150_000,
        block_number: 19_500_000,
        orders: vec![
            make_order(1, OrderSide::Buy, OrderType::Limit),
            make_order(2, OrderSide::Sell, OrderType::Market),
            make_order(3, OrderSide::Buy, OrderType::StopLoss),
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode DefiTx 3 orders");
    let (decoded, _): (DefiTx, usize) = decode_from_slice(&bytes).expect("decode DefiTx 3 orders");
    assert_eq!(val, decoded);
    assert_eq!(decoded.orders.len(), 3);
}

// --- Test 18: big_endian config roundtrip ---
#[test]
fn test_big_endian_config_roundtrip() {
    let val = CryptoOrder {
        order_id: 999_888,
        pair: TradingPair {
            base: String::from("AVAX"),
            quote: String::from("USDT"),
        },
        side: OrderSide::Buy,
        order_type: OrderType::TakeProfit,
        amount_satoshi: 2_500_000_000,
        price_satoshi: 40_000_000_000,
        filled_satoshi: 1_000_000_000,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode big endian CryptoOrder");
    let (decoded, _): (CryptoOrder, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big endian CryptoOrder");
    assert_eq!(val, decoded);
}

// --- Test 19: fixed_int config roundtrip ---
#[test]
fn test_fixed_int_config_roundtrip() {
    let val = LiquidityPool {
        pool_id: 7,
        pair: TradingPair {
            base: String::from("MATIC"),
            quote: String::from("ETH"),
        },
        reserve_a: 5_000_000_000,
        reserve_b: 2_000_000_000,
        lp_tokens: 3_162_277_660,
        fee_bps: 5,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode fixed_int LiquidityPool");
    let (decoded, _): (LiquidityPool, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed_int LiquidityPool");
    assert_eq!(val, decoded);
}

// --- Test 20: consumed bytes check ---
#[test]
fn test_consumed_bytes_check() {
    let val = TradingPair {
        base: String::from("BNB"),
        quote: String::from("BUSD"),
    };
    let bytes = encode_to_vec(&val).expect("encode TradingPair for consumed bytes check");
    let (decoded, consumed): (TradingPair, usize) =
        decode_from_slice(&bytes).expect("decode TradingPair for consumed bytes check");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// --- Test 21: Vec<CryptoOrder> roundtrip ---
#[test]
fn test_vec_crypto_order_roundtrip() {
    let orders: Vec<CryptoOrder> = (0..5)
        .map(|i| CryptoOrder {
            order_id: i,
            pair: TradingPair {
                base: format!("TOKEN{}", i),
                quote: String::from("USDT"),
            },
            side: if i % 2 == 0 {
                OrderSide::Buy
            } else {
                OrderSide::Sell
            },
            order_type: OrderType::Limit,
            amount_satoshi: (i + 1) * 100_000_000,
            price_satoshi: (i + 1) * 10_000_000_000,
            filled_satoshi: 0,
        })
        .collect();
    let bytes = encode_to_vec(&orders).expect("encode Vec<CryptoOrder>");
    let (decoded, _): (Vec<CryptoOrder>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<CryptoOrder>");
    assert_eq!(orders, decoded);
    assert_eq!(decoded.len(), 5);
}

// --- Test 22: Vec<LiquidityPool> roundtrip ---
#[test]
fn test_vec_liquidity_pool_roundtrip() {
    let pools: Vec<LiquidityPool> = (1..=4)
        .map(|i| LiquidityPool {
            pool_id: i,
            pair: TradingPair {
                base: format!("COIN{}", i),
                quote: String::from("ETH"),
            },
            reserve_a: i * 1_000_000_000,
            reserve_b: i * 2_000_000_000,
            lp_tokens: i * 1_414_213_562,
            fee_bps: (i * 10) as u16,
        })
        .collect();
    let bytes = encode_to_vec(&pools).expect("encode Vec<LiquidityPool>");
    let (decoded, _): (Vec<LiquidityPool>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<LiquidityPool>");
    assert_eq!(pools, decoded);
    assert_eq!(decoded.len(), 4);
}

// --- Test 23: fully filled order (filled == amount) ---
#[test]
fn test_fully_filled_order_roundtrip() {
    let amount = 10_000_000_000_u64;
    let val = CryptoOrder {
        order_id: 300_001,
        pair: TradingPair {
            base: String::from("BTC"),
            quote: String::from("USDT"),
        },
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        amount_satoshi: amount,
        price_satoshi: 65_000_00_000_000,
        filled_satoshi: amount,
    };
    let bytes = encode_to_vec(&val).expect("encode fully filled order");
    let (decoded, _): (CryptoOrder, usize) =
        decode_from_slice(&bytes).expect("decode fully filled order");
    assert_eq!(val, decoded);
    assert_eq!(decoded.filled_satoshi, decoded.amount_satoshi);
}

// --- Test 24: partial fill ---
#[test]
fn test_partial_fill_order_roundtrip() {
    let val = CryptoOrder {
        order_id: 400_002,
        pair: TradingPair {
            base: String::from("ETH"),
            quote: String::from("USDT"),
        },
        side: OrderSide::Sell,
        order_type: OrderType::Limit,
        amount_satoshi: 1_000_000_000,
        price_satoshi: 3_500_000_000_000,
        filled_satoshi: 400_000_000,
    };
    let bytes = encode_to_vec(&val).expect("encode partial fill order");
    let (decoded, _): (CryptoOrder, usize) =
        decode_from_slice(&bytes).expect("decode partial fill order");
    assert_eq!(val, decoded);
    assert!(decoded.filled_satoshi < decoded.amount_satoshi);
    assert_eq!(decoded.filled_satoshi, 400_000_000);
}

// --- Test 25: reverted tx roundtrip ---
#[test]
fn test_reverted_tx_roundtrip() {
    let val = DefiTx {
        tx_hash: String::from("0xdeadbeefcafebabedeadbeefcafebabedeadbeef"),
        state: TxState::Reverted,
        gas_used: 80_000,
        block_number: 20_000_001,
        orders: vec![],
    };
    let bytes = encode_to_vec(&val).expect("encode reverted tx");
    let (decoded, _): (DefiTx, usize) = decode_from_slice(&bytes).expect("decode reverted tx");
    assert_eq!(val, decoded);
    assert_eq!(decoded.state, TxState::Reverted);
}

// --- Test 26: high-fee pool (100 bps) ---
#[test]
fn test_high_fee_pool_roundtrip() {
    let val = LiquidityPool {
        pool_id: 999,
        pair: TradingPair {
            base: String::from("SHIB"),
            quote: String::from("ETH"),
        },
        reserve_a: 1_000_000_000_000_000,
        reserve_b: 50_000_000_000,
        lp_tokens: 224_000_000_000_000,
        fee_bps: 100,
    };
    let bytes = encode_to_vec(&val).expect("encode high-fee pool");
    let (decoded, _): (LiquidityPool, usize) =
        decode_from_slice(&bytes).expect("decode high-fee pool");
    assert_eq!(val, decoded);
    assert_eq!(decoded.fee_bps, 100);
}

// --- Test 27: zero reserve pool ---
#[test]
fn test_zero_reserve_pool_roundtrip() {
    let val = LiquidityPool {
        pool_id: 0,
        pair: TradingPair {
            base: String::from("NEWTOKEN"),
            quote: String::from("USDC"),
        },
        reserve_a: 0,
        reserve_b: 0,
        lp_tokens: 0,
        fee_bps: 30,
    };
    let bytes = encode_to_vec(&val).expect("encode zero reserve pool");
    let (decoded, _): (LiquidityPool, usize) =
        decode_from_slice(&bytes).expect("decode zero reserve pool");
    assert_eq!(val, decoded);
    assert_eq!(decoded.reserve_a, 0);
    assert_eq!(decoded.reserve_b, 0);
}

// --- Test 28: stop loss vs take profit produce distinct byte sequences ---
#[test]
fn test_stop_loss_vs_take_profit_distinct_bytes() {
    let stop_loss = OrderType::StopLoss;
    let take_profit = OrderType::TakeProfit;
    let bytes_sl = encode_to_vec(&stop_loss).expect("encode StopLoss");
    let bytes_tp = encode_to_vec(&take_profit).expect("encode TakeProfit");
    assert_ne!(
        bytes_sl, bytes_tp,
        "StopLoss and TakeProfit must encode to distinct bytes"
    );
}

// --- Test 29: max satoshi values roundtrip ---
#[test]
fn test_max_satoshi_values_roundtrip() {
    let val = CryptoOrder {
        order_id: u64::MAX,
        pair: TradingPair {
            base: String::from("BTC"),
            quote: String::from("USDT"),
        },
        side: OrderSide::Buy,
        order_type: OrderType::Market,
        amount_satoshi: u64::MAX,
        price_satoshi: u64::MAX,
        filled_satoshi: u64::MAX,
    };
    let bytes = encode_to_vec(&val).expect("encode max satoshi CryptoOrder");
    let (decoded, _): (CryptoOrder, usize) =
        decode_from_slice(&bytes).expect("decode max satoshi CryptoOrder");
    assert_eq!(val, decoded);
    assert_eq!(decoded.amount_satoshi, u64::MAX);
    assert_eq!(decoded.price_satoshi, u64::MAX);
    assert_eq!(decoded.filled_satoshi, u64::MAX);
}

// --- Test 30: confirmed block transaction roundtrip ---
#[test]
fn test_confirmed_block_transaction_roundtrip() {
    let val = DefiTx {
        tx_hash: String::from("0xfeedfacecafebabedeadbeef0011223344556677"),
        state: TxState::Confirmed,
        gas_used: 200_000,
        block_number: 21_000_000,
        orders: vec![CryptoOrder {
            order_id: 555_001,
            pair: TradingPair {
                base: String::from("LINK"),
                quote: String::from("ETH"),
            },
            side: OrderSide::Buy,
            order_type: OrderType::TrailingStop,
            amount_satoshi: 250_000_000,
            price_satoshi: 18_000_000_000,
            filled_satoshi: 250_000_000,
        }],
    };
    let bytes = encode_to_vec(&val).expect("encode confirmed block transaction");
    let (decoded, consumed): (DefiTx, usize) =
        decode_from_slice(&bytes).expect("decode confirmed block transaction");
    assert_eq!(val, decoded);
    assert_eq!(decoded.state, TxState::Confirmed);
    assert_eq!(decoded.block_number, 21_000_000);
    assert_eq!(consumed, bytes.len());
}
