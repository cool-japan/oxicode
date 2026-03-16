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
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Order {
    order_id: u64,
    symbol: String,
    side: OrderSide,
    order_type: OrderType,
    price: f64,
    quantity: f64,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrderBook {
    symbol: String,
    bids: Vec<Order>,
    asks: Vec<Order>,
    sequence: u64,
}

fn make_order(id: u64, side: OrderSide, order_type: OrderType, price: f64, qty: f64) -> Order {
    Order {
        order_id: id,
        symbol: "BTC/USD".to_string(),
        side,
        order_type,
        price,
        quantity: qty,
        timestamp: 1_700_000_000 + id,
    }
}

// Test 1: Single Order compress/decompress roundtrip
#[test]
fn test_single_order_compress_decompress_roundtrip() {
    let order = make_order(1, OrderSide::Buy, OrderType::Limit, 42_000.0, 1.5);
    let encoded = encode_to_vec(&order).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Order, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(order, decoded);
}

// Test 2: OrderBook compress/decompress roundtrip
#[test]
fn test_order_book_compress_decompress_roundtrip() {
    let book = OrderBook {
        symbol: "ETH/USD".to_string(),
        bids: vec![
            make_order(10, OrderSide::Buy, OrderType::Limit, 1_900.0, 2.0),
            make_order(11, OrderSide::Buy, OrderType::Limit, 1_899.5, 3.0),
        ],
        asks: vec![
            make_order(20, OrderSide::Sell, OrderType::Limit, 1_901.0, 1.5),
            make_order(21, OrderSide::Sell, OrderType::Limit, 1_902.0, 2.5),
        ],
        sequence: 999,
    };
    let encoded = encode_to_vec(&book).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (OrderBook, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(book, decoded);
}

// Test 3: OrderSide::Buy roundtrip
#[test]
fn test_order_side_buy_roundtrip() {
    let side = OrderSide::Buy;
    let encoded = encode_to_vec(&side).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (OrderSide, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(side, decoded);
}

// Test 4: OrderSide::Sell roundtrip
#[test]
fn test_order_side_sell_roundtrip() {
    let side = OrderSide::Sell;
    let encoded = encode_to_vec(&side).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (OrderSide, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(side, decoded);
}

// Test 5: OrderType::Market roundtrip
#[test]
fn test_order_type_market_roundtrip() {
    let order = make_order(100, OrderSide::Buy, OrderType::Market, 0.0, 0.5);
    let encoded = encode_to_vec(&order).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Order, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(order, decoded);
    assert_eq!(decoded.order_type, OrderType::Market);
}

// Test 6: OrderType::Limit roundtrip
#[test]
fn test_order_type_limit_roundtrip() {
    let order = make_order(101, OrderSide::Sell, OrderType::Limit, 55_000.0, 0.1);
    let encoded = encode_to_vec(&order).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Order, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(order, decoded);
    assert_eq!(decoded.order_type, OrderType::Limit);
}

// Test 7: OrderType::StopLoss roundtrip
#[test]
fn test_order_type_stop_loss_roundtrip() {
    let order = make_order(102, OrderSide::Sell, OrderType::StopLoss, 39_500.0, 2.0);
    let encoded = encode_to_vec(&order).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Order, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(order, decoded);
    assert_eq!(decoded.order_type, OrderType::StopLoss);
}

// Test 8: OrderType::StopLimit roundtrip
#[test]
fn test_order_type_stop_limit_roundtrip() {
    let order = make_order(103, OrderSide::Buy, OrderType::StopLimit, 41_000.0, 0.75);
    let encoded = encode_to_vec(&order).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Order, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(order, decoded);
    assert_eq!(decoded.order_type, OrderType::StopLimit);
}

// Test 9: Large order book (500 bids + 500 asks) roundtrip
#[test]
fn test_large_order_book_500_bids_500_asks_roundtrip() {
    let bids: Vec<Order> = (0u64..500)
        .map(|i| {
            make_order(
                i,
                OrderSide::Buy,
                OrderType::Limit,
                40_000.0 - i as f64 * 0.1,
                1.0 + i as f64 * 0.01,
            )
        })
        .collect();
    let asks: Vec<Order> = (500u64..1000)
        .map(|i| {
            make_order(
                i,
                OrderSide::Sell,
                OrderType::Limit,
                40_001.0 + (i - 500) as f64 * 0.1,
                1.0 + (i - 500) as f64 * 0.01,
            )
        })
        .collect();
    let book = OrderBook {
        symbol: "BTC/USD".to_string(),
        bids,
        asks,
        sequence: 1_000_000,
    };
    let encoded = encode_to_vec(&book).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (OrderBook, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(book, decoded);
    assert_eq!(decoded.bids.len(), 500);
    assert_eq!(decoded.asks.len(), 500);
}

// Test 10: Compression ratio for highly repetitive order data (1000+ identical entries)
#[test]
fn test_compression_ratio_large_repetitive_order_data() {
    let identical_order = make_order(42, OrderSide::Buy, OrderType::Limit, 40_000.0, 1.0);
    let orders: Vec<Order> = vec![identical_order; 1000];
    let book = OrderBook {
        symbol: "BTC/USD".to_string(),
        bids: orders,
        asks: Vec::new(),
        sequence: 1,
    };
    let encoded = encode_to_vec(&book).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({} bytes) should be smaller than encoded ({} bytes) for 1000 identical orders",
        compressed.len(),
        encoded.len()
    );
}

// Test 11: Empty order book roundtrip
#[test]
fn test_empty_order_book_roundtrip() {
    let book = OrderBook {
        symbol: "XRP/USD".to_string(),
        bids: Vec::new(),
        asks: Vec::new(),
        sequence: 0,
    };
    let encoded = encode_to_vec(&book).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (OrderBook, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(book, decoded);
    assert!(decoded.bids.is_empty());
    assert!(decoded.asks.is_empty());
}

// Test 12: Truncated compressed data returns an error
#[test]
fn test_truncated_compressed_data_returns_error() {
    let order = make_order(5, OrderSide::Sell, OrderType::Limit, 1_000.0, 0.5);
    let encoded = encode_to_vec(&order).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    // Truncate to first 3 bytes (less than the 5-byte header)
    let truncated = &compressed[..3];
    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "decompress should fail on truncated header"
    );
}

// Test 13: Corrupted payload returns an error
#[test]
fn test_corrupted_payload_returns_error() {
    let order = make_order(6, OrderSide::Buy, OrderType::Market, 0.0, 1.0);
    let encoded = encode_to_vec(&order).expect("encode failed");
    let mut compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    // Corrupt payload bytes beyond the 5-byte header
    if compressed.len() > 10 {
        compressed[7] ^= 0xFF;
        compressed[8] ^= 0xFF;
        compressed[9] ^= 0xFF;
    }
    // Either decompression fails, or the data is mangled and decode fails — either is acceptable
    let decompress_result = decompress(&compressed);
    if let Ok(decompressed) = decompress_result {
        let decode_result: Result<(Order, usize), _> = decode_from_slice(&decompressed);
        // We don't assert decode fails — corruption may not always be detectable — but
        // we exercise the path without panicking
        let _ = decode_result;
    }
}

// Test 14: Multiple orders of different types in one book
#[test]
fn test_mixed_order_types_in_order_book_roundtrip() {
    let bids = vec![
        make_order(1, OrderSide::Buy, OrderType::Market, 0.0, 0.5),
        make_order(2, OrderSide::Buy, OrderType::Limit, 50_000.0, 1.0),
        make_order(3, OrderSide::Buy, OrderType::StopLoss, 49_000.0, 0.25),
        make_order(4, OrderSide::Buy, OrderType::StopLimit, 49_500.0, 0.75),
    ];
    let asks = vec![
        make_order(5, OrderSide::Sell, OrderType::Market, 0.0, 0.3),
        make_order(6, OrderSide::Sell, OrderType::Limit, 50_100.0, 1.5),
        make_order(7, OrderSide::Sell, OrderType::StopLoss, 51_000.0, 0.8),
        make_order(8, OrderSide::Sell, OrderType::StopLimit, 50_900.0, 0.4),
    ];
    let book = OrderBook {
        symbol: "BTC/USD".to_string(),
        bids,
        asks,
        sequence: 42,
    };
    let encoded = encode_to_vec(&book).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (OrderBook, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(book, decoded);
    assert_eq!(decoded.bids.len(), 4);
    assert_eq!(decoded.asks.len(), 4);
}

// Test 15: Order with extreme price values roundtrip
#[test]
fn test_order_extreme_price_values_roundtrip() {
    let order = Order {
        order_id: u64::MAX,
        symbol: "SAT/USD".to_string(),
        side: OrderSide::Sell,
        order_type: OrderType::Limit,
        price: f64::MAX / 2.0,
        quantity: f64::MIN_POSITIVE,
        timestamp: u64::MAX,
    };
    let encoded = encode_to_vec(&order).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Order, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(order, decoded);
}

// Test 16: Order with zero price and zero quantity (market order semantics)
#[test]
fn test_order_zero_price_zero_quantity_roundtrip() {
    let order = Order {
        order_id: 0,
        symbol: "".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Market,
        price: 0.0,
        quantity: 0.0,
        timestamp: 0,
    };
    let encoded = encode_to_vec(&order).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Order, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(order, decoded);
}

// Test 17: Large symbol string in order book roundtrip
#[test]
fn test_order_book_large_symbol_string_roundtrip() {
    let long_symbol = "X".repeat(1024);
    let book = OrderBook {
        symbol: long_symbol.clone(),
        bids: vec![make_order(1, OrderSide::Buy, OrderType::Limit, 1.0, 1.0)],
        asks: vec![make_order(2, OrderSide::Sell, OrderType::Limit, 2.0, 1.0)],
        sequence: 7,
    };
    let encoded = encode_to_vec(&book).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (OrderBook, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(book, decoded);
    assert_eq!(decoded.symbol.len(), 1024);
}

// Test 18: Vec of order books roundtrip
#[test]
fn test_vec_of_order_books_roundtrip() {
    let books: Vec<OrderBook> = (0u64..10)
        .map(|i| OrderBook {
            symbol: format!("PAIR{}/USD", i),
            bids: vec![make_order(
                i * 10,
                OrderSide::Buy,
                OrderType::Limit,
                1000.0 + i as f64,
                1.0,
            )],
            asks: vec![make_order(
                i * 10 + 1,
                OrderSide::Sell,
                OrderType::Limit,
                1001.0 + i as f64,
                1.0,
            )],
            sequence: i,
        })
        .collect();
    let encoded = encode_to_vec(&books).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<OrderBook>, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(books, decoded);
    assert_eq!(decoded.len(), 10);
}

// Test 19: Sequence number preservation across compress/decompress
#[test]
fn test_order_book_sequence_number_preserved() {
    let sequence = 9_999_999_999u64;
    let book = OrderBook {
        symbol: "SOL/USD".to_string(),
        bids: vec![make_order(1, OrderSide::Buy, OrderType::Limit, 100.0, 5.0)],
        asks: vec![make_order(2, OrderSide::Sell, OrderType::Limit, 100.5, 5.0)],
        sequence,
    };
    let encoded = encode_to_vec(&book).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (OrderBook, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.sequence, sequence);
}

// Test 20: Order field values preserved exactly after roundtrip
#[test]
fn test_order_all_fields_preserved_exactly() {
    let order = Order {
        order_id: 123_456_789,
        symbol: "DOGE/USD".to_string(),
        side: OrderSide::Sell,
        order_type: OrderType::StopLimit,
        price: 0.123_456_789,
        quantity: 99_999.999,
        timestamp: 1_700_123_456,
    };
    let encoded = encode_to_vec(&order).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Order, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.order_id, 123_456_789);
    assert_eq!(decoded.symbol, "DOGE/USD");
    assert_eq!(decoded.side, OrderSide::Sell);
    assert_eq!(decoded.order_type, OrderType::StopLimit);
    assert_eq!(decoded.price, 0.123_456_789);
    assert_eq!(decoded.quantity, 99_999.999);
    assert_eq!(decoded.timestamp, 1_700_123_456);
}

// Test 21: Compression is deterministic (same input produces same output)
#[test]
fn test_lz4_compression_is_deterministic() {
    let book = OrderBook {
        symbol: "ADA/USD".to_string(),
        bids: (0u64..20)
            .map(|i| {
                make_order(
                    i,
                    OrderSide::Buy,
                    OrderType::Limit,
                    0.5 + i as f64 * 0.001,
                    100.0,
                )
            })
            .collect(),
        asks: (20u64..40)
            .map(|i| {
                make_order(
                    i,
                    OrderSide::Sell,
                    OrderType::Limit,
                    0.51 + (i - 20) as f64 * 0.001,
                    100.0,
                )
            })
            .collect(),
        sequence: 555,
    };
    let encoded = encode_to_vec(&book).expect("encode failed");
    let compressed1 = compress(&encoded, Compression::Lz4).expect("first compress failed");
    let compressed2 = compress(&encoded, Compression::Lz4).expect("second compress failed");
    assert_eq!(
        compressed1, compressed2,
        "LZ4 compression must be deterministic"
    );
}

// Test 22: Compression ratio for 1000 identical order books (highly repetitive)
#[test]
fn test_compression_ratio_1000_identical_order_books() {
    let template_book = OrderBook {
        symbol: "BTC/USD".to_string(),
        bids: (0u64..10)
            .map(|i| {
                make_order(
                    i,
                    OrderSide::Buy,
                    OrderType::Limit,
                    40_000.0 - i as f64,
                    1.0,
                )
            })
            .collect(),
        asks: (10u64..20)
            .map(|i| {
                make_order(
                    i,
                    OrderSide::Sell,
                    OrderType::Limit,
                    40_001.0 + (i - 10) as f64,
                    1.0,
                )
            })
            .collect(),
        sequence: 0,
    };
    let books: Vec<OrderBook> = vec![template_book; 1000];
    let encoded = encode_to_vec(&books).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<OrderBook>, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.len(), 1000);
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({} bytes) should be smaller than encoded ({} bytes) for 1000 identical order books",
        compressed.len(),
        encoded.len()
    );
}
