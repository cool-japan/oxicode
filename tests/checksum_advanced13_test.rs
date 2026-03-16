#![cfg(feature = "checksum")]
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
use oxicode::checksum::{unwrap_with_checksum, wrap_with_checksum, HEADER_SIZE};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
enum TxType {
    Transfer,
    Stake,
    Unstake,
    Mint,
    Burn,
    Governance,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Transaction {
    tx_id: u64,
    tx_type: TxType,
    from: String,
    to: String,
    amount: u64,
    fee: u64,
    nonce: u64,
    timestamp: u64,
    memo: Option<String>,
}

// Test 1: Transaction::Transfer roundtrip via checksum
#[test]
fn test_transaction_transfer_roundtrip_checksum() {
    let tx = Transaction {
        tx_id: 1,
        tx_type: TxType::Transfer,
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 1_000_000,
        fee: 100,
        nonce: 0,
        timestamp: 1_700_000_000,
        memo: Some("payment for services".to_string()),
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Transaction, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(tx, decoded);
}

// Test 2: Transaction::Stake roundtrip via checksum
#[test]
fn test_transaction_stake_roundtrip_checksum() {
    let tx = Transaction {
        tx_id: 2,
        tx_type: TxType::Stake,
        from: "validator_node_1".to_string(),
        to: "staking_pool".to_string(),
        amount: 50_000_000,
        fee: 500,
        nonce: 1,
        timestamp: 1_700_001_000,
        memo: None,
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Transaction, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(tx, decoded);
}

// Test 3: Transaction::Unstake roundtrip via checksum
#[test]
fn test_transaction_unstake_roundtrip_checksum() {
    let tx = Transaction {
        tx_id: 3,
        tx_type: TxType::Unstake,
        from: "staking_pool".to_string(),
        to: "validator_node_1".to_string(),
        amount: 25_000_000,
        fee: 250,
        nonce: 2,
        timestamp: 1_700_002_000,
        memo: Some("partial unstake".to_string()),
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Transaction, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(tx, decoded);
}

// Test 4: Transaction::Mint roundtrip via checksum
#[test]
fn test_transaction_mint_roundtrip_checksum() {
    let tx = Transaction {
        tx_id: 4,
        tx_type: TxType::Mint,
        from: "treasury".to_string(),
        to: "community_fund".to_string(),
        amount: 10_000_000_000,
        fee: 0,
        nonce: 0,
        timestamp: 1_700_003_000,
        memo: Some("genesis allocation".to_string()),
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Transaction, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(tx, decoded);
}

// Test 5: Transaction::Burn roundtrip via checksum
#[test]
fn test_transaction_burn_roundtrip_checksum() {
    let tx = Transaction {
        tx_id: 5,
        tx_type: TxType::Burn,
        from: "0xdeadbeef".to_string(),
        to: "burn_address".to_string(),
        amount: 500_000,
        fee: 50,
        nonce: 7,
        timestamp: 1_700_004_000,
        memo: None,
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Transaction, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(tx, decoded);
}

// Test 6: Transaction::Governance roundtrip via checksum
#[test]
fn test_transaction_governance_roundtrip_checksum() {
    let tx = Transaction {
        tx_id: 6,
        tx_type: TxType::Governance,
        from: "dao_member_42".to_string(),
        to: "governance_contract".to_string(),
        amount: 0,
        fee: 10,
        nonce: 3,
        timestamp: 1_700_005_000,
        memo: Some("vote: proposal #17 yes".to_string()),
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Transaction, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(tx, decoded);
}

// Test 7: HEADER_SIZE == 16
#[test]
fn test_header_size_is_16() {
    assert_eq!(HEADER_SIZE, 16);
}

// Test 8: Wrapped length == HEADER_SIZE + encoded length
#[test]
fn test_wrapped_length_equals_header_plus_encoded() {
    let tx = Transaction {
        tx_id: 100,
        tx_type: TxType::Transfer,
        from: "sender".to_string(),
        to: "receiver".to_string(),
        amount: 999,
        fee: 1,
        nonce: 5,
        timestamp: 1_710_000_000,
        memo: None,
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    assert_eq!(wrapped.len(), HEADER_SIZE + raw.len());
}

// Test 9: Bit-flip corruption → Err (aggressive: flip all bytes after index 4)
#[test]
fn test_corruption_detected_aggressive_bit_flip() {
    let tx = Transaction {
        tx_id: 200,
        tx_type: TxType::Stake,
        from: "attacker".to_string(),
        to: "victim".to_string(),
        amount: 9_999_999,
        fee: 999,
        nonce: 42,
        timestamp: 1_720_000_000,
        memo: Some("corrupted".to_string()),
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let mut corrupted = wrapped.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = unwrap_with_checksum(&corrupted);
    assert!(
        result.is_err(),
        "aggressively corrupted data must return Err"
    );
}

// Test 10: Total replacement → Err (replace with zeros)
#[test]
fn test_complete_data_replacement_detected() {
    let tx = Transaction {
        tx_id: 300,
        tx_type: TxType::Burn,
        from: "origin".to_string(),
        to: "null_address".to_string(),
        amount: 1,
        fee: 0,
        nonce: 0,
        timestamp: 0,
        memo: None,
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let replacement = vec![0u8; wrapped.len()];
    let result = unwrap_with_checksum(&replacement);
    assert!(result.is_err(), "completely replaced data must return Err");
}

// Test 11: Truncation → Err (remove last byte)
#[test]
fn test_truncated_data_detected() {
    let tx = Transaction {
        tx_id: 400,
        tx_type: TxType::Mint,
        from: "minter".to_string(),
        to: "recipient".to_string(),
        amount: 100_000,
        fee: 10,
        nonce: 1,
        timestamp: 1_730_000_000,
        memo: Some("truncation test".to_string()),
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let truncated = &wrapped[..wrapped.len() - 1];
    let result = unwrap_with_checksum(truncated);
    assert!(result.is_err(), "truncated data must return Err");
}

// Test 12: Vec<Transaction> roundtrip via checksum
#[test]
fn test_vec_of_transactions_roundtrip_checksum() {
    let txs = vec![
        Transaction {
            tx_id: 1001,
            tx_type: TxType::Transfer,
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 500,
            fee: 5,
            nonce: 0,
            timestamp: 1_700_010_000,
            memo: None,
        },
        Transaction {
            tx_id: 1002,
            tx_type: TxType::Governance,
            from: "charlie".to_string(),
            to: "dao".to_string(),
            amount: 0,
            fee: 1,
            nonce: 1,
            timestamp: 1_700_010_001,
            memo: Some("vote yes".to_string()),
        },
        Transaction {
            tx_id: 1003,
            tx_type: TxType::Unstake,
            from: "pool".to_string(),
            to: "dave".to_string(),
            amount: 1_000_000,
            fee: 100,
            nonce: 2,
            timestamp: 1_700_010_002,
            memo: None,
        },
    ];
    let raw = encode_to_vec(&txs).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Vec<Transaction>, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(txs, decoded);
}

// Test 13: Option<Transaction> Some via checksum
#[test]
fn test_option_some_transaction_roundtrip_checksum() {
    let val: Option<Transaction> = Some(Transaction {
        tx_id: 5000,
        tx_type: TxType::Transfer,
        from: "eve".to_string(),
        to: "frank".to_string(),
        amount: 42_000,
        fee: 42,
        nonce: 99,
        timestamp: 1_740_000_000,
        memo: Some("option some test".to_string()),
    });
    let raw = encode_to_vec(&val).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Option<Transaction>, _) =
        decode_from_slice(&payload).expect("decode failed");
    assert_eq!(val, decoded);
}

// Test 14: Option<Transaction> None via checksum
#[test]
fn test_option_none_transaction_roundtrip_checksum() {
    let val: Option<Transaction> = None;
    let raw = encode_to_vec(&val).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Option<Transaction>, _) =
        decode_from_slice(&payload).expect("decode failed");
    assert_eq!(val, decoded);
}

// Test 15: Transaction with None memo
#[test]
fn test_transaction_none_memo_roundtrip() {
    let tx = Transaction {
        tx_id: 7777,
        tx_type: TxType::Stake,
        from: "staker_1".to_string(),
        to: "validator_set".to_string(),
        amount: 2_000_000,
        fee: 200,
        nonce: 10,
        timestamp: 1_750_000_000,
        memo: None,
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Transaction, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(tx, decoded);
    assert!(decoded.memo.is_none());
}

// Test 16: Transaction with Some memo (long string)
#[test]
fn test_transaction_long_memo_roundtrip() {
    let long_memo = "This transaction represents a cross-chain atomic swap between two sovereign blockchain networks, executed via a hashed timelock contract with a 24-hour expiration window. ".repeat(10);
    let tx = Transaction {
        tx_id: 8888,
        tx_type: TxType::Transfer,
        from: "cross_chain_bridge_alpha".to_string(),
        to: "cross_chain_bridge_beta".to_string(),
        amount: 999_999_999,
        fee: 9_999,
        nonce: 1337,
        timestamp: 1_760_000_000,
        memo: Some(long_memo),
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Transaction, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(tx, decoded);
    assert!(decoded.memo.as_ref().expect("memo must be Some").len() > 100);
}

// Test 17: Deterministic: wrap same data twice → identical output
#[test]
fn test_wrap_same_data_twice_identical_output() {
    let tx = Transaction {
        tx_id: 9999,
        tx_type: TxType::Governance,
        from: "proposer".to_string(),
        to: "governance_contract".to_string(),
        amount: 0,
        fee: 5,
        nonce: 77,
        timestamp: 1_770_000_000,
        memo: Some("determinism check: proposal #99".to_string()),
    };
    let raw1 = encode_to_vec(&tx).expect("encode first failed");
    let raw2 = encode_to_vec(&tx).expect("encode second failed");
    let wrapped1 = wrap_with_checksum(&raw1);
    let wrapped2 = wrap_with_checksum(&raw2);
    assert_eq!(
        wrapped1, wrapped2,
        "wrapping identical data must be deterministic"
    );
}

// Test 18: u64::MAX amount and fee roundtrip
#[test]
fn test_u64_max_amount_and_fee_roundtrip() {
    let tx = Transaction {
        tx_id: u64::MAX,
        tx_type: TxType::Mint,
        from: "max_minter".to_string(),
        to: "max_recipient".to_string(),
        amount: u64::MAX,
        fee: u64::MAX,
        nonce: u64::MAX,
        timestamp: u64::MAX,
        memo: None,
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Transaction, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(tx.amount, decoded.amount);
    assert_eq!(u64::MAX, decoded.amount);
    assert_eq!(u64::MAX, decoded.fee);
}

// Test 19: Consumed bytes == wrapped length
#[test]
fn test_consumed_bytes_equals_wrapped_length() {
    let tx = Transaction {
        tx_id: 11111,
        tx_type: TxType::Transfer,
        from: "sender_consumed".to_string(),
        to: "receiver_consumed".to_string(),
        amount: 12345,
        fee: 12,
        nonce: 6,
        timestamp: 1_780_000_000,
        memo: Some("consumed bytes test".to_string()),
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (_, consumed) = decode_from_slice::<Transaction>(&payload).expect("decode failed");
    assert_eq!(wrapped.len(), HEADER_SIZE + consumed);
}

// Test 20: Unwrap gives back exact encoded bytes
#[test]
fn test_unwrap_gives_back_exact_encoded_bytes() {
    let tx = Transaction {
        tx_id: 22222,
        tx_type: TxType::Unstake,
        from: "unstaker".to_string(),
        to: "wallet".to_string(),
        amount: 333_333,
        fee: 33,
        nonce: 11,
        timestamp: 1_790_000_000,
        memo: None,
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    assert_eq!(
        raw, payload,
        "unwrapped payload must equal original encoded bytes"
    );
}

// Test 21: Unicode from/to addresses roundtrip
#[test]
fn test_unicode_from_to_addresses_roundtrip() {
    let tx = Transaction {
        tx_id: 33333,
        tx_type: TxType::Transfer,
        from: "送信者_アリス_🔑".to_string(),
        to: "受取人_ボブ_💎".to_string(),
        amount: 88_888,
        fee: 88,
        nonce: 0,
        timestamp: 1_800_000_000,
        memo: Some("国際送金 — international transfer ₿".to_string()),
    };
    let raw = encode_to_vec(&tx).expect("encode failed");
    let wrapped = wrap_with_checksum(&raw);
    let payload = unwrap_with_checksum(&wrapped).expect("unwrap failed");
    let (decoded, _): (Transaction, _) = decode_from_slice(&payload).expect("decode failed");
    assert_eq!(tx, decoded);
    assert_eq!(decoded.from, "送信者_アリス_🔑");
    assert_eq!(decoded.to, "受取人_ボブ_💎");
}

// Test 22: All 6 TxType variants encode to different bytes (via normal encode_to_vec)
#[test]
fn test_all_tx_type_variants_encode_to_different_bytes() {
    let variants = [
        TxType::Transfer,
        TxType::Stake,
        TxType::Unstake,
        TxType::Mint,
        TxType::Burn,
        TxType::Governance,
    ];
    let encoded: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode variant failed"))
        .collect();
    for i in 0..encoded.len() {
        for j in (i + 1)..encoded.len() {
            assert_ne!(
                encoded[i], encoded[j],
                "TxType variants at index {} and {} must encode differently",
                i, j
            );
        }
    }
    assert_eq!(encoded.len(), 6);
}
