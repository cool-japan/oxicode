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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TransactionType {
    Transfer,
    Mint,
    Burn,
    Stake,
    Unstake,
    Swap,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CryptoTransaction {
    tx_hash: String,
    from: String,
    to: String,
    amount: u64,
    fee: u64,
    tx_type: TransactionType,
    nonce: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BlockHeader {
    height: u64,
    prev_hash: String,
    tx_root: String,
    timestamp: u64,
    difficulty: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Block {
    header: BlockHeader,
    transactions: Vec<CryptoTransaction>,
}

fn make_transfer_tx() -> CryptoTransaction {
    CryptoTransaction {
        tx_hash: "0xabc123def456abc123def456abc123def456abc123def456abc123def456abc1".to_string(),
        from: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        to: "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string(),
        amount: 1_000_000_000_000_000_000,
        fee: 21_000,
        tx_type: TransactionType::Transfer,
        nonce: 42,
    }
}

fn make_block_header(height: u64) -> BlockHeader {
    BlockHeader {
        height,
        prev_hash: "0x0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        tx_root: "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string(),
        timestamp: 1_700_000_000,
        difficulty: 0x1d00ffff,
    }
}

#[test]
fn test_transaction_type_transfer_roundtrip() {
    let cfg = config::standard();
    let tx_type = TransactionType::Transfer;
    let bytes = encode_to_vec(&tx_type, cfg).expect("encode TransactionType::Transfer");
    let (decoded, _): (TransactionType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TransactionType::Transfer");
    assert_eq!(tx_type, decoded);
}

#[test]
fn test_transaction_type_mint_roundtrip() {
    let cfg = config::standard();
    let tx_type = TransactionType::Mint;
    let bytes = encode_to_vec(&tx_type, cfg).expect("encode TransactionType::Mint");
    let (decoded, _): (TransactionType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TransactionType::Mint");
    assert_eq!(tx_type, decoded);
}

#[test]
fn test_transaction_type_burn_roundtrip() {
    let cfg = config::standard();
    let tx_type = TransactionType::Burn;
    let bytes = encode_to_vec(&tx_type, cfg).expect("encode TransactionType::Burn");
    let (decoded, _): (TransactionType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TransactionType::Burn");
    assert_eq!(tx_type, decoded);
}

#[test]
fn test_transaction_type_stake_roundtrip() {
    let cfg = config::standard();
    let tx_type = TransactionType::Stake;
    let bytes = encode_to_vec(&tx_type, cfg).expect("encode TransactionType::Stake");
    let (decoded, _): (TransactionType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TransactionType::Stake");
    assert_eq!(tx_type, decoded);
}

#[test]
fn test_transaction_type_unstake_roundtrip() {
    let cfg = config::standard();
    let tx_type = TransactionType::Unstake;
    let bytes = encode_to_vec(&tx_type, cfg).expect("encode TransactionType::Unstake");
    let (decoded, _): (TransactionType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TransactionType::Unstake");
    assert_eq!(tx_type, decoded);
}

#[test]
fn test_transaction_type_swap_roundtrip() {
    let cfg = config::standard();
    let tx_type = TransactionType::Swap;
    let bytes = encode_to_vec(&tx_type, cfg).expect("encode TransactionType::Swap");
    let (decoded, _): (TransactionType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TransactionType::Swap");
    assert_eq!(tx_type, decoded);
}

#[test]
fn test_crypto_transaction_transfer_standard_config() {
    let cfg = config::standard();
    let tx = make_transfer_tx();
    let bytes = encode_to_vec(&tx, cfg).expect("encode CryptoTransaction Transfer");
    let (decoded, consumed): (CryptoTransaction, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CryptoTransaction Transfer");
    assert_eq!(tx, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_crypto_transaction_mint_standard_config() {
    let cfg = config::standard();
    let tx = CryptoTransaction {
        tx_hash: "0xmint000000000000000000000000000000000000000000000000000000000000".to_string(),
        from: "0x0000000000000000000000000000000000000000".to_string(),
        to: "0xrecipient000000000000000000000000000000".to_string(),
        amount: 500_000_000,
        fee: 0,
        tx_type: TransactionType::Mint,
        nonce: 1,
    };
    let bytes = encode_to_vec(&tx, cfg).expect("encode CryptoTransaction Mint");
    let (decoded, _): (CryptoTransaction, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CryptoTransaction Mint");
    assert_eq!(tx, decoded);
}

#[test]
fn test_crypto_transaction_burn_big_endian() {
    let cfg = config::standard().with_big_endian();
    let tx = CryptoTransaction {
        tx_hash: "0xburn00000000000000000000000000000000000000000000000000000000burn".to_string(),
        from: "0xburner0000000000000000000000000000000000".to_string(),
        to: "0x0000000000000000000000000000000000000000".to_string(),
        amount: 999_999,
        fee: 5_000,
        tx_type: TransactionType::Burn,
        nonce: 7,
    };
    let bytes = encode_to_vec(&tx, cfg).expect("encode CryptoTransaction Burn big-endian");
    let (decoded, _): (CryptoTransaction, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CryptoTransaction Burn big-endian");
    assert_eq!(tx, decoded);
}

#[test]
fn test_crypto_transaction_stake_fixed_int_encoding() {
    let cfg = config::standard().with_fixed_int_encoding();
    let tx = CryptoTransaction {
        tx_hash: "0xstake0000000000000000000000000000000000000000000000000000000000".to_string(),
        from: "0xstaker000000000000000000000000000000000".to_string(),
        to: "0xvalidator0000000000000000000000000000000".to_string(),
        amount: 3_200_000_000_000_000_000,
        fee: 10_000,
        tx_type: TransactionType::Stake,
        nonce: 100,
    };
    let bytes = encode_to_vec(&tx, cfg).expect("encode CryptoTransaction Stake fixed-int");
    let (decoded, _): (CryptoTransaction, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CryptoTransaction Stake fixed-int");
    assert_eq!(tx, decoded);
}

#[test]
fn test_crypto_transaction_swap_legacy_config() {
    let cfg = config::legacy();
    let tx = CryptoTransaction {
        tx_hash: "0xswap000000000000000000000000000000000000000000000000000000000000".to_string(),
        from: "0xswapper00000000000000000000000000000000".to_string(),
        to: "0xpool000000000000000000000000000000000000".to_string(),
        amount: 100_000,
        fee: 300,
        tx_type: TransactionType::Swap,
        nonce: 5,
    };
    let bytes = encode_to_vec(&tx, cfg).expect("encode CryptoTransaction Swap legacy");
    let (decoded, _): (CryptoTransaction, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CryptoTransaction Swap legacy");
    assert_eq!(tx, decoded);
}

#[test]
fn test_block_header_roundtrip_standard() {
    let cfg = config::standard();
    let header = make_block_header(100_000);
    let bytes = encode_to_vec(&header, cfg).expect("encode BlockHeader standard");
    let (decoded, consumed): (BlockHeader, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BlockHeader standard");
    assert_eq!(header, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_block_header_big_endian_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let header = BlockHeader {
        height: 840_000,
        prev_hash: "0x00000000000000000002a7c4c1e48d76c5a37902395a1c0d10f1523fb58174ef".to_string(),
        tx_root: "0x4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b".to_string(),
        timestamp: 1_231_006_505,
        difficulty: 486_604_799,
    };
    let bytes = encode_to_vec(&header, cfg).expect("encode BlockHeader big-endian");
    let (decoded, _): (BlockHeader, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BlockHeader big-endian");
    assert_eq!(header, decoded);
}

#[test]
fn test_block_header_fixed_int_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let header = make_block_header(1);
    let bytes = encode_to_vec(&header, cfg).expect("encode BlockHeader fixed-int");
    let (decoded, _): (BlockHeader, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BlockHeader fixed-int");
    assert_eq!(header, decoded);
}

#[test]
fn test_empty_block_roundtrip() {
    let cfg = config::standard();
    let block = Block {
        header: make_block_header(0),
        transactions: Vec::new(),
    };
    let bytes = encode_to_vec(&block, cfg).expect("encode empty Block");
    let (decoded, consumed): (Block, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode empty Block");
    assert_eq!(block, decoded);
    assert_eq!(consumed, bytes.len());
    assert!(decoded.transactions.is_empty());
}

#[test]
fn test_block_with_single_transaction() {
    let cfg = config::standard();
    let block = Block {
        header: make_block_header(500),
        transactions: vec![make_transfer_tx()],
    };
    let bytes = encode_to_vec(&block, cfg).expect("encode Block with single tx");
    let (decoded, _): (Block, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Block with single tx");
    assert_eq!(block, decoded);
    assert_eq!(decoded.transactions.len(), 1);
}

#[test]
fn test_block_with_multiple_transaction_types() {
    let cfg = config::standard();
    let transactions = vec![
        CryptoTransaction {
            tx_hash: "0xhash_transfer_000000000000000000000000000000000000000000000000".to_string(),
            from: "0xalice0000000000000000000000000000000000".to_string(),
            to: "0xbob00000000000000000000000000000000000".to_string(),
            amount: 1_000,
            fee: 10,
            tx_type: TransactionType::Transfer,
            nonce: 1,
        },
        CryptoTransaction {
            tx_hash: "0xhash_mint_000000000000000000000000000000000000000000000000000".to_string(),
            from: "0x0000000000000000000000000000000000000000".to_string(),
            to: "0xalice0000000000000000000000000000000000".to_string(),
            amount: 5_000_000,
            fee: 0,
            tx_type: TransactionType::Mint,
            nonce: 2,
        },
        CryptoTransaction {
            tx_hash: "0xhash_stake_00000000000000000000000000000000000000000000000000".to_string(),
            from: "0xalice0000000000000000000000000000000000".to_string(),
            to: "0xvalidator0000000000000000000000000000000".to_string(),
            amount: 3_200_000,
            fee: 100,
            tx_type: TransactionType::Stake,
            nonce: 3,
        },
    ];
    let block = Block {
        header: make_block_header(1_000),
        transactions,
    };
    let bytes = encode_to_vec(&block, cfg).expect("encode Block with multiple tx types");
    let (decoded, _): (Block, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Block with multiple tx types");
    assert_eq!(block, decoded);
    assert_eq!(decoded.transactions.len(), 3);
    assert_eq!(decoded.transactions[0].tx_type, TransactionType::Transfer);
    assert_eq!(decoded.transactions[1].tx_type, TransactionType::Mint);
    assert_eq!(decoded.transactions[2].tx_type, TransactionType::Stake);
}

#[test]
fn test_vec_of_transactions_roundtrip() {
    let cfg = config::standard();
    let txs: Vec<CryptoTransaction> = vec![
        CryptoTransaction {
            tx_hash: "0xaaaa".to_string(),
            from: "0x1111".to_string(),
            to: "0x2222".to_string(),
            amount: 100,
            fee: 1,
            tx_type: TransactionType::Transfer,
            nonce: 0,
        },
        CryptoTransaction {
            tx_hash: "0xbbbb".to_string(),
            from: "0x3333".to_string(),
            to: "0x4444".to_string(),
            amount: 200,
            fee: 2,
            tx_type: TransactionType::Burn,
            nonce: 1,
        },
        CryptoTransaction {
            tx_hash: "0xcccc".to_string(),
            from: "0x5555".to_string(),
            to: "0x6666".to_string(),
            amount: 300,
            fee: 3,
            tx_type: TransactionType::Unstake,
            nonce: 2,
        },
        CryptoTransaction {
            tx_hash: "0xdddd".to_string(),
            from: "0x7777".to_string(),
            to: "0x8888".to_string(),
            amount: 400,
            fee: 4,
            tx_type: TransactionType::Swap,
            nonce: 3,
        },
    ];
    let bytes = encode_to_vec(&txs, cfg).expect("encode Vec<CryptoTransaction>");
    let (decoded, consumed): (Vec<CryptoTransaction>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<CryptoTransaction>");
    assert_eq!(txs, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_genesis_block_height_zero() {
    let cfg = config::standard();
    let genesis = Block {
        header: BlockHeader {
            height: 0,
            prev_hash: "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            tx_root: "0x4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"
                .to_string(),
            timestamp: 1_231_006_505,
            difficulty: 0x1d00ffff,
        },
        transactions: Vec::new(),
    };
    let bytes = encode_to_vec(&genesis, cfg).expect("encode genesis Block");
    let (decoded, _): (Block, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode genesis Block");
    assert_eq!(genesis, decoded);
    assert_eq!(decoded.header.height, 0);
}

#[test]
fn test_transaction_max_values() {
    let cfg = config::standard();
    let tx = CryptoTransaction {
        tx_hash: "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string(),
        from: "0xffffffffffffffffffffffffffffffffffffffff".to_string(),
        to: "0xffffffffffffffffffffffffffffffffffffffff".to_string(),
        amount: u64::MAX,
        fee: u64::MAX,
        tx_type: TransactionType::Transfer,
        nonce: u64::MAX,
    };
    let bytes = encode_to_vec(&tx, cfg).expect("encode CryptoTransaction max values");
    let (decoded, _): (CryptoTransaction, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode CryptoTransaction max values");
    assert_eq!(tx, decoded);
    assert_eq!(decoded.amount, u64::MAX);
    assert_eq!(decoded.nonce, u64::MAX);
}

#[test]
fn test_block_big_endian_fixed_int_combined() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let block = Block {
        header: make_block_header(21_000_000),
        transactions: vec![CryptoTransaction {
            tx_hash: "0xcombined_test_hash_0000000000000000000000000000000000000000000".to_string(),
            from: "0xsender000000000000000000000000000000000".to_string(),
            to: "0xreceiver00000000000000000000000000000000".to_string(),
            amount: 2_100_000_000_000_000,
            fee: 50_000,
            tx_type: TransactionType::Transfer,
            nonce: 999,
        }],
    };
    let bytes = encode_to_vec(&block, cfg).expect("encode Block big-endian fixed-int");
    let (decoded, consumed): (Block, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Block big-endian fixed-int");
    assert_eq!(block, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_block_encode_size_increases_with_transactions() {
    let cfg = config::standard();
    let empty_block = Block {
        header: make_block_header(1),
        transactions: Vec::new(),
    };
    let single_tx_block = Block {
        header: make_block_header(1),
        transactions: vec![make_transfer_tx()],
    };
    let two_tx_block = Block {
        header: make_block_header(1),
        transactions: vec![make_transfer_tx(), make_transfer_tx()],
    };
    let empty_bytes = encode_to_vec(&empty_block, cfg).expect("encode empty block");
    let single_bytes = encode_to_vec(&single_tx_block, cfg).expect("encode single-tx block");
    let two_bytes = encode_to_vec(&two_tx_block, cfg).expect("encode two-tx block");
    assert!(empty_bytes.len() < single_bytes.len());
    assert!(single_bytes.len() < two_bytes.len());
}
