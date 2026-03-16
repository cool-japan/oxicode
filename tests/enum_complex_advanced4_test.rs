//! Advanced complex enum encoding tests — DatabaseOp / QueryResult / Transaction

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
// Type definitions
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum DatabaseOp {
    Select {
        table: String,
        limit: u32,
    },
    Insert {
        table: String,
        data: Vec<u8>,
    },
    Update {
        table: String,
        key: u64,
        data: Vec<u8>,
    },
    Delete {
        table: String,
        key: u64,
    },
    CreateTable {
        name: String,
        columns: Vec<String>,
    },
    DropTable(String),
    BeginTx,
    Commit,
    Rollback,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum QueryResult {
    Ok { rows_affected: u64 },
    Rows(Vec<Vec<u8>>),
    Error { code: u32, message: String },
    Empty,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Transaction {
    tx_id: u64,
    ops: Vec<DatabaseOp>,
    result: Option<QueryResult>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_database_op_select_roundtrip() {
    let op = DatabaseOp::Select {
        table: String::from("users"),
        limit: 100,
    };
    let bytes = encode_to_vec(&op).expect("encode DatabaseOp::Select");
    let (decoded, _): (DatabaseOp, usize) =
        decode_from_slice(&bytes).expect("decode DatabaseOp::Select");
    assert_eq!(op, decoded);
}

#[test]
fn test_database_op_insert_roundtrip() {
    let op = DatabaseOp::Insert {
        table: String::from("orders"),
        data: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    let bytes = encode_to_vec(&op).expect("encode DatabaseOp::Insert");
    let (decoded, _): (DatabaseOp, usize) =
        decode_from_slice(&bytes).expect("decode DatabaseOp::Insert");
    assert_eq!(op, decoded);
}

#[test]
fn test_database_op_update_roundtrip() {
    let op = DatabaseOp::Update {
        table: String::from("products"),
        key: 9_999_999_999_u64,
        data: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };
    let bytes = encode_to_vec(&op).expect("encode DatabaseOp::Update");
    let (decoded, _): (DatabaseOp, usize) =
        decode_from_slice(&bytes).expect("decode DatabaseOp::Update");
    assert_eq!(op, decoded);
}

#[test]
fn test_database_op_delete_roundtrip() {
    let op = DatabaseOp::Delete {
        table: String::from("sessions"),
        key: 42_u64,
    };
    let bytes = encode_to_vec(&op).expect("encode DatabaseOp::Delete");
    let (decoded, _): (DatabaseOp, usize) =
        decode_from_slice(&bytes).expect("decode DatabaseOp::Delete");
    assert_eq!(op, decoded);
}

#[test]
fn test_database_op_create_table_roundtrip() {
    let op = DatabaseOp::CreateTable {
        name: String::from("inventory"),
        columns: vec![
            String::from("id"),
            String::from("name"),
            String::from("quantity"),
            String::from("price"),
        ],
    };
    let bytes = encode_to_vec(&op).expect("encode DatabaseOp::CreateTable");
    let (decoded, _): (DatabaseOp, usize) =
        decode_from_slice(&bytes).expect("decode DatabaseOp::CreateTable");
    assert_eq!(op, decoded);
}

#[test]
fn test_database_op_drop_table_roundtrip() {
    let op = DatabaseOp::DropTable(String::from("legacy_table"));
    let bytes = encode_to_vec(&op).expect("encode DatabaseOp::DropTable");
    let (decoded, _): (DatabaseOp, usize) =
        decode_from_slice(&bytes).expect("decode DatabaseOp::DropTable");
    assert_eq!(op, decoded);
}

#[test]
fn test_database_op_unit_variants_roundtrip() {
    for (label, op) in [
        ("BeginTx", DatabaseOp::BeginTx),
        ("Commit", DatabaseOp::Commit),
        ("Rollback", DatabaseOp::Rollback),
    ] {
        let bytes = encode_to_vec(&op).expect(label);
        let (decoded, consumed): (DatabaseOp, usize) = decode_from_slice(&bytes).expect(label);
        assert_eq!(op, decoded, "variant mismatch for {label}");
        assert_eq!(consumed, bytes.len(), "consumed bytes mismatch for {label}");
    }
}

#[test]
fn test_query_result_ok_roundtrip() {
    let result = QueryResult::Ok {
        rows_affected: 1_024_u64,
    };
    let bytes = encode_to_vec(&result).expect("encode QueryResult::Ok");
    let (decoded, _): (QueryResult, usize) =
        decode_from_slice(&bytes).expect("decode QueryResult::Ok");
    assert_eq!(result, decoded);
}

#[test]
fn test_query_result_rows_roundtrip() {
    let result = QueryResult::Rows(vec![vec![0x01, 0x02, 0x03], vec![0xAA, 0xBB], vec![0xFF]]);
    let bytes = encode_to_vec(&result).expect("encode QueryResult::Rows");
    let (decoded, _): (QueryResult, usize) =
        decode_from_slice(&bytes).expect("decode QueryResult::Rows");
    assert_eq!(result, decoded);
}

#[test]
fn test_query_result_error_roundtrip() {
    let result = QueryResult::Error {
        code: 404_u32,
        message: String::from("Table not found"),
    };
    let bytes = encode_to_vec(&result).expect("encode QueryResult::Error");
    let (decoded, _): (QueryResult, usize) =
        decode_from_slice(&bytes).expect("decode QueryResult::Error");
    assert_eq!(result, decoded);
}

#[test]
fn test_query_result_empty_roundtrip() {
    let result = QueryResult::Empty;
    let bytes = encode_to_vec(&result).expect("encode QueryResult::Empty");
    let (decoded, consumed): (QueryResult, usize) =
        decode_from_slice(&bytes).expect("decode QueryResult::Empty");
    assert_eq!(result, decoded);
    assert_eq!(consumed, bytes.len(), "all bytes should be consumed");
}

#[test]
fn test_transaction_with_option_none_roundtrip() {
    let tx = Transaction {
        tx_id: 1_u64,
        ops: vec![DatabaseOp::BeginTx, DatabaseOp::Commit],
        result: None,
    };
    let bytes = encode_to_vec(&tx).expect("encode Transaction (result=None)");
    let (decoded, _): (Transaction, usize) =
        decode_from_slice(&bytes).expect("decode Transaction (result=None)");
    assert_eq!(tx, decoded);
}

#[test]
fn test_transaction_with_option_some_roundtrip() {
    let tx = Transaction {
        tx_id: 2_u64,
        ops: vec![
            DatabaseOp::BeginTx,
            DatabaseOp::Insert {
                table: String::from("logs"),
                data: vec![42, 43, 44],
            },
            DatabaseOp::Commit,
        ],
        result: Some(QueryResult::Ok { rows_affected: 1 }),
    };
    let bytes = encode_to_vec(&tx).expect("encode Transaction (result=Some)");
    let (decoded, _): (Transaction, usize) =
        decode_from_slice(&bytes).expect("decode Transaction (result=Some)");
    assert_eq!(tx, decoded);
}

#[test]
fn test_transaction_multiple_ops_roundtrip() {
    let tx = Transaction {
        tx_id: 3_u64,
        ops: vec![
            DatabaseOp::BeginTx,
            DatabaseOp::Select {
                table: String::from("accounts"),
                limit: 50,
            },
            DatabaseOp::Update {
                table: String::from("accounts"),
                key: 7,
                data: vec![0xCA, 0xFE],
            },
            DatabaseOp::Delete {
                table: String::from("accounts"),
                key: 99,
            },
            DatabaseOp::Rollback,
        ],
        result: Some(QueryResult::Error {
            code: 500,
            message: String::from("Deadlock detected"),
        }),
    };
    let bytes = encode_to_vec(&tx).expect("encode Transaction multi-ops");
    let (decoded, _): (Transaction, usize) =
        decode_from_slice(&bytes).expect("decode Transaction multi-ops");
    assert_eq!(tx, decoded);
}

#[test]
fn test_consumed_bytes_equals_encoded_len() {
    let op = DatabaseOp::Select {
        table: String::from("test"),
        limit: 10,
    };
    let bytes = encode_to_vec(&op).expect("encode for consumed-bytes check");
    let (_, consumed): (DatabaseOp, usize) =
        decode_from_slice(&bytes).expect("decode for consumed-bytes check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

#[test]
fn test_vec_of_database_ops_roundtrip() {
    let ops: Vec<DatabaseOp> = vec![
        DatabaseOp::BeginTx,
        DatabaseOp::CreateTable {
            name: String::from("events"),
            columns: vec![String::from("id"), String::from("ts")],
        },
        DatabaseOp::Insert {
            table: String::from("events"),
            data: vec![1, 2, 3],
        },
        DatabaseOp::Commit,
    ];
    let bytes = encode_to_vec(&ops).expect("encode Vec<DatabaseOp>");
    let (decoded, _): (Vec<DatabaseOp>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<DatabaseOp>");
    assert_eq!(ops, decoded);
}

#[test]
fn test_vec_of_query_results_roundtrip() {
    let results: Vec<QueryResult> = vec![
        QueryResult::Empty,
        QueryResult::Ok { rows_affected: 5 },
        QueryResult::Rows(vec![vec![0x00], vec![0x01, 0x02]]),
        QueryResult::Error {
            code: 42,
            message: String::from("oops"),
        },
    ];
    let bytes = encode_to_vec(&results).expect("encode Vec<QueryResult>");
    let (decoded, _): (Vec<QueryResult>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<QueryResult>");
    assert_eq!(results, decoded);
}

#[test]
fn test_config_legacy_database_op() {
    let op = DatabaseOp::Update {
        table: String::from("cfg_test"),
        key: 0xDEAD_BEEF_u64,
        data: vec![9, 8, 7],
    };
    let cfg = config::legacy();
    let bytes = encode_to_vec_with_config(&op, cfg).expect("encode DatabaseOp with legacy config");
    let (decoded, _): (DatabaseOp, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode DatabaseOp with legacy config");
    assert_eq!(op, decoded);
}

#[test]
fn test_config_legacy_transaction() {
    let tx = Transaction {
        tx_id: 100_u64,
        ops: vec![DatabaseOp::DropTable(String::from("old"))],
        result: Some(QueryResult::Empty),
    };
    let cfg = config::legacy();
    let bytes = encode_to_vec_with_config(&tx, cfg).expect("encode Transaction with legacy config");
    let (decoded, _): (Transaction, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Transaction with legacy config");
    assert_eq!(tx, decoded);
}

#[test]
fn test_discriminant_unit_variants_differ() {
    let begin_bytes = encode_to_vec(&DatabaseOp::BeginTx).expect("encode BeginTx");
    let commit_bytes = encode_to_vec(&DatabaseOp::Commit).expect("encode Commit");
    let rollback_bytes = encode_to_vec(&DatabaseOp::Rollback).expect("encode Rollback");

    assert_ne!(
        begin_bytes, commit_bytes,
        "BeginTx and Commit must have different discriminants"
    );
    assert_ne!(
        commit_bytes, rollback_bytes,
        "Commit and Rollback must have different discriminants"
    );
    assert_ne!(
        begin_bytes, rollback_bytes,
        "BeginTx and Rollback must have different discriminants"
    );
}

#[test]
fn test_discriminant_query_result_variants_differ() {
    let ok_bytes = encode_to_vec(&QueryResult::Ok { rows_affected: 0 }).expect("encode Ok");
    let empty_bytes = encode_to_vec(&QueryResult::Empty).expect("encode Empty");
    let rows_bytes = encode_to_vec(&QueryResult::Rows(vec![])).expect("encode Rows");

    assert_ne!(
        ok_bytes, empty_bytes,
        "Ok and Empty discriminants must differ"
    );
    assert_ne!(
        ok_bytes, rows_bytes,
        "Ok and Rows discriminants must differ"
    );
    assert_ne!(
        empty_bytes, rows_bytes,
        "Empty and Rows discriminants must differ"
    );
}

#[test]
fn test_empty_collections_in_variants() {
    let op_empty_data = DatabaseOp::Insert {
        table: String::from("empty_insert"),
        data: vec![],
    };
    let op_empty_cols = DatabaseOp::CreateTable {
        name: String::from("no_columns"),
        columns: vec![],
    };
    let result_empty_rows = QueryResult::Rows(vec![]);
    let tx_empty_ops = Transaction {
        tx_id: 0_u64,
        ops: vec![],
        result: None,
    };

    for label in [
        "empty insert data",
        "empty columns",
        "empty rows",
        "empty ops tx",
    ] {
        let bytes = match label {
            "empty insert data" => encode_to_vec(&op_empty_data).expect(label),
            "empty columns" => encode_to_vec(&op_empty_cols).expect(label),
            "empty rows" => encode_to_vec(&result_empty_rows).expect(label),
            "empty ops tx" => encode_to_vec(&tx_empty_ops).expect(label),
            _ => unreachable!(),
        };
        assert!(
            !bytes.is_empty(),
            "encoded bytes must not be empty for {label}"
        );
    }

    let (dec_insert, _): (DatabaseOp, usize) =
        decode_from_slice(&encode_to_vec(&op_empty_data).expect("enc")).expect("dec empty insert");
    assert_eq!(op_empty_data, dec_insert);

    let (dec_create, _): (DatabaseOp, usize) =
        decode_from_slice(&encode_to_vec(&op_empty_cols).expect("enc")).expect("dec empty cols");
    assert_eq!(op_empty_cols, dec_create);

    let (dec_rows, _): (QueryResult, usize) =
        decode_from_slice(&encode_to_vec(&result_empty_rows).expect("enc"))
            .expect("dec empty rows");
    assert_eq!(result_empty_rows, dec_rows);

    let (dec_tx, _): (Transaction, usize) =
        decode_from_slice(&encode_to_vec(&tx_empty_ops).expect("enc")).expect("dec empty ops tx");
    assert_eq!(tx_empty_ops, dec_tx);
}
