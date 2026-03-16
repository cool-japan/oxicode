//! Advanced checksum tests – ledger and transaction domain types (set 11)

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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum, HEADER_SIZE};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct LedgerEntry {
    entry_id: u64,
    debit_cents: u64,
    credit_cents: u64,
    description: String,
    account: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TransactionType {
    Debit,
    Credit,
    Transfer {
        from_account: String,
        to_account: String,
    },
    Adjustment {
        reason: String,
    },
    Reversal {
        original_id: u64,
    },
}

// ---------------------------------------------------------------------------
// Test 1: LedgerEntry roundtrip through checksum layer
// ---------------------------------------------------------------------------
#[test]
fn test_ledger_entry_checksum_roundtrip() {
    let entry = LedgerEntry {
        entry_id: 1001,
        debit_cents: 50_000,
        credit_cents: 0,
        description: "Office supplies".to_string(),
        account: "EXP-4200".to_string(),
    };
    let wrapped = encode_with_checksum(&entry).expect("encode LedgerEntry failed");
    let (decoded, _): (LedgerEntry, _) =
        decode_with_checksum(&wrapped).expect("decode LedgerEntry failed");
    assert_eq!(entry, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: TransactionType::Debit roundtrip through checksum layer
// ---------------------------------------------------------------------------
#[test]
fn test_transaction_type_debit_roundtrip() {
    let tx = TransactionType::Debit;
    let wrapped = encode_with_checksum(&tx).expect("encode Debit failed");
    let (decoded, _): (TransactionType, _) =
        decode_with_checksum(&wrapped).expect("decode Debit failed");
    assert_eq!(tx, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: TransactionType::Credit roundtrip through checksum layer
// ---------------------------------------------------------------------------
#[test]
fn test_transaction_type_credit_roundtrip() {
    let tx = TransactionType::Credit;
    let wrapped = encode_with_checksum(&tx).expect("encode Credit failed");
    let (decoded, _): (TransactionType, _) =
        decode_with_checksum(&wrapped).expect("decode Credit failed");
    assert_eq!(tx, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: TransactionType::Transfer roundtrip with non-ASCII account names
// ---------------------------------------------------------------------------
#[test]
fn test_transaction_type_transfer_roundtrip() {
    let tx = TransactionType::Transfer {
        from_account: "ASSET-1010".to_string(),
        to_account: "LIAB-2020".to_string(),
    };
    let wrapped = encode_with_checksum(&tx).expect("encode Transfer failed");
    let (decoded, _): (TransactionType, _) =
        decode_with_checksum(&wrapped).expect("decode Transfer failed");
    assert_eq!(tx, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: TransactionType::Adjustment roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_transaction_type_adjustment_roundtrip() {
    let tx = TransactionType::Adjustment {
        reason: "Year-end accrual correction".to_string(),
    };
    let wrapped = encode_with_checksum(&tx).expect("encode Adjustment failed");
    let (decoded, _): (TransactionType, _) =
        decode_with_checksum(&wrapped).expect("decode Adjustment failed");
    assert_eq!(tx, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: TransactionType::Reversal roundtrip with max u64 original_id
// ---------------------------------------------------------------------------
#[test]
fn test_transaction_type_reversal_max_id_roundtrip() {
    let tx = TransactionType::Reversal {
        original_id: u64::MAX,
    };
    let wrapped = encode_with_checksum(&tx).expect("encode Reversal failed");
    let (decoded, _): (TransactionType, _) =
        decode_with_checksum(&wrapped).expect("decode Reversal failed");
    assert_eq!(tx, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: HEADER_SIZE is exactly 16 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_header_size_is_16() {
    assert_eq!(
        HEADER_SIZE, 16,
        "HEADER_SIZE must be 16 bytes (magic[3] + version[1] + len[4] + crc[8])"
    );
}

// ---------------------------------------------------------------------------
// Test 8: encoded length of LedgerEntry equals HEADER_SIZE + raw payload size
// ---------------------------------------------------------------------------
#[test]
fn test_wrapped_len_equals_header_plus_payload() {
    let entry = LedgerEntry {
        entry_id: 7,
        debit_cents: 100,
        credit_cents: 100,
        description: "Balanced".to_string(),
        account: "EQ-3000".to_string(),
    };
    let raw = encode_to_vec(&entry).expect("raw encode failed");
    let wrapped = encode_with_checksum(&entry).expect("checksum encode failed");
    assert_eq!(
        wrapped.len(),
        HEADER_SIZE + raw.len(),
        "wrapped length must be HEADER_SIZE + raw payload length"
    );
}

// ---------------------------------------------------------------------------
// Test 9: corruption in LedgerEntry payload is detected
// ---------------------------------------------------------------------------
#[test]
fn test_ledger_entry_corruption_detected() {
    let entry = LedgerEntry {
        entry_id: 999,
        debit_cents: 1_000_000,
        credit_cents: 0,
        description: "Capital expenditure".to_string(),
        account: "CAPEX-7700".to_string(),
    };
    let mut wrapped = encode_with_checksum(&entry).expect("encode failed");
    // Flip a byte in the middle of the payload
    let mid = HEADER_SIZE + wrapped.len().saturating_sub(HEADER_SIZE) / 2;
    wrapped[mid] ^= 0xAA;
    let result: Result<(LedgerEntry, usize), _> = decode_with_checksum(&wrapped);
    assert!(result.is_err(), "corrupted payload must return Err");
    assert!(
        matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
        "expected ChecksumMismatch, got: {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 10: corruption in TransactionType payload is detected
// ---------------------------------------------------------------------------
#[test]
fn test_transaction_type_corruption_detected() {
    let tx = TransactionType::Transfer {
        from_account: "ACC-A".to_string(),
        to_account: "ACC-B".to_string(),
    };
    let mut wrapped = encode_with_checksum(&tx).expect("encode failed");
    wrapped[HEADER_SIZE] ^= 0xFF;
    let result: Result<(TransactionType, usize), _> = decode_with_checksum(&wrapped);
    assert!(result.is_err(), "first-byte corruption must be detected");
}

// ---------------------------------------------------------------------------
// Test 11: encoding is deterministic for LedgerEntry
// ---------------------------------------------------------------------------
#[test]
fn test_ledger_entry_encoding_is_deterministic() {
    let entry = LedgerEntry {
        entry_id: 42,
        debit_cents: 9_999,
        credit_cents: 0,
        description: "Consistent".to_string(),
        account: "REV-4000".to_string(),
    };
    let first = encode_with_checksum(&entry).expect("first encode failed");
    let second = encode_with_checksum(&entry).expect("second encode failed");
    assert_eq!(first, second, "encoding must be deterministic");
}

// ---------------------------------------------------------------------------
// Test 12: encoding is deterministic for TransactionType
// ---------------------------------------------------------------------------
#[test]
fn test_transaction_type_encoding_is_deterministic() {
    let tx = TransactionType::Reversal { original_id: 12345 };
    let first = encode_with_checksum(&tx).expect("first encode failed");
    let second = encode_with_checksum(&tx).expect("second encode failed");
    assert_eq!(first, second, "encoding must be deterministic");
}

// ---------------------------------------------------------------------------
// Test 13: Vec<LedgerEntry> roundtrip through checksum layer
// ---------------------------------------------------------------------------
#[test]
fn test_vec_ledger_entry_roundtrip() {
    let entries: Vec<LedgerEntry> = (0..20)
        .map(|i| LedgerEntry {
            entry_id: i,
            debit_cents: i * 100,
            credit_cents: i * 50,
            description: format!("Entry {}", i),
            account: format!("ACC-{:04}", i),
        })
        .collect();
    let wrapped = encode_with_checksum(&entries).expect("encode Vec<LedgerEntry> failed");
    let (decoded, _): (Vec<LedgerEntry>, _) =
        decode_with_checksum(&wrapped).expect("decode Vec<LedgerEntry> failed");
    assert_eq!(entries, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Vec<TransactionType> roundtrip through checksum layer
// ---------------------------------------------------------------------------
#[test]
fn test_vec_transaction_type_roundtrip() {
    let txs = vec![
        TransactionType::Debit,
        TransactionType::Credit,
        TransactionType::Transfer {
            from_account: "A".to_string(),
            to_account: "B".to_string(),
        },
        TransactionType::Adjustment {
            reason: "Q4 close".to_string(),
        },
        TransactionType::Reversal { original_id: 0 },
    ];
    let wrapped = encode_with_checksum(&txs).expect("encode Vec<TransactionType> failed");
    let (decoded, _): (Vec<TransactionType>, _) =
        decode_with_checksum(&wrapped).expect("decode Vec<TransactionType> failed");
    assert_eq!(txs, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Option<LedgerEntry> Some roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_ledger_entry_some_roundtrip() {
    let entry = Some(LedgerEntry {
        entry_id: 555,
        debit_cents: 777,
        credit_cents: 333,
        description: "Optional entry".to_string(),
        account: "OPT-9900".to_string(),
    });
    let wrapped = encode_with_checksum(&entry).expect("encode Option<LedgerEntry> failed");
    let (decoded, _): (Option<LedgerEntry>, _) =
        decode_with_checksum(&wrapped).expect("decode Option<LedgerEntry> failed");
    assert_eq!(entry, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Option<LedgerEntry> None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_ledger_entry_none_roundtrip() {
    let entry: Option<LedgerEntry> = None;
    let wrapped = encode_with_checksum(&entry).expect("encode None failed");
    let (decoded, _): (Option<LedgerEntry>, _) =
        decode_with_checksum(&wrapped).expect("decode None failed");
    assert_eq!(entry, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: primitive u64 roundtrip (entry_id boundary value zero)
// ---------------------------------------------------------------------------
#[test]
fn test_primitive_u64_zero_roundtrip() {
    let value: u64 = 0u64;
    let wrapped = encode_with_checksum(&value).expect("encode u64 zero failed");
    let (decoded, _): (u64, _) = decode_with_checksum(&wrapped).expect("decode u64 zero failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: primitive u64 MAX roundtrip (debit_cents boundary)
// ---------------------------------------------------------------------------
#[test]
fn test_primitive_u64_max_roundtrip() {
    let value: u64 = u64::MAX;
    let wrapped = encode_with_checksum(&value).expect("encode u64::MAX failed");
    let (decoded, _): (u64, _) = decode_with_checksum(&wrapped).expect("decode u64::MAX failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: consumed bytes equal total wrapped length for LedgerEntry
// ---------------------------------------------------------------------------
#[test]
fn test_consumed_bytes_equals_wrapped_len_for_ledger_entry() {
    let entry = LedgerEntry {
        entry_id: 2048,
        debit_cents: 4096,
        credit_cents: 2048,
        description: "Consumed bytes check".to_string(),
        account: "CHK-0001".to_string(),
    };
    let wrapped = encode_with_checksum(&entry).expect("encode failed");
    let (_, consumed) = decode_with_checksum::<LedgerEntry>(&wrapped).expect("decode failed");
    assert_eq!(
        consumed,
        wrapped.len(),
        "consumed must equal total wrapped length"
    );
}

// ---------------------------------------------------------------------------
// Test 20: LedgerEntry raw encode/decode (without checksum) via decode_from_slice
// ---------------------------------------------------------------------------
#[test]
fn test_ledger_entry_raw_encode_decode_from_slice() {
    let entry = LedgerEntry {
        entry_id: 3,
        debit_cents: 500,
        credit_cents: 250,
        description: "Raw slice test".to_string(),
        account: "RAW-0010".to_string(),
    };
    let bytes = encode_to_vec(&entry).expect("raw encode failed");
    let (decoded, consumed): (LedgerEntry, _) =
        decode_from_slice(&bytes).expect("raw decode failed");
    assert_eq!(entry, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "raw consumed must equal raw bytes length"
    );
}

// ---------------------------------------------------------------------------
// Test 21: LedgerEntry with zero-length strings roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ledger_entry_empty_strings_roundtrip() {
    let entry = LedgerEntry {
        entry_id: 0,
        debit_cents: 0,
        credit_cents: 0,
        description: String::new(),
        account: String::new(),
    };
    let wrapped = encode_with_checksum(&entry).expect("encode empty-string entry failed");
    let (decoded, _): (LedgerEntry, _) =
        decode_with_checksum(&wrapped).expect("decode empty-string entry failed");
    assert_eq!(entry, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: all five TransactionType variants have distinct encodings
// ---------------------------------------------------------------------------
#[test]
fn test_all_transaction_variants_have_distinct_encodings() {
    let variants = vec![
        TransactionType::Debit,
        TransactionType::Credit,
        TransactionType::Transfer {
            from_account: "X".to_string(),
            to_account: "Y".to_string(),
        },
        TransactionType::Adjustment {
            reason: "R".to_string(),
        },
        TransactionType::Reversal { original_id: 1 },
    ];

    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_with_checksum(v).expect("encode variant failed"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "variants {} and {} must have distinct encodings",
                i, j
            );
        }
    }
}
