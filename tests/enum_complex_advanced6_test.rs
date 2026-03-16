//! Financial data enum roundtrip tests — 22 tests covering Currency, OrderSide,
//! OrderType, Order, TradeEvent, Vec<Order>, big-endian config, fixed-int config,
//! discriminant uniqueness, consumed-bytes verification, and Option<u64> in Limit variant.

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

// ---------------------------------------------------------------------------
// Type definitions
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum Currency {
    Usd,
    Eur,
    Jpy,
    Gbp,
    Btc,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum OrderType {
    Market,
    Limit { price: u64, expiry_ms: Option<u64> },
    Stop { trigger: u64 },
    StopLimit { trigger: u64, limit: u64 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Order {
    id: u64,
    side: OrderSide,
    order_type: OrderType,
    quantity: u64,
    currency: Currency,
    filled: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TradeEvent {
    Placed(Order),
    Filled {
        order_id: u64,
        fill_qty: u64,
        fill_price: u64,
    },
    Cancelled {
        order_id: u64,
        reason: String,
    },
    Rejected {
        order_id: u64,
        code: u32,
    },
    Expired {
        order_id: u64,
    },
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn make_order(id: u64, side: OrderSide, order_type: OrderType, currency: Currency) -> Order {
    Order {
        id,
        side,
        order_type,
        quantity: 100,
        currency,
        filled: 0,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Currency::Usd roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_currency_usd_roundtrip() {
    let val = Currency::Usd;
    let bytes = encode_to_vec(&val).expect("encode Currency::Usd");
    let (decoded, _): (Currency, usize) = decode_from_slice(&bytes).expect("decode Currency::Usd");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Currency::Eur roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_currency_eur_roundtrip() {
    let val = Currency::Eur;
    let bytes = encode_to_vec(&val).expect("encode Currency::Eur");
    let (decoded, _): (Currency, usize) = decode_from_slice(&bytes).expect("decode Currency::Eur");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Currency::Jpy roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_currency_jpy_roundtrip() {
    let val = Currency::Jpy;
    let bytes = encode_to_vec(&val).expect("encode Currency::Jpy");
    let (decoded, _): (Currency, usize) = decode_from_slice(&bytes).expect("decode Currency::Jpy");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Currency::Gbp and Currency::Btc roundtrip — both in one test to
//         confirm all five Currency discriminants are distinct
// ---------------------------------------------------------------------------

#[test]
fn test_currency_gbp_btc_roundtrip() {
    let gbp = Currency::Gbp;
    let gbp_bytes = encode_to_vec(&gbp).expect("encode Gbp");
    let (gbp_dec, _): (Currency, usize) = decode_from_slice(&gbp_bytes).expect("decode Gbp");
    assert_eq!(gbp, gbp_dec);

    let btc = Currency::Btc;
    let btc_bytes = encode_to_vec(&btc).expect("encode Btc");
    let (btc_dec, _): (Currency, usize) = decode_from_slice(&btc_bytes).expect("decode Btc");
    assert_eq!(btc, btc_dec);
}

// ---------------------------------------------------------------------------
// Test 5: OrderSide::Buy roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_order_side_buy_roundtrip() {
    let val = OrderSide::Buy;
    let bytes = encode_to_vec(&val).expect("encode Buy");
    let (decoded, _): (OrderSide, usize) = decode_from_slice(&bytes).expect("decode Buy");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: OrderSide::Sell roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_order_side_sell_roundtrip() {
    let val = OrderSide::Sell;
    let bytes = encode_to_vec(&val).expect("encode Sell");
    let (decoded, _): (OrderSide, usize) = decode_from_slice(&bytes).expect("decode Sell");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: OrderType::Market roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_order_type_market_roundtrip() {
    let val = OrderType::Market;
    let bytes = encode_to_vec(&val).expect("encode Market");
    let (decoded, _): (OrderType, usize) = decode_from_slice(&bytes).expect("decode Market");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: OrderType::Limit with expiry_ms = Some(u64)
// ---------------------------------------------------------------------------

#[test]
fn test_order_type_limit_with_expiry_some_roundtrip() {
    let val = OrderType::Limit {
        price: 50_000_00,
        expiry_ms: Some(1_700_000_000_000),
    };
    let bytes = encode_to_vec(&val).expect("encode Limit Some");
    let (decoded, _): (OrderType, usize) = decode_from_slice(&bytes).expect("decode Limit Some");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: OrderType::Limit with expiry_ms = None
// ---------------------------------------------------------------------------

#[test]
fn test_order_type_limit_with_expiry_none_roundtrip() {
    let val = OrderType::Limit {
        price: 48_500_00,
        expiry_ms: None,
    };
    let bytes = encode_to_vec(&val).expect("encode Limit None");
    let (decoded, _): (OrderType, usize) = decode_from_slice(&bytes).expect("decode Limit None");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: OrderType::Stop roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_order_type_stop_roundtrip() {
    let val = OrderType::Stop { trigger: 47_000_00 };
    let bytes = encode_to_vec(&val).expect("encode Stop");
    let (decoded, _): (OrderType, usize) = decode_from_slice(&bytes).expect("decode Stop");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: OrderType::StopLimit roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_order_type_stop_limit_roundtrip() {
    let val = OrderType::StopLimit {
        trigger: 46_000_00,
        limit: 45_500_00,
    };
    let bytes = encode_to_vec(&val).expect("encode StopLimit");
    let (decoded, _): (OrderType, usize) = decode_from_slice(&bytes).expect("decode StopLimit");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Order struct roundtrip — buy market order
// ---------------------------------------------------------------------------

#[test]
fn test_order_buy_market_roundtrip() {
    let val = make_order(1001, OrderSide::Buy, OrderType::Market, Currency::Usd);
    let bytes = encode_to_vec(&val).expect("encode buy market");
    let (decoded, _): (Order, usize) = decode_from_slice(&bytes).expect("decode buy market");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Order struct roundtrip — sell limit order with expiry
// ---------------------------------------------------------------------------

#[test]
fn test_order_sell_limit_roundtrip() {
    let val = Order {
        id: 2002,
        side: OrderSide::Sell,
        order_type: OrderType::Limit {
            price: 3_200_00,
            expiry_ms: Some(9_999_999),
        },
        quantity: 5,
        currency: Currency::Eur,
        filled: 2,
    };
    let bytes = encode_to_vec(&val).expect("encode sell limit");
    let (decoded, _): (Order, usize) = decode_from_slice(&bytes).expect("decode sell limit");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: TradeEvent::Placed roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_trade_event_placed_roundtrip() {
    let order = make_order(3003, OrderSide::Buy, OrderType::Market, Currency::Btc);
    let val = TradeEvent::Placed(order);
    let bytes = encode_to_vec(&val).expect("encode Placed");
    let (decoded, _): (TradeEvent, usize) = decode_from_slice(&bytes).expect("decode Placed");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: TradeEvent::Filled roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_trade_event_filled_roundtrip() {
    let val = TradeEvent::Filled {
        order_id: 4004,
        fill_qty: 10,
        fill_price: 51_000_00,
    };
    let bytes = encode_to_vec(&val).expect("encode Filled");
    let (decoded, _): (TradeEvent, usize) = decode_from_slice(&bytes).expect("decode Filled");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: TradeEvent::Cancelled roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_trade_event_cancelled_roundtrip() {
    let val = TradeEvent::Cancelled {
        order_id: 5005,
        reason: "Insufficient funds".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode Cancelled");
    let (decoded, _): (TradeEvent, usize) = decode_from_slice(&bytes).expect("decode Cancelled");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: TradeEvent::Rejected roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_trade_event_rejected_roundtrip() {
    let val = TradeEvent::Rejected {
        order_id: 6006,
        code: 403,
    };
    let bytes = encode_to_vec(&val).expect("encode Rejected");
    let (decoded, _): (TradeEvent, usize) = decode_from_slice(&bytes).expect("decode Rejected");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: TradeEvent::Expired roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_trade_event_expired_roundtrip() {
    let val = TradeEvent::Expired { order_id: 7007 };
    let bytes = encode_to_vec(&val).expect("encode Expired");
    let (decoded, _): (TradeEvent, usize) = decode_from_slice(&bytes).expect("decode Expired");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Vec<Order> roundtrip — multiple orders with varied fields
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_orders_roundtrip() {
    let orders: Vec<Order> = vec![
        Order {
            id: 1,
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            quantity: 1,
            currency: Currency::Usd,
            filled: 0,
        },
        Order {
            id: 2,
            side: OrderSide::Sell,
            order_type: OrderType::Limit {
                price: 100_00,
                expiry_ms: None,
            },
            quantity: 50,
            currency: Currency::Jpy,
            filled: 20,
        },
        Order {
            id: 3,
            side: OrderSide::Buy,
            order_type: OrderType::StopLimit {
                trigger: 90_00,
                limit: 89_00,
            },
            quantity: 200,
            currency: Currency::Gbp,
            filled: 0,
        },
    ];
    let bytes = encode_to_vec(&orders).expect("encode Vec<Order>");
    let (decoded, _): (Vec<Order>, usize) = decode_from_slice(&bytes).expect("decode Vec<Order>");
    assert_eq!(orders, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Big-endian config roundtrip for Order
// ---------------------------------------------------------------------------

#[test]
fn test_order_big_endian_config_roundtrip() {
    let val = Order {
        id: 8008,
        side: OrderSide::Sell,
        order_type: OrderType::Stop { trigger: 55_000_00 },
        quantity: 300,
        currency: Currency::Eur,
        filled: 150,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode big-endian Order");
    let (decoded, _): (Order, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big-endian Order");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Fixed-int config roundtrip for TradeEvent::Filled
// ---------------------------------------------------------------------------

#[test]
fn test_trade_event_fixed_int_config_roundtrip() {
    let val = TradeEvent::Filled {
        order_id: 9009,
        fill_qty: 75,
        fill_price: 62_000_00,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode fixed-int TradeEvent");
    let (decoded, _): (TradeEvent, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed-int TradeEvent");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: Discriminant uniqueness — each Currency variant encodes to a
//          distinct first byte, and consumed bytes equal total encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_discriminant_uniqueness_and_consumed_bytes() {
    let variants: Vec<Currency> = vec![
        Currency::Usd,
        Currency::Eur,
        Currency::Jpy,
        Currency::Gbp,
        Currency::Btc,
    ];

    let encoded_list: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode Currency variant"))
        .collect();

    // All discriminant bytes (first byte of each encoding) must be unique.
    let discriminants: Vec<u8> = encoded_list.iter().map(|b| b[0]).collect();
    let mut sorted = discriminants.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(
        sorted.len(),
        variants.len(),
        "Currency discriminants are not all unique: {:?}",
        discriminants
    );

    // For each variant, the number of consumed bytes must equal the encoded length.
    for (variant, bytes) in variants.iter().zip(encoded_list.iter()) {
        let (_, consumed): (Currency, usize) =
            decode_from_slice(bytes).expect("decode for consumed-bytes check");
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed bytes mismatch for {:?}: consumed={}, encoded={}",
            variant,
            consumed,
            bytes.len()
        );
    }
}
