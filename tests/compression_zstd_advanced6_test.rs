#![cfg(feature = "compression-zstd")]
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

#[derive(Debug, PartialEq, Encode, Decode)]
enum ExperimentStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DataPoint {
    timestamp_us: u64,
    value: f64,
    channel: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ExperimentResult {
    experiment_id: u64,
    status: ExperimentStatus,
    data_points: Vec<DataPoint>,
    sample_rate_hz: u32,
    notes: Option<String>,
}

// ── test 1 ────────────────────────────────────────────────────────────────────
#[test]
fn test_experiment_result_completed_roundtrip_zstd() {
    let result = ExperimentResult {
        experiment_id: 1001,
        status: ExperimentStatus::Completed,
        data_points: vec![
            DataPoint {
                timestamp_us: 0,
                value: 1.23,
                channel: 0,
            },
            DataPoint {
                timestamp_us: 1000,
                value: 4.56,
                channel: 1,
            },
        ],
        sample_rate_hz: 1000,
        notes: Some("first run".to_string()),
    };
    let encoded = encode_to_vec(&result).expect("encode ExperimentResult");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress ExperimentResult");
    let decompressed = decompress(&compressed).expect("decompress ExperimentResult");
    let (decoded, _): (ExperimentResult, usize) =
        decode_from_slice(&decompressed).expect("decode ExperimentResult");
    assert_eq!(result, decoded);
}

// ── test 2 ────────────────────────────────────────────────────────────────────
#[test]
fn test_experiment_result_running_roundtrip_zstd() {
    let result = ExperimentResult {
        experiment_id: 2002,
        status: ExperimentStatus::Running,
        data_points: vec![DataPoint {
            timestamp_us: 500_000,
            value: -0.001,
            channel: 3,
        }],
        sample_rate_hz: 44100,
        notes: None,
    };
    let encoded = encode_to_vec(&result).expect("encode Running ExperimentResult");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress Running ExperimentResult");
    let decompressed = decompress(&compressed).expect("decompress Running ExperimentResult");
    let (decoded, _): (ExperimentResult, usize) =
        decode_from_slice(&decompressed).expect("decode Running ExperimentResult");
    assert_eq!(result, decoded);
}

// ── test 3 ────────────────────────────────────────────────────────────────────
#[test]
fn test_experiment_result_failed_roundtrip_zstd() {
    let result = ExperimentResult {
        experiment_id: 3003,
        status: ExperimentStatus::Failed,
        data_points: vec![],
        sample_rate_hz: 8000,
        notes: Some("sensor malfunction at t=0".to_string()),
    };
    let encoded = encode_to_vec(&result).expect("encode Failed ExperimentResult");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress Failed ExperimentResult");
    let decompressed = decompress(&compressed).expect("decompress Failed ExperimentResult");
    let (decoded, _): (ExperimentResult, usize) =
        decode_from_slice(&decompressed).expect("decode Failed ExperimentResult");
    assert_eq!(result, decoded);
    assert_eq!(decoded.data_points.len(), 0);
}

// ── test 4 ────────────────────────────────────────────────────────────────────
#[test]
fn test_experiment_result_cancelled_roundtrip_zstd() {
    let result = ExperimentResult {
        experiment_id: 4004,
        status: ExperimentStatus::Cancelled,
        data_points: vec![DataPoint {
            timestamp_us: 100,
            value: 0.0,
            channel: 0,
        }],
        sample_rate_hz: 250,
        notes: Some("cancelled by operator".to_string()),
    };
    let encoded = encode_to_vec(&result).expect("encode Cancelled ExperimentResult");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress Cancelled ExperimentResult");
    let decompressed = decompress(&compressed).expect("decompress Cancelled ExperimentResult");
    let (decoded, _): (ExperimentResult, usize) =
        decode_from_slice(&decompressed).expect("decode Cancelled ExperimentResult");
    assert_eq!(result, decoded);
}

// ── test 5 ────────────────────────────────────────────────────────────────────
#[test]
fn test_single_data_point_roundtrip_zstd() {
    let point = DataPoint {
        timestamp_us: 999_999_999,
        value: f64::MAX,
        channel: 255,
    };
    let encoded = encode_to_vec(&point).expect("encode single DataPoint");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress single DataPoint");
    let decompressed = decompress(&compressed).expect("decompress single DataPoint");
    let (decoded, _): (DataPoint, usize) =
        decode_from_slice(&decompressed).expect("decode single DataPoint");
    assert_eq!(point, decoded);
}

// ── test 6 ────────────────────────────────────────────────────────────────────
#[test]
fn test_vec_of_data_points_roundtrip_zstd() {
    let points: Vec<DataPoint> = (0u64..50)
        .map(|i| DataPoint {
            timestamp_us: i * 20_000,
            value: (i as f64) * 0.1 - 2.5,
            channel: (i % 8) as u8,
        })
        .collect();
    let encoded = encode_to_vec(&points).expect("encode Vec<DataPoint>");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Vec<DataPoint>");
    let decompressed = decompress(&compressed).expect("decompress Vec<DataPoint>");
    let (decoded, _): (Vec<DataPoint>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<DataPoint>");
    assert_eq!(points, decoded);
    assert_eq!(decoded.len(), 50);
}

// ── test 7 ────────────────────────────────────────────────────────────────────
#[test]
fn test_vec_of_experiment_results_roundtrip_zstd() {
    let results: Vec<ExperimentResult> = (0u64..8)
        .map(|i| ExperimentResult {
            experiment_id: 10_000 + i,
            status: ExperimentStatus::Completed,
            data_points: vec![DataPoint {
                timestamp_us: i * 1_000_000,
                value: i as f64 * 3.14,
                channel: (i % 4) as u8,
            }],
            sample_rate_hz: 1000 * (i as u32 + 1),
            notes: None,
        })
        .collect();
    let encoded = encode_to_vec(&results).expect("encode Vec<ExperimentResult>");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Vec<ExperimentResult>");
    let decompressed = decompress(&compressed).expect("decompress Vec<ExperimentResult>");
    let (decoded, _): (Vec<ExperimentResult>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<ExperimentResult>");
    assert_eq!(results, decoded);
}

// ── test 8 ────────────────────────────────────────────────────────────────────
#[test]
fn test_large_data_many_data_points_zstd() {
    let points: Vec<DataPoint> = (0u64..2000)
        .map(|i| DataPoint {
            timestamp_us: i * 500,
            value: (i as f64).sin(),
            channel: (i % 16) as u8,
        })
        .collect();
    let result = ExperimentResult {
        experiment_id: 99999,
        status: ExperimentStatus::Completed,
        data_points: points,
        sample_rate_hz: 2000,
        notes: Some("high-frequency acquisition".to_string()),
    };
    let encoded = encode_to_vec(&result).expect("encode large ExperimentResult");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress large ExperimentResult");
    let decompressed = decompress(&compressed).expect("decompress large ExperimentResult");
    let (decoded, _): (ExperimentResult, usize) =
        decode_from_slice(&decompressed).expect("decode large ExperimentResult");
    assert_eq!(result, decoded);
    assert_eq!(decoded.data_points.len(), 2000);
}

// ── test 9 ────────────────────────────────────────────────────────────────────
#[test]
fn test_empty_data_points_vec_zstd() {
    let result = ExperimentResult {
        experiment_id: 0,
        status: ExperimentStatus::Cancelled,
        data_points: vec![],
        sample_rate_hz: 0,
        notes: None,
    };
    let encoded = encode_to_vec(&result).expect("encode empty-data ExperimentResult");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress empty-data ExperimentResult");
    let decompressed = decompress(&compressed).expect("decompress empty-data ExperimentResult");
    let (decoded, _): (ExperimentResult, usize) =
        decode_from_slice(&decompressed).expect("decode empty-data ExperimentResult");
    assert_eq!(result, decoded);
    assert!(decoded.data_points.is_empty());
}

// ── test 10 ───────────────────────────────────────────────────────────────────
#[test]
fn test_experiment_status_all_variants_roundtrip_zstd() {
    let statuses = [
        ExperimentStatus::Running,
        ExperimentStatus::Completed,
        ExperimentStatus::Failed,
        ExperimentStatus::Cancelled,
    ];
    for status in statuses {
        let encoded = encode_to_vec(&status).expect("encode ExperimentStatus variant");
        let compressed =
            compress(&encoded, Compression::Zstd).expect("compress ExperimentStatus variant");
        let decompressed = decompress(&compressed).expect("decompress ExperimentStatus variant");
        let (decoded, _): (ExperimentStatus, usize) =
            decode_from_slice(&decompressed).expect("decode ExperimentStatus variant");
        assert_eq!(status, decoded);
    }
}

// ── test 11 ───────────────────────────────────────────────────────────────────
#[test]
fn test_compression_ratio_repetitive_data_points_zstd() {
    let base = DataPoint {
        timestamp_us: 12345678,
        value: 3.141592653589793,
        channel: 7,
    };
    let points: Vec<DataPoint> = (0..300)
        .map(|_| DataPoint {
            timestamp_us: base.timestamp_us,
            value: base.value,
            channel: base.channel,
        })
        .collect();
    let encoded = encode_to_vec(&points).expect("encode repetitive DataPoints");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress repetitive DataPoints");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd must compress repetitive data: original={} compressed={}",
        encoded.len(),
        compressed.len()
    );
}

// ── test 12 ───────────────────────────────────────────────────────────────────
#[test]
fn test_compression_ratio_repetitive_experiment_results_zstd() {
    let results: Vec<ExperimentResult> = (0..50)
        .map(|i| ExperimentResult {
            experiment_id: i % 5,
            status: ExperimentStatus::Completed,
            data_points: vec![DataPoint {
                timestamp_us: 0,
                value: 1.0,
                channel: 0,
            }],
            sample_rate_hz: 1000,
            notes: Some("repeated experiment note".to_string()),
        })
        .collect();
    let encoded = encode_to_vec(&results).expect("encode repetitive ExperimentResults");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress repetitive ExperimentResults");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd must compress repetitive experiment results: original={} compressed={}",
        encoded.len(),
        compressed.len()
    );
}

// ── test 13 ───────────────────────────────────────────────────────────────────
#[test]
fn test_decompressed_matches_original_bytes_zstd() {
    let result = ExperimentResult {
        experiment_id: 7777,
        status: ExperimentStatus::Completed,
        data_points: vec![
            DataPoint {
                timestamp_us: 0,
                value: 0.0,
                channel: 0,
            },
            DataPoint {
                timestamp_us: 1000,
                value: 1.0,
                channel: 1,
            },
            DataPoint {
                timestamp_us: 2000,
                value: 2.0,
                channel: 2,
            },
        ],
        sample_rate_hz: 1000,
        notes: Some("byte-level verification".to_string()),
    };
    let encoded = encode_to_vec(&result).expect("encode for byte-level check");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress for byte-level check");
    let decompressed = decompress(&compressed).expect("decompress for byte-level check");
    assert_eq!(
        encoded, decompressed,
        "decompressed bytes must exactly match original encoded bytes"
    );
}

// ── test 14 ───────────────────────────────────────────────────────────────────
#[test]
fn test_idempotent_compress_decompress_cycle_zstd() {
    let result = ExperimentResult {
        experiment_id: 5555,
        status: ExperimentStatus::Running,
        data_points: vec![DataPoint {
            timestamp_us: 42,
            value: 2.718,
            channel: 4,
        }],
        sample_rate_hz: 100,
        notes: None,
    };
    let encoded = encode_to_vec(&result).expect("encode for idempotence test");
    let compressed1 = compress(&encoded, Compression::Zstd).expect("first compress");
    let decompressed1 = decompress(&compressed1).expect("first decompress");
    let compressed2 = compress(&decompressed1, Compression::Zstd).expect("second compress");
    let decompressed2 = decompress(&compressed2).expect("second decompress");
    assert_eq!(
        encoded, decompressed1,
        "first decompress must match original"
    );
    assert_eq!(
        encoded, decompressed2,
        "second decompress must match original"
    );
}

// ── test 15 ───────────────────────────────────────────────────────────────────
#[test]
fn test_consumed_bytes_experiment_result_zstd() {
    let result = ExperimentResult {
        experiment_id: 6666,
        status: ExperimentStatus::Completed,
        data_points: vec![DataPoint {
            timestamp_us: 9999,
            value: -1.5,
            channel: 2,
        }],
        sample_rate_hz: 500,
        notes: Some("consumed bytes check".to_string()),
    };
    let encoded = encode_to_vec(&result).expect("encode for consumed bytes test");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress for consumed bytes test");
    let decompressed = decompress(&compressed).expect("decompress for consumed bytes test");
    let (decoded, consumed): (ExperimentResult, usize) =
        decode_from_slice(&decompressed).expect("decode for consumed bytes test");
    assert_eq!(result, decoded);
    assert_eq!(
        consumed,
        decompressed.len(),
        "consumed bytes must equal decompressed length"
    );
}

// ── test 16 ───────────────────────────────────────────────────────────────────
#[test]
fn test_option_notes_none_roundtrip_zstd() {
    let result = ExperimentResult {
        experiment_id: 11,
        status: ExperimentStatus::Completed,
        data_points: vec![DataPoint {
            timestamp_us: 1,
            value: 1.1,
            channel: 1,
        }],
        sample_rate_hz: 10,
        notes: None,
    };
    let encoded = encode_to_vec(&result).expect("encode None notes");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress None notes");
    let decompressed = decompress(&compressed).expect("decompress None notes");
    let (decoded, _): (ExperimentResult, usize) =
        decode_from_slice(&decompressed).expect("decode None notes");
    assert_eq!(result, decoded);
    assert!(decoded.notes.is_none());
}

// ── test 17 ───────────────────────────────────────────────────────────────────
#[test]
fn test_option_notes_some_roundtrip_zstd() {
    let result = ExperimentResult {
        experiment_id: 22,
        status: ExperimentStatus::Failed,
        data_points: vec![],
        sample_rate_hz: 48000,
        notes: Some("signal overload detected on channel 3".to_string()),
    };
    let encoded = encode_to_vec(&result).expect("encode Some notes");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Some notes");
    let decompressed = decompress(&compressed).expect("decompress Some notes");
    let (decoded, _): (ExperimentResult, usize) =
        decode_from_slice(&decompressed).expect("decode Some notes");
    assert_eq!(result, decoded);
    assert!(decoded.notes.is_some());
}

// ── test 18 ───────────────────────────────────────────────────────────────────
#[test]
fn test_zstd_level_variant_experiment_result_roundtrip() {
    let result = ExperimentResult {
        experiment_id: 8888,
        status: ExperimentStatus::Completed,
        data_points: (0u64..10)
            .map(|i| DataPoint {
                timestamp_us: i * 100,
                value: i as f64,
                channel: 0,
            })
            .collect(),
        sample_rate_hz: 10000,
        notes: Some("zstd level variant test".to_string()),
    };
    let encoded = encode_to_vec(&result).expect("encode for ZstdLevel variant");
    let compressed =
        compress(&encoded, Compression::ZstdLevel(9)).expect("compress with ZstdLevel(9)");
    let decompressed = decompress(&compressed).expect("decompress ZstdLevel(9) data");
    let (decoded, _): (ExperimentResult, usize) =
        decode_from_slice(&decompressed).expect("decode ZstdLevel(9) result");
    assert_eq!(result, decoded);
}

// ── test 19 ───────────────────────────────────────────────────────────────────
#[test]
fn test_zstd_level_high_compression_experiment_result_roundtrip() {
    let result = ExperimentResult {
        experiment_id: 9999,
        status: ExperimentStatus::Completed,
        data_points: (0u64..20)
            .map(|i| DataPoint {
                timestamp_us: i * 50,
                value: (i as f64).cos(),
                channel: (i % 4) as u8,
            })
            .collect(),
        sample_rate_hz: 20000,
        notes: Some("high compression level test".to_string()),
    };
    let encoded = encode_to_vec(&result).expect("encode for ZstdLevel(19)");
    let compressed =
        compress(&encoded, Compression::ZstdLevel(19)).expect("compress with ZstdLevel(19)");
    let decompressed = decompress(&compressed).expect("decompress ZstdLevel(19) data");
    let (decoded, _): (ExperimentResult, usize) =
        decode_from_slice(&decompressed).expect("decode ZstdLevel(19) result");
    assert_eq!(result, decoded);
}

// ── test 20 ───────────────────────────────────────────────────────────────────
#[test]
fn test_multiple_experiments_mixed_status_zstd() {
    let experiments: Vec<ExperimentResult> = vec![
        ExperimentResult {
            experiment_id: 1,
            status: ExperimentStatus::Running,
            data_points: vec![DataPoint {
                timestamp_us: 0,
                value: 0.5,
                channel: 0,
            }],
            sample_rate_hz: 1000,
            notes: None,
        },
        ExperimentResult {
            experiment_id: 2,
            status: ExperimentStatus::Completed,
            data_points: vec![
                DataPoint {
                    timestamp_us: 0,
                    value: 1.0,
                    channel: 1,
                },
                DataPoint {
                    timestamp_us: 1000,
                    value: 2.0,
                    channel: 1,
                },
            ],
            sample_rate_hz: 1000,
            notes: Some("baseline".to_string()),
        },
        ExperimentResult {
            experiment_id: 3,
            status: ExperimentStatus::Failed,
            data_points: vec![],
            sample_rate_hz: 500,
            notes: Some("power failure".to_string()),
        },
        ExperimentResult {
            experiment_id: 4,
            status: ExperimentStatus::Cancelled,
            data_points: vec![DataPoint {
                timestamp_us: 100,
                value: -99.9,
                channel: 3,
            }],
            sample_rate_hz: 200,
            notes: None,
        },
    ];
    let encoded = encode_to_vec(&experiments).expect("encode mixed-status experiments");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress mixed-status experiments");
    let decompressed = decompress(&compressed).expect("decompress mixed-status experiments");
    let (decoded, _): (Vec<ExperimentResult>, usize) =
        decode_from_slice(&decompressed).expect("decode mixed-status experiments");
    assert_eq!(experiments, decoded);
    assert_eq!(decoded.len(), 4);
}

// ── test 21 ───────────────────────────────────────────────────────────────────
#[test]
fn test_data_point_extreme_values_zstd() {
    let points = vec![
        DataPoint {
            timestamp_us: 0,
            value: f64::MIN,
            channel: 0,
        },
        DataPoint {
            timestamp_us: u64::MAX,
            value: f64::MAX,
            channel: 255,
        },
        DataPoint {
            timestamp_us: u64::MAX / 2,
            value: 0.0,
            channel: 128,
        },
        DataPoint {
            timestamp_us: 1,
            value: f64::EPSILON,
            channel: 1,
        },
    ];
    let encoded = encode_to_vec(&points).expect("encode extreme-value DataPoints");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress extreme-value DataPoints");
    let decompressed = decompress(&compressed).expect("decompress extreme-value DataPoints");
    let (decoded, _): (Vec<DataPoint>, usize) =
        decode_from_slice(&decompressed).expect("decode extreme-value DataPoints");
    assert_eq!(points, decoded);
}

// ── test 22 ───────────────────────────────────────────────────────────────────
#[test]
fn test_error_on_corrupt_zstd_data() {
    let result = ExperimentResult {
        experiment_id: 1234,
        status: ExperimentStatus::Completed,
        data_points: vec![DataPoint {
            timestamp_us: 500,
            value: 42.0,
            channel: 0,
        }],
        sample_rate_hz: 100,
        notes: Some("corruption test".to_string()),
    };
    let encoded = encode_to_vec(&result).expect("encode for corruption test");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress for corruption test");
    let mut corrupted = compressed.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    assert!(
        decompress(&corrupted).is_err(),
        "decompressing corrupted Zstd data must return an error"
    );
}
