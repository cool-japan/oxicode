#![cfg(feature = "serde")]
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
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Shared test types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Invoice {
    invoice_id: String,
    amount: f64,
    currency: String,
    line_items: Vec<LineItem>,
    paid: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct LineItem {
    description: String,
    quantity: u32,
    unit_price: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum PaymentMethod {
    Cash,
    CreditCard { last_four: String },
    BankTransfer(String),
    Crypto { coin: String, address: String },
}

// ---------------------------------------------------------------------------
// Test 1: Invoice roundtrip with 3 line items
// ---------------------------------------------------------------------------

#[test]
fn test_invoice_with_three_line_items_roundtrip() {
    let original = Invoice {
        invoice_id: "INV-2026-001".to_string(),
        amount: 349.97,
        currency: "USD".to_string(),
        line_items: vec![
            LineItem {
                description: "Widget A".to_string(),
                quantity: 2,
                unit_price: 49.99,
            },
            LineItem {
                description: "Widget B".to_string(),
                quantity: 1,
                unit_price: 199.99,
            },
            LineItem {
                description: "Shipping".to_string(),
                quantity: 1,
                unit_price: 50.00,
            },
        ],
        paid: false,
    };
    let bytes =
        encode_to_vec(&original, config::standard()).expect("encode Invoice with 3 line items");
    let (decoded, _): (Invoice, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode Invoice with 3 line items");
    assert_eq!(original, decoded);
    assert_eq!(
        decoded.line_items.len(),
        3,
        "must have exactly 3 line items"
    );
}

// ---------------------------------------------------------------------------
// Test 2: LineItem roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_line_item_roundtrip() {
    let original = LineItem {
        description: "Professional Services".to_string(),
        quantity: 8,
        unit_price: 150.0,
    };
    let bytes = encode_to_vec(&original, config::standard()).expect("encode LineItem");
    let (decoded, _): (LineItem, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode LineItem");
    assert_eq!(original, decoded);
    assert_eq!(decoded.quantity, 8, "quantity must be preserved");
    assert_eq!(
        decoded.unit_price.to_bits(),
        original.unit_price.to_bits(),
        "unit_price bits must match"
    );
}

// ---------------------------------------------------------------------------
// Test 3: PaymentMethod::Cash roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_payment_method_cash_roundtrip() {
    let original = PaymentMethod::Cash;
    let bytes = encode_to_vec(&original, config::standard()).expect("encode PaymentMethod::Cash");
    let (decoded, _): (PaymentMethod, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode PaymentMethod::Cash");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: PaymentMethod::CreditCard roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_payment_method_credit_card_roundtrip() {
    let original = PaymentMethod::CreditCard {
        last_four: "4242".to_string(),
    };
    let bytes =
        encode_to_vec(&original, config::standard()).expect("encode PaymentMethod::CreditCard");
    let (decoded, _): (PaymentMethod, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode PaymentMethod::CreditCard");
    assert_eq!(original, decoded);
    if let PaymentMethod::CreditCard { last_four } = decoded {
        assert_eq!(last_four, "4242", "last_four digits must be preserved");
    } else {
        panic!("decoded variant must be CreditCard");
    }
}

// ---------------------------------------------------------------------------
// Test 5: PaymentMethod::BankTransfer roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_payment_method_bank_transfer_roundtrip() {
    let original = PaymentMethod::BankTransfer("IBAN-DE89-3704-0044-0532-0130-00".to_string());
    let bytes =
        encode_to_vec(&original, config::standard()).expect("encode PaymentMethod::BankTransfer");
    let (decoded, _): (PaymentMethod, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode PaymentMethod::BankTransfer");
    assert_eq!(original, decoded);
    if let PaymentMethod::BankTransfer(iban) = &decoded {
        assert!(iban.starts_with("IBAN-DE"), "IBAN prefix must be preserved");
    } else {
        panic!("decoded variant must be BankTransfer");
    }
}

// ---------------------------------------------------------------------------
// Test 6: PaymentMethod::Crypto roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_payment_method_crypto_roundtrip() {
    let original = PaymentMethod::Crypto {
        coin: "BTC".to_string(),
        address: "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq".to_string(),
    };
    let bytes = encode_to_vec(&original, config::standard()).expect("encode PaymentMethod::Crypto");
    let (decoded, _): (PaymentMethod, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode PaymentMethod::Crypto");
    assert_eq!(original, decoded);
    if let PaymentMethod::Crypto { coin, address } = decoded {
        assert_eq!(coin, "BTC", "coin must be preserved");
        assert_eq!(
            address, "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq",
            "address must be preserved"
        );
    } else {
        panic!("decoded variant must be Crypto");
    }
}

// ---------------------------------------------------------------------------
// Test 7: u32 serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_u32_serde_roundtrip() {
    let cases: Vec<u32> = vec![0, 1, 127, 128, 255, 256, 65535, u32::MAX];
    for original in cases {
        let bytes = encode_to_vec(&original, config::standard()).expect("encode u32");
        let (decoded, _): (u32, usize) =
            decode_owned_from_slice(&bytes, config::standard()).expect("decode u32");
        assert_eq!(original, decoded, "u32 mismatch for value {original}");
    }
}

// ---------------------------------------------------------------------------
// Test 8: String serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_string_serde_roundtrip() {
    let cases = vec![
        "".to_string(),
        "hello, oxicode!".to_string(),
        "invoice #INV-2026-001".to_string(),
        "a".repeat(1000),
    ];
    for original in cases {
        let bytes = encode_to_vec(&original, config::standard()).expect("encode String");
        let (decoded, _): (String, usize) =
            decode_owned_from_slice(&bytes, config::standard()).expect("decode String");
        assert_eq!(original, decoded, "String mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 9: Vec<u8> serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_u8_serde_roundtrip() {
    let original: Vec<u8> = vec![0u8, 1, 127, 128, 200, 255];
    let bytes = encode_to_vec(&original, config::standard()).expect("encode Vec<u8>");
    let (decoded, _): (Vec<u8>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode Vec<u8>");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 6, "Vec<u8> length must be preserved");
}

// ---------------------------------------------------------------------------
// Test 10: bool serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_bool_serde_roundtrip() {
    for original in [true, false] {
        let bytes = encode_to_vec(&original, config::standard()).expect("encode bool");
        let (decoded, _): (bool, usize) =
            decode_owned_from_slice(&bytes, config::standard()).expect("decode bool");
        assert_eq!(original, decoded, "bool mismatch for value {original}");
    }
}

// ---------------------------------------------------------------------------
// Test 11: f64 serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_f64_serde_roundtrip() {
    let cases: Vec<f64> = vec![
        0.0_f64,
        1.0_f64,
        -1.0_f64,
        std::f64::consts::PI,
        f64::MAX,
        f64::MIN_POSITIVE,
        349.97_f64,
        1_000_000.0_f64,
    ];
    for original in cases {
        let bytes = encode_to_vec(&original, config::standard()).expect("encode f64");
        let (decoded, _): (f64, usize) =
            decode_owned_from_slice(&bytes, config::standard()).expect("decode f64");
        assert_eq!(
            original.to_bits(),
            decoded.to_bits(),
            "f64 bits mismatch for {original}"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 12: Option<String> Some serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_string_some_serde_roundtrip() {
    let original: Option<String> = Some("payment reference XR-9912".to_string());
    let bytes = encode_to_vec(&original, config::standard()).expect("encode Option<String> Some");
    let (decoded, _): (Option<String>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode Option<String> Some");
    assert_eq!(original, decoded);
    assert!(decoded.is_some(), "decoded Option must remain Some");
}

// ---------------------------------------------------------------------------
// Test 13: Option<String> None serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_string_none_serde_roundtrip() {
    let original: Option<String> = None;
    let bytes = encode_to_vec(&original, config::standard()).expect("encode Option<String> None");
    let (decoded, _): (Option<String>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode Option<String> None");
    assert_eq!(original, decoded);
    assert!(decoded.is_none(), "decoded Option must remain None");
}

// ---------------------------------------------------------------------------
// Test 14: Vec<Invoice> roundtrip (3 items)
// ---------------------------------------------------------------------------

#[test]
fn test_vec_invoice_roundtrip_three_items() {
    let make_invoice = |id: &str, amount: f64, paid: bool, n_items: u32| Invoice {
        invoice_id: id.to_string(),
        amount,
        currency: "EUR".to_string(),
        line_items: (0..n_items)
            .map(|i| LineItem {
                description: format!("Item-{i}"),
                quantity: i + 1,
                unit_price: 10.0 * (i as f64 + 1.0),
            })
            .collect(),
        paid,
    };

    let original: Vec<Invoice> = vec![
        make_invoice("INV-001", 100.0, true, 1),
        make_invoice("INV-002", 250.0, false, 2),
        make_invoice("INV-003", 500.0, false, 3),
    ];
    let bytes = encode_to_vec(&original, config::standard()).expect("encode Vec<Invoice>");
    let (decoded, _): (Vec<Invoice>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode Vec<Invoice>");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 3, "must have exactly 3 invoices");
}

// ---------------------------------------------------------------------------
// Test 15: Vec<PaymentMethod> roundtrip (4 variants)
// ---------------------------------------------------------------------------

#[test]
fn test_vec_payment_method_roundtrip_all_variants() {
    let original: Vec<PaymentMethod> = vec![
        PaymentMethod::Cash,
        PaymentMethod::CreditCard {
            last_four: "1234".to_string(),
        },
        PaymentMethod::BankTransfer("SWIFT-COBADEFFXXX".to_string()),
        PaymentMethod::Crypto {
            coin: "ETH".to_string(),
            address: "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
        },
    ];
    let bytes = encode_to_vec(&original, config::standard()).expect("encode Vec<PaymentMethod>");
    let (decoded, _): (Vec<PaymentMethod>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode Vec<PaymentMethod>");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 4, "must have exactly 4 payment methods");
}

// ---------------------------------------------------------------------------
// Test 16: i64 negative serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_i64_negative_serde_roundtrip() {
    let cases: Vec<i64> = vec![i64::MIN, -1_000_000_000_000_i64, -1, -42, -127, -128];
    for original in cases {
        let bytes = encode_to_vec(&original, config::standard()).expect("encode i64 negative");
        let (decoded, _): (i64, usize) =
            decode_owned_from_slice(&bytes, config::standard()).expect("decode i64 negative");
        assert_eq!(original, decoded, "i64 mismatch for value {original}");
    }
}

// ---------------------------------------------------------------------------
// Test 17: Empty Vec<LineItem> in Invoice
// ---------------------------------------------------------------------------

#[test]
fn test_invoice_empty_line_items_roundtrip() {
    let original = Invoice {
        invoice_id: "INV-EMPTY-001".to_string(),
        amount: 0.0,
        currency: "USD".to_string(),
        line_items: vec![],
        paid: false,
    };
    let bytes =
        encode_to_vec(&original, config::standard()).expect("encode Invoice with empty line_items");
    let (decoded, _): (Invoice, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode Invoice with empty line_items");
    assert_eq!(original, decoded);
    assert!(
        decoded.line_items.is_empty(),
        "line_items must remain empty"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Invoice with unicode currency symbol
// ---------------------------------------------------------------------------

#[test]
fn test_invoice_unicode_currency_roundtrip() {
    let original = Invoice {
        invoice_id: "INV-UNICODE-2026".to_string(),
        amount: 9_999.99,
        currency: "¥".to_string(),
        line_items: vec![LineItem {
            description: "東京サービス".to_string(),
            quantity: 1,
            unit_price: 9_999.99,
        }],
        paid: true,
    };
    let bytes =
        encode_to_vec(&original, config::standard()).expect("encode Invoice with unicode currency");
    let (decoded, _): (Invoice, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode Invoice with unicode currency");
    assert_eq!(original, decoded);
    assert_eq!(
        decoded.currency, "¥",
        "unicode currency symbol must be preserved"
    );
    assert_eq!(
        decoded.line_items[0].description, "東京サービス",
        "unicode description must be preserved"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Invoice with zero amount
// ---------------------------------------------------------------------------

#[test]
fn test_invoice_zero_amount_roundtrip() {
    let original = Invoice {
        invoice_id: "INV-ZERO-001".to_string(),
        amount: 0.0,
        currency: "USD".to_string(),
        line_items: vec![LineItem {
            description: "Free Sample".to_string(),
            quantity: 1,
            unit_price: 0.0,
        }],
        paid: true,
    };
    let bytes =
        encode_to_vec(&original, config::standard()).expect("encode Invoice with zero amount");
    let (decoded, _): (Invoice, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode Invoice with zero amount");
    assert_eq!(original, decoded);
    assert_eq!(
        decoded.amount.to_bits(),
        0_f64.to_bits(),
        "zero amount bits must be preserved"
    );
    assert_eq!(
        decoded.line_items[0].unit_price.to_bits(),
        0_f64.to_bits(),
        "zero unit_price bits must be preserved"
    );
}

// ---------------------------------------------------------------------------
// Test 20: consumed bytes equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_bytes_equals_encoded_length() {
    let original = Invoice {
        invoice_id: "INV-BYTES-CHECK".to_string(),
        amount: 123.45,
        currency: "GBP".to_string(),
        line_items: vec![LineItem {
            description: "Consulting".to_string(),
            quantity: 3,
            unit_price: 41.15,
        }],
        paid: false,
    };
    let bytes =
        encode_to_vec(&original, config::standard()).expect("encode Invoice for bytes check");
    let encoded_len = bytes.len();
    let (decoded, consumed): (Invoice, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode Invoice for bytes check");
    assert_eq!(original, decoded, "decoded value must match original");
    assert_eq!(
        consumed, encoded_len,
        "consumed bytes ({consumed}) must equal encoded length ({encoded_len})"
    );
}

// ---------------------------------------------------------------------------
// Test 21: encode_to_vec with fixed_int config
// ---------------------------------------------------------------------------

#[test]
fn test_encode_to_vec_with_fixed_int_config() {
    let original = Invoice {
        invoice_id: "INV-FIXED-INT-001".to_string(),
        amount: 65535.0,
        currency: "JPY".to_string(),
        line_items: vec![LineItem {
            description: "Hardware component".to_string(),
            quantity: u32::MAX,
            unit_price: 0.01,
        }],
        paid: false,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec(&original, cfg).expect("encode Invoice with fixed_int_encoding");
    let (decoded, consumed): (Invoice, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Invoice with fixed_int_encoding");
    assert_eq!(original, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length under fixed_int config"
    );
    assert_eq!(
        decoded.line_items[0].quantity,
        u32::MAX,
        "u32::MAX quantity must survive fixed_int roundtrip"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Two equal values produce same bytes
// ---------------------------------------------------------------------------

#[test]
fn test_two_equal_values_produce_same_bytes() {
    let value_a = Invoice {
        invoice_id: "INV-DETERMINISTIC".to_string(),
        amount: 777.77,
        currency: "CHF".to_string(),
        line_items: vec![LineItem {
            description: "Advisory".to_string(),
            quantity: 7,
            unit_price: 111.11,
        }],
        paid: true,
    };
    let value_b = Invoice {
        invoice_id: "INV-DETERMINISTIC".to_string(),
        amount: 777.77,
        currency: "CHF".to_string(),
        line_items: vec![LineItem {
            description: "Advisory".to_string(),
            quantity: 7,
            unit_price: 111.11,
        }],
        paid: true,
    };
    assert_eq!(
        value_a, value_b,
        "test values must be equal before encoding"
    );
    let bytes_a = encode_to_vec(&value_a, config::standard()).expect("encode first Invoice copy");
    let bytes_b = encode_to_vec(&value_b, config::standard()).expect("encode second Invoice copy");
    assert_eq!(
        bytes_a, bytes_b,
        "two equal values must produce identical byte sequences"
    );
    assert!(!bytes_a.is_empty(), "encoded bytes must not be empty");
}
