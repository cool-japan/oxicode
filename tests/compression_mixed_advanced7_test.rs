//! Distributed database / data warehouse domain tests for LZ4 and Zstd compression in OxiCode.
//!
//! Tests both compression algorithms with realistic warehouse schema structures,
//! verifying round-trip correctness, size reduction, and cross-algorithm consistency.

#![cfg(all(feature = "compression-lz4", feature = "compression-zstd"))]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types — distributed database / data warehouse
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ColumnType {
    Int32,
    Int64,
    Float32,
    Float64,
    Varchar(u32),
    Boolean,
    Timestamp,
    Blob,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TableColumn {
    name: String,
    col_type: ColumnType,
    nullable: bool,
    primary_key: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Row {
    values: Vec<Option<String>>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TableData {
    schema: Vec<TableColumn>,
    rows: Vec<Row>,
    table_name: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QueryPlan {
    query: String,
    estimated_rows: u64,
    cost: f64,
    tables: Vec<String>,
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn make_orders_schema() -> Vec<TableColumn> {
    vec![
        TableColumn {
            name: "order_id".into(),
            col_type: ColumnType::Int64,
            nullable: false,
            primary_key: true,
        },
        TableColumn {
            name: "customer_id".into(),
            col_type: ColumnType::Int32,
            nullable: false,
            primary_key: false,
        },
        TableColumn {
            name: "amount".into(),
            col_type: ColumnType::Float64,
            nullable: true,
            primary_key: false,
        },
        TableColumn {
            name: "status".into(),
            col_type: ColumnType::Varchar(32),
            nullable: true,
            primary_key: false,
        },
        TableColumn {
            name: "created_at".into(),
            col_type: ColumnType::Timestamp,
            nullable: false,
            primary_key: false,
        },
    ]
}

fn make_repetitive_table(rows: usize) -> TableData {
    let schema = vec![
        TableColumn {
            name: "event_type".into(),
            col_type: ColumnType::Varchar(64),
            nullable: false,
            primary_key: false,
        },
        TableColumn {
            name: "user_id".into(),
            col_type: ColumnType::Int64,
            nullable: false,
            primary_key: false,
        },
        TableColumn {
            name: "active".into(),
            col_type: ColumnType::Boolean,
            nullable: false,
            primary_key: false,
        },
    ];
    let row = Row {
        values: vec![
            Some("page_view".into()),
            Some("12345".into()),
            Some("true".into()),
        ],
    };
    TableData {
        schema,
        rows: vec![row; rows],
        table_name: "analytics_events".into(),
    }
}

// ---------------------------------------------------------------------------
// 1. LZ4 roundtrip — TableColumn (Int32)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_table_column_int32() {
    let col = TableColumn {
        name: "shard_count".into(),
        col_type: ColumnType::Int32,
        nullable: false,
        primary_key: false,
    };
    let encoded = encode_to_vec(&col).expect("encode TableColumn Int32");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let decompressed = decompress(&compressed).expect("lz4 decompress");
    let (decoded, _): (TableColumn, usize) =
        decode_from_slice(&decompressed).expect("decode TableColumn Int32");
    assert_eq!(col, decoded);
}

// ---------------------------------------------------------------------------
// 2. LZ4 roundtrip — TableColumn (Int64)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_table_column_int64() {
    let col = TableColumn {
        name: "row_offset".into(),
        col_type: ColumnType::Int64,
        nullable: true,
        primary_key: false,
    };
    let encoded = encode_to_vec(&col).expect("encode TableColumn Int64");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let decompressed = decompress(&compressed).expect("lz4 decompress");
    let (decoded, _): (TableColumn, usize) =
        decode_from_slice(&decompressed).expect("decode TableColumn Int64");
    assert_eq!(col, decoded);
}

// ---------------------------------------------------------------------------
// 3. LZ4 roundtrip — TableColumn (Float32)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_table_column_float32() {
    let col = TableColumn {
        name: "sample_rate".into(),
        col_type: ColumnType::Float32,
        nullable: true,
        primary_key: false,
    };
    let encoded = encode_to_vec(&col).expect("encode TableColumn Float32");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let decompressed = decompress(&compressed).expect("lz4 decompress");
    let (decoded, _): (TableColumn, usize) =
        decode_from_slice(&decompressed).expect("decode TableColumn Float32");
    assert_eq!(col, decoded);
}

// ---------------------------------------------------------------------------
// 4. LZ4 roundtrip — TableColumn (Float64)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_table_column_float64() {
    let col = TableColumn {
        name: "revenue".into(),
        col_type: ColumnType::Float64,
        nullable: true,
        primary_key: false,
    };
    let encoded = encode_to_vec(&col).expect("encode TableColumn Float64");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let decompressed = decompress(&compressed).expect("lz4 decompress");
    let (decoded, _): (TableColumn, usize) =
        decode_from_slice(&decompressed).expect("decode TableColumn Float64");
    assert_eq!(col, decoded);
}

// ---------------------------------------------------------------------------
// 5. LZ4 roundtrip — TableColumn (Varchar)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_table_column_varchar() {
    let col = TableColumn {
        name: "region_code".into(),
        col_type: ColumnType::Varchar(16),
        nullable: true,
        primary_key: false,
    };
    let encoded = encode_to_vec(&col).expect("encode TableColumn Varchar");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let decompressed = decompress(&compressed).expect("lz4 decompress");
    let (decoded, _): (TableColumn, usize) =
        decode_from_slice(&decompressed).expect("decode TableColumn Varchar");
    assert_eq!(col, decoded);
}

// ---------------------------------------------------------------------------
// 6. LZ4 roundtrip — TableColumn (Boolean)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_table_column_boolean() {
    let col = TableColumn {
        name: "is_deleted".into(),
        col_type: ColumnType::Boolean,
        nullable: false,
        primary_key: false,
    };
    let encoded = encode_to_vec(&col).expect("encode TableColumn Boolean");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let decompressed = decompress(&compressed).expect("lz4 decompress");
    let (decoded, _): (TableColumn, usize) =
        decode_from_slice(&decompressed).expect("decode TableColumn Boolean");
    assert_eq!(col, decoded);
}

// ---------------------------------------------------------------------------
// 7. LZ4 roundtrip — TableColumn (Timestamp)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_table_column_timestamp() {
    let col = TableColumn {
        name: "ingested_at".into(),
        col_type: ColumnType::Timestamp,
        nullable: true,
        primary_key: false,
    };
    let encoded = encode_to_vec(&col).expect("encode TableColumn Timestamp");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let decompressed = decompress(&compressed).expect("lz4 decompress");
    let (decoded, _): (TableColumn, usize) =
        decode_from_slice(&decompressed).expect("decode TableColumn Timestamp");
    assert_eq!(col, decoded);
}

// ---------------------------------------------------------------------------
// 8. LZ4 roundtrip — TableColumn (Blob)
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_roundtrip_table_column_blob() {
    let col = TableColumn {
        name: "payload_bytes".into(),
        col_type: ColumnType::Blob,
        nullable: true,
        primary_key: false,
    };
    let encoded = encode_to_vec(&col).expect("encode TableColumn Blob");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let decompressed = decompress(&compressed).expect("lz4 decompress");
    let (decoded, _): (TableColumn, usize) =
        decode_from_slice(&decompressed).expect("decode TableColumn Blob");
    assert_eq!(col, decoded);
}

// ---------------------------------------------------------------------------
// 9. Zstd roundtrip — full TableData with orders schema
// ---------------------------------------------------------------------------
#[test]
fn test_zstd_roundtrip_table_data_orders() {
    let table = TableData {
        schema: make_orders_schema(),
        rows: vec![
            Row {
                values: vec![
                    Some("1001".into()),
                    Some("42".into()),
                    Some("199.99".into()),
                    Some("SHIPPED".into()),
                    Some("2025-01-15T10:00:00Z".into()),
                ],
            },
            Row {
                values: vec![
                    Some("1002".into()),
                    Some("43".into()),
                    None,
                    Some("PENDING".into()),
                    Some("2025-01-16T08:30:00Z".into()),
                ],
            },
        ],
        table_name: "fact_orders".into(),
    };
    let encoded = encode_to_vec(&table).expect("encode TableData");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");
    let decompressed = decompress(&compressed).expect("zstd decompress");
    let (decoded, _): (TableData, usize) =
        decode_from_slice(&decompressed).expect("decode TableData");
    assert_eq!(table, decoded);
}

// ---------------------------------------------------------------------------
// 10. LZ4 compressed < original for repetitive rows
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_compressed_smaller_than_original_for_repetitive_rows() {
    let table = make_repetitive_table(500);
    let encoded = encode_to_vec(&table).expect("encode repetitive TableData");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 compressed ({} bytes) must be smaller than original ({} bytes) for repetitive rows",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// 11. Zstd compressed < original for repetitive rows
// ---------------------------------------------------------------------------
#[test]
fn test_zstd_compressed_smaller_than_original_for_repetitive_rows() {
    let table = make_repetitive_table(500);
    let encoded = encode_to_vec(&table).expect("encode repetitive TableData");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) must be smaller than original ({} bytes) for repetitive rows",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// 12. Both algorithms on same data produce equal decoded output
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_and_zstd_same_data_same_decoded_output() {
    let table = make_repetitive_table(100);
    let encoded = encode_to_vec(&table).expect("encode TableData for dual algo test");

    let lz4_compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress");
    let zstd_compressed = compress(&encoded, Compression::Zstd).expect("zstd compress");

    let lz4_decompressed = decompress(&lz4_compressed).expect("lz4 decompress");
    let zstd_decompressed = decompress(&zstd_compressed).expect("zstd decompress");

    let (lz4_decoded, _): (TableData, usize) =
        decode_from_slice(&lz4_decompressed).expect("decode lz4 TableData");
    let (zstd_decoded, _): (TableData, usize) =
        decode_from_slice(&zstd_decompressed).expect("decode zstd TableData");

    assert_eq!(
        lz4_decoded, zstd_decoded,
        "Both algorithms must decode to identical TableData"
    );
    assert_eq!(table, lz4_decoded);
}

// ---------------------------------------------------------------------------
// 13. Large table (1000+ rows) — LZ4 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_large_table_1000_rows_roundtrip() {
    let schema = vec![
        TableColumn {
            name: "session_id".into(),
            col_type: ColumnType::Varchar(36),
            nullable: false,
            primary_key: true,
        },
        TableColumn {
            name: "duration_ms".into(),
            col_type: ColumnType::Int64,
            nullable: false,
            primary_key: false,
        },
        TableColumn {
            name: "bytes_sent".into(),
            col_type: ColumnType::Float64,
            nullable: true,
            primary_key: false,
        },
    ];
    let rows: Vec<Row> = (0u64..1000)
        .map(|i| Row {
            values: vec![
                Some(format!("sess-{:08x}", i)),
                Some(format!("{}", i * 37 + 1)),
                Some(format!("{:.2}", i as f64 * 1.5)),
            ],
        })
        .collect();
    let table = TableData {
        schema,
        rows,
        table_name: "dim_sessions".into(),
    };

    let encoded = encode_to_vec(&table).expect("encode large table");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress large table");
    let decompressed = decompress(&compressed).expect("lz4 decompress large table");
    let (decoded, _): (TableData, usize) =
        decode_from_slice(&decompressed).expect("decode large table");
    assert_eq!(table, decoded);
    assert_eq!(decoded.rows.len(), 1000);
}

// ---------------------------------------------------------------------------
// 14. Large table (1000+ rows) — Zstd roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_zstd_large_table_1000_rows_roundtrip() {
    let table = make_repetitive_table(1000);
    let encoded = encode_to_vec(&table).expect("encode large repetitive table");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress large table");
    let decompressed = decompress(&compressed).expect("zstd decompress large table");
    let (decoded, _): (TableData, usize) =
        decode_from_slice(&decompressed).expect("decode large repetitive table");
    assert_eq!(table, decoded);
    assert_eq!(decoded.rows.len(), 1000);
}

// ---------------------------------------------------------------------------
// 15. All ColumnType variants encoded together — LZ4 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_all_column_type_variants_roundtrip() {
    let schema = vec![
        TableColumn {
            name: "c_int32".into(),
            col_type: ColumnType::Int32,
            nullable: false,
            primary_key: true,
        },
        TableColumn {
            name: "c_int64".into(),
            col_type: ColumnType::Int64,
            nullable: false,
            primary_key: false,
        },
        TableColumn {
            name: "c_float32".into(),
            col_type: ColumnType::Float32,
            nullable: true,
            primary_key: false,
        },
        TableColumn {
            name: "c_float64".into(),
            col_type: ColumnType::Float64,
            nullable: true,
            primary_key: false,
        },
        TableColumn {
            name: "c_varchar".into(),
            col_type: ColumnType::Varchar(255),
            nullable: true,
            primary_key: false,
        },
        TableColumn {
            name: "c_boolean".into(),
            col_type: ColumnType::Boolean,
            nullable: false,
            primary_key: false,
        },
        TableColumn {
            name: "c_timestamp".into(),
            col_type: ColumnType::Timestamp,
            nullable: true,
            primary_key: false,
        },
        TableColumn {
            name: "c_blob".into(),
            col_type: ColumnType::Blob,
            nullable: true,
            primary_key: false,
        },
    ];
    let table = TableData {
        schema,
        rows: vec![Row {
            values: vec![
                Some("1".into()),
                Some("9999999999".into()),
                Some("3.14".into()),
                Some("2.718281828".into()),
                Some("warehouse_node_eu_west_1".into()),
                Some("false".into()),
                Some("2025-06-01T00:00:00Z".into()),
                Some("deadbeef".into()),
            ],
        }],
        table_name: "wide_fact_table".into(),
    };
    let encoded = encode_to_vec(&table).expect("encode all-variants table");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress all variants");
    let decompressed = decompress(&compressed).expect("lz4 decompress all variants");
    let (decoded, _): (TableData, usize) =
        decode_from_slice(&decompressed).expect("decode all-variants table");
    assert_eq!(table, decoded);
    assert_eq!(decoded.schema.len(), 8);
}

// ---------------------------------------------------------------------------
// 16. All ColumnType variants — Zstd roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_zstd_all_column_type_variants_roundtrip() {
    let schema = vec![
        TableColumn {
            name: "c_int32".into(),
            col_type: ColumnType::Int32,
            nullable: false,
            primary_key: true,
        },
        TableColumn {
            name: "c_int64".into(),
            col_type: ColumnType::Int64,
            nullable: false,
            primary_key: false,
        },
        TableColumn {
            name: "c_float32".into(),
            col_type: ColumnType::Float32,
            nullable: true,
            primary_key: false,
        },
        TableColumn {
            name: "c_float64".into(),
            col_type: ColumnType::Float64,
            nullable: true,
            primary_key: false,
        },
        TableColumn {
            name: "c_varchar".into(),
            col_type: ColumnType::Varchar(1024),
            nullable: true,
            primary_key: false,
        },
        TableColumn {
            name: "c_boolean".into(),
            col_type: ColumnType::Boolean,
            nullable: false,
            primary_key: false,
        },
        TableColumn {
            name: "c_timestamp".into(),
            col_type: ColumnType::Timestamp,
            nullable: true,
            primary_key: false,
        },
        TableColumn {
            name: "c_blob".into(),
            col_type: ColumnType::Blob,
            nullable: true,
            primary_key: false,
        },
    ];
    let row = Row {
        values: (0..8).map(|i| Some(format!("value_{}", i))).collect(),
    };
    let table = TableData {
        schema,
        rows: vec![row; 10],
        table_name: "zstd_wide_table".into(),
    };
    let encoded = encode_to_vec(&table).expect("encode zstd all-variants");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress all variants");
    let decompressed = decompress(&compressed).expect("zstd decompress all variants");
    let (decoded, _): (TableData, usize) =
        decode_from_slice(&decompressed).expect("decode zstd all-variants");
    assert_eq!(table, decoded);
}

// ---------------------------------------------------------------------------
// 17. Empty table (no rows) — LZ4 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_empty_table_roundtrip() {
    let table = TableData {
        schema: make_orders_schema(),
        rows: vec![],
        table_name: "empty_staging_table".into(),
    };
    let encoded = encode_to_vec(&table).expect("encode empty table");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress empty table");
    let decompressed = decompress(&compressed).expect("lz4 decompress empty table");
    let (decoded, _): (TableData, usize) =
        decode_from_slice(&decompressed).expect("decode empty table");
    assert_eq!(table, decoded);
    assert_eq!(decoded.rows.len(), 0);
}

// ---------------------------------------------------------------------------
// 18. Empty table (no rows) — Zstd roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_zstd_empty_table_roundtrip() {
    let table = TableData {
        schema: vec![TableColumn {
            name: "id".into(),
            col_type: ColumnType::Int64,
            nullable: false,
            primary_key: true,
        }],
        rows: vec![],
        table_name: "empty_dimension_table".into(),
    };
    let encoded = encode_to_vec(&table).expect("encode empty dimension table");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress empty table");
    let decompressed = decompress(&compressed).expect("zstd decompress empty table");
    let (decoded, _): (TableData, usize) =
        decode_from_slice(&decompressed).expect("decode empty dimension table");
    assert_eq!(table, decoded);
    assert_eq!(decoded.rows.len(), 0);
}

// ---------------------------------------------------------------------------
// 19. Nested QueryPlan — LZ4 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_query_plan_roundtrip() {
    let plan = QueryPlan {
        query: "SELECT f.order_id, d.region, SUM(f.amount) \
                FROM fact_orders f \
                JOIN dim_region d ON f.region_id = d.id \
                GROUP BY f.order_id, d.region \
                HAVING SUM(f.amount) > 1000 \
                ORDER BY 3 DESC LIMIT 100"
            .into(),
        estimated_rows: 85_000,
        cost: 42_731.55,
        tables: vec![
            "fact_orders".into(),
            "dim_region".into(),
            "dim_customer".into(),
            "agg_daily_sales".into(),
        ],
    };
    let encoded = encode_to_vec(&plan).expect("encode QueryPlan");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress QueryPlan");
    let decompressed = decompress(&compressed).expect("lz4 decompress QueryPlan");
    let (decoded, _): (QueryPlan, usize) =
        decode_from_slice(&decompressed).expect("decode QueryPlan");
    assert_eq!(plan, decoded);
    assert_eq!(decoded.tables.len(), 4);
}

// ---------------------------------------------------------------------------
// 20. Nested QueryPlan — Zstd roundtrip (Vec of 50 plans)
// ---------------------------------------------------------------------------
#[test]
fn test_zstd_query_plan_vec_roundtrip() {
    let plans: Vec<QueryPlan> = (0u64..50)
        .map(|i| QueryPlan {
            query: format!(
                "SELECT * FROM shard_{} WHERE partition_key = {} \
                 AND ts BETWEEN '2025-01-01' AND '2025-12-31'",
                i % 8,
                i
            ),
            estimated_rows: i * 1000 + 500,
            cost: i as f64 * 12.75 + 0.5,
            tables: vec![
                format!("shard_{}", i % 8),
                "dim_date".into(),
                "dim_user".into(),
            ],
        })
        .collect();
    let encoded = encode_to_vec(&plans).expect("encode Vec<QueryPlan>");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress Vec<QueryPlan>");
    let decompressed = decompress(&compressed).expect("zstd decompress Vec<QueryPlan>");
    let (decoded, _): (Vec<QueryPlan>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<QueryPlan>");
    assert_eq!(plans, decoded);
    assert_eq!(decoded.len(), 50);
}

// ---------------------------------------------------------------------------
// 21. Row with nullable values — LZ4 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_lz4_row_with_nulls_roundtrip() {
    let schema = vec![
        TableColumn {
            name: "user_id".into(),
            col_type: ColumnType::Int64,
            nullable: false,
            primary_key: true,
        },
        TableColumn {
            name: "email".into(),
            col_type: ColumnType::Varchar(256),
            nullable: true,
            primary_key: false,
        },
        TableColumn {
            name: "last_login".into(),
            col_type: ColumnType::Timestamp,
            nullable: true,
            primary_key: false,
        },
        TableColumn {
            name: "score".into(),
            col_type: ColumnType::Float64,
            nullable: true,
            primary_key: false,
        },
    ];
    let rows = vec![
        Row {
            values: vec![
                Some("1".into()),
                Some("alice@example.com".into()),
                None,
                Some("9.8".into()),
            ],
        },
        Row {
            values: vec![
                Some("2".into()),
                None,
                Some("2025-03-01T12:00:00Z".into()),
                None,
            ],
        },
        Row {
            values: vec![Some("3".into()), None, None, None],
        },
    ];
    let table = TableData {
        schema,
        rows,
        table_name: "dim_users".into(),
    };
    let encoded = encode_to_vec(&table).expect("encode nullable rows");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress nullable rows");
    let decompressed = decompress(&compressed).expect("lz4 decompress nullable rows");
    let (decoded, _): (TableData, usize) =
        decode_from_slice(&decompressed).expect("decode nullable rows");
    assert_eq!(table, decoded);
    assert!(
        decoded.rows[1].values[1].is_none(),
        "email should be None for row 2"
    );
    assert!(
        decoded.rows[2].values[2].is_none(),
        "last_login should be None for row 3"
    );
}

// ---------------------------------------------------------------------------
// 22. Zstd compresses better than LZ4 on large repetitive data
// ---------------------------------------------------------------------------
#[test]
fn test_zstd_ratio_better_than_lz4_on_highly_repetitive_data() {
    let table = make_repetitive_table(2000);
    let encoded = encode_to_vec(&table).expect("encode 2000-row table for ratio comparison");

    let lz4_compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress 2000 rows");
    let zstd_compressed = compress(&encoded, Compression::Zstd).expect("zstd compress 2000 rows");

    let lz4_decompressed = decompress(&lz4_compressed).expect("lz4 decompress 2000 rows");
    let (lz4_decoded, _): (TableData, usize) =
        decode_from_slice(&lz4_decompressed).expect("lz4 decode 2000-row table");
    assert_eq!(table, lz4_decoded);

    let zstd_decompressed = decompress(&zstd_compressed).expect("zstd decompress 2000 rows");
    let (zstd_decoded, _): (TableData, usize) =
        decode_from_slice(&zstd_decompressed).expect("zstd decode 2000-row table");
    assert_eq!(table, zstd_decoded);

    assert!(
        zstd_compressed.len() <= lz4_compressed.len(),
        "Zstd ({} bytes) should compress at least as well as LZ4 ({} bytes) on 2000 identical rows",
        zstd_compressed.len(),
        lz4_compressed.len()
    );
}
