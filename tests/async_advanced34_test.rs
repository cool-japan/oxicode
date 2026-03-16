#![cfg(feature = "async-tokio")]
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
use oxicode::async_tokio::{AsyncDecoder, AsyncEncoder};
use oxicode::streaming::StreamingConfig;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use tokio::io::duplex;

// ---------------------------------------------------------------------------
// Blockchain / distributed ledger transaction domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TxType {
    Transfer,
    Stake,
    Unstake,
    Swap,
    Deploy,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Transaction {
    tx_id: u64,
    tx_type: TxType,
    from: String,
    to: String,
    amount: u64,
    fee: u64,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Block {
    block_number: u64,
    transactions: Vec<Transaction>,
    hash: u64,
    prev_hash: u64,
}

// ---------------------------------------------------------------------------
// Helper: build a sample Transaction
// ---------------------------------------------------------------------------

fn make_tx(
    id: u64,
    tx_type: TxType,
    from: &str,
    to: &str,
    amount: u64,
    fee: u64,
    ts: u64,
) -> Transaction {
    Transaction {
        tx_id: id,
        tx_type,
        from: from.to_string(),
        to: to.to_string(),
        amount,
        fee,
        timestamp: ts,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Single Transaction roundtrip via duplex channel
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_single_transaction_duplex_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let original = make_tx(
            1,
            TxType::Transfer,
            "0xALICE",
            "0xBOB",
            1_000_000,
            100,
            1_700_000_000,
        );

        let (writer, reader) = duplex(65536);
        let original_clone = original.clone();

        let write_task = tokio::spawn(async move {
            let mut encoder = AsyncEncoder::new(writer);
            encoder.write_item(&original_clone).await.expect("write");
            encoder.finish().await.expect("finish");
        });

        let read_task = tokio::spawn(async move {
            let mut decoder = AsyncDecoder::new(reader);
            decoder.read_item().await.expect("read").expect("some")
        });

        write_task.await.expect("write task");
        let decoded = read_task.await.expect("read task");
        assert_eq!(original, decoded);
    });
}

// ---------------------------------------------------------------------------
// Test 2: TxType::Transfer variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_txtype_transfer_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let original = make_tx(
            2,
            TxType::Transfer,
            "0xSENDER",
            "0xRECEIVER",
            500,
            10,
            1_700_000_001,
        );

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_item(&original).await.expect("write");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded = decoder.read_item().await.expect("read").expect("some");

        assert_eq!(original, decoded);
        assert_eq!(decoded.tx_type, TxType::Transfer);
    });
}

// ---------------------------------------------------------------------------
// Test 3: TxType::Stake variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_txtype_stake_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let original = make_tx(
            3,
            TxType::Stake,
            "0xVALIDATOR1",
            "0xSTAKING_CONTRACT",
            10_000_000,
            500,
            1_700_000_002,
        );

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_item(&original).await.expect("write");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded = decoder.read_item().await.expect("read").expect("some");

        assert_eq!(original, decoded);
        assert_eq!(decoded.tx_type, TxType::Stake);
        assert_eq!(decoded.amount, 10_000_000);
    });
}

// ---------------------------------------------------------------------------
// Test 4: TxType::Unstake variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_txtype_unstake_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let original = make_tx(
            4,
            TxType::Unstake,
            "0xVALIDATOR1",
            "0xSTAKING_CONTRACT",
            5_000_000,
            250,
            1_700_000_003,
        );

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_item(&original).await.expect("write");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded = decoder.read_item().await.expect("read").expect("some");

        assert_eq!(original, decoded);
        assert_eq!(decoded.tx_type, TxType::Unstake);
    });
}

// ---------------------------------------------------------------------------
// Test 5: TxType::Swap variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_txtype_swap_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let original = make_tx(
            5,
            TxType::Swap,
            "0xDEX_ROUTER",
            "0xLIQUIDITY_POOL",
            2_500_000,
            75,
            1_700_000_004,
        );

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_item(&original).await.expect("write");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded = decoder.read_item().await.expect("read").expect("some");

        assert_eq!(original, decoded);
        assert_eq!(decoded.tx_type, TxType::Swap);
    });
}

// ---------------------------------------------------------------------------
// Test 6: TxType::Deploy variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_txtype_deploy_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let original = make_tx(
            6,
            TxType::Deploy,
            "0xDEPLOYER",
            "0x0000000000000000",
            0,
            5_000,
            1_700_000_005,
        );

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_item(&original).await.expect("write");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded = decoder.read_item().await.expect("read").expect("some");

        assert_eq!(original, decoded);
        assert_eq!(decoded.tx_type, TxType::Deploy);
        assert_eq!(decoded.amount, 0);
    });
}

// ---------------------------------------------------------------------------
// Test 7: Block roundtrip with multiple transactions
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_block_roundtrip() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let txs = vec![
            make_tx(
                100,
                TxType::Transfer,
                "0xALICE",
                "0xBOB",
                1_000,
                10,
                1_700_001_000,
            ),
            make_tx(
                101,
                TxType::Stake,
                "0xCHARLIE",
                "0xSC",
                5_000,
                50,
                1_700_001_001,
            ),
            make_tx(
                102,
                TxType::Swap,
                "0xDEX",
                "0xPOOL",
                2_000,
                20,
                1_700_001_002,
            ),
        ];
        let original = Block {
            block_number: 42_000,
            transactions: txs,
            hash: 0xDEADBEEFCAFEBABE,
            prev_hash: 0xCAFEBABEDEADBEEF,
        };

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_item(&original).await.expect("write");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded = decoder.read_item().await.expect("read").expect("some");

        assert_eq!(original, decoded);
        assert_eq!(decoded.block_number, 42_000);
        assert_eq!(decoded.transactions.len(), 3);
        assert_eq!(decoded.hash, 0xDEADBEEFCAFEBABE);
    });
}

// ---------------------------------------------------------------------------
// Test 8: Batch transactions (10 items) write_all / read_all
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_batch_transactions_write_all_read_all() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let txs: Vec<Transaction> = (0u64..10)
            .map(|i| {
                make_tx(
                    1_000 + i,
                    TxType::Transfer,
                    "0xSRC",
                    "0xDST",
                    i * 100,
                    i,
                    1_700_002_000 + i,
                )
            })
            .collect();

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_all(txs.clone()).await.expect("write_all");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded: Vec<Transaction> = decoder.read_all().await.expect("read_all");

        assert_eq!(txs, decoded);
        assert_eq!(decoded.len(), 10);
    });
}

// ---------------------------------------------------------------------------
// Test 9: Empty stream returns None (no transactions written)
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_empty_stream_returns_none() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let encoder = AsyncEncoder::new(cursor);
            encoder.finish().await.expect("finish empty");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let result = decoder
            .read_item::<Transaction>()
            .await
            .expect("read empty");

        assert!(result.is_none(), "expected None from empty ledger stream");
    });
}

// ---------------------------------------------------------------------------
// Test 10: Large batch — 50 transactions all survive roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_large_batch_50_transactions() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let txs: Vec<Transaction> = (0u64..50)
            .map(|i| {
                let tx_type = match i % 5 {
                    0 => TxType::Transfer,
                    1 => TxType::Stake,
                    2 => TxType::Unstake,
                    3 => TxType::Swap,
                    _ => TxType::Deploy,
                };
                make_tx(
                    2_000 + i,
                    tx_type,
                    &format!("0xSRC_{}", i),
                    &format!("0xDST_{}", i),
                    i * 1_000,
                    i * 10 + 1,
                    1_700_003_000 + i,
                )
            })
            .collect();

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder
                .write_all(txs.clone())
                .await
                .expect("write_all large");
            encoder.finish().await.expect("finish large");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded: Vec<Transaction> = decoder.read_all().await.expect("read_all large");

        assert_eq!(decoded.len(), 50);
        assert_eq!(txs, decoded);
    });
}

// ---------------------------------------------------------------------------
// Test 11: Progress check — items_processed > 0 after encoding transactions
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_progress_items_processed() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let config = StreamingConfig::default().with_flush_per_item(true);
        let txs: Vec<Transaction> = (0u64..5)
            .map(|i| {
                make_tx(
                    3_000 + i,
                    TxType::Transfer,
                    "0xA",
                    "0xB",
                    i * 50,
                    5,
                    1_700_004_000 + i,
                )
            })
            .collect();

        let mut buf = Vec::<u8>::new();
        let cursor = Cursor::new(&mut buf);
        let mut encoder = AsyncEncoder::with_config(cursor, config);

        for tx in &txs {
            encoder.write_item(tx).await.expect("write item");
        }

        assert!(
            encoder.progress().items_processed > 0,
            "expected items_processed > 0 after writes with flush_per_item"
        );

        encoder.finish().await.expect("finish");
    });
}

// ---------------------------------------------------------------------------
// Test 12: write_all with duplex channel for blockchain transactions
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_write_all_via_duplex() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let txs: Vec<Transaction> = (0u64..8)
            .map(|i| {
                make_tx(
                    4_000 + i,
                    TxType::Swap,
                    "0xDEX",
                    "0xPOOL",
                    i * 200,
                    20,
                    1_700_005_000 + i,
                )
            })
            .collect();

        let (writer, reader) = duplex(65536);
        let txs_to_write = txs.clone();

        let write_task = tokio::spawn(async move {
            let mut encoder = AsyncEncoder::new(writer);
            encoder
                .write_all(txs_to_write)
                .await
                .expect("write_all duplex");
            encoder.finish().await.expect("finish duplex");
        });

        let read_task = tokio::spawn(async move {
            let mut decoder = AsyncDecoder::new(reader);
            decoder.read_all().await.expect("read_all duplex")
        });

        write_task.await.expect("write task");
        let decoded: Vec<Transaction> = read_task.await.expect("read task");

        assert_eq!(txs, decoded);
        assert_eq!(decoded.len(), 8);
    });
}

// ---------------------------------------------------------------------------
// Test 13: All TxType variants in one batch roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_all_tx_types_in_one_batch() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let txs = vec![
            make_tx(5_001, TxType::Transfer, "0xA", "0xB", 100, 5, 1_700_006_001),
            make_tx(5_002, TxType::Stake, "0xC", "0xSC", 500, 25, 1_700_006_002),
            make_tx(
                5_003,
                TxType::Unstake,
                "0xC",
                "0xSC",
                500,
                25,
                1_700_006_003,
            ),
            make_tx(
                5_004,
                TxType::Swap,
                "0xDEX",
                "0xPOOL",
                250,
                12,
                1_700_006_004,
            ),
            make_tx(
                5_005,
                TxType::Deploy,
                "0xDEPLOYER",
                "0x0",
                0,
                5_000,
                1_700_006_005,
            ),
        ];

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder
                .write_all(txs.clone())
                .await
                .expect("write_all variants");
            encoder.finish().await.expect("finish variants");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded: Vec<Transaction> = decoder.read_all().await.expect("read_all variants");

        assert_eq!(txs.len(), decoded.len());
        assert_eq!(decoded[0].tx_type, TxType::Transfer);
        assert_eq!(decoded[1].tx_type, TxType::Stake);
        assert_eq!(decoded[2].tx_type, TxType::Unstake);
        assert_eq!(decoded[3].tx_type, TxType::Swap);
        assert_eq!(decoded[4].tx_type, TxType::Deploy);
        assert_eq!(txs, decoded);
    });
}

// ---------------------------------------------------------------------------
// Test 14: Concurrent reads via duplex (writer and reader in parallel tasks)
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_concurrent_reads_via_duplex() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let txs: Vec<Transaction> = (0u64..15)
            .map(|i| {
                make_tx(
                    6_000 + i,
                    TxType::Transfer,
                    "0xP2P_SRC",
                    "0xP2P_DST",
                    i * 333,
                    33,
                    1_700_007_000 + i,
                )
            })
            .collect();

        let (writer, reader) = duplex(65536);
        let txs_to_write = txs.clone();

        let write_task = tokio::spawn(async move {
            let mut encoder = AsyncEncoder::new(writer);
            for tx in &txs_to_write {
                encoder.write_item(tx).await.expect("write item concurrent");
            }
            encoder.finish().await.expect("finish concurrent");
        });

        let read_task = tokio::spawn(async move {
            let mut decoder = AsyncDecoder::new(reader);
            let mut results = Vec::new();
            while let Some(item) = decoder.read_item().await.expect("read concurrent") {
                results.push(item);
            }
            results
        });

        write_task.await.expect("write task concurrent");
        let decoded: Vec<Transaction> = read_task.await.expect("read task concurrent");

        assert_eq!(txs, decoded);
        assert_eq!(decoded.len(), 15);
    });
}

// ---------------------------------------------------------------------------
// Test 15: Transaction with max amount and max fee (boundary values)
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_transaction_max_amount_and_fee() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let original = make_tx(
            7_001,
            TxType::Transfer,
            "0xWHALE",
            "0xEXCHANGE",
            u64::MAX,
            u64::MAX,
            u64::MAX,
        );

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_item(&original).await.expect("write");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded = decoder.read_item().await.expect("read").expect("some");

        assert_eq!(original, decoded);
        assert_eq!(decoded.amount, u64::MAX);
        assert_eq!(decoded.fee, u64::MAX);
        assert_eq!(decoded.timestamp, u64::MAX);
    });
}

// ---------------------------------------------------------------------------
// Test 16: Transaction with zero amount and zero fee (coinbase-style)
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_transaction_zero_amount_zero_fee() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let original = make_tx(
            7_002,
            TxType::Deploy,
            "0xMINER",
            "0xCOINBASE_CONTRACT",
            0,
            0,
            1_700_008_000,
        );

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_item(&original).await.expect("write");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded = decoder.read_item().await.expect("read").expect("some");

        assert_eq!(original, decoded);
        assert_eq!(decoded.amount, 0);
        assert_eq!(decoded.fee, 0);
    });
}

// ---------------------------------------------------------------------------
// Test 17: Block with empty transactions list roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_block_with_no_transactions() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let original = Block {
            block_number: 0,
            transactions: vec![],
            hash: 0x0000000000000000,
            prev_hash: 0xFFFFFFFFFFFFFFFF,
        };

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_item(&original).await.expect("write");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded = decoder.read_item().await.expect("read").expect("some");

        assert_eq!(original, decoded);
        assert!(
            decoded.transactions.is_empty(),
            "expected no transactions in genesis-like block"
        );
    });
}

// ---------------------------------------------------------------------------
// Test 18: Block with 20 transactions roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_block_with_20_transactions() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let txs: Vec<Transaction> = (0u64..20)
            .map(|i| {
                let tx_type = if i % 2 == 0 {
                    TxType::Transfer
                } else {
                    TxType::Stake
                };
                make_tx(
                    8_000 + i,
                    tx_type,
                    &format!("0xADDR_{}", i),
                    "0xCONTRACT",
                    i * 50,
                    5,
                    1_700_009_000 + i,
                )
            })
            .collect();

        let original = Block {
            block_number: 100_000,
            transactions: txs,
            hash: 0xABCDEF0123456789,
            prev_hash: 0x9876543210FEDCBA,
        };

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_item(&original).await.expect("write");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded = decoder.read_item().await.expect("read").expect("some");

        assert_eq!(original, decoded);
        assert_eq!(decoded.transactions.len(), 20);
        assert_eq!(decoded.block_number, 100_000);
    });
}

// ---------------------------------------------------------------------------
// Test 19: Sequential read_item calls on multi-transaction stream
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_sequential_read_item_by_item() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let txs = vec![
            make_tx(9_001, TxType::Transfer, "0xA", "0xB", 100, 1, 1_700_010_001),
            make_tx(9_002, TxType::Stake, "0xC", "0xD", 200, 2, 1_700_010_002),
            make_tx(9_003, TxType::Unstake, "0xE", "0xF", 300, 3, 1_700_010_003),
        ];

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_all(txs.clone()).await.expect("write_all seq");
            encoder.finish().await.expect("finish seq");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);

        for (idx, expected) in txs.iter().enumerate() {
            let item = decoder
                .read_item()
                .await
                .unwrap_or_else(|e| panic!("read_item[{}] failed: {}", idx, e))
                .unwrap_or_else(|| panic!("expected Some at index {}", idx));
            assert_eq!(expected, &item, "mismatch at index {}", idx);
        }

        let eof = decoder.read_item::<Transaction>().await.expect("eof read");
        assert!(
            eof.is_none(),
            "expected None after all transactions consumed"
        );
    });
}

// ---------------------------------------------------------------------------
// Test 20: Transaction with unicode addresses (long from/to strings)
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_transaction_long_unicode_addresses() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let long_from = "0x".to_string() + &"a1b2c3d4e5f6".repeat(20);
        let long_to = "0x".to_string() + &"f6e5d4c3b2a1".repeat(20);

        let original = make_tx(
            10_001,
            TxType::Swap,
            &long_from,
            &long_to,
            999_999,
            999,
            1_700_011_001,
        );

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_item(&original).await.expect("write");
            encoder.finish().await.expect("finish");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded = decoder.read_item().await.expect("read").expect("some");

        assert_eq!(original, decoded);
        assert_eq!(decoded.from, long_from);
        assert_eq!(decoded.to, long_to);
    });
}

// ---------------------------------------------------------------------------
// Test 21: Sync encode_to_vec / decode_from_slice consistency with async result
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_sync_async_consistency() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let original = make_tx(
            11_001,
            TxType::Transfer,
            "0xSYNC_SRC",
            "0xSYNC_DST",
            42_000,
            420,
            1_700_012_001,
        );

        // Sync roundtrip
        let sync_bytes = encode_to_vec(&original).expect("encode_to_vec");
        let (sync_decoded, _): (Transaction, _) =
            decode_from_slice(&sync_bytes).expect("decode_from_slice");
        assert_eq!(original, sync_decoded, "sync roundtrip mismatch");

        // Async streaming roundtrip
        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder.write_item(&original).await.expect("write async");
            encoder.finish().await.expect("finish async");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let async_decoded = decoder
            .read_item()
            .await
            .expect("read async")
            .expect("some async");

        assert_eq!(original, async_decoded, "async roundtrip mismatch");
        assert_eq!(
            sync_decoded, async_decoded,
            "sync and async results must match"
        );
    });
}

// ---------------------------------------------------------------------------
// Test 22: Multiple blocks streamed in sequence (blockchain chain simulation)
// ---------------------------------------------------------------------------

#[test]
fn test_ledger_multiple_blocks_chain_sequence() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        use std::io::Cursor;

        let blocks: Vec<Block> = (0u64..5)
            .map(|i| {
                let txs = (0u64..3)
                    .map(|j| {
                        make_tx(
                            i * 100 + j,
                            TxType::Transfer,
                            "0xMINER",
                            "0xREWARD",
                            j * 10,
                            1,
                            1_700_013_000 + i * 100 + j,
                        )
                    })
                    .collect();
                Block {
                    block_number: i,
                    transactions: txs,
                    hash: i * 0x1234567890ABCDEF,
                    prev_hash: if i == 0 {
                        0
                    } else {
                        (i - 1) * 0x1234567890ABCDEF
                    },
                }
            })
            .collect();

        let mut buf = Vec::<u8>::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncEncoder::new(cursor);
            encoder
                .write_all(blocks.clone())
                .await
                .expect("write_all blocks");
            encoder.finish().await.expect("finish blocks");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncDecoder::new(cursor);
        let decoded: Vec<Block> = decoder.read_all().await.expect("read_all blocks");

        assert_eq!(blocks.len(), decoded.len());
        assert_eq!(decoded[0].block_number, 0);
        assert_eq!(decoded[4].block_number, 4);
        // Verify chain linkage: each block's prev_hash matches previous block's hash
        for i in 1..decoded.len() {
            assert_eq!(
                decoded[i].prev_hash,
                decoded[i - 1].hash,
                "chain linkage broken at block {}",
                i
            );
        }
        assert_eq!(blocks, decoded);
    });
}
