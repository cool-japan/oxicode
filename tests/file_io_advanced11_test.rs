//! Advanced file I/O encoding tests for OxiCode (set 11)

#![cfg(feature = "std")]
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
use oxicode::{Decode, Encode};
use std::env::temp_dir;

#[derive(Debug, PartialEq, Encode, Decode)]
struct PipelineStage {
    stage_id: u32,
    name: String,
    input_schema: Vec<String>,
    output_schema: Vec<String>,
    parallel: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PipelineError {
    Timeout { stage: u32, ms: u64 },
    SchemaViolation { field: String },
    ResourceExhausted,
    UserCancelled(String),
}

// Test 1: PipelineStage basic roundtrip to file
#[test]
fn test_adv11_01_pipeline_stage_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 1));
    let stage = PipelineStage {
        stage_id: 100,
        name: "data_ingestion".to_string(),
        input_schema: vec!["raw_bytes".to_string(), "metadata".to_string()],
        output_schema: vec!["parsed_record".to_string()],
        parallel: true,
    };
    oxicode::encode_to_file(&stage, &path).expect("encode PipelineStage to file");
    let decoded: PipelineStage =
        oxicode::decode_from_file(&path).expect("decode PipelineStage from file");
    assert_eq!(stage, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 2: PipelineError::Timeout roundtrip
#[test]
fn test_adv11_02_pipeline_error_timeout_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 2));
    let err = PipelineError::Timeout {
        stage: 3,
        ms: 30_000,
    };
    oxicode::encode_to_file(&err, &path).expect("encode PipelineError::Timeout to file");
    let decoded: PipelineError =
        oxicode::decode_from_file(&path).expect("decode PipelineError::Timeout from file");
    assert_eq!(err, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 3: PipelineError::SchemaViolation roundtrip
#[test]
fn test_adv11_03_pipeline_error_schema_violation_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 3));
    let err = PipelineError::SchemaViolation {
        field: "user_id".to_string(),
    };
    oxicode::encode_to_file(&err, &path).expect("encode PipelineError::SchemaViolation to file");
    let decoded: PipelineError =
        oxicode::decode_from_file(&path).expect("decode PipelineError::SchemaViolation from file");
    assert_eq!(err, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 4: PipelineError::ResourceExhausted roundtrip
#[test]
fn test_adv11_04_pipeline_error_resource_exhausted_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 4));
    let err = PipelineError::ResourceExhausted;
    oxicode::encode_to_file(&err, &path).expect("encode PipelineError::ResourceExhausted to file");
    let decoded: PipelineError = oxicode::decode_from_file(&path)
        .expect("decode PipelineError::ResourceExhausted from file");
    assert_eq!(err, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 5: PipelineError::UserCancelled roundtrip
#[test]
fn test_adv11_05_pipeline_error_user_cancelled_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 5));
    let err = PipelineError::UserCancelled("operator requested abort".to_string());
    oxicode::encode_to_file(&err, &path).expect("encode PipelineError::UserCancelled to file");
    let decoded: PipelineError =
        oxicode::decode_from_file(&path).expect("decode PipelineError::UserCancelled from file");
    assert_eq!(err, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 6: Vec<PipelineStage> roundtrip
#[test]
fn test_adv11_06_vec_pipeline_stage_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 6));
    let stages: Vec<PipelineStage> = (0..6)
        .map(|i| PipelineStage {
            stage_id: i as u32,
            name: format!("stage_{:02}", i),
            input_schema: vec![format!("in_field_{}", i)],
            output_schema: vec![format!("out_field_{}", i), format!("meta_{}", i)],
            parallel: i % 2 == 0,
        })
        .collect();
    oxicode::encode_to_file(&stages, &path).expect("encode Vec<PipelineStage> to file");
    let decoded: Vec<PipelineStage> =
        oxicode::decode_from_file(&path).expect("decode Vec<PipelineStage> from file");
    assert_eq!(stages, decoded);
    assert_eq!(decoded.len(), 6);
    std::fs::remove_file(&path).ok();
}

// Test 7: Vec<PipelineError> all variants roundtrip
#[test]
fn test_adv11_07_vec_all_pipeline_error_variants_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 7));
    let errors: Vec<PipelineError> = vec![
        PipelineError::Timeout { stage: 0, ms: 1 },
        PipelineError::Timeout {
            stage: u32::MAX,
            ms: u64::MAX,
        },
        PipelineError::SchemaViolation {
            field: "timestamp".to_string(),
        },
        PipelineError::SchemaViolation {
            field: "".to_string(),
        },
        PipelineError::ResourceExhausted,
        PipelineError::UserCancelled("quit".to_string()),
        PipelineError::UserCancelled("".to_string()),
    ];
    oxicode::encode_to_file(&errors, &path)
        .expect("encode Vec<PipelineError> all variants to file");
    let decoded: Vec<PipelineError> =
        oxicode::decode_from_file(&path).expect("decode Vec<PipelineError> all variants from file");
    assert_eq!(errors, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 8: PipelineStage with empty schemas roundtrip
#[test]
fn test_adv11_08_pipeline_stage_empty_schemas_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 8));
    let stage = PipelineStage {
        stage_id: 0,
        name: "source_stage".to_string(),
        input_schema: vec![],
        output_schema: vec!["raw_event".to_string()],
        parallel: false,
    };
    oxicode::encode_to_file(&stage, &path)
        .expect("encode PipelineStage empty input_schema to file");
    let decoded: PipelineStage = oxicode::decode_from_file(&path)
        .expect("decode PipelineStage empty input_schema from file");
    assert_eq!(stage, decoded);
    assert!(decoded.input_schema.is_empty());
    std::fs::remove_file(&path).ok();
}

// Test 9: PipelineStage with fixed-int config roundtrip
#[test]
fn test_adv11_09_pipeline_stage_fixed_int_config_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 9));
    let cfg = oxicode::config::standard().with_fixed_int_encoding();
    let stage = PipelineStage {
        stage_id: 999,
        name: "transform_fixed_int".to_string(),
        input_schema: vec!["col_a".to_string(), "col_b".to_string()],
        output_schema: vec!["col_c".to_string()],
        parallel: true,
    };
    oxicode::encode_to_file_with_config(&stage, &path, cfg)
        .expect("encode PipelineStage with fixed-int config to file");
    let decoded: PipelineStage = oxicode::decode_from_file_with_config(&path, cfg)
        .expect("decode PipelineStage with fixed-int config from file");
    assert_eq!(stage, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 10: PipelineError big-endian config roundtrip
#[test]
fn test_adv11_10_pipeline_error_big_endian_config_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 10));
    let cfg = oxicode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let err = PipelineError::Timeout {
        stage: 7,
        ms: 5_000,
    };
    oxicode::encode_to_file_with_config(&err, &path, cfg)
        .expect("encode PipelineError big-endian to file");
    let decoded: PipelineError = oxicode::decode_from_file_with_config(&path, cfg)
        .expect("decode PipelineError big-endian from file");
    assert_eq!(err, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 11: File size matches encode_to_vec for PipelineStage
#[test]
fn test_adv11_11_file_size_matches_encode_to_vec_pipeline_stage() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 11));
    let stage = PipelineStage {
        stage_id: 42,
        name: "size_check_stage".to_string(),
        input_schema: vec!["event".to_string(), "context".to_string()],
        output_schema: vec!["enriched_event".to_string()],
        parallel: false,
    };
    oxicode::encode_to_file(&stage, &path).expect("encode PipelineStage to file for size check");
    let metadata = std::fs::metadata(&path).expect("get file metadata for size check");
    let vec_bytes =
        oxicode::encode_to_vec(&stage).expect("encode PipelineStage to vec for size check");
    assert_eq!(
        metadata.len() as usize,
        vec_bytes.len(),
        "file size must equal encode_to_vec byte length"
    );
    std::fs::remove_file(&path).ok();
}

// Test 12: Overwrite PipelineStage file with new value
#[test]
fn test_adv11_12_overwrite_pipeline_stage_file() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 12));
    let first = PipelineStage {
        stage_id: 1,
        name: "first_stage".to_string(),
        input_schema: vec!["input_a".to_string()],
        output_schema: vec!["output_a".to_string()],
        parallel: false,
    };
    let second = PipelineStage {
        stage_id: 2,
        name: "second_stage_overwrites_first".to_string(),
        input_schema: vec!["input_b".to_string(), "input_c".to_string()],
        output_schema: vec!["output_b".to_string()],
        parallel: true,
    };
    oxicode::encode_to_file(&first, &path).expect("encode first PipelineStage to file");
    oxicode::encode_to_file(&second, &path)
        .expect("encode second PipelineStage to file (overwrite)");
    let decoded: PipelineStage =
        oxicode::decode_from_file(&path).expect("decode overwritten PipelineStage from file");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 13: Non-existent file returns error for PipelineStage
#[test]
fn test_adv11_13_nonexistent_file_pipeline_stage_error() {
    let path = temp_dir().join(format!(
        "oxicode_adv11_{}_{}_nonexistent.bin",
        std::process::id(),
        13
    ));
    std::fs::remove_file(&path).ok();
    let result = oxicode::decode_from_file::<PipelineStage>(&path);
    assert!(
        result.is_err(),
        "Expected error when decoding from non-existent file"
    );
}

// Test 14: Sequential encode_into_std_write / decode_from_std_read for PipelineStage + PipelineError
#[test]
fn test_adv11_14_sequential_encode_into_std_write_decode_from_std_read() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 14));
    let cfg = oxicode::config::standard();

    let make_stage = || PipelineStage {
        stage_id: 10,
        name: "sequential_stage".to_string(),
        input_schema: vec!["field_x".to_string()],
        output_schema: vec!["field_y".to_string()],
        parallel: false,
    };
    let make_error = || PipelineError::SchemaViolation {
        field: "required_field".to_string(),
    };
    let make_cancelled = || PipelineError::UserCancelled("user pressed ctrl+c".to_string());

    let mut file = std::fs::File::create(&path).expect("create file for sequential write");
    let n1 = oxicode::encode_into_std_write(make_stage(), &mut file, cfg)
        .expect("encode PipelineStage via encode_into_std_write");
    let n2 = oxicode::encode_into_std_write(make_error(), &mut file, cfg)
        .expect("encode PipelineError::SchemaViolation via encode_into_std_write");
    let n3 = oxicode::encode_into_std_write(make_cancelled(), &mut file, cfg)
        .expect("encode PipelineError::UserCancelled via encode_into_std_write");
    assert!(n1 > 0);
    assert!(n2 > 0);
    assert!(n3 > 0);
    drop(file);

    let raw = std::fs::read(&path).expect("read sequential file");
    let mut cursor = std::io::Cursor::new(raw);

    let decoded_stage: PipelineStage =
        oxicode::decode_from_std_read(&mut cursor, cfg).expect("decode PipelineStage");
    let decoded_error: PipelineError = oxicode::decode_from_std_read(&mut cursor, cfg)
        .expect("decode PipelineError::SchemaViolation");
    let decoded_cancelled: PipelineError = oxicode::decode_from_std_read(&mut cursor, cfg)
        .expect("decode PipelineError::UserCancelled");

    assert_eq!(make_stage(), decoded_stage);
    assert_eq!(make_error(), decoded_error);
    assert_eq!(make_cancelled(), decoded_cancelled);
    std::fs::remove_file(&path).ok();
}

// Test 15: Option<PipelineStage> Some roundtrip
#[test]
fn test_adv11_15_option_pipeline_stage_some_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 15));
    let val: Option<PipelineStage> = Some(PipelineStage {
        stage_id: 77,
        name: "optional_stage".to_string(),
        input_schema: vec!["opt_in".to_string()],
        output_schema: vec!["opt_out".to_string()],
        parallel: true,
    });
    oxicode::encode_to_file(&val, &path).expect("encode Option<PipelineStage> Some to file");
    let decoded: Option<PipelineStage> =
        oxicode::decode_from_file(&path).expect("decode Option<PipelineStage> Some from file");
    assert_eq!(val, decoded);
    assert!(decoded.is_some());
    std::fs::remove_file(&path).ok();
}

// Test 16: Option<PipelineStage> None roundtrip
#[test]
fn test_adv11_16_option_pipeline_stage_none_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 16));
    let val: Option<PipelineStage> = None;
    oxicode::encode_to_file(&val, &path).expect("encode Option<PipelineStage> None to file");
    let decoded: Option<PipelineStage> =
        oxicode::decode_from_file(&path).expect("decode Option<PipelineStage> None from file");
    assert_eq!(val, decoded);
    assert!(decoded.is_none());
    std::fs::remove_file(&path).ok();
}

// Test 17: PipelineStage with large schema fields (many columns)
#[test]
fn test_adv11_17_pipeline_stage_large_schema_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 17));
    let stage = PipelineStage {
        stage_id: u32::MAX,
        name: "wide_transform".to_string(),
        input_schema: (0..50).map(|i| format!("input_col_{:03}", i)).collect(),
        output_schema: (0..30).map(|i| format!("output_col_{:03}", i)).collect(),
        parallel: true,
    };
    assert_eq!(stage.input_schema.len(), 50);
    assert_eq!(stage.output_schema.len(), 30);
    oxicode::encode_to_file(&stage, &path).expect("encode large-schema PipelineStage to file");
    let decoded: PipelineStage =
        oxicode::decode_from_file(&path).expect("decode large-schema PipelineStage from file");
    assert_eq!(stage, decoded);
    assert_eq!(decoded.input_schema.len(), 50);
    assert_eq!(decoded.output_schema.len(), 30);
    std::fs::remove_file(&path).ok();
}

// Test 18: PipelineError::Timeout with boundary values (stage=0, ms=u64::MAX)
#[test]
fn test_adv11_18_pipeline_error_timeout_boundary_values_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 18));
    let err = PipelineError::Timeout {
        stage: 0,
        ms: u64::MAX,
    };
    oxicode::encode_to_file(&err, &path)
        .expect("encode PipelineError::Timeout boundary values to file");
    let decoded: PipelineError = oxicode::decode_from_file(&path)
        .expect("decode PipelineError::Timeout boundary values from file");
    assert_eq!(err, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 19: PipelineStage with unicode name and schema entries
#[test]
fn test_adv11_19_pipeline_stage_unicode_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 19));
    let stage = PipelineStage {
        stage_id: 55,
        name: "变换阶段_Étape_トランスフォーム".to_string(),
        input_schema: vec![
            "フィールド_A".to_string(),
            "champ_données".to_string(),
            "поле_данных".to_string(),
        ],
        output_schema: vec!["出力_результат".to_string()],
        parallel: false,
    };
    oxicode::encode_to_file(&stage, &path).expect("encode unicode PipelineStage to file");
    let decoded: PipelineStage =
        oxicode::decode_from_file(&path).expect("decode unicode PipelineStage from file");
    assert_eq!(stage, decoded);
    assert_eq!(
        decoded.stage_id, 55,
        "stage_id must survive unicode roundtrip"
    );
    assert!(
        !decoded.input_schema.is_empty(),
        "input_schema must not be empty after unicode roundtrip"
    );
    std::fs::remove_file(&path).ok();
}

// Test 20: PipelineError::UserCancelled with long message roundtrip
#[test]
fn test_adv11_20_pipeline_error_user_cancelled_long_message_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 20));
    let long_msg = "abort: ".repeat(500) + "pipeline terminated by operator at step 42";
    let err = PipelineError::UserCancelled(long_msg.clone());
    oxicode::encode_to_file(&err, &path)
        .expect("encode PipelineError::UserCancelled long message to file");
    let decoded: PipelineError = oxicode::decode_from_file(&path)
        .expect("decode PipelineError::UserCancelled long message from file");
    assert_eq!(err, decoded);
    if let PipelineError::UserCancelled(msg) = &decoded {
        assert_eq!(msg, &long_msg);
    } else {
        panic!("expected UserCancelled variant");
    }
    std::fs::remove_file(&path).ok();
}

// Test 21: Raw bytes from encode_to_file match encode_to_vec for PipelineError
#[test]
fn test_adv11_21_raw_bytes_match_encode_to_vec_pipeline_error() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 21));
    let err = PipelineError::Timeout {
        stage: 12,
        ms: 60_000,
    };
    oxicode::encode_to_file(&err, &path).expect("encode PipelineError to file for raw bytes check");
    let file_bytes = std::fs::read(&path).expect("read raw bytes from file");
    let vec_bytes =
        oxicode::encode_to_vec(&err).expect("encode PipelineError to vec for raw bytes check");
    assert_eq!(
        file_bytes, vec_bytes,
        "raw file bytes must match encode_to_vec output"
    );
    std::fs::remove_file(&path).ok();
}

// Test 22: Vec<(PipelineStage, PipelineError)> tuple pairs roundtrip
#[test]
fn test_adv11_22_vec_tuple_pipeline_stage_error_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv11_{}_{}.bin", std::process::id(), 22));
    let pairs: Vec<(PipelineStage, PipelineError)> = vec![
        (
            PipelineStage {
                stage_id: 1,
                name: "ingest".to_string(),
                input_schema: vec!["raw".to_string()],
                output_schema: vec!["parsed".to_string()],
                parallel: false,
            },
            PipelineError::Timeout { stage: 1, ms: 100 },
        ),
        (
            PipelineStage {
                stage_id: 2,
                name: "validate".to_string(),
                input_schema: vec!["parsed".to_string()],
                output_schema: vec!["validated".to_string()],
                parallel: true,
            },
            PipelineError::SchemaViolation {
                field: "email".to_string(),
            },
        ),
        (
            PipelineStage {
                stage_id: 3,
                name: "output".to_string(),
                input_schema: vec!["validated".to_string()],
                output_schema: vec![],
                parallel: false,
            },
            PipelineError::ResourceExhausted,
        ),
    ];
    oxicode::encode_to_file(&pairs, &path)
        .expect("encode Vec<(PipelineStage, PipelineError)> to file");
    let decoded: Vec<(PipelineStage, PipelineError)> = oxicode::decode_from_file(&path)
        .expect("decode Vec<(PipelineStage, PipelineError)> from file");
    assert_eq!(pairs, decoded);
    assert_eq!(decoded.len(), 3);
    std::fs::remove_file(&path).ok();
}
