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
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Shared test types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Transaction {
    id: u64,
    from: String,
    to: String,
    amount: f64,
    currency: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum TxStatus {
    Pending,
    Confirmed(u64),
    Failed { reason: String, code: u32 },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Ledger {
    transactions: Vec<Transaction>,
    total: f64,
    currency: String,
}

// ---------------------------------------------------------------------------
// Test 1: u64 primitive roundtrip via serde
// ---------------------------------------------------------------------------

#[test]
fn test_serde_u64_roundtrip() {
    let original: u64 = 18_446_744_073_709_551_615;
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode u64 failed");
    let (decoded, _): (u64, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode u64 failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: i64 primitive roundtrip via serde
// ---------------------------------------------------------------------------

#[test]
fn test_serde_i64_roundtrip() {
    let cases: Vec<i64> = vec![i64::MIN, -1, 0, 1, i64::MAX];
    for original in cases {
        let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode i64 failed");
        let (decoded, _): (i64, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode i64 failed");
        assert_eq!(original, decoded, "i64 mismatch for {original}");
    }
}

// ---------------------------------------------------------------------------
// Test 3: f64 primitive roundtrip via serde
// ---------------------------------------------------------------------------

#[test]
fn test_serde_f64_roundtrip() {
    let cases: Vec<f64> = vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        std::f64::consts::PI,
        std::f64::consts::E,
        f64::MAX,
        f64::MIN_POSITIVE,
        1.23456789e100_f64,
        -9.87654321e-50_f64,
    ];
    for original in cases {
        let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode f64 failed");
        let (decoded, _): (f64, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode f64 failed");
        assert_eq!(
            original.to_bits(),
            decoded.to_bits(),
            "f64 bits mismatch for {original}"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 4: bool primitive roundtrip via serde
// ---------------------------------------------------------------------------

#[test]
fn test_serde_bool_roundtrip() {
    for original in [true, false] {
        let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode bool failed");
        let (decoded, _): (bool, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode bool failed");
        assert_eq!(original, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 5: String primitive roundtrip via serde
// ---------------------------------------------------------------------------

#[test]
fn test_serde_string_roundtrip() {
    let cases = vec![
        "".to_string(),
        "hello, world".to_string(),
        "unicode: 日本語テスト αβγδ emoji 🦀".to_string(),
        "x".repeat(4096),
    ];
    for original in cases {
        let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode String failed");
        let (decoded, _): (String, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode String failed");
        assert_eq!(original, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 6: Transaction struct roundtrip with a small amount
// ---------------------------------------------------------------------------

#[test]
fn test_serde_transaction_small_amount_roundtrip() {
    let original = Transaction {
        id: 1,
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 0.01,
        currency: "USD".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Transaction small amount failed");
    let (decoded, _): (Transaction, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Transaction small amount failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Transaction struct roundtrip with a large amount
// ---------------------------------------------------------------------------

#[test]
fn test_serde_transaction_large_amount_roundtrip() {
    let original = Transaction {
        id: 9_999_999_999_u64,
        from: "treasury-account-001".to_string(),
        to: "settlement-account-999".to_string(),
        amount: 1_000_000_000.0_f64,
        currency: "EUR".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Transaction large amount failed");
    let (decoded, _): (Transaction, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Transaction large amount failed");
    assert_eq!(original, decoded);
    assert_eq!(
        decoded.amount.to_bits(),
        original.amount.to_bits(),
        "f64 amount bits mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Transaction struct roundtrip with negative/zero amount edge cases
// ---------------------------------------------------------------------------

#[test]
fn test_serde_transaction_edge_amounts_roundtrip() {
    let cases = vec![
        Transaction {
            id: 0,
            from: "a".to_string(),
            to: "b".to_string(),
            amount: 0.0,
            currency: "BTC".to_string(),
        },
        Transaction {
            id: u64::MAX,
            from: "x".to_string(),
            to: "y".to_string(),
            amount: f64::MAX,
            currency: "ETH".to_string(),
        },
        Transaction {
            id: 42,
            from: "src".to_string(),
            to: "dst".to_string(),
            amount: f64::MIN_POSITIVE,
            currency: "JPY".to_string(),
        },
    ];
    for original in cases {
        let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode Transaction edge amount failed");
        let (decoded, _): (Transaction, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode Transaction edge amount failed");
        assert_eq!(original, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 9: TxStatus::Pending roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_txstatus_pending_roundtrip() {
    let original = TxStatus::Pending;
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode TxStatus::Pending failed");
    let (decoded, _): (TxStatus, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode TxStatus::Pending failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: TxStatus::Confirmed(block_number) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_txstatus_confirmed_roundtrip() {
    let block_numbers = vec![0_u64, 1, 12_345_678, u64::MAX];
    for block in block_numbers {
        let original = TxStatus::Confirmed(block);
        let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode TxStatus::Confirmed failed");
        let (decoded, _): (TxStatus, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode TxStatus::Confirmed failed");
        assert_eq!(original, decoded, "TxStatus::Confirmed({block}) mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 11: TxStatus::Failed { reason, code } struct variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_txstatus_failed_roundtrip() {
    let cases = vec![
        TxStatus::Failed {
            reason: "insufficient funds".to_string(),
            code: 1001,
        },
        TxStatus::Failed {
            reason: "".to_string(),
            code: 0,
        },
        TxStatus::Failed {
            reason: "network timeout on node cluster 7".to_string(),
            code: u32::MAX,
        },
    ];
    for original in cases {
        let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode TxStatus::Failed failed");
        let (decoded, _): (TxStatus, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode TxStatus::Failed failed");
        assert_eq!(original, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 12: Ledger with 0 transactions roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_ledger_empty_roundtrip() {
    let original = Ledger {
        transactions: vec![],
        total: 0.0,
        currency: "USD".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode empty Ledger failed");
    let (decoded, _): (Ledger, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode empty Ledger failed");
    assert_eq!(original, decoded);
    assert!(
        decoded.transactions.is_empty(),
        "transactions must be empty after roundtrip"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Ledger with 5 transactions roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_ledger_five_transactions_roundtrip() {
    let transactions: Vec<Transaction> = (1_u64..=5)
        .map(|i| Transaction {
            id: i,
            from: format!("sender-{i:03}"),
            to: format!("receiver-{i:03}"),
            amount: i as f64 * 100.0,
            currency: "GBP".to_string(),
        })
        .collect();
    let total: f64 = transactions.iter().map(|t| t.amount).sum();
    let original = Ledger {
        transactions,
        total,
        currency: "GBP".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Ledger 5 txs failed");
    let (decoded, _): (Ledger, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Ledger 5 txs failed");
    assert_eq!(original, decoded);
    assert_eq!(
        decoded.transactions.len(),
        5,
        "must have exactly 5 transactions"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Ledger with 100 transactions roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_ledger_hundred_transactions_roundtrip() {
    let transactions: Vec<Transaction> = (1_u64..=100)
        .map(|i| Transaction {
            id: i,
            from: format!("account-from-{i:04}"),
            to: format!("account-to-{i:04}"),
            amount: (i as f64) * 9.99,
            currency: "CHF".to_string(),
        })
        .collect();
    let total: f64 = transactions.iter().map(|t| t.amount).sum();
    let original = Ledger {
        transactions,
        total,
        currency: "CHF".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Ledger 100 txs failed");
    let (decoded, _): (Ledger, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Ledger 100 txs failed");
    assert_eq!(original.transactions.len(), decoded.transactions.len());
    assert_eq!(original.currency, decoded.currency);
    assert_eq!(original.total.to_bits(), decoded.total.to_bits());
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Vec<Transaction> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_vec_transaction_roundtrip() {
    let original: Vec<Transaction> = (0_u64..30)
        .map(|i| Transaction {
            id: i,
            from: format!("wallet-{:x}", i * 0xDEAD),
            to: format!("wallet-{:x}", i * 0xBEEF),
            amount: (i as f64) * 1.5 + 0.001,
            currency: if i % 2 == 0 {
                "USD".to_string()
            } else {
                "EUR".to_string()
            },
        })
        .collect();
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Vec<Transaction> failed");
    let (decoded, _): (Vec<Transaction>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Vec<Transaction> failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 30, "must have exactly 30 transactions");
}

// ---------------------------------------------------------------------------
// Test 16: Vec<TxStatus> with all variant kinds roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_vec_txstatus_roundtrip() {
    let original: Vec<TxStatus> = vec![
        TxStatus::Pending,
        TxStatus::Confirmed(1),
        TxStatus::Failed {
            reason: "gas limit exceeded".to_string(),
            code: 4001,
        },
        TxStatus::Pending,
        TxStatus::Confirmed(8_000_000),
        TxStatus::Failed {
            reason: "nonce too low".to_string(),
            code: 4002,
        },
        TxStatus::Confirmed(u64::MAX),
    ];
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Vec<TxStatus> failed");
    let (decoded, _): (Vec<TxStatus>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Vec<TxStatus> failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Option<Transaction> Some and None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_option_transaction_roundtrip() {
    let some_tx: Option<Transaction> = Some(Transaction {
        id: 777,
        from: "optional-sender".to_string(),
        to: "optional-receiver".to_string(),
        amount: 3.14,
        currency: "XRP".to_string(),
    });
    let enc_some = oxicode::serde::encode_to_vec(&some_tx, oxicode::config::standard())
        .expect("encode Option<Transaction> Some failed");
    let (decoded_some, _): (Option<Transaction>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc_some, oxicode::config::standard())
            .expect("decode Option<Transaction> Some failed");
    assert_eq!(some_tx, decoded_some);

    let none_tx: Option<Transaction> = None;
    let enc_none = oxicode::serde::encode_to_vec(&none_tx, oxicode::config::standard())
        .expect("encode Option<Transaction> None failed");
    let (decoded_none, _): (Option<Transaction>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc_none, oxicode::config::standard())
            .expect("decode Option<Transaction> None failed");
    assert_eq!(none_tx, decoded_none);
    assert!(decoded_none.is_none(), "decoded None must remain None");
}

// ---------------------------------------------------------------------------
// Test 18: Nested types — Vec<(TxStatus, Transaction)> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_nested_txstatus_transaction_roundtrip() {
    let original: Vec<(TxStatus, Transaction)> = vec![
        (
            TxStatus::Confirmed(100),
            Transaction {
                id: 1,
                from: "alice".to_string(),
                to: "bob".to_string(),
                amount: 50.0,
                currency: "USD".to_string(),
            },
        ),
        (
            TxStatus::Pending,
            Transaction {
                id: 2,
                from: "charlie".to_string(),
                to: "dave".to_string(),
                amount: 0.001,
                currency: "BTC".to_string(),
            },
        ),
        (
            TxStatus::Failed {
                reason: "slippage too high".to_string(),
                code: 5000,
            },
            Transaction {
                id: 3,
                from: "eve".to_string(),
                to: "frank".to_string(),
                amount: 99999.99,
                currency: "EUR".to_string(),
            },
        ),
    ];
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode nested Vec<(TxStatus, Transaction)> failed");
    let (decoded, _): (Vec<(TxStatus, Transaction)>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode nested Vec<(TxStatus, Transaction)> failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Consumed bytes check — verify returned usize equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_serde_consumed_bytes_equals_encoded_length() {
    let original = Transaction {
        id: 42,
        from: "bytes-check-sender".to_string(),
        to: "bytes-check-receiver".to_string(),
        amount: 123.456,
        currency: "CAD".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Transaction for bytes check failed");
    let encoded_len = enc.len();
    let (decoded, consumed): (Transaction, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Transaction for bytes check failed");
    assert_eq!(original, decoded, "decoded value must match original");
    assert_eq!(
        consumed, encoded_len,
        "consumed bytes ({consumed}) must equal encoded length ({encoded_len})"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Large Ledger with 1000 transactions roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_ledger_thousand_transactions_roundtrip() {
    let transactions: Vec<Transaction> = (0_u64..1000)
        .map(|i| Transaction {
            id: i,
            from: format!("src-account-{i:06}"),
            to: format!("dst-account-{i:06}"),
            amount: (i as f64) * 0.123_456_789 + 1.0,
            currency: match i % 4 {
                0 => "USD",
                1 => "EUR",
                2 => "JPY",
                _ => "GBP",
            }
            .to_string(),
        })
        .collect();
    let total: f64 = transactions.iter().map(|t| t.amount).sum();
    let original = Ledger {
        transactions,
        total,
        currency: "MIXED".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Ledger 1000 txs failed");
    let (decoded, consumed): (Ledger, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Ledger 1000 txs failed");
    assert_eq!(
        original.transactions.len(),
        decoded.transactions.len(),
        "transaction count mismatch"
    );
    assert_eq!(original.currency, decoded.currency);
    assert_eq!(original.total.to_bits(), decoded.total.to_bits());
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded buffer length"
    );
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: fixed_int_encoding config — Transaction roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_transaction_fixed_int_config_roundtrip() {
    let original = Transaction {
        id: 0xCAFE_BABE_DEAD_BEEF_u64,
        from: "fixed-int-source".to_string(),
        to: "fixed-int-destination".to_string(),
        amount: 65535.0,
        currency: "XLM".to_string(),
    };
    let config = oxicode::config::standard().with_fixed_int_encoding();
    let enc = oxicode::serde::encode_to_vec(&original, config)
        .expect("encode Transaction with fixed_int_encoding failed");
    let (decoded, _): (Transaction, usize) = oxicode::serde::decode_owned_from_slice(&enc, config)
        .expect("decode Transaction with fixed_int_encoding failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: big_endian config — Ledger roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_ledger_big_endian_config_roundtrip() {
    let transactions: Vec<Transaction> = (1_u64..=10)
        .map(|i| Transaction {
            id: i * 0x0102_0304_0506_0708_u64,
            from: format!("big-endian-sender-{i}"),
            to: format!("big-endian-receiver-{i}"),
            amount: (i as f64) * std::f64::consts::PI,
            currency: "SOL".to_string(),
        })
        .collect();
    let total: f64 = transactions.iter().map(|t| t.amount).sum();
    let original = Ledger {
        transactions,
        total,
        currency: "SOL".to_string(),
    };
    let config = oxicode::config::standard().with_big_endian();
    let enc = oxicode::serde::encode_to_vec(&original, config)
        .expect("encode Ledger with big_endian config failed");
    let (decoded, _): (Ledger, usize) = oxicode::serde::decode_owned_from_slice(&enc, config)
        .expect("decode Ledger with big_endian config failed");
    assert_eq!(original.transactions.len(), decoded.transactions.len());
    assert_eq!(original.total.to_bits(), decoded.total.to_bits());
    assert_eq!(original, decoded);
}
