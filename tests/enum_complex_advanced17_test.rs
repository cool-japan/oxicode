//! Advanced tests for retail / e-commerce order processing enums (set 17)

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
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum OrderStatus {
    Draft,
    Submitted,
    Processing,
    Shipped { tracking_id: String },
    Delivered,
    Cancelled { reason: String },
    Refunded { amount: u64, partial: bool },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PaymentStatus {
    Unpaid,
    Pending,
    Paid,
    PartiallyRefunded { refund_amount: u64 },
    FullyRefunded,
    Failed { code: u16 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ShippingMethod {
    Standard,
    Express,
    Overnight,
    InStorePickup,
    DigitalDelivery,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OrderLine {
    product_id: u64,
    quantity: u32,
    unit_price: u64,
    discount: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Order {
    order_id: u64,
    status: OrderStatus,
    payment: PaymentStatus,
    shipping: ShippingMethod,
    lines: Vec<OrderLine>,
    total: u64,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn make_order_line(product_id: u64, quantity: u32, unit_price: u64, discount: u32) -> OrderLine {
    OrderLine {
        product_id,
        quantity,
        unit_price,
        discount,
    }
}

// ---------------------------------------------------------------------------
// Tests – OrderStatus variants
// ---------------------------------------------------------------------------

#[test]
fn test_order_status_draft_roundtrip() {
    let status = OrderStatus::Draft;
    let encoded = encode_to_vec(&status).expect("encode Draft");
    let (decoded, _): (OrderStatus, _) = decode_from_slice(&encoded).expect("decode Draft");
    assert_eq!(status, decoded);
}

#[test]
fn test_order_status_submitted_roundtrip() {
    let status = OrderStatus::Submitted;
    let encoded = encode_to_vec(&status).expect("encode Submitted");
    let (decoded, _): (OrderStatus, _) = decode_from_slice(&encoded).expect("decode Submitted");
    assert_eq!(status, decoded);
}

#[test]
fn test_order_status_processing_roundtrip() {
    let status = OrderStatus::Processing;
    let encoded = encode_to_vec(&status).expect("encode Processing");
    let (decoded, _): (OrderStatus, _) = decode_from_slice(&encoded).expect("decode Processing");
    assert_eq!(status, decoded);
}

#[test]
fn test_order_status_shipped_roundtrip() {
    let status = OrderStatus::Shipped {
        tracking_id: String::from("TRACK-2024-XYZ-99887766"),
    };
    let encoded = encode_to_vec(&status).expect("encode Shipped");
    let (decoded, _): (OrderStatus, _) = decode_from_slice(&encoded).expect("decode Shipped");
    assert_eq!(status, decoded);
}

#[test]
fn test_order_status_delivered_roundtrip() {
    let status = OrderStatus::Delivered;
    let encoded = encode_to_vec(&status).expect("encode Delivered");
    let (decoded, _): (OrderStatus, _) = decode_from_slice(&encoded).expect("decode Delivered");
    assert_eq!(status, decoded);
}

#[test]
fn test_order_status_cancelled_roundtrip() {
    let status = OrderStatus::Cancelled {
        reason: String::from("Customer requested cancellation before shipment"),
    };
    let encoded = encode_to_vec(&status).expect("encode Cancelled");
    let (decoded, _): (OrderStatus, _) = decode_from_slice(&encoded).expect("decode Cancelled");
    assert_eq!(status, decoded);
}

#[test]
fn test_order_status_refunded_full_roundtrip() {
    let status = OrderStatus::Refunded {
        amount: 19999,
        partial: false,
    };
    let encoded = encode_to_vec(&status).expect("encode Refunded full");
    let (decoded, _): (OrderStatus, _) = decode_from_slice(&encoded).expect("decode Refunded full");
    assert_eq!(status, decoded);
}

#[test]
fn test_order_status_refunded_partial_roundtrip() {
    let status = OrderStatus::Refunded {
        amount: 5000,
        partial: true,
    };
    let encoded = encode_to_vec(&status).expect("encode Refunded partial");
    let (decoded, _): (OrderStatus, _) =
        decode_from_slice(&encoded).expect("decode Refunded partial");
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Tests – PaymentStatus variants
// ---------------------------------------------------------------------------

#[test]
fn test_payment_status_all_variants_roundtrip() {
    let variants = vec![
        PaymentStatus::Unpaid,
        PaymentStatus::Pending,
        PaymentStatus::Paid,
        PaymentStatus::PartiallyRefunded {
            refund_amount: 1234,
        },
        PaymentStatus::FullyRefunded,
        PaymentStatus::Failed { code: 402 },
    ];
    for variant in variants {
        let encoded = encode_to_vec(&variant).expect("encode PaymentStatus variant");
        let (decoded, _): (PaymentStatus, _) =
            decode_from_slice(&encoded).expect("decode PaymentStatus variant");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Tests – ShippingMethod variants
// ---------------------------------------------------------------------------

#[test]
fn test_shipping_method_all_variants_roundtrip() {
    let methods = [
        ShippingMethod::Standard,
        ShippingMethod::Express,
        ShippingMethod::Overnight,
        ShippingMethod::InStorePickup,
        ShippingMethod::DigitalDelivery,
    ];
    for method in methods {
        let encoded = encode_to_vec(&method).expect("encode ShippingMethod");
        let (decoded, _): (ShippingMethod, _) =
            decode_from_slice(&encoded).expect("decode ShippingMethod");
        assert_eq!(method, decoded);
    }
}

// ---------------------------------------------------------------------------
// Tests – Order struct roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_order_full_roundtrip() {
    let order = Order {
        order_id: 100_000_001,
        status: OrderStatus::Shipped {
            tracking_id: String::from("FX-20240315-001"),
        },
        payment: PaymentStatus::Paid,
        shipping: ShippingMethod::Express,
        lines: vec![
            make_order_line(42, 2, 3500, 0),
            make_order_line(77, 1, 9900, 500),
        ],
        total: 16400,
    };
    let encoded = encode_to_vec(&order).expect("encode Order");
    let (decoded, _): (Order, _) = decode_from_slice(&encoded).expect("decode Order");
    assert_eq!(order, decoded);
}

// ---------------------------------------------------------------------------
// Tests – Config: big_endian
// ---------------------------------------------------------------------------

#[test]
fn test_order_big_endian_config() {
    let order = Order {
        order_id: 999_888_777,
        status: OrderStatus::Processing,
        payment: PaymentStatus::Pending,
        shipping: ShippingMethod::Standard,
        lines: vec![make_order_line(1, 10, 200, 10)],
        total: 1900,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&order, cfg).expect("encode big_endian");
    let (decoded, _): (Order, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode big_endian");
    assert_eq!(order, decoded);
}

// ---------------------------------------------------------------------------
// Tests – Config: fixed_int
// ---------------------------------------------------------------------------

#[test]
fn test_order_fixed_int_config() {
    let order = Order {
        order_id: 1,
        status: OrderStatus::Delivered,
        payment: PaymentStatus::Paid,
        shipping: ShippingMethod::Overnight,
        lines: vec![make_order_line(5, 3, 150, 0)],
        total: 450,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&order, cfg).expect("encode fixed_int");
    let (decoded, _): (Order, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode fixed_int");
    assert_eq!(order, decoded);
}

// ---------------------------------------------------------------------------
// Tests – Consumed bytes
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_bytes_matches_encoded_length() {
    let status = OrderStatus::Cancelled {
        reason: String::from("Out of stock"),
    };
    let encoded = encode_to_vec(&status).expect("encode for consumed bytes");
    let (_, consumed): (OrderStatus, _) =
        decode_from_slice(&encoded).expect("decode for consumed bytes");
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Tests – Vec<Order>
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_orders_roundtrip() {
    let orders = vec![
        Order {
            order_id: 1,
            status: OrderStatus::Draft,
            payment: PaymentStatus::Unpaid,
            shipping: ShippingMethod::Standard,
            lines: vec![make_order_line(10, 1, 999, 0)],
            total: 999,
        },
        Order {
            order_id: 2,
            status: OrderStatus::Submitted,
            payment: PaymentStatus::Pending,
            shipping: ShippingMethod::Express,
            lines: vec![
                make_order_line(20, 2, 500, 50),
                make_order_line(30, 1, 1200, 0),
            ],
            total: 2150,
        },
        Order {
            order_id: 3,
            status: OrderStatus::Delivered,
            payment: PaymentStatus::Paid,
            shipping: ShippingMethod::DigitalDelivery,
            lines: vec![],
            total: 0,
        },
    ];
    let encoded = encode_to_vec(&orders).expect("encode Vec<Order>");
    let (decoded, _): (Vec<Order>, _) = decode_from_slice(&encoded).expect("decode Vec<Order>");
    assert_eq!(orders, decoded);
}

// ---------------------------------------------------------------------------
// Tests – Discriminant uniqueness
// ---------------------------------------------------------------------------

#[test]
fn test_order_status_discriminants_are_unique() {
    let variants: Vec<OrderStatus> = vec![
        OrderStatus::Draft,
        OrderStatus::Submitted,
        OrderStatus::Processing,
        OrderStatus::Shipped {
            tracking_id: String::from("T"),
        },
        OrderStatus::Delivered,
        OrderStatus::Cancelled {
            reason: String::from("R"),
        },
        OrderStatus::Refunded {
            amount: 1,
            partial: false,
        },
    ];
    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode for discriminant check"))
        .collect();
    // The first byte(s) encode the discriminant; verify no two unit-variant
    // encodings share the exact same bytes (they differ in discriminant).
    let unit_encodings: Vec<&[u8]> = [
        encodings[0].as_slice(), // Draft
        encodings[1].as_slice(), // Submitted
        encodings[2].as_slice(), // Processing
        encodings[4].as_slice(), // Delivered
    ]
    .to_vec();
    for i in 0..unit_encodings.len() {
        for j in (i + 1)..unit_encodings.len() {
            assert_ne!(
                unit_encodings[i], unit_encodings[j],
                "discriminants {i} and {j} must differ"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Tests – Complex nested data
// ---------------------------------------------------------------------------

#[test]
fn test_order_with_many_lines_and_complex_status() {
    let lines: Vec<OrderLine> = (0..50)
        .map(|i| {
            make_order_line(
                i as u64 + 1000,
                i as u32 + 1,
                (i as u64 + 1) * 100,
                i as u32,
            )
        })
        .collect();
    let total: u64 = lines
        .iter()
        .map(|l| {
            let gross = l.quantity as u64 * l.unit_price;
            let disc = l.discount as u64;
            gross.saturating_sub(disc)
        })
        .sum();
    let order = Order {
        order_id: u64::MAX,
        status: OrderStatus::Refunded {
            amount: total / 2,
            partial: true,
        },
        payment: PaymentStatus::PartiallyRefunded {
            refund_amount: total / 2,
        },
        shipping: ShippingMethod::Overnight,
        lines,
        total,
    };
    let encoded = encode_to_vec(&order).expect("encode complex Order");
    let (decoded, consumed): (Order, _) =
        decode_from_slice(&encoded).expect("decode complex Order");
    assert_eq!(order, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_order_empty_lines_digital_delivery() {
    let order = Order {
        order_id: 55555,
        status: OrderStatus::Shipped {
            tracking_id: String::from("DIGITAL-NO-TRACKING"),
        },
        payment: PaymentStatus::Paid,
        shipping: ShippingMethod::DigitalDelivery,
        lines: vec![],
        total: 0,
    };
    let encoded = encode_to_vec(&order).expect("encode digital delivery");
    let (decoded, _): (Order, _) = decode_from_slice(&encoded).expect("decode digital delivery");
    assert_eq!(order, decoded);
}

#[test]
fn test_payment_failed_code_boundary_values() {
    // Test boundary values for the failure code field
    for code in [0u16, 1, 255, 256, u16::MAX] {
        let payment = PaymentStatus::Failed { code };
        let encoded = encode_to_vec(&payment).expect("encode Failed payment");
        let (decoded, _): (PaymentStatus, _) =
            decode_from_slice(&encoded).expect("decode Failed payment");
        assert_eq!(payment, decoded, "Failed{{code: {code}}} must roundtrip");
    }
}

#[test]
fn test_order_line_zero_discount_roundtrip() {
    let line = make_order_line(99999, u32::MAX, u64::MAX / 2, 0);
    let encoded = encode_to_vec(&line).expect("encode OrderLine zero discount");
    let (decoded, consumed): (OrderLine, _) =
        decode_from_slice(&encoded).expect("decode OrderLine zero discount");
    assert_eq!(line, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_order_in_store_pickup_refunded_status() {
    let order = Order {
        order_id: 77777,
        status: OrderStatus::Refunded {
            amount: 3000,
            partial: false,
        },
        payment: PaymentStatus::FullyRefunded,
        shipping: ShippingMethod::InStorePickup,
        lines: vec![make_order_line(300, 2, 1500, 0)],
        total: 3000,
    };
    let encoded = encode_to_vec(&order).expect("encode in-store refunded order");
    let (decoded, _): (Order, _) =
        decode_from_slice(&encoded).expect("decode in-store refunded order");
    assert_eq!(order, decoded);
}

#[test]
fn test_order_legacy_config_roundtrip() {
    let order = Order {
        order_id: 42,
        status: OrderStatus::Cancelled {
            reason: String::from("Duplicate order"),
        },
        payment: PaymentStatus::Failed { code: 500 },
        shipping: ShippingMethod::InStorePickup,
        lines: vec![
            make_order_line(101, 5, 800, 100),
            make_order_line(202, 1, 2000, 0),
        ],
        total: 5900,
    };
    let cfg = config::legacy();
    let encoded = encode_to_vec_with_config(&order, cfg).expect("encode legacy config");
    let (decoded, _): (Order, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode legacy config");
    assert_eq!(order, decoded);
}
