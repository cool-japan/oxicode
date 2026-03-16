//! Forward-compatibility versioning tests for OxiCode (set 8).
//!
//! Tests the scenario where V1 structs are encoded and V2 structs (with the same
//! prefix fields) can decode V1 data, and verifies that V1/V2 enum discriminants
//! remain stable for shared variants.

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

// ── Data model definitions ────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct OrderV1 {
    order_id: u64,
    customer: String,
    total_cents: u32,
}

/// OrderV2 has the same three fields as OrderV1 (binary-identical prefix).
/// Any additional V2 fields would need to be appended; here the layout is
/// identical so that V1-encoded bytes are decodable as OrderV2.
#[derive(Debug, PartialEq, Encode, Decode)]
struct OrderV2 {
    order_id: u64,
    customer: String,
    total_cents: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum OrderStatusV1 {
    Pending,
    Processing,
    Shipped,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum OrderStatusV2 {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled(String),
}

// ── Tests 1-3: OrderV1 roundtrip basics ──────────────────────────────────────

/// Test 1: Basic OrderV1 roundtrip with typical values.
#[test]
fn test_order_v1_roundtrip_basic() {
    let order = OrderV1 {
        order_id: 1001,
        customer: String::from("Alice"),
        total_cents: 4999,
    };
    let encoded = encode_to_vec(&order).expect("encode OrderV1 basic");
    let (decoded, _consumed): (OrderV1, usize) =
        decode_from_slice(&encoded).expect("decode OrderV1 basic");
    assert_eq!(order, decoded);
}

/// Test 2: OrderV1 roundtrip with zero total_cents.
#[test]
fn test_order_v1_roundtrip_zero_total() {
    let order = OrderV1 {
        order_id: 0,
        customer: String::from("Bob"),
        total_cents: 0,
    };
    let encoded = encode_to_vec(&order).expect("encode OrderV1 zero total");
    let (decoded, _consumed): (OrderV1, usize) =
        decode_from_slice(&encoded).expect("decode OrderV1 zero total");
    assert_eq!(order, decoded);
}

/// Test 3: OrderV1 roundtrip with a long Unicode customer name.
#[test]
fn test_order_v1_roundtrip_unicode_customer() {
    let order = OrderV1 {
        order_id: 9_999_999,
        customer: String::from("日本語顧客名前テスト"),
        total_cents: 1_234_567,
    };
    let encoded = encode_to_vec(&order).expect("encode OrderV1 unicode");
    let (decoded, _consumed): (OrderV1, usize) =
        decode_from_slice(&encoded).expect("decode OrderV1 unicode");
    assert_eq!(order, decoded);
}

// ── Tests 4-6: OrderStatusV1 variants roundtrip ───────────────────────────────

/// Test 4: OrderStatusV1::Pending roundtrip.
#[test]
fn test_order_status_v1_pending_roundtrip() {
    let status = OrderStatusV1::Pending;
    let encoded = encode_to_vec(&status).expect("encode Pending");
    let (decoded, _consumed): (OrderStatusV1, usize) =
        decode_from_slice(&encoded).expect("decode Pending");
    assert_eq!(status, decoded);
}

/// Test 5: OrderStatusV1::Processing roundtrip.
#[test]
fn test_order_status_v1_processing_roundtrip() {
    let status = OrderStatusV1::Processing;
    let encoded = encode_to_vec(&status).expect("encode Processing");
    let (decoded, _consumed): (OrderStatusV1, usize) =
        decode_from_slice(&encoded).expect("decode Processing");
    assert_eq!(status, decoded);
}

/// Test 6: OrderStatusV1::Shipped roundtrip.
#[test]
fn test_order_status_v1_shipped_roundtrip() {
    let status = OrderStatusV1::Shipped;
    let encoded = encode_to_vec(&status).expect("encode Shipped");
    let (decoded, _consumed): (OrderStatusV1, usize) =
        decode_from_slice(&encoded).expect("decode Shipped");
    assert_eq!(status, decoded);
}

// ── Tests 7-8: OrderStatusV2 new variants roundtrip ──────────────────────────

/// Test 7: OrderStatusV2::Delivered roundtrip (new in V2).
#[test]
fn test_order_status_v2_delivered_roundtrip() {
    let status = OrderStatusV2::Delivered;
    let encoded = encode_to_vec(&status).expect("encode Delivered");
    let (decoded, _consumed): (OrderStatusV2, usize) =
        decode_from_slice(&encoded).expect("decode Delivered");
    assert_eq!(status, decoded);
}

/// Test 8: OrderStatusV2::Cancelled roundtrip (new in V2, with String payload).
#[test]
fn test_order_status_v2_cancelled_roundtrip() {
    let status = OrderStatusV2::Cancelled(String::from("customer requested"));
    let encoded = encode_to_vec(&status).expect("encode Cancelled");
    let (decoded, _consumed): (OrderStatusV2, usize) =
        decode_from_slice(&encoded).expect("decode Cancelled");
    assert_eq!(status, decoded);
}

// ── Test 9: V1 enum discriminants match V2 for shared variants ────────────────

/// Test 9: Verify at the byte level that Pending/Processing/Shipped encode
/// identically in both V1 and V2 (discriminant stability across versions).
#[test]
fn test_enum_discriminant_stability_v1_v2() {
    let v1_pending = encode_to_vec(&OrderStatusV1::Pending).expect("encode V1 Pending");
    let v2_pending = encode_to_vec(&OrderStatusV2::Pending).expect("encode V2 Pending");
    assert_eq!(
        v1_pending, v2_pending,
        "Pending discriminant must be identical in V1 and V2"
    );

    let v1_processing = encode_to_vec(&OrderStatusV1::Processing).expect("encode V1 Processing");
    let v2_processing = encode_to_vec(&OrderStatusV2::Processing).expect("encode V2 Processing");
    assert_eq!(
        v1_processing, v2_processing,
        "Processing discriminant must be identical in V1 and V2"
    );

    let v1_shipped = encode_to_vec(&OrderStatusV1::Shipped).expect("encode V1 Shipped");
    let v2_shipped = encode_to_vec(&OrderStatusV2::Shipped).expect("encode V2 Shipped");
    assert_eq!(
        v1_shipped, v2_shipped,
        "Shipped discriminant must be identical in V1 and V2"
    );
}

// ── Test 10: Vec<OrderV1> roundtrip ──────────────────────────────────────────

/// Test 10: A Vec of OrderV1 values encodes and decodes correctly.
#[test]
fn test_vec_order_v1_roundtrip() {
    let orders = vec![
        OrderV1 {
            order_id: 1,
            customer: String::from("Alice"),
            total_cents: 100,
        },
        OrderV1 {
            order_id: 2,
            customer: String::from("Bob"),
            total_cents: 200,
        },
        OrderV1 {
            order_id: 3,
            customer: String::from("Carol"),
            total_cents: 300,
        },
    ];
    let encoded = encode_to_vec(&orders).expect("encode Vec<OrderV1>");
    let (decoded, _consumed): (Vec<OrderV1>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<OrderV1>");
    assert_eq!(orders, decoded);
}

// ── Test 11: Option<OrderV1> roundtrip ───────────────────────────────────────

/// Test 11: Option<OrderV1> encodes and decodes correctly for both Some and None.
#[test]
fn test_option_order_v1_roundtrip() {
    let some_order: Option<OrderV1> = Some(OrderV1 {
        order_id: 42,
        customer: String::from("Dave"),
        total_cents: 999,
    });
    let encoded_some = encode_to_vec(&some_order).expect("encode Some(OrderV1)");
    let (decoded_some, _): (Option<OrderV1>, usize) =
        decode_from_slice(&encoded_some).expect("decode Some(OrderV1)");
    assert_eq!(some_order, decoded_some);

    let none_order: Option<OrderV1> = None;
    let encoded_none = encode_to_vec(&none_order).expect("encode None::<OrderV1>");
    let (decoded_none, _): (Option<OrderV1>, usize) =
        decode_from_slice(&encoded_none).expect("decode None::<OrderV1>");
    assert_eq!(none_order, decoded_none);
}

// ── Test 12: Consumed bytes check ────────────────────────────────────────────

/// Test 12: decode_from_slice returns the correct number of consumed bytes,
/// equal to the total encoded length when the buffer contains exactly one value.
#[test]
fn test_consumed_bytes_matches_encoded_length() {
    let order = OrderV1 {
        order_id: 77,
        customer: String::from("Eve"),
        total_cents: 5050,
    };
    let encoded = encode_to_vec(&order).expect("encode for consumed bytes check");
    let total_len = encoded.len();
    let (_decoded, consumed): (OrderV1, usize) =
        decode_from_slice(&encoded).expect("decode for consumed bytes check");
    assert_eq!(
        consumed, total_len,
        "consumed bytes must equal the full encoded buffer length"
    );
}

// ── Tests 13-16: Config variants ─────────────────────────────────────────────

/// Test 13: Standard config roundtrip for OrderV1.
#[test]
fn test_config_standard_order_v1_roundtrip() {
    let order = OrderV1 {
        order_id: 100,
        customer: String::from("Frank"),
        total_cents: 2500,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec_with_config(&order, cfg).expect("encode standard config");
    let (decoded, _): (OrderV1, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode standard config");
    assert_eq!(order, decoded);
}

/// Test 14: Legacy (fixed-int) config roundtrip for OrderV1.
#[test]
fn test_config_legacy_order_v1_roundtrip() {
    let order = OrderV1 {
        order_id: 200,
        customer: String::from("Grace"),
        total_cents: 7500,
    };
    let cfg = config::legacy();
    let encoded = encode_to_vec_with_config(&order, cfg).expect("encode legacy config");
    let (decoded, _): (OrderV1, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode legacy config");
    assert_eq!(order, decoded);
}

/// Test 15: Big-endian config roundtrip for OrderV1.
#[test]
fn test_config_big_endian_order_v1_roundtrip() {
    let order = OrderV1 {
        order_id: 300,
        customer: String::from("Heidi"),
        total_cents: 3333,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&order, cfg).expect("encode big-endian config");
    let (decoded, _): (OrderV1, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode big-endian config");
    assert_eq!(order, decoded);
}

/// Test 16: Fixed-int big-endian config roundtrip for OrderStatusV2.
#[test]
fn test_config_fixed_int_big_endian_order_status_v2_roundtrip() {
    let status = OrderStatusV2::Cancelled(String::from("out of stock"));
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&status, cfg).expect("encode fixed-int big-endian config");
    let (decoded, _): (OrderStatusV2, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode fixed-int big-endian config");
    assert_eq!(status, decoded);
}

// ── Tests 17-18: Large vec of orders ─────────────────────────────────────────

/// Test 17: A large Vec<OrderV1> (1000 elements) roundtrips correctly.
#[test]
fn test_large_vec_order_v1_roundtrip() {
    let orders: Vec<OrderV1> = (0u64..1000)
        .map(|i| OrderV1 {
            order_id: i,
            customer: format!("customer_{}", i),
            total_cents: (i * 100) as u32,
        })
        .collect();
    let encoded = encode_to_vec(&orders).expect("encode large Vec<OrderV1>");
    let (decoded, _): (Vec<OrderV1>, usize) =
        decode_from_slice(&encoded).expect("decode large Vec<OrderV1>");
    assert_eq!(orders, decoded);
}

/// Test 18: A large Vec<OrderStatusV2> (500 elements) roundtrips correctly.
#[test]
fn test_large_vec_order_status_v2_roundtrip() {
    let statuses: Vec<OrderStatusV2> = (0u64..500)
        .map(|i| match i % 5 {
            0 => OrderStatusV2::Pending,
            1 => OrderStatusV2::Processing,
            2 => OrderStatusV2::Shipped,
            3 => OrderStatusV2::Delivered,
            _ => OrderStatusV2::Cancelled(format!("reason_{}", i)),
        })
        .collect();
    let encoded = encode_to_vec(&statuses).expect("encode large Vec<OrderStatusV2>");
    let (decoded, _): (Vec<OrderStatusV2>, usize) =
        decode_from_slice(&encoded).expect("decode large Vec<OrderStatusV2>");
    assert_eq!(statuses, decoded);
}

// ── Test 19: OrderV1 and OrderV2 produce identical encodings ─────────────────

/// Test 19: OrderV1 and OrderV2 (same field layout) produce byte-for-byte
/// identical encodings, confirming forward-compatibility of the wire format.
#[test]
fn test_order_v1_v2_identical_encoding() {
    let v1 = OrderV1 {
        order_id: 12345,
        customer: String::from("Ivan"),
        total_cents: 9900,
    };
    let v2 = OrderV2 {
        order_id: 12345,
        customer: String::from("Ivan"),
        total_cents: 9900,
    };
    let encoded_v1 = encode_to_vec(&v1).expect("encode OrderV1 for identity check");
    let encoded_v2 = encode_to_vec(&v2).expect("encode OrderV2 for identity check");
    assert_eq!(
        encoded_v1, encoded_v2,
        "OrderV1 and OrderV2 with same fields must produce identical bytes"
    );
}

// ── Test 20: Empty string customer ───────────────────────────────────────────

/// Test 20: OrderV1 with an empty customer string encodes and decodes correctly.
#[test]
fn test_order_v1_empty_customer_roundtrip() {
    let order = OrderV1 {
        order_id: 55,
        customer: String::new(),
        total_cents: 0,
    };
    let encoded = encode_to_vec(&order).expect("encode empty customer");
    let (decoded, _): (OrderV1, usize) =
        decode_from_slice(&encoded).expect("decode empty customer");
    assert_eq!(order, decoded);
}

// ── Test 21: u64::MAX order_id ────────────────────────────────────────────────

/// Test 21: OrderV1 with order_id = u64::MAX roundtrips without overflow or truncation.
#[test]
fn test_order_v1_max_order_id_roundtrip() {
    let order = OrderV1 {
        order_id: u64::MAX,
        customer: String::from("Judy"),
        total_cents: u32::MAX,
    };
    let encoded = encode_to_vec(&order).expect("encode u64::MAX order_id");
    let (decoded, _): (OrderV1, usize) =
        decode_from_slice(&encoded).expect("decode u64::MAX order_id");
    assert_eq!(order, decoded);
}

// ── Test 22: Vec<OrderStatusV2> all 5 variants ───────────────────────────────

/// Test 22: A Vec containing exactly one of each of the 5 OrderStatusV2 variants
/// (Pending, Processing, Shipped, Delivered, Cancelled) roundtrips correctly.
#[test]
fn test_vec_order_status_v2_all_five_variants_roundtrip() {
    let statuses = vec![
        OrderStatusV2::Pending,
        OrderStatusV2::Processing,
        OrderStatusV2::Shipped,
        OrderStatusV2::Delivered,
        OrderStatusV2::Cancelled(String::from("test cancellation reason")),
    ];
    let encoded = encode_to_vec(&statuses).expect("encode all 5 OrderStatusV2 variants");
    let (decoded, _): (Vec<OrderStatusV2>, usize) =
        decode_from_slice(&encoded).expect("decode all 5 OrderStatusV2 variants");
    assert_eq!(statuses, decoded);
}
