//! Tests for Fintech / payment processing enums — advanced roundtrip coverage.
//!
//! Domain types model a simplified payment processing system with payment methods,
//! card brands, transaction statuses, and payment records — exercising complex
//! enum variants (named-field, newtype, and unit variants) across a variety of
//! OxiCode config options, consumed-byte checks, and vec roundtrips.

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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CardBrand {
    Visa,
    Mastercard,
    Amex,
    Discover,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PaymentMethod {
    Card { last_four: u16, brand: CardBrand },
    BankTransfer { account_hash: u64 },
    Wallet(String),
    Crypto { coin: String, network: u8 },
    Cash,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TransactionStatus {
    Pending,
    Processing,
    Completed,
    Failed { code: u32, message: String },
    Refunded { amount: u64 },
    Disputed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Payment {
    payment_id: u64,
    amount: u64,
    currency: String,
    method: PaymentMethod,
    status: TransactionStatus,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn roundtrip_payment_method(method: &PaymentMethod) -> PaymentMethod {
    let bytes = encode_to_vec(method).expect("encode PaymentMethod");
    let (decoded, _): (PaymentMethod, usize) =
        decode_from_slice(&bytes).expect("decode PaymentMethod");
    decoded
}

fn roundtrip_transaction_status(status: &TransactionStatus) -> TransactionStatus {
    let bytes = encode_to_vec(status).expect("encode TransactionStatus");
    let (decoded, _): (TransactionStatus, usize) =
        decode_from_slice(&bytes).expect("decode TransactionStatus");
    decoded
}

fn roundtrip_payment(payment: &Payment) -> Payment {
    let bytes = encode_to_vec(payment).expect("encode Payment");
    let (decoded, _): (Payment, usize) = decode_from_slice(&bytes).expect("decode Payment");
    decoded
}

// ---------------------------------------------------------------------------
// Test 1: PaymentMethod::Cash (unit variant)
// ---------------------------------------------------------------------------

#[test]
fn test_payment_method_cash_roundtrip() {
    let method = PaymentMethod::Cash;
    let decoded = roundtrip_payment_method(&method);
    assert_eq!(method, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: PaymentMethod::Wallet (newtype variant with String)
// ---------------------------------------------------------------------------

#[test]
fn test_payment_method_wallet_roundtrip() {
    let method = PaymentMethod::Wallet("ApplePay".to_string());
    let decoded = roundtrip_payment_method(&method);
    assert_eq!(method, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: PaymentMethod::BankTransfer (named-field with u64)
// ---------------------------------------------------------------------------

#[test]
fn test_payment_method_bank_transfer_roundtrip() {
    let method = PaymentMethod::BankTransfer {
        account_hash: 0xDEAD_BEEF_CAFE_0001,
    };
    let decoded = roundtrip_payment_method(&method);
    assert_eq!(method, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: PaymentMethod::Crypto (named-field with String + u8)
// ---------------------------------------------------------------------------

#[test]
fn test_payment_method_crypto_roundtrip() {
    let method = PaymentMethod::Crypto {
        coin: "BTC".to_string(),
        network: 1,
    };
    let decoded = roundtrip_payment_method(&method);
    assert_eq!(method, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: PaymentMethod::Card — Visa brand
// ---------------------------------------------------------------------------

#[test]
fn test_payment_method_card_visa_roundtrip() {
    let method = PaymentMethod::Card {
        last_four: 4242,
        brand: CardBrand::Visa,
    };
    let decoded = roundtrip_payment_method(&method);
    assert_eq!(method, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: All CardBrand variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_card_brand_all_variants_roundtrip() {
    let brands = [
        CardBrand::Visa,
        CardBrand::Mastercard,
        CardBrand::Amex,
        CardBrand::Discover,
        CardBrand::Unknown,
    ];
    for brand in &brands {
        let bytes = encode_to_vec(brand).expect("encode CardBrand");
        let (decoded, consumed): (CardBrand, usize) =
            decode_from_slice(&bytes).expect("decode CardBrand");
        assert_eq!(brand, &decoded);
        assert_eq!(consumed, bytes.len());
    }
}

// ---------------------------------------------------------------------------
// Test 7: TransactionStatus::Pending and Processing (unit variants)
// ---------------------------------------------------------------------------

#[test]
fn test_transaction_status_unit_variants_roundtrip() {
    let statuses = [
        TransactionStatus::Pending,
        TransactionStatus::Processing,
        TransactionStatus::Completed,
        TransactionStatus::Disputed,
    ];
    for status in &statuses {
        let decoded = roundtrip_transaction_status(status);
        assert_eq!(status, &decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 8: TransactionStatus::Failed with error code and message
// ---------------------------------------------------------------------------

#[test]
fn test_transaction_status_failed_roundtrip() {
    let status = TransactionStatus::Failed {
        code: 4001,
        message: "Insufficient funds".to_string(),
    };
    let decoded = roundtrip_transaction_status(&status);
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: TransactionStatus::Refunded with amount
// ---------------------------------------------------------------------------

#[test]
fn test_transaction_status_refunded_roundtrip() {
    let status = TransactionStatus::Refunded { amount: 9_999_999 };
    let decoded = roundtrip_transaction_status(&status);
    assert_eq!(status, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Full Payment struct roundtrip — card payment completed
// ---------------------------------------------------------------------------

#[test]
fn test_payment_struct_card_completed_roundtrip() {
    let payment = Payment {
        payment_id: 100_000_001,
        amount: 4999,
        currency: "USD".to_string(),
        method: PaymentMethod::Card {
            last_four: 1234,
            brand: CardBrand::Mastercard,
        },
        status: TransactionStatus::Completed,
    };
    let decoded = roundtrip_payment(&payment);
    assert_eq!(payment, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Full Payment struct roundtrip — crypto payment pending
// ---------------------------------------------------------------------------

#[test]
fn test_payment_struct_crypto_pending_roundtrip() {
    let payment = Payment {
        payment_id: 200_000_002,
        amount: 1_500_000,
        currency: "SATS".to_string(),
        method: PaymentMethod::Crypto {
            coin: "ETH".to_string(),
            network: 137,
        },
        status: TransactionStatus::Pending,
    };
    let decoded = roundtrip_payment(&payment);
    assert_eq!(payment, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Consumed bytes equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_bytes_equals_encoded_length() {
    let payment = Payment {
        payment_id: 55,
        amount: 100,
        currency: "EUR".to_string(),
        method: PaymentMethod::Cash,
        status: TransactionStatus::Completed,
    };
    let bytes = encode_to_vec(&payment).expect("encode Payment");
    let (_, consumed): (Payment, usize) = decode_from_slice(&bytes).expect("decode Payment");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Vec<Payment> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_payment_roundtrip() {
    let payments = vec![
        Payment {
            payment_id: 1,
            amount: 500,
            currency: "USD".to_string(),
            method: PaymentMethod::Cash,
            status: TransactionStatus::Completed,
        },
        Payment {
            payment_id: 2,
            amount: 9800,
            currency: "GBP".to_string(),
            method: PaymentMethod::Wallet("GooglePay".to_string()),
            status: TransactionStatus::Processing,
        },
        Payment {
            payment_id: 3,
            amount: 250_000,
            currency: "JPY".to_string(),
            method: PaymentMethod::BankTransfer {
                account_hash: 0xABCD_0000_1234_5678,
            },
            status: TransactionStatus::Failed {
                code: 5002,
                message: "Account frozen".to_string(),
            },
        },
    ];
    let bytes = encode_to_vec(&payments).expect("encode Vec<Payment>");
    let (decoded, consumed): (Vec<Payment>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Payment>");
    assert_eq!(payments, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 14: Config big_endian roundtrip — Payment
// ---------------------------------------------------------------------------

#[test]
fn test_payment_big_endian_config_roundtrip() {
    let payment = Payment {
        payment_id: 7_777_777,
        amount: 12_345,
        currency: "CAD".to_string(),
        method: PaymentMethod::Card {
            last_four: 9999,
            brand: CardBrand::Amex,
        },
        status: TransactionStatus::Completed,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec_with_config(&payment, cfg).expect("encode big_endian");
    let (decoded, consumed): (Payment, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big_endian");
    assert_eq!(payment, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 15: Config fixed_int roundtrip — Payment
// ---------------------------------------------------------------------------

#[test]
fn test_payment_fixed_int_config_roundtrip() {
    let payment = Payment {
        payment_id: 3_000_000,
        amount: 88_000,
        currency: "AUD".to_string(),
        method: PaymentMethod::BankTransfer {
            account_hash: 0x1234_5678_9ABC_DEF0,
        },
        status: TransactionStatus::Refunded { amount: 88_000 },
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&payment, cfg).expect("encode fixed_int");
    let (decoded, consumed): (Payment, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed_int");
    assert_eq!(payment, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 16: Standard config vs legacy config produce different bytes for Payment
// ---------------------------------------------------------------------------

#[test]
fn test_standard_vs_legacy_config_bytes_differ() {
    let payment = Payment {
        payment_id: 1_000_000,
        amount: 50_000,
        currency: "USD".to_string(),
        method: PaymentMethod::Cash,
        status: TransactionStatus::Completed,
    };
    let standard_bytes =
        encode_to_vec_with_config(&payment, config::standard()).expect("encode standard");
    let legacy_bytes =
        encode_to_vec_with_config(&payment, config::legacy()).expect("encode legacy");
    // Standard uses varint encoding; legacy uses fixed-width — bytes must differ.
    assert_ne!(
        standard_bytes, legacy_bytes,
        "standard and legacy configs must produce different byte representations"
    );
}

// ---------------------------------------------------------------------------
// Test 17: TransactionStatus::Failed with very long message
// ---------------------------------------------------------------------------

#[test]
fn test_transaction_status_failed_long_message_roundtrip() {
    let long_message = "FRAUD_DETECTED: The transaction was flagged by our anti-fraud system \
        due to unusual spending patterns on account ending in 0001. \
        Please contact customer support at support@payments.example.com \
        reference number TXN-2026-03-15-00000001."
        .to_string();
    let status = TransactionStatus::Failed {
        code: 9999,
        message: long_message.clone(),
    };
    let decoded = roundtrip_transaction_status(&status);
    match decoded {
        TransactionStatus::Failed { code, message } => {
            assert_eq!(code, 9999);
            assert_eq!(message, long_message);
        }
        other => panic!("expected Failed variant, got {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// Test 18: TransactionStatus::Refunded with max u64 amount
// ---------------------------------------------------------------------------

#[test]
fn test_transaction_status_refunded_max_amount() {
    let status = TransactionStatus::Refunded { amount: u64::MAX };
    let decoded = roundtrip_transaction_status(&status);
    match decoded {
        TransactionStatus::Refunded { amount } => {
            assert_eq!(amount, u64::MAX);
        }
        other => panic!("expected Refunded variant, got {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// Test 19: PaymentMethod discriminant uniqueness — each variant encodes differently
// ---------------------------------------------------------------------------

#[test]
fn test_payment_method_discriminant_uniqueness() {
    let methods = [
        PaymentMethod::Card {
            last_four: 0,
            brand: CardBrand::Visa,
        },
        PaymentMethod::BankTransfer { account_hash: 0 },
        PaymentMethod::Wallet(String::new()),
        PaymentMethod::Crypto {
            coin: String::new(),
            network: 0,
        },
        PaymentMethod::Cash,
    ];
    let encoded: Vec<Vec<u8>> = methods
        .iter()
        .map(|m| encode_to_vec(m).expect("encode PaymentMethod variant"))
        .collect();
    // Verify that no two variants produce identical byte encodings.
    for i in 0..encoded.len() {
        for j in (i + 1)..encoded.len() {
            assert_ne!(
                encoded[i], encoded[j],
                "PaymentMethod variants {} and {} must have distinct encodings",
                i, j
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 20: TransactionStatus discriminant uniqueness
// ---------------------------------------------------------------------------

#[test]
fn test_transaction_status_discriminant_uniqueness() {
    let statuses = [
        TransactionStatus::Pending,
        TransactionStatus::Processing,
        TransactionStatus::Completed,
        TransactionStatus::Failed {
            code: 0,
            message: String::new(),
        },
        TransactionStatus::Refunded { amount: 0 },
        TransactionStatus::Disputed,
    ];
    let encoded: Vec<Vec<u8>> = statuses
        .iter()
        .map(|s| encode_to_vec(s).expect("encode TransactionStatus variant"))
        .collect();
    for i in 0..encoded.len() {
        for j in (i + 1)..encoded.len() {
            assert_ne!(
                encoded[i], encoded[j],
                "TransactionStatus variants {} and {} must have distinct encodings",
                i, j
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 21: Payment with Amex Discover cards and all statuses — batch roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_payment_all_method_status_combinations_batch() {
    let methods = [
        PaymentMethod::Card {
            last_four: 3782,
            brand: CardBrand::Amex,
        },
        PaymentMethod::Card {
            last_four: 6011,
            brand: CardBrand::Discover,
        },
        PaymentMethod::Card {
            last_four: 0000,
            brand: CardBrand::Unknown,
        },
    ];
    let statuses = [
        TransactionStatus::Pending,
        TransactionStatus::Disputed,
        TransactionStatus::Refunded { amount: 10_000 },
    ];
    let mut payment_id: u64 = 90_000;
    for method in &methods {
        for status in &statuses {
            payment_id += 1;
            let payment = Payment {
                payment_id,
                amount: payment_id * 100,
                currency: "USD".to_string(),
                method: method.clone(),
                status: status.clone(),
            };
            let decoded = roundtrip_payment(&payment);
            assert_eq!(
                payment, decoded,
                "roundtrip failed for payment_id={}",
                payment_id
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 22: Payment fixed_int big_endian combined config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_payment_combined_fixed_int_big_endian_roundtrip() {
    let payment = Payment {
        payment_id: 0xFFFF_FFFF_0000_0001,
        amount: u64::MAX / 2,
        currency: "CHF".to_string(),
        method: PaymentMethod::Crypto {
            coin: "XMR".to_string(),
            network: 255,
        },
        status: TransactionStatus::Failed {
            code: u32::MAX,
            message: "Terminal failure: gateway unreachable".to_string(),
        },
    };
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let bytes = encode_to_vec_with_config(&payment, cfg).expect("encode combined config");
    let (decoded, consumed): (Payment, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode combined config");
    assert_eq!(payment, decoded);
    assert_eq!(consumed, bytes.len());
}
