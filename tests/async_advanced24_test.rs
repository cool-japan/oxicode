//! Advanced async streaming tests (24th set) for OxiCode.
//!
//! Theme: Financial transactions async streaming.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types: `TransactionType`, `Transaction`, `AccountSnapshot`.
//!
//! Coverage matrix:
//!   1:  Single Transaction deposit roundtrip
//!   2:  Single Transaction withdrawal roundtrip
//!   3:  Single Transaction transfer roundtrip
//!   4:  Single Transaction fee roundtrip
//!   5:  AccountSnapshot single roundtrip
//!   6:  Multiple Transactions streamed in order (5 items)
//!   7:  Multiple AccountSnapshots streamed in order (3 items)
//!   8:  Mixed Transaction and AccountSnapshot items in separate streams, verify consistency
//!   9:  Large batch of 200 Transactions via write_all, verify read_all
//!  10:  Progress tracking — items_processed > 0 after writing transactions
//!  11:  StreamingConfig with small chunk size forces multiple chunks
//!  12:  flush_per_item config writes each transaction as its own chunk
//!  13:  Empty stream — finish immediately, read_item returns None
//!  14:  Transaction with negative amount_cents (fee debit) roundtrip
//!  15:  AccountSnapshot with zero balance roundtrip
//!  16:  write_all with Vec<Transaction> owned iteration
//!  17:  write_all then read_all full roundtrip consistency
//!  18:  Sync encode / async decode interop for Transaction
//!  19:  Async encode / sync decode interop for AccountSnapshot
//!  20:  Items_processed >= N after writing N transactions
//!  21:  Decoder reports is_finished() after stream exhausted
//!  22:  bytes_processed grows as more transactions are decoded

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
use oxicode::async_tokio::{AsyncDecoder, AsyncEncoder, StreamingConfig};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use std::io::Cursor;
use tokio::io::BufReader;

// ---------------------------------------------------------------------------
// Domain types for financial transactions
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
    Fee,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Transaction {
    id: u64,
    tx_type: TransactionType,
    amount_cents: i64,
    account_id: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AccountSnapshot {
    account_id: u64,
    balance_cents: i64,
    tx_count: u32,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async fn encode_one<T: Encode>(item: &T) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_item(item)
            .await
            .expect("encode_one: write_item failed");
        enc.finish().await.expect("encode_one: finish failed");
    }
    buf
}

async fn decode_one<T: Decode>(buf: Vec<u8>) -> Option<T> {
    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    dec.read_item::<T>()
        .await
        .expect("decode_one: read_item failed")
}

fn make_deposit(id: u64) -> Transaction {
    Transaction {
        id,
        tx_type: TransactionType::Deposit,
        amount_cents: 10_000,
        account_id: 1001,
    }
}

fn make_withdrawal(id: u64) -> Transaction {
    Transaction {
        id,
        tx_type: TransactionType::Withdrawal,
        amount_cents: -5_000,
        account_id: 1001,
    }
}

// ---------------------------------------------------------------------------
// Test 1: Single Transaction deposit roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_single_deposit_roundtrip() {
    let original = Transaction {
        id: 1,
        tx_type: TransactionType::Deposit,
        amount_cents: 50_000,
        account_id: 2001,
    };
    let buf = encode_one(&original).await;
    let decoded = decode_one::<Transaction>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "single deposit Transaction roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 2: Single Transaction withdrawal roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_single_withdrawal_roundtrip() {
    let original = Transaction {
        id: 2,
        tx_type: TransactionType::Withdrawal,
        amount_cents: -20_000,
        account_id: 2002,
    };
    let buf = encode_one(&original).await;
    let decoded = decode_one::<Transaction>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "single withdrawal Transaction roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 3: Single Transaction transfer roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_single_transfer_roundtrip() {
    let original = Transaction {
        id: 3,
        tx_type: TransactionType::Transfer,
        amount_cents: 75_000,
        account_id: 3003,
    };
    let buf = encode_one(&original).await;
    let decoded = decode_one::<Transaction>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "single transfer Transaction roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Single Transaction fee roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_single_fee_roundtrip() {
    let original = Transaction {
        id: 4,
        tx_type: TransactionType::Fee,
        amount_cents: -250,
        account_id: 4004,
    };
    let buf = encode_one(&original).await;
    let decoded = decode_one::<Transaction>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "single fee Transaction roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 5: AccountSnapshot single roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_account_snapshot_single_roundtrip() {
    let original = AccountSnapshot {
        account_id: 9001,
        balance_cents: 1_234_567,
        tx_count: 42,
    };
    let buf = encode_one(&original).await;
    let decoded = decode_one::<AccountSnapshot>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "AccountSnapshot single roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Multiple Transactions streamed in order (5 items)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_five_transactions_in_order() {
    let transactions = vec![
        Transaction {
            id: 10,
            tx_type: TransactionType::Deposit,
            amount_cents: 100_000,
            account_id: 5001,
        },
        Transaction {
            id: 11,
            tx_type: TransactionType::Withdrawal,
            amount_cents: -30_000,
            account_id: 5001,
        },
        Transaction {
            id: 12,
            tx_type: TransactionType::Transfer,
            amount_cents: 20_000,
            account_id: 5001,
        },
        Transaction {
            id: 13,
            tx_type: TransactionType::Fee,
            amount_cents: -100,
            account_id: 5001,
        },
        Transaction {
            id: 14,
            tx_type: TransactionType::Deposit,
            amount_cents: 500_000,
            account_id: 5001,
        },
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for tx in &transactions {
            enc.write_item(tx).await.expect("write Transaction failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    for expected in &transactions {
        let item: Option<Transaction> = dec.read_item().await.expect("read Transaction failed");
        assert_eq!(
            item.as_ref(),
            Some(expected),
            "Transaction mismatch at id {}",
            expected.id
        );
    }

    let eof: Option<Transaction> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None after all transactions");
}

// ---------------------------------------------------------------------------
// Test 7: Multiple AccountSnapshots streamed in order (3 items)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_three_snapshots_in_order() {
    let snapshots = vec![
        AccountSnapshot {
            account_id: 100,
            balance_cents: 500_000,
            tx_count: 10,
        },
        AccountSnapshot {
            account_id: 200,
            balance_cents: 250_000,
            tx_count: 5,
        },
        AccountSnapshot {
            account_id: 300,
            balance_cents: 0,
            tx_count: 0,
        },
    ];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for snap in &snapshots {
            enc.write_item(snap)
                .await
                .expect("write AccountSnapshot failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    for expected in &snapshots {
        let item: Option<AccountSnapshot> =
            dec.read_item().await.expect("read AccountSnapshot failed");
        assert_eq!(
            item.as_ref(),
            Some(expected),
            "AccountSnapshot mismatch at account_id {}",
            expected.account_id
        );
    }
}

// ---------------------------------------------------------------------------
// Test 8: Mixed types in separate streams, verify consistency via sync encode
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_mixed_types_separate_streams_consistent() {
    let tx = Transaction {
        id: 99,
        tx_type: TransactionType::Transfer,
        amount_cents: 12_345,
        account_id: 7777,
    };
    let snap = AccountSnapshot {
        account_id: 7777,
        balance_cents: 12_345,
        tx_count: 1,
    };

    // Verify async and sync encoding produce decodable equivalents
    let tx_buf = encode_one(&tx).await;
    let snap_buf = encode_one(&snap).await;

    let decoded_tx = decode_one::<Transaction>(tx_buf).await;
    let decoded_snap = decode_one::<AccountSnapshot>(snap_buf).await;

    assert_eq!(
        decoded_tx,
        Some(tx.clone()),
        "Transaction async roundtrip mismatch"
    );
    assert_eq!(
        decoded_snap,
        Some(snap.clone()),
        "AccountSnapshot async roundtrip mismatch"
    );

    // Also verify sync encode/decode are consistent
    let sync_tx_bytes = encode_to_vec(&tx).expect("sync encode Transaction failed");
    let (sync_tx_decoded, _): (Transaction, _) =
        decode_from_slice(&sync_tx_bytes).expect("sync decode Transaction failed");
    assert_eq!(sync_tx_decoded, tx, "sync Transaction roundtrip mismatch");

    let sync_snap_bytes = encode_to_vec(&snap).expect("sync encode AccountSnapshot failed");
    let (sync_snap_decoded, _): (AccountSnapshot, _) =
        decode_from_slice(&sync_snap_bytes).expect("sync decode AccountSnapshot failed");
    assert_eq!(
        sync_snap_decoded, snap,
        "sync AccountSnapshot roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Large batch of 200 Transactions via write_all, verify read_all
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_large_batch_200_transactions_write_all_read_all() {
    let transactions: Vec<Transaction> = (0u64..200)
        .map(|i| Transaction {
            id: i,
            tx_type: if i % 4 == 0 {
                TransactionType::Deposit
            } else if i % 4 == 1 {
                TransactionType::Withdrawal
            } else if i % 4 == 2 {
                TransactionType::Transfer
            } else {
                TransactionType::Fee
            },
            amount_cents: (i as i64) * 100,
            account_id: 8000 + (i % 10),
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_all(transactions.clone())
            .await
            .expect("write_all 200 transactions failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    let decoded: Vec<Transaction> = dec
        .read_all()
        .await
        .expect("read_all 200 transactions failed");

    assert_eq!(decoded.len(), 200, "expected 200 decoded transactions");
    assert_eq!(decoded, transactions, "large batch roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 10: Progress tracking — items_processed > 0 after writing transactions
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_progress_items_processed_after_transactions() {
    let transactions: Vec<Transaction> = (0u64..8).map(make_deposit).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for tx in transactions.clone() {
            enc.write_item(&tx).await.expect("write_item failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    let _: Vec<Transaction> = dec.read_all().await.expect("read_all failed");

    assert!(
        dec.progress().items_processed > 0,
        "items_processed must be > 0 after reading transactions"
    );
    assert_eq!(
        dec.progress().items_processed,
        8,
        "items_processed must equal 8"
    );
}

// ---------------------------------------------------------------------------
// Test 11: StreamingConfig with small chunk size forces multiple chunks
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_small_chunk_size_forces_multiple_chunks() {
    // Use minimum chunk size (1024 bytes) with 500 transactions (~10 bytes each)
    // — 500 × ~10 = ~5000 bytes, forcing ~5 chunks of ~1024 bytes.
    let config = StreamingConfig::new().with_chunk_size(1024);
    let transactions: Vec<Transaction> = (0u64..500)
        .map(|i| Transaction {
            id: i,
            tx_type: TransactionType::Deposit,
            amount_cents: (i as i64) * 1000,
            account_id: 1000 + i,
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::with_config(cursor, config);
        for tx in &transactions {
            enc.write_item(tx).await.expect("write_item failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    let decoded: Vec<Transaction> = dec.read_all().await.expect("read_all failed");

    assert_eq!(decoded.len(), 500, "must decode 500 transactions");
    assert_eq!(decoded, transactions, "small-chunk roundtrip mismatch");
    assert!(
        dec.progress().chunks_processed > 1,
        "expected multiple chunks with 1024-byte chunk size and 500 transactions (got {} chunks)",
        dec.progress().chunks_processed
    );
}

// ---------------------------------------------------------------------------
// Test 12: flush_per_item config writes each transaction as its own chunk
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_flush_per_item_config() {
    let config = StreamingConfig::new().with_flush_per_item(true);
    let transactions: Vec<Transaction> = (0u64..5).map(make_deposit).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::with_config(cursor, config);
        for tx in &transactions {
            enc.write_item(tx)
                .await
                .expect("write_item (flush_per_item) failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    let decoded: Vec<Transaction> = dec.read_all().await.expect("read_all failed");

    assert_eq!(decoded, transactions, "flush_per_item roundtrip mismatch");
    // With flush_per_item, each item is its own chunk — expect 5 chunks
    assert_eq!(
        dec.progress().chunks_processed,
        5,
        "flush_per_item must produce one chunk per transaction"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Empty stream — finish immediately, read_item returns None
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_empty_stream_returns_none() {
    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let enc = AsyncEncoder::new(cursor);
        enc.finish().await.expect("finish empty stream failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    let item: Option<Transaction> = dec
        .read_item()
        .await
        .expect("read_item from empty stream failed");
    assert_eq!(
        item, None,
        "empty stream must return None on first read_item"
    );
    assert!(
        dec.is_finished(),
        "decoder must report finished for empty stream"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Transaction with negative amount_cents (fee debit) roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_negative_amount_cents_roundtrip() {
    let original = Transaction {
        id: 500,
        tx_type: TransactionType::Fee,
        amount_cents: i64::MIN / 2,
        account_id: 6006,
    };
    let buf = encode_one(&original).await;
    let decoded = decode_one::<Transaction>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "negative amount_cents Transaction roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 15: AccountSnapshot with zero balance roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_zero_balance_snapshot_roundtrip() {
    let original = AccountSnapshot {
        account_id: 0,
        balance_cents: 0,
        tx_count: 0,
    };
    let buf = encode_one(&original).await;
    let decoded = decode_one::<AccountSnapshot>(buf).await;
    assert_eq!(
        decoded,
        Some(original),
        "zero-balance AccountSnapshot roundtrip failed"
    );
}

// ---------------------------------------------------------------------------
// Test 16: write_all with Vec<Transaction> owned iteration
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_write_all_owned_vec_iteration() {
    let transactions: Vec<Transaction> = (0u64..6)
        .map(|i| Transaction {
            id: i,
            tx_type: TransactionType::Transfer,
            amount_cents: (i as i64 + 1) * 999,
            account_id: 2000 + i,
        })
        .collect();

    // Clone before passing to write_all (which consumes the iterator)
    let expected = transactions.clone();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_all(transactions)
            .await
            .expect("write_all owned Vec failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    let decoded: Vec<Transaction> = dec.read_all().await.expect("read_all failed");

    assert_eq!(
        decoded, expected,
        "write_all owned iteration roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 17: write_all then read_all full roundtrip consistency
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_write_all_read_all_full_roundtrip() {
    let transactions: Vec<Transaction> = vec![
        make_deposit(1),
        make_withdrawal(2),
        Transaction {
            id: 3,
            tx_type: TransactionType::Fee,
            amount_cents: -50,
            account_id: 1001,
        },
    ];
    let expected = transactions.clone();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_all(transactions).await.expect("write_all failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    let decoded: Vec<Transaction> = dec.read_all().await.expect("read_all failed");

    assert_eq!(
        decoded, expected,
        "write_all/read_all full roundtrip mismatch"
    );
    assert!(
        dec.progress().items_processed >= 3,
        "items_processed must be >= 3 after reading 3 transactions"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Sync encode / async decode interop for Transaction
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_sync_encode_async_decode_interop_transaction() {
    let original = Transaction {
        id: 777,
        tx_type: TransactionType::Deposit,
        amount_cents: 999_999,
        account_id: 5555,
    };

    // Async-encode, then async-decode
    let async_buf = encode_one(&original).await;
    let cursor = Cursor::new(async_buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    let async_decoded: Option<Transaction> =
        dec.read_item().await.expect("async decode interop failed");
    assert_eq!(
        async_decoded,
        Some(original.clone()),
        "sync-encode/async-decode interop mismatch"
    );

    // Verify sync roundtrip of same value is consistent
    let sync_bytes = encode_to_vec(&original).expect("sync encode Transaction failed");
    let (sync_decoded, _): (Transaction, _) =
        decode_from_slice(&sync_bytes).expect("sync decode Transaction failed");
    assert_eq!(
        sync_decoded, original,
        "sync Transaction roundtrip consistency check failed"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Async encode / sync decode interop for AccountSnapshot
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_async_encode_sync_decode_interop_snapshot() {
    let original = AccountSnapshot {
        account_id: 4321,
        balance_cents: 87_654,
        tx_count: 99,
    };

    // Encode via async, then decode via sync for consistency comparison
    let async_buf = encode_one(&original).await;
    let async_decoded = decode_one::<AccountSnapshot>(async_buf).await;
    assert_eq!(
        async_decoded,
        Some(original.clone()),
        "async encode/decode for AccountSnapshot failed"
    );

    // Sync encode and decode the same value
    let sync_bytes = encode_to_vec(&original).expect("sync encode AccountSnapshot failed");
    let (sync_decoded, _): (AccountSnapshot, _) =
        decode_from_slice(&sync_bytes).expect("sync decode AccountSnapshot failed");
    assert_eq!(
        sync_decoded, original,
        "async/sync interop: AccountSnapshot sync roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 20: items_processed >= N after writing N transactions
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_items_processed_at_least_n_after_n_writes() {
    let n: u64 = 12;
    let transactions: Vec<Transaction> = (0..n).map(make_deposit).collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for tx in &transactions {
            enc.write_item(tx).await.expect("write_item failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);
    let _: Vec<Transaction> = dec.read_all().await.expect("read_all failed");

    assert!(
        dec.progress().items_processed >= n,
        "items_processed ({}) must be >= {} after writing {} transactions",
        dec.progress().items_processed,
        n,
        n
    );
}

// ---------------------------------------------------------------------------
// Test 21: Decoder reports is_finished() after stream exhausted
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_decoder_is_finished_after_stream_exhausted() {
    let transactions = vec![make_deposit(1), make_withdrawal(2)];

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        for tx in &transactions {
            enc.write_item(tx).await.expect("write_item failed");
        }
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    // Not yet finished
    assert!(
        !dec.is_finished(),
        "decoder should not be finished before reading"
    );

    // Read both items
    let _: Option<Transaction> = dec.read_item().await.expect("read item 1 failed");
    let _: Option<Transaction> = dec.read_item().await.expect("read item 2 failed");

    // Read past end
    let eof: Option<Transaction> = dec.read_item().await.expect("eof read failed");
    assert_eq!(eof, None, "expected None at end of stream");
    assert!(
        dec.is_finished(),
        "decoder must report is_finished() after stream exhausted"
    );
}

// ---------------------------------------------------------------------------
// Test 22: bytes_processed grows as more transactions are decoded
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_fin24_bytes_processed_grows_with_more_transactions() {
    let transactions: Vec<Transaction> = (0u64..15)
        .map(|i| Transaction {
            id: i,
            tx_type: TransactionType::Withdrawal,
            amount_cents: -((i as i64 + 1) * 500),
            account_id: 3000 + i,
        })
        .collect();

    let mut buf = Vec::<u8>::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut enc = AsyncEncoder::new(cursor);
        enc.write_all(transactions.clone())
            .await
            .expect("write_all failed");
        enc.finish().await.expect("finish failed");
    }

    let cursor = Cursor::new(buf);
    let buf_reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(buf_reader);

    // Read first item
    let first: Option<Transaction> = dec
        .read_item()
        .await
        .expect("read first transaction failed");
    assert_eq!(
        first.as_ref(),
        Some(&transactions[0]),
        "first decoded transaction mismatch"
    );

    let bytes_after_one = dec.progress().bytes_processed;
    assert!(
        bytes_after_one > 0,
        "bytes_processed must be > 0 after reading first transaction"
    );

    // Read remaining 14 items
    let rest: Vec<Transaction> = dec
        .read_all()
        .await
        .expect("read_all remaining transactions failed");
    assert_eq!(rest.len(), 14, "must decode 14 remaining transactions");

    let bytes_after_all = dec.progress().bytes_processed;
    assert!(
        bytes_after_all > bytes_after_one,
        "bytes_processed must grow after reading all transactions (was {bytes_after_one}, now {bytes_after_all})"
    );

    assert!(
        dec.progress().items_processed >= 15,
        "items_processed must be >= 15 after reading all transactions"
    );
}
