//! Tests for e-commerce / order-management enums and structs — advanced enum roundtrip coverage.

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
enum PaymentMethod {
    CreditCard { last_four: String, network: String },
    DebitCard { last_four: String },
    PayPal { email: String },
    Crypto { currency: String, wallet: String },
    CashOnDelivery,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ShippingCarrier {
    FedEx,
    Ups,
    Usps,
    Dhl,
    LocalCourier,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum OrderState {
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Refunded,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Order {
    id: u64,
    customer_id: u64,
    state: OrderState,
    payment: PaymentMethod,
    total_cents: u64,
    item_count: u32,
    notes: Option<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ShipmentTracking {
    order_id: u64,
    carrier: ShippingCarrier,
    tracking_number: String,
    estimated_days: u8,
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn make_order(
    id: u64,
    customer_id: u64,
    state: OrderState,
    payment: PaymentMethod,
    total_cents: u64,
    item_count: u32,
    notes: Option<String>,
) -> Order {
    Order {
        id,
        customer_id,
        state,
        payment,
        total_cents,
        item_count,
        notes,
    }
}

fn make_tracking(
    order_id: u64,
    carrier: ShippingCarrier,
    tracking_number: &str,
    estimated_days: u8,
) -> ShipmentTracking {
    ShipmentTracking {
        order_id,
        carrier,
        tracking_number: tracking_number.to_string(),
        estimated_days,
    }
}

// ── test 1: PaymentMethod::CreditCard roundtrip ───────────────────────────────

#[test]
fn test_payment_method_credit_card_roundtrip() {
    let val = PaymentMethod::CreditCard {
        last_four: "4242".to_string(),
        network: "Visa".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode CreditCard");
    let (decoded, consumed): (PaymentMethod, usize) =
        decode_from_slice(&bytes).expect("decode CreditCard");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for CreditCard"
    );
}

// ── test 2: PaymentMethod::DebitCard roundtrip ────────────────────────────────

#[test]
fn test_payment_method_debit_card_roundtrip() {
    let val = PaymentMethod::DebitCard {
        last_four: "1234".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode DebitCard");
    let (decoded, consumed): (PaymentMethod, usize) =
        decode_from_slice(&bytes).expect("decode DebitCard");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for DebitCard"
    );
}

// ── test 3: PaymentMethod::PayPal roundtrip ───────────────────────────────────

#[test]
fn test_payment_method_paypal_roundtrip() {
    let val = PaymentMethod::PayPal {
        email: "buyer@example.com".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode PayPal");
    let (decoded, consumed): (PaymentMethod, usize) =
        decode_from_slice(&bytes).expect("decode PayPal");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for PayPal"
    );
}

// ── test 4: PaymentMethod::Crypto roundtrip ───────────────────────────────────

#[test]
fn test_payment_method_crypto_roundtrip() {
    let val = PaymentMethod::Crypto {
        currency: "BTC".to_string(),
        wallet: "1A2b3C4d5E6f7G8h9I0j".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode Crypto");
    let (decoded, consumed): (PaymentMethod, usize) =
        decode_from_slice(&bytes).expect("decode Crypto");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for Crypto"
    );
}

// ── test 5: PaymentMethod::CashOnDelivery roundtrip ──────────────────────────

#[test]
fn test_payment_method_cash_on_delivery_roundtrip() {
    let val = PaymentMethod::CashOnDelivery;
    let bytes = encode_to_vec(&val).expect("encode CashOnDelivery");
    let (decoded, consumed): (PaymentMethod, usize) =
        decode_from_slice(&bytes).expect("decode CashOnDelivery");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for CashOnDelivery"
    );
}

// ── test 6: all PaymentMethod variants produce distinct encodings ─────────────

#[test]
fn test_payment_method_discriminant_uniqueness() {
    let variants: Vec<PaymentMethod> = vec![
        PaymentMethod::CreditCard {
            last_four: "0000".to_string(),
            network: "Mastercard".to_string(),
        },
        PaymentMethod::DebitCard {
            last_four: "0000".to_string(),
        },
        PaymentMethod::PayPal {
            email: "x@x.com".to_string(),
        },
        PaymentMethod::Crypto {
            currency: "ETH".to_string(),
            wallet: "0xDEAD".to_string(),
        },
        PaymentMethod::CashOnDelivery,
    ];

    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode PaymentMethod variant"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "PaymentMethod variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ── test 7: all ShippingCarrier variants roundtrip and pairwise distinct ───────

#[test]
fn test_shipping_carrier_all_variants_roundtrip_and_differ() {
    let variants = [
        ShippingCarrier::FedEx,
        ShippingCarrier::Ups,
        ShippingCarrier::Usps,
        ShippingCarrier::Dhl,
        ShippingCarrier::LocalCourier,
    ];

    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode ShippingCarrier"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "ShippingCarrier variants {i} and {j} must differ"
            );
        }
    }

    let expected = [
        ShippingCarrier::FedEx,
        ShippingCarrier::Ups,
        ShippingCarrier::Usps,
        ShippingCarrier::Dhl,
        ShippingCarrier::LocalCourier,
    ];
    for (bytes, exp) in encodings.iter().zip(expected.iter()) {
        let (decoded, _): (ShippingCarrier, usize) =
            decode_from_slice(bytes).expect("decode ShippingCarrier");
        assert_eq!(&decoded, exp);
    }
}

// ── test 8: all OrderState variants roundtrip ─────────────────────────────────

#[test]
fn test_order_state_all_variants_roundtrip() {
    let variants = [
        OrderState::Pending,
        OrderState::Confirmed,
        OrderState::Processing,
        OrderState::Shipped,
        OrderState::Delivered,
        OrderState::Cancelled,
        OrderState::Refunded,
    ];

    for variant in &variants {
        let bytes = encode_to_vec(variant).expect("encode OrderState");
        let (decoded, consumed): (OrderState, usize) =
            decode_from_slice(&bytes).expect("decode OrderState");
        assert_eq!(variant, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for OrderState"
        );
    }
}

// ── test 9: all OrderState variants yield distinct encodings ──────────────────

#[test]
fn test_order_state_discriminant_uniqueness() {
    let variants = [
        OrderState::Pending,
        OrderState::Confirmed,
        OrderState::Processing,
        OrderState::Shipped,
        OrderState::Delivered,
        OrderState::Cancelled,
        OrderState::Refunded,
    ];

    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode OrderState for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "OrderState variants {i} and {j} must differ"
            );
        }
    }
}

// ── test 10: Order with CreditCard payment and Some notes roundtrip ───────────

#[test]
fn test_order_credit_card_some_notes_roundtrip() {
    let val = make_order(
        100001,
        55001,
        OrderState::Processing,
        PaymentMethod::CreditCard {
            last_four: "9999".to_string(),
            network: "AmericanExpress".to_string(),
        },
        14999,
        3,
        Some("Please leave at door.".to_string()),
    );
    let bytes = encode_to_vec(&val).expect("encode Order credit card some notes");
    let (decoded, consumed): (Order, usize) =
        decode_from_slice(&bytes).expect("decode Order credit card some notes");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len(), "consumed must equal encoded length");
}

// ── test 11: Order with PayPal payment and None notes roundtrip ───────────────

#[test]
fn test_order_paypal_none_notes_roundtrip() {
    let val = make_order(
        200002,
        55002,
        OrderState::Shipped,
        PaymentMethod::PayPal {
            email: "shopper@mail.com".to_string(),
        },
        4999,
        1,
        None,
    );
    let bytes = encode_to_vec(&val).expect("encode Order paypal none notes");
    let (decoded, _): (Order, usize) =
        decode_from_slice(&bytes).expect("decode Order paypal none notes");
    assert_eq!(val, decoded);
}

// ── test 12: Order with Crypto payment and Delivered state roundtrip ──────────

#[test]
fn test_order_crypto_delivered_roundtrip() {
    let val = make_order(
        300003,
        55003,
        OrderState::Delivered,
        PaymentMethod::Crypto {
            currency: "ETH".to_string(),
            wallet: "0xABCDEF1234567890".to_string(),
        },
        250000,
        5,
        Some("Gift wrapping requested.".to_string()),
    );
    let bytes = encode_to_vec(&val).expect("encode Order crypto delivered");
    let (decoded, consumed): (Order, usize) =
        decode_from_slice(&bytes).expect("decode Order crypto delivered");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len(), "consumed must equal encoded length");
}

// ── test 13: Order with CashOnDelivery and Cancelled state roundtrip ──────────

#[test]
fn test_order_cash_on_delivery_cancelled_roundtrip() {
    let val = make_order(
        400004,
        55004,
        OrderState::Cancelled,
        PaymentMethod::CashOnDelivery,
        799,
        1,
        None,
    );
    let bytes = encode_to_vec(&val).expect("encode Order cash on delivery cancelled");
    let (decoded, _): (Order, usize) =
        decode_from_slice(&bytes).expect("decode Order cash on delivery cancelled");
    assert_eq!(val, decoded);
}

// ── test 14: ShipmentTracking roundtrip for all carriers ──────────────────────

#[test]
fn test_shipment_tracking_all_carriers_roundtrip() {
    let records = vec![
        make_tracking(1001, ShippingCarrier::FedEx, "FX123456789US", 2),
        make_tracking(1002, ShippingCarrier::Ups, "1Z999AA10123456784", 3),
        make_tracking(1003, ShippingCarrier::Usps, "9400111899223756977801", 5),
        make_tracking(1004, ShippingCarrier::Dhl, "JD014600006261234567", 4),
        make_tracking(1005, ShippingCarrier::LocalCourier, "LC-2026-00042", 1),
    ];

    for record in &records {
        let bytes = encode_to_vec(record).expect("encode ShipmentTracking");
        let (decoded, consumed): (ShipmentTracking, usize) =
            decode_from_slice(&bytes).expect("decode ShipmentTracking");
        assert_eq!(record, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for ShipmentTracking"
        );
    }
}

// ── test 15: Some vs None notes on identical Orders encode differently ─────────

#[test]
fn test_order_some_none_notes_encode_differently() {
    let payment_some = PaymentMethod::DebitCard {
        last_four: "5678".to_string(),
    };
    let payment_none = PaymentMethod::DebitCard {
        last_four: "5678".to_string(),
    };

    let with_notes = make_order(
        500005,
        55005,
        OrderState::Confirmed,
        payment_some,
        3299,
        2,
        Some("Fragile — handle with care.".to_string()),
    );
    let without_notes = make_order(
        500005,
        55005,
        OrderState::Confirmed,
        payment_none,
        3299,
        2,
        None,
    );

    let bytes_some = encode_to_vec(&with_notes).expect("encode Order with notes");
    let bytes_none = encode_to_vec(&without_notes).expect("encode Order without notes");
    assert_ne!(
        bytes_some, bytes_none,
        "Some and None notes must yield different encodings"
    );
}

// ── test 16: Vec<Order> roundtrip ─────────────────────────────────────────────

#[test]
fn test_vec_order_roundtrip() {
    let orders: Vec<Order> = vec![
        make_order(
            600001,
            66001,
            OrderState::Pending,
            PaymentMethod::CreditCard {
                last_four: "1111".to_string(),
                network: "Visa".to_string(),
            },
            9900,
            1,
            None,
        ),
        make_order(
            600002,
            66002,
            OrderState::Shipped,
            PaymentMethod::PayPal {
                email: "user2@shop.io".to_string(),
            },
            29900,
            4,
            Some("Express shipping.".to_string()),
        ),
        make_order(
            600003,
            66003,
            OrderState::Delivered,
            PaymentMethod::CashOnDelivery,
            1499,
            1,
            None,
        ),
        make_order(
            600004,
            66004,
            OrderState::Refunded,
            PaymentMethod::Crypto {
                currency: "LTC".to_string(),
                wallet: "LaBcDeFg1234".to_string(),
            },
            5000,
            2,
            Some("Customer reported missing item.".to_string()),
        ),
    ];

    let bytes = encode_to_vec(&orders).expect("encode Vec<Order>");
    let (decoded, consumed): (Vec<Order>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Order>");
    assert_eq!(orders, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for Vec<Order>"
    );
}

// ── test 17: big-endian config Order roundtrip ────────────────────────────────

#[test]
fn test_order_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = make_order(
        700001,
        77001,
        OrderState::Processing,
        PaymentMethod::CreditCard {
            last_four: "3737".to_string(),
            network: "Mastercard".to_string(),
        },
        59999,
        7,
        Some("Include invoice.".to_string()),
    );
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode Order big-endian");
    let (decoded, _): (Order, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Order big-endian");
    assert_eq!(val, decoded);
}

// ── test 18: fixed-int config Order roundtrip ─────────────────────────────────

#[test]
fn test_order_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = make_order(
        800001,
        88001,
        OrderState::Delivered,
        PaymentMethod::DebitCard {
            last_four: "8765".to_string(),
        },
        12300,
        2,
        None,
    );
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode Order fixed-int");
    let (decoded, _): (Order, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Order fixed-int");
    assert_eq!(val, decoded);
}

// ── test 19: big-endian config ShipmentTracking roundtrip ────────────────────

#[test]
fn test_shipment_tracking_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = make_tracking(900001, ShippingCarrier::Dhl, "DHL-BE-TEST-9999", 3);
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode ShipmentTracking big-endian");
    let (decoded, consumed): (ShipmentTracking, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode ShipmentTracking big-endian");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length (big-endian)"
    );
}

// ── test 20: Orders with different states but identical other fields differ ────

#[test]
fn test_different_order_states_encode_differently() {
    let make = |state: OrderState| {
        make_order(
            999001,
            99001,
            state,
            PaymentMethod::CashOnDelivery,
            5000,
            1,
            None,
        )
    };

    let bytes_pending = encode_to_vec(&make(OrderState::Pending)).expect("encode Pending");
    let bytes_confirmed = encode_to_vec(&make(OrderState::Confirmed)).expect("encode Confirmed");
    let bytes_refunded = encode_to_vec(&make(OrderState::Refunded)).expect("encode Refunded");

    assert_ne!(
        bytes_pending, bytes_confirmed,
        "Pending and Confirmed states must differ"
    );
    assert_ne!(
        bytes_pending, bytes_refunded,
        "Pending and Refunded states must differ"
    );
    assert_ne!(
        bytes_confirmed, bytes_refunded,
        "Confirmed and Refunded states must differ"
    );
}

// ── test 21: ShipmentTracking consumed bytes equals encoded length ─────────────

#[test]
fn test_shipment_tracking_consumed_bytes_equals_encoded_length() {
    let val = ShipmentTracking {
        order_id: 1_000_001,
        carrier: ShippingCarrier::FedEx,
        tracking_number: "FX-LONG-TRACKING-NUMBER-2026-OXICODE".to_string(),
        estimated_days: 7,
    };
    let bytes = encode_to_vec(&val).expect("encode ShipmentTracking for length check");
    let (decoded, consumed): (ShipmentTracking, usize) =
        decode_from_slice(&bytes).expect("decode ShipmentTracking for length check");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must exactly equal total encoded length"
    );
}

// ── test 22: Order with DebitCard, Refunded state, and long notes — content preserved

#[test]
fn test_order_field_content_fully_preserved() {
    let long_notes = "Customer requested expedited processing due to upcoming event. \
                      Ensure priority handling and same-day dispatch if possible. \
                      Billing address differs from shipping address — verify before dispatch."
        .to_string();

    let val = Order {
        id: 1_100_001,
        customer_id: 110_001,
        state: OrderState::Refunded,
        payment: PaymentMethod::DebitCard {
            last_four: "4321".to_string(),
        },
        total_cents: 999_999,
        item_count: 99,
        notes: Some(long_notes.clone()),
    };

    let bytes = encode_to_vec(&val).expect("encode Order long notes");
    let (decoded, consumed): (Order, usize) =
        decode_from_slice(&bytes).expect("decode Order long notes");

    assert_eq!(val, decoded);
    assert_eq!(decoded.id, 1_100_001, "order id must be preserved");
    assert_eq!(decoded.item_count, 99, "item_count must be preserved");
    assert_eq!(
        decoded.total_cents, 999_999,
        "total_cents must be preserved"
    );
    assert_eq!(
        decoded.notes.as_deref().expect("notes must be Some"),
        long_notes,
        "long notes string must be faithfully preserved"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal encoded length"
    );
}
