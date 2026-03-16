#![cfg(feature = "compression-lz4")]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

fn compress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Lz4).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

// ── Domain types for HFT market data ───────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PriceLevel {
    price: u64,       // price in ticks (fixed-point, e.g. cents)
    quantity: u64,    // shares / lots
    order_count: u32, // number of resting orders at this level
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrderBookSnapshot {
    symbol: String,
    sequence_number: u64,
    exchange_ts_ns: u64,
    bids: Vec<PriceLevel>,
    asks: Vec<PriceLevel>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ExecType {
    New,
    PartialFill,
    Fill,
    Canceled,
    Replaced,
    Rejected,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TradeExecutionReport {
    exec_id: String,
    order_id: String,
    symbol: String,
    side: Side,
    exec_type: ExecType,
    exec_price: u64,
    exec_qty: u64,
    leaves_qty: u64,
    cum_qty: u64,
    transact_time_ns: u64,
    venue: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MarketMicrostructure {
    symbol: String,
    best_bid: u64,
    best_ask: u64,
    spread_bps: u32,  // basis points * 100
    bid_depth_5: u64, // total qty in top 5 bid levels
    ask_depth_5: u64,
    imbalance_ratio: u32, // (bid_depth - ask_depth) / (bid_depth + ask_depth) * 10000
    midpoint: u64,
    ts_ns: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FixNewOrderSingle {
    cl_ord_id: String,
    symbol: String,
    side: Side,
    order_qty: u64,
    price: u64,
    time_in_force: u8, // 0=Day, 1=GTC, 2=IOC, 3=FOK
    sender_comp_id: String,
    target_comp_id: String,
    sending_time_ns: u64,
    msg_seq_num: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TickData {
    symbol: String,
    ts_ns: u64,
    bid: u64,
    ask: u64,
    last_price: u64,
    last_size: u64,
    tick_direction: u8, // 0=uptick, 1=downtick, 2=zero-uptick, 3=zero-downtick
    condition_flags: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VwapCalculation {
    symbol: String,
    period_start_ns: u64,
    period_end_ns: u64,
    vwap_price: u64, // fixed-point
    total_volume: u64,
    total_notional: u64, // price * qty summed
    num_trades: u32,
    twap_price: u64,
    participation_rate: u32, // bps
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LatencyMeasurement {
    event_id: String,
    wire_to_decode_ns: u64,
    decode_to_strategy_ns: u64,
    strategy_to_order_ns: u64,
    order_to_wire_ns: u64,
    total_tick_to_trade_ns: u64,
    exchange_ack_ns: u64,
    fill_report_ns: u64,
    percentile_label: String, // e.g. "p50", "p99"
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CircuitBreakerState {
    Normal,
    LimitUpLimitDown,
    TradingHalt,
    MarketWideBreaker,
    RegulatoryHalt,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CircuitBreakerEvent {
    symbol: String,
    state: CircuitBreakerState,
    trigger_price: u64,
    reference_price: u64,
    upper_band: u64,
    lower_band: u64,
    triggered_at_ns: u64,
    expected_resume_ns: u64,
    reason: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DarkPoolPrint {
    print_id: String,
    symbol: String,
    price: u64,
    size: u64,
    reporting_venue: String,
    contra_party_type: u8, // 0=market_maker, 1=institutional, 2=retail, 3=unknown
    nbbo_mid_at_print: u64,
    price_improvement_bps: i32,
    ts_ns: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OptionsGreeks {
    symbol: String,
    underlying: String,
    strike: u64,
    expiry_yyyymmdd: u32,
    is_call: bool,
    delta: i64, // scaled by 1e8
    gamma: i64,
    theta: i64,
    vega: i64,
    rho: i64,
    implied_vol: u64, // scaled by 1e8
    theo_price: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FuturesContractSpec {
    symbol: String,
    underlying_index: String,
    contract_size: u64,
    tick_size: u64,
    tick_value: u64,
    expiry_yyyymmdd: u32,
    first_notice_day: u32,
    last_trade_day: u32,
    settlement_type: u8, // 0=cash, 1=physical
    margin_initial: u64,
    margin_maintenance: u64,
    exchange: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RiskLimitCheck {
    account_id: String,
    symbol: String,
    order_id: String,
    proposed_qty: u64,
    proposed_side: Side,
    current_position: i64,
    max_position_limit: u64,
    max_order_size: u64,
    max_notional: u64,
    current_notional: u64,
    passed: bool,
    rejection_reason: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PositionSnapshot {
    account_id: String,
    symbol: String,
    net_qty: i64,
    avg_entry_price: u64,
    current_price: u64,
    unrealized_pnl: i64,
    realized_pnl: i64,
    margin_used: u64,
    snap_ts_ns: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PnlCalculation {
    account_id: String,
    trading_day: u32,
    gross_pnl: i64,
    commissions: u64,
    fees: u64,
    net_pnl: i64,
    num_trades: u32,
    win_count: u32,
    loss_count: u32,
    largest_win: i64,
    largest_loss: i64,
    sharpe_scaled: i64, // sharpe * 10000
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MarketMakerQuote {
    quote_id: String,
    symbol: String,
    bid_price: u64,
    bid_size: u64,
    ask_price: u64,
    ask_size: u64,
    quote_condition: u8, // 0=firm, 1=indicative, 2=closed
    max_show_qty: u64,
    reserve_qty: u64,
    ts_ns: u64,
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[test]
fn test_order_book_snapshot_roundtrip() {
    let book = OrderBookSnapshot {
        symbol: "AAPL".into(),
        sequence_number: 9_000_000_001,
        exchange_ts_ns: 1_710_500_000_000_000_000,
        bids: vec![
            PriceLevel {
                price: 17325,
                quantity: 400,
                order_count: 12,
            },
            PriceLevel {
                price: 17320,
                quantity: 1200,
                order_count: 38,
            },
            PriceLevel {
                price: 17315,
                quantity: 3500,
                order_count: 74,
            },
        ],
        asks: vec![
            PriceLevel {
                price: 17330,
                quantity: 350,
                order_count: 9,
            },
            PriceLevel {
                price: 17335,
                quantity: 900,
                order_count: 27,
            },
            PriceLevel {
                price: 17340,
                quantity: 2800,
                order_count: 61,
            },
        ],
    };
    let enc = encode_to_vec(&book).expect("encode order book");
    let compressed = compress_lz4(&enc).expect("compress order book");
    let decompressed = decompress_lz4(&compressed).expect("decompress order book");
    let (decoded, _): (OrderBookSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode order book");
    assert_eq!(book, decoded);
}

#[test]
fn test_trade_execution_report_roundtrip() {
    let report = TradeExecutionReport {
        exec_id: "EX-20260315-000001".into(),
        order_id: "ORD-HFT-44812".into(),
        symbol: "MSFT".into(),
        side: Side::Buy,
        exec_type: ExecType::Fill,
        exec_price: 42050,
        exec_qty: 100,
        leaves_qty: 0,
        cum_qty: 100,
        transact_time_ns: 1_710_500_100_000_000_000,
        venue: "XNYS".into(),
    };
    let enc = encode_to_vec(&report).expect("encode exec report");
    let compressed = compress_lz4(&enc).expect("compress exec report");
    let decompressed = decompress_lz4(&compressed).expect("decompress exec report");
    let (decoded, _): (TradeExecutionReport, usize) =
        decode_from_slice(&decompressed).expect("decode exec report");
    assert_eq!(report, decoded);
}

#[test]
fn test_market_microstructure_roundtrip() {
    let micro = MarketMicrostructure {
        symbol: "NVDA".into(),
        best_bid: 87550,
        best_ask: 87560,
        spread_bps: 11,
        bid_depth_5: 45_000,
        ask_depth_5: 38_000,
        imbalance_ratio: 842, // tilt toward bids
        midpoint: 87555,
        ts_ns: 1_710_500_200_000_000_000,
    };
    let enc = encode_to_vec(&micro).expect("encode microstructure");
    let compressed = compress_lz4(&enc).expect("compress microstructure");
    let decompressed = decompress_lz4(&compressed).expect("decompress microstructure");
    let (decoded, _): (MarketMicrostructure, usize) =
        decode_from_slice(&decompressed).expect("decode microstructure");
    assert_eq!(micro, decoded);
}

#[test]
fn test_fix_new_order_single_roundtrip() {
    let nos = FixNewOrderSingle {
        cl_ord_id: "CL-20260315-HFT-99901".into(),
        symbol: "TSLA".into(),
        side: Side::Sell,
        order_qty: 50,
        price: 17800,
        time_in_force: 2, // IOC
        sender_comp_id: "HFTFIRM01".into(),
        target_comp_id: "XNAS".into(),
        sending_time_ns: 1_710_500_300_000_000_000,
        msg_seq_num: 1_000_042,
    };
    let enc = encode_to_vec(&nos).expect("encode FIX NOS");
    let compressed = compress_lz4(&enc).expect("compress FIX NOS");
    let decompressed = decompress_lz4(&compressed).expect("decompress FIX NOS");
    let (decoded, _): (FixNewOrderSingle, usize) =
        decode_from_slice(&decompressed).expect("decode FIX NOS");
    assert_eq!(nos, decoded);
}

#[test]
fn test_tick_data_batch_roundtrip() {
    let ticks: Vec<TickData> = (0..100)
        .map(|i| TickData {
            symbol: "SPY".into(),
            ts_ns: 1_710_500_400_000_000_000 + i * 1_000_000, // 1ms apart
            bid: 50100 + (i % 5) as u64,
            ask: 50105 + (i % 5) as u64,
            last_price: 50102 + (i % 3) as u64,
            last_size: 100 + (i % 10) as u64 * 50,
            tick_direction: (i % 4) as u8,
            condition_flags: 0x0001,
        })
        .collect();
    let enc = encode_to_vec(&ticks).expect("encode tick batch");
    let compressed = compress_lz4(&enc).expect("compress tick batch");
    let decompressed = decompress_lz4(&compressed).expect("decompress tick batch");
    let (decoded, _): (Vec<TickData>, usize) =
        decode_from_slice(&decompressed).expect("decode tick batch");
    assert_eq!(ticks, decoded);
}

#[test]
fn test_vwap_calculation_roundtrip() {
    let vwap = VwapCalculation {
        symbol: "AMZN".into(),
        period_start_ns: 1_710_496_200_000_000_000,
        period_end_ns: 1_710_519_600_000_000_000,
        vwap_price: 18542,
        total_volume: 25_000_000,
        total_notional: 463_550_000_000,
        num_trades: 142_000,
        twap_price: 18538,
        participation_rate: 350,
    };
    let enc = encode_to_vec(&vwap).expect("encode VWAP");
    let compressed = compress_lz4(&enc).expect("compress VWAP");
    let decompressed = decompress_lz4(&compressed).expect("decompress VWAP");
    let (decoded, _): (VwapCalculation, usize) =
        decode_from_slice(&decompressed).expect("decode VWAP");
    assert_eq!(vwap, decoded);
}

#[test]
fn test_latency_measurement_roundtrip() {
    let lat = LatencyMeasurement {
        event_id: "LAT-20260315-P99-001".into(),
        wire_to_decode_ns: 120,
        decode_to_strategy_ns: 85,
        strategy_to_order_ns: 310,
        order_to_wire_ns: 95,
        total_tick_to_trade_ns: 610,
        exchange_ack_ns: 4_200,
        fill_report_ns: 8_900,
        percentile_label: "p99".into(),
    };
    let enc = encode_to_vec(&lat).expect("encode latency");
    let compressed = compress_lz4(&enc).expect("compress latency");
    let decompressed = decompress_lz4(&compressed).expect("decompress latency");
    let (decoded, _): (LatencyMeasurement, usize) =
        decode_from_slice(&decompressed).expect("decode latency");
    assert_eq!(lat, decoded);
}

#[test]
fn test_circuit_breaker_event_roundtrip() {
    let cb = CircuitBreakerEvent {
        symbol: "GME".into(),
        state: CircuitBreakerState::LimitUpLimitDown,
        trigger_price: 2500,
        reference_price: 2200,
        upper_band: 2530,
        lower_band: 1870,
        triggered_at_ns: 1_710_505_000_000_000_000,
        expected_resume_ns: 1_710_505_300_000_000_000,
        reason: "LULD band breach - price exceeded upper limit".into(),
    };
    let enc = encode_to_vec(&cb).expect("encode circuit breaker");
    let compressed = compress_lz4(&enc).expect("compress circuit breaker");
    let decompressed = decompress_lz4(&compressed).expect("decompress circuit breaker");
    let (decoded, _): (CircuitBreakerEvent, usize) =
        decode_from_slice(&decompressed).expect("decode circuit breaker");
    assert_eq!(cb, decoded);
}

#[test]
fn test_dark_pool_print_roundtrip() {
    let print = DarkPoolPrint {
        print_id: "DP-20260315-XYZ-0001".into(),
        symbol: "META".into(),
        price: 50200,
        size: 15_000,
        reporting_venue: "SIGMA-X".into(),
        contra_party_type: 1,
        nbbo_mid_at_print: 50195,
        price_improvement_bps: 10,
        ts_ns: 1_710_508_000_000_000_000,
    };
    let enc = encode_to_vec(&print).expect("encode dark pool print");
    let compressed = compress_lz4(&enc).expect("compress dark pool print");
    let decompressed = decompress_lz4(&compressed).expect("decompress dark pool print");
    let (decoded, _): (DarkPoolPrint, usize) =
        decode_from_slice(&decompressed).expect("decode dark pool print");
    assert_eq!(print, decoded);
}

#[test]
fn test_options_greeks_roundtrip() {
    let greeks = OptionsGreeks {
        symbol: "AAPL260320C00175000".into(),
        underlying: "AAPL".into(),
        strike: 17500,
        expiry_yyyymmdd: 20260320,
        is_call: true,
        delta: 55_000_000,       // 0.55
        gamma: 3_200_000,        // 0.032
        theta: -12_500_000,      // -0.125
        vega: 28_000_000,        // 0.28
        rho: 5_100_000,          // 0.051
        implied_vol: 32_500_000, // 32.5%
        theo_price: 415,
    };
    let enc = encode_to_vec(&greeks).expect("encode greeks");
    let compressed = compress_lz4(&enc).expect("compress greeks");
    let decompressed = decompress_lz4(&compressed).expect("decompress greeks");
    let (decoded, _): (OptionsGreeks, usize) =
        decode_from_slice(&decompressed).expect("decode greeks");
    assert_eq!(greeks, decoded);
}

#[test]
fn test_futures_contract_spec_roundtrip() {
    let spec = FuturesContractSpec {
        symbol: "ESM6".into(),
        underlying_index: "S&P 500".into(),
        contract_size: 50,
        tick_size: 25,
        tick_value: 1250,
        expiry_yyyymmdd: 20260619,
        first_notice_day: 0, // cash-settled, N/A
        last_trade_day: 20260619,
        settlement_type: 0,
        margin_initial: 12_650_000,
        margin_maintenance: 11_500_000,
        exchange: "CME".into(),
    };
    let enc = encode_to_vec(&spec).expect("encode futures spec");
    let compressed = compress_lz4(&enc).expect("compress futures spec");
    let decompressed = decompress_lz4(&compressed).expect("decompress futures spec");
    let (decoded, _): (FuturesContractSpec, usize) =
        decode_from_slice(&decompressed).expect("decode futures spec");
    assert_eq!(spec, decoded);
}

#[test]
fn test_risk_limit_check_pass_roundtrip() {
    let check = RiskLimitCheck {
        account_id: "ACCT-HFT-001".into(),
        symbol: "GOOG".into(),
        order_id: "ORD-20260315-77001".into(),
        proposed_qty: 200,
        proposed_side: Side::Buy,
        current_position: 500,
        max_position_limit: 5000,
        max_order_size: 1000,
        max_notional: 10_000_000_00,
        current_notional: 850_000_00,
        passed: true,
        rejection_reason: String::new(),
    };
    let enc = encode_to_vec(&check).expect("encode risk check");
    let compressed = compress_lz4(&enc).expect("compress risk check");
    let decompressed = decompress_lz4(&compressed).expect("decompress risk check");
    let (decoded, _): (RiskLimitCheck, usize) =
        decode_from_slice(&decompressed).expect("decode risk check");
    assert_eq!(check, decoded);
}

#[test]
fn test_risk_limit_check_reject_roundtrip() {
    let check = RiskLimitCheck {
        account_id: "ACCT-HFT-001".into(),
        symbol: "GOOG".into(),
        order_id: "ORD-20260315-77002".into(),
        proposed_qty: 6000,
        proposed_side: Side::Buy,
        current_position: 4800,
        max_position_limit: 5000,
        max_order_size: 1000,
        max_notional: 10_000_000_00,
        current_notional: 9_800_000_00,
        passed: false,
        rejection_reason: "Exceeds max position limit and max order size".into(),
    };
    let enc = encode_to_vec(&check).expect("encode rejected risk check");
    let compressed = compress_lz4(&enc).expect("compress rejected risk check");
    let decompressed = decompress_lz4(&compressed).expect("decompress rejected risk check");
    let (decoded, _): (RiskLimitCheck, usize) =
        decode_from_slice(&decompressed).expect("decode rejected risk check");
    assert_eq!(check, decoded);
}

#[test]
fn test_position_snapshot_roundtrip() {
    let pos = PositionSnapshot {
        account_id: "ACCT-MM-007".into(),
        symbol: "TSLA".into(),
        net_qty: -300,
        avg_entry_price: 17850,
        current_price: 17800,
        unrealized_pnl: 15_000,
        realized_pnl: -4_200,
        margin_used: 535_000,
        snap_ts_ns: 1_710_510_000_000_000_000,
    };
    let enc = encode_to_vec(&pos).expect("encode position snapshot");
    let compressed = compress_lz4(&enc).expect("compress position snapshot");
    let decompressed = decompress_lz4(&compressed).expect("decompress position snapshot");
    let (decoded, _): (PositionSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode position snapshot");
    assert_eq!(pos, decoded);
}

#[test]
fn test_pnl_calculation_roundtrip() {
    let pnl = PnlCalculation {
        account_id: "ACCT-ALGO-003".into(),
        trading_day: 20260315,
        gross_pnl: 425_000,
        commissions: 18_500,
        fees: 7_200,
        net_pnl: 399_300,
        num_trades: 14_200,
        win_count: 8_100,
        loss_count: 6_100,
        largest_win: 52_000,
        largest_loss: -38_000,
        sharpe_scaled: 21_500, // 2.15 Sharpe
    };
    let enc = encode_to_vec(&pnl).expect("encode P&L");
    let compressed = compress_lz4(&enc).expect("compress P&L");
    let decompressed = decompress_lz4(&compressed).expect("decompress P&L");
    let (decoded, _): (PnlCalculation, usize) =
        decode_from_slice(&decompressed).expect("decode P&L");
    assert_eq!(pnl, decoded);
}

#[test]
fn test_market_maker_quote_roundtrip() {
    let quote = MarketMakerQuote {
        quote_id: "QT-MM-20260315-00001".into(),
        symbol: "AAPL".into(),
        bid_price: 17320,
        bid_size: 500,
        ask_price: 17325,
        ask_size: 500,
        quote_condition: 0, // firm
        max_show_qty: 100,
        reserve_qty: 400,
        ts_ns: 1_710_512_000_000_000_000,
    };
    let enc = encode_to_vec(&quote).expect("encode MM quote");
    let compressed = compress_lz4(&enc).expect("compress MM quote");
    let decompressed = decompress_lz4(&compressed).expect("decompress MM quote");
    let (decoded, _): (MarketMakerQuote, usize) =
        decode_from_slice(&decompressed).expect("decode MM quote");
    assert_eq!(quote, decoded);
}

#[test]
fn test_large_order_book_compresses_smaller() {
    // A deep order book with 200 levels each side — highly repetitive structure
    let bids: Vec<PriceLevel> = (0..200)
        .map(|i| PriceLevel {
            price: 50000 - i as u64,
            quantity: 1000 + (i % 20) as u64 * 100,
            order_count: 5 + (i % 10) as u32,
        })
        .collect();
    let asks: Vec<PriceLevel> = (0..200)
        .map(|i| PriceLevel {
            price: 50001 + i as u64,
            quantity: 800 + (i % 15) as u64 * 100,
            order_count: 3 + (i % 8) as u32,
        })
        .collect();
    let book = OrderBookSnapshot {
        symbol: "AAPL".into(),
        sequence_number: 100_000_000,
        exchange_ts_ns: 1_710_500_000_000_000_000,
        bids,
        asks,
    };
    let enc = encode_to_vec(&book).expect("encode deep book");
    let compressed = compress_lz4(&enc).expect("compress deep book");
    assert!(
        compressed.len() < enc.len(),
        "compressed {} should be < uncompressed {}",
        compressed.len(),
        enc.len()
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress deep book");
    let (decoded, _): (OrderBookSnapshot, usize) =
        decode_from_slice(&decompressed).expect("decode deep book");
    assert_eq!(book, decoded);
}

#[test]
fn test_tick_data_stream_compresses_smaller() {
    // 1000 ticks with similar structure should compress well
    let ticks: Vec<TickData> = (0..1000)
        .map(|i| TickData {
            symbol: "SPY".into(),
            ts_ns: 1_710_500_000_000_000_000 + i * 500_000, // 0.5ms apart
            bid: 50100 + (i % 10) as u64,
            ask: 50110 + (i % 10) as u64,
            last_price: 50105 + (i % 7) as u64,
            last_size: 100,
            tick_direction: (i % 2) as u8,
            condition_flags: 0x0001,
        })
        .collect();
    let enc = encode_to_vec(&ticks).expect("encode tick stream");
    let compressed = compress_lz4(&enc).expect("compress tick stream");
    assert!(
        compressed.len() < enc.len(),
        "compressed {} should be < uncompressed {}",
        compressed.len(),
        enc.len()
    );
}

#[test]
fn test_batch_execution_reports_roundtrip() {
    let reports: Vec<TradeExecutionReport> = (0..50)
        .map(|i| TradeExecutionReport {
            exec_id: format!("EX-20260315-{:06}", i),
            order_id: format!("ORD-HFT-{:05}", i),
            symbol: match i % 5 {
                0 => "AAPL",
                1 => "MSFT",
                2 => "GOOG",
                3 => "AMZN",
                _ => "META",
            }
            .into(),
            side: if i % 2 == 0 { Side::Buy } else { Side::Sell },
            exec_type: match i % 4 {
                0 => ExecType::Fill,
                1 => ExecType::PartialFill,
                2 => ExecType::New,
                _ => ExecType::Fill,
            },
            exec_price: 10000 + (i * 7) as u64,
            exec_qty: 100 + (i % 10) as u64 * 50,
            leaves_qty: if i % 4 == 1 { 50 } else { 0 },
            cum_qty: 100 + (i % 10) as u64 * 50,
            transact_time_ns: 1_710_500_000_000_000_000 + i * 2_000_000,
            venue: "XNYS".into(),
        })
        .collect();
    let enc = encode_to_vec(&reports).expect("encode batch reports");
    let compressed = compress_lz4(&enc).expect("compress batch reports");
    let decompressed = decompress_lz4(&compressed).expect("decompress batch reports");
    let (decoded, _): (Vec<TradeExecutionReport>, usize) =
        decode_from_slice(&decompressed).expect("decode batch reports");
    assert_eq!(reports, decoded);
}

#[test]
fn test_multiple_circuit_breaker_states_roundtrip() {
    let events: Vec<CircuitBreakerEvent> = vec![
        CircuitBreakerEvent {
            symbol: "AMC".into(),
            state: CircuitBreakerState::LimitUpLimitDown,
            trigger_price: 850,
            reference_price: 720,
            upper_band: 864,
            lower_band: 576,
            triggered_at_ns: 1_710_501_000_000_000_000,
            expected_resume_ns: 1_710_501_300_000_000_000,
            reason: "Price exceeded upper LULD band".into(),
        },
        CircuitBreakerEvent {
            symbol: "AMC".into(),
            state: CircuitBreakerState::TradingHalt,
            trigger_price: 900,
            reference_price: 720,
            upper_band: 864,
            lower_band: 576,
            triggered_at_ns: 1_710_501_400_000_000_000,
            expected_resume_ns: 1_710_502_200_000_000_000,
            reason: "Repeated LULD triggers escalated to halt".into(),
        },
        CircuitBreakerEvent {
            symbol: "SPY".into(),
            state: CircuitBreakerState::MarketWideBreaker,
            trigger_price: 45000,
            reference_price: 50100,
            upper_band: 0,
            lower_band: 46593,
            triggered_at_ns: 1_710_503_000_000_000_000,
            expected_resume_ns: 1_710_503_900_000_000_000,
            reason: "Level 1 MWCB triggered: S&P 500 decline > 7%".into(),
        },
    ];
    let enc = encode_to_vec(&events).expect("encode CB events");
    let compressed = compress_lz4(&enc).expect("compress CB events");
    let decompressed = decompress_lz4(&compressed).expect("decompress CB events");
    let (decoded, _): (Vec<CircuitBreakerEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode CB events");
    assert_eq!(events, decoded);
}

#[test]
fn test_options_greeks_chain_compresses_smaller() {
    // Build a chain of 50 strikes for the same underlying and expiry
    let chain: Vec<OptionsGreeks> = (0..50)
        .map(|i| {
            let strike = 15000 + i * 250;
            OptionsGreeks {
                symbol: format!("AAPL260320C{:08}", strike),
                underlying: "AAPL".into(),
                strike,
                expiry_yyyymmdd: 20260320,
                is_call: true,
                delta: 95_000_000 - i as i64 * 1_800_000,
                gamma: 1_000_000 + (i as i64 % 10) * 200_000,
                theta: -5_000_000 - (i as i64 % 5) * 1_000_000,
                vega: 20_000_000 + (i as i64 % 8) * 500_000,
                rho: 4_000_000 - i as i64 * 50_000,
                implied_vol: 25_000_000 + (i as u64 % 12) * 500_000,
                theo_price: if strike < 17500 {
                    17500 - strike
                } else {
                    50 + i * 3
                },
            }
        })
        .collect();
    let enc = encode_to_vec(&chain).expect("encode greeks chain");
    let compressed = compress_lz4(&enc).expect("compress greeks chain");
    assert!(
        compressed.len() < enc.len(),
        "compressed {} should be < uncompressed {}",
        compressed.len(),
        enc.len()
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress greeks chain");
    let (decoded, _): (Vec<OptionsGreeks>, usize) =
        decode_from_slice(&decompressed).expect("decode greeks chain");
    assert_eq!(chain, decoded);
}

#[test]
fn test_full_portfolio_snapshot_roundtrip() {
    // Combine positions and P&L for a multi-asset portfolio
    let positions: Vec<PositionSnapshot> = vec![
        PositionSnapshot {
            account_id: "ACCT-STAT-ARB-01".into(),
            symbol: "AAPL".into(),
            net_qty: 2000,
            avg_entry_price: 17300,
            current_price: 17325,
            unrealized_pnl: 50_000,
            realized_pnl: 120_000,
            margin_used: 346_500,
            snap_ts_ns: 1_710_519_000_000_000_000,
        },
        PositionSnapshot {
            account_id: "ACCT-STAT-ARB-01".into(),
            symbol: "MSFT".into(),
            net_qty: -1500,
            avg_entry_price: 42100,
            current_price: 42050,
            unrealized_pnl: 75_000,
            realized_pnl: -30_000,
            margin_used: 630_750,
            snap_ts_ns: 1_710_519_000_000_000_000,
        },
        PositionSnapshot {
            account_id: "ACCT-STAT-ARB-01".into(),
            symbol: "GOOG".into(),
            net_qty: 800,
            avg_entry_price: 15200,
            current_price: 15180,
            unrealized_pnl: -16_000,
            realized_pnl: 45_000,
            margin_used: 121_440,
            snap_ts_ns: 1_710_519_000_000_000_000,
        },
    ];
    let enc = encode_to_vec(&positions).expect("encode portfolio");
    let compressed = compress_lz4(&enc).expect("compress portfolio");
    let decompressed = decompress_lz4(&compressed).expect("decompress portfolio");
    let (decoded, _): (Vec<PositionSnapshot>, usize) =
        decode_from_slice(&decompressed).expect("decode portfolio");
    assert_eq!(positions, decoded);
}
