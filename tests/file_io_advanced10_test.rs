//! Advanced file I/O encoding tests for OxiCode (set 10)

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
struct Experiment {
    id: u64,
    name: String,
    parameters: Vec<f64>,
    tags: Vec<String>,
    success: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ExperimentStatus {
    Pending,
    Running { progress_pct: u8 },
    Complete { result_id: u64 },
    Failed { error: String },
}

// Test 1: Experiment roundtrip to file
#[test]
fn test_adv10_01_experiment_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_1.bin", std::process::id()));
    let exp = Experiment {
        id: 1001,
        name: "baseline_experiment".to_string(),
        parameters: vec![0.1, 0.5, 1.0, 2.5, 10.0],
        tags: vec!["control".to_string(), "baseline".to_string()],
        success: true,
    };
    oxicode::encode_to_file(&exp, &path).expect("encode Experiment to file");
    let decoded: Experiment =
        oxicode::decode_from_file(&path).expect("decode Experiment from file");
    assert_eq!(exp, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 2: ExperimentStatus::Pending roundtrip
#[test]
fn test_adv10_02_experiment_status_pending_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_2.bin", std::process::id()));
    let status = ExperimentStatus::Pending;
    oxicode::encode_to_file(&status, &path).expect("encode ExperimentStatus::Pending to file");
    let decoded: ExperimentStatus =
        oxicode::decode_from_file(&path).expect("decode ExperimentStatus::Pending from file");
    assert_eq!(status, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 3: ExperimentStatus::Complete roundtrip
#[test]
fn test_adv10_03_experiment_status_complete_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_3.bin", std::process::id()));
    let status = ExperimentStatus::Complete {
        result_id: 99_999_999,
    };
    oxicode::encode_to_file(&status, &path).expect("encode ExperimentStatus::Complete to file");
    let decoded: ExperimentStatus =
        oxicode::decode_from_file(&path).expect("decode ExperimentStatus::Complete from file");
    assert_eq!(status, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 4: Vec<Experiment> roundtrip
#[test]
fn test_adv10_04_vec_experiment_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_4.bin", std::process::id()));
    let experiments: Vec<Experiment> = (0..5)
        .map(|i| Experiment {
            id: i as u64,
            name: format!("experiment_{}", i),
            parameters: vec![i as f64 * 0.1, i as f64 * 0.2],
            tags: vec![format!("tag_{}", i)],
            success: i % 2 == 0,
        })
        .collect();
    oxicode::encode_to_file(&experiments, &path).expect("encode Vec<Experiment> to file");
    let decoded: Vec<Experiment> =
        oxicode::decode_from_file(&path).expect("decode Vec<Experiment> from file");
    assert_eq!(experiments, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 5: u32 to file
#[test]
fn test_adv10_05_u32_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_5.bin", std::process::id()));
    let val: u32 = 2_718_281;
    oxicode::encode_to_file(&val, &path).expect("encode u32 to file");
    let decoded: u32 = oxicode::decode_from_file(&path).expect("decode u32 from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 6: String to file
#[test]
fn test_adv10_06_string_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_6.bin", std::process::id()));
    let val = "OxiCode advanced file I/O test #10 — experiment data".to_string();
    oxicode::encode_to_file(&val, &path).expect("encode String to file");
    let decoded: String = oxicode::decode_from_file(&path).expect("decode String from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 7: Vec<u8> to file
#[test]
fn test_adv10_07_vec_u8_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_7.bin", std::process::id()));
    let val: Vec<u8> = (0u8..=255).collect();
    oxicode::encode_to_file(&val, &path).expect("encode Vec<u8> to file");
    let decoded: Vec<u8> = oxicode::decode_from_file(&path).expect("decode Vec<u8> from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 8: Fixed-int config roundtrip
#[test]
fn test_adv10_08_fixed_int_config_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_8.bin", std::process::id()));
    let cfg = oxicode::config::standard().with_fixed_int_encoding();
    let exp = Experiment {
        id: 42,
        name: "fixed_int_test".to_string(),
        parameters: vec![3.25, 2.5],
        tags: vec!["physics".to_string()],
        success: false,
    };
    oxicode::encode_to_file_with_config(&exp, &path, cfg)
        .expect("encode Experiment fixed-int to file");
    let decoded: Experiment = oxicode::decode_from_file_with_config(&path, cfg)
        .expect("decode Experiment fixed-int from file");
    assert_eq!(exp, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 9: Big-endian config roundtrip (verify bytes)
#[test]
fn test_adv10_09_big_endian_config_verify_bytes() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_9.bin", std::process::id()));
    let cfg_be = oxicode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let val: u32 = 0xDEADBEEF;
    oxicode::encode_to_file_with_config(&val, &path, cfg_be)
        .expect("encode u32 big-endian fixed-int to file");

    let raw = std::fs::read(&path).expect("read raw bytes for big-endian verification");
    assert_eq!(
        raw,
        vec![0xDE, 0xAD, 0xBE, 0xEF],
        "big-endian byte order should match"
    );

    let decoded: u32 = oxicode::decode_from_file_with_config(&path, cfg_be)
        .expect("decode u32 big-endian from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 10: Large Experiment (100 parameters, 20 tags)
#[test]
fn test_adv10_10_large_experiment_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_10.bin", std::process::id()));
    let exp = Experiment {
        id: u64::MAX / 2,
        name: "large_experiment_with_many_parameters".to_string(),
        parameters: (0..100).map(|i| i as f64 * 0.01).collect(),
        tags: (0..20).map(|i| format!("tag_category_{:02}", i)).collect(),
        success: true,
    };
    assert_eq!(exp.parameters.len(), 100);
    assert_eq!(exp.tags.len(), 20);
    oxicode::encode_to_file(&exp, &path).expect("encode large Experiment to file");
    let decoded: Experiment =
        oxicode::decode_from_file(&path).expect("decode large Experiment from file");
    assert_eq!(exp, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 11: Empty parameters list
#[test]
fn test_adv10_11_empty_parameters_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_11.bin", std::process::id()));
    let exp = Experiment {
        id: 0,
        name: "empty_params".to_string(),
        parameters: vec![],
        tags: vec!["no-params".to_string()],
        success: false,
    };
    oxicode::encode_to_file(&exp, &path).expect("encode Experiment with empty parameters to file");
    let decoded: Experiment = oxicode::decode_from_file(&path)
        .expect("decode Experiment with empty parameters from file");
    assert_eq!(exp, decoded);
    assert!(decoded.parameters.is_empty());
    std::fs::remove_file(&path).ok();
}

// Test 12: Multiple values sequential write/read (encode_into_std_write / decode_from_std_read)
#[test]
fn test_adv10_12_sequential_encode_into_std_write_decode_from_std_read() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_12.bin", std::process::id()));
    let cfg = oxicode::config::standard();

    // Build values twice: once to write (moved), once to compare (kept)
    let make_exp1 = || Experiment {
        id: 1,
        name: "first".to_string(),
        parameters: vec![1.0],
        tags: vec!["a".to_string()],
        success: true,
    };
    let make_exp2 = || Experiment {
        id: 2,
        name: "second".to_string(),
        parameters: vec![2.0, 3.0],
        tags: vec!["b".to_string(), "c".to_string()],
        success: false,
    };
    let make_status = || ExperimentStatus::Running { progress_pct: 75 };

    let mut file = std::fs::File::create(&path).expect("create file for sequential write");
    let n1 = oxicode::encode_into_std_write(make_exp1(), &mut file, cfg)
        .expect("encode first Experiment via encode_into_std_write");
    let n2 = oxicode::encode_into_std_write(make_exp2(), &mut file, cfg)
        .expect("encode second Experiment via encode_into_std_write");
    let n3 = oxicode::encode_into_std_write(make_status(), &mut file, cfg)
        .expect("encode ExperimentStatus via encode_into_std_write");
    assert!(n1 > 0);
    assert!(n2 > 0);
    assert!(n3 > 0);
    drop(file);

    let raw = std::fs::read(&path).expect("read sequential file");
    let mut cursor = std::io::Cursor::new(raw);

    let decoded1: Experiment =
        oxicode::decode_from_std_read(&mut cursor, cfg).expect("decode first Experiment");
    let decoded2: Experiment =
        oxicode::decode_from_std_read(&mut cursor, cfg).expect("decode second Experiment");
    let decoded_status: ExperimentStatus =
        oxicode::decode_from_std_read(&mut cursor, cfg).expect("decode ExperimentStatus");

    assert_eq!(make_exp1(), decoded1);
    assert_eq!(make_exp2(), decoded2);
    assert_eq!(make_status(), decoded_status);
    std::fs::remove_file(&path).ok();
}

// Test 13: Empty Vec<f64> roundtrip
#[test]
fn test_adv10_13_empty_vec_f64_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_13.bin", std::process::id()));
    let val: Vec<f64> = vec![];
    oxicode::encode_to_file(&val, &path).expect("encode empty Vec<f64> to file");
    let decoded: Vec<f64> =
        oxicode::decode_from_file(&path).expect("decode empty Vec<f64> from file");
    assert_eq!(val, decoded);
    assert!(decoded.is_empty());
    std::fs::remove_file(&path).ok();
}

// Test 14: Option<Experiment> Some roundtrip
#[test]
fn test_adv10_14_option_experiment_some_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_14.bin", std::process::id()));
    let val: Option<Experiment> = Some(Experiment {
        id: 555,
        name: "option_some_experiment".to_string(),
        parameters: vec![1.1, 2.2, 3.3],
        tags: vec!["optional".to_string()],
        success: true,
    });
    oxicode::encode_to_file(&val, &path).expect("encode Option<Experiment> Some to file");
    let decoded: Option<Experiment> =
        oxicode::decode_from_file(&path).expect("decode Option<Experiment> Some from file");
    assert_eq!(val, decoded);
    assert!(decoded.is_some());
    std::fs::remove_file(&path).ok();
}

// Test 15: Option<Experiment> None roundtrip
#[test]
fn test_adv10_15_option_experiment_none_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_15.bin", std::process::id()));
    let val: Option<Experiment> = None;
    oxicode::encode_to_file(&val, &path).expect("encode Option<Experiment> None to file");
    let decoded: Option<Experiment> =
        oxicode::decode_from_file(&path).expect("decode Option<Experiment> None from file");
    assert_eq!(val, decoded);
    assert!(decoded.is_none());
    std::fs::remove_file(&path).ok();
}

// Test 16: Overwrite file with new value
#[test]
fn test_adv10_16_overwrite_file_with_new_value() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_16.bin", std::process::id()));
    let first = Experiment {
        id: 1,
        name: "first_write".to_string(),
        parameters: vec![1.0],
        tags: vec!["first".to_string()],
        success: false,
    };
    let second = Experiment {
        id: 2,
        name: "second_write_overwrites_first".to_string(),
        parameters: vec![2.0, 4.0, 8.0],
        tags: vec!["second".to_string(), "overwrite".to_string()],
        success: true,
    };
    oxicode::encode_to_file(&first, &path).expect("encode first Experiment to file");
    oxicode::encode_to_file(&second, &path).expect("encode second Experiment to file (overwrite)");
    let decoded: Experiment =
        oxicode::decode_from_file(&path).expect("decode overwritten Experiment from file");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 17: Non-existent file returns error
#[test]
fn test_adv10_17_nonexistent_file_returns_error() {
    let path = temp_dir().join(format!(
        "oxicode_adv10_{}_17_nonexistent_experiment_xyz.bin",
        std::process::id()
    ));
    std::fs::remove_file(&path).ok();
    let result = oxicode::decode_from_file::<Experiment>(&path);
    assert!(
        result.is_err(),
        "Expected error when decoding from non-existent file"
    );
}

// Test 18: bool roundtrip
#[test]
fn test_adv10_18_bool_roundtrip() {
    let path_true = temp_dir().join(format!("oxicode_adv10_{}_18t.bin", std::process::id()));
    let path_false = temp_dir().join(format!("oxicode_adv10_{}_18f.bin", std::process::id()));

    oxicode::encode_to_file(&true, &path_true).expect("encode true to file");
    oxicode::encode_to_file(&false, &path_false).expect("encode false to file");

    let decoded_true: bool = oxicode::decode_from_file(&path_true).expect("decode true from file");
    let decoded_false: bool =
        oxicode::decode_from_file(&path_false).expect("decode false from file");

    assert!(decoded_true, "decoded true should be true");
    assert!(!decoded_false, "decoded false should be false");

    std::fs::remove_file(&path_true).ok();
    std::fs::remove_file(&path_false).ok();
}

// Test 19: u64::MAX roundtrip
#[test]
fn test_adv10_19_u64_max_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_19.bin", std::process::id()));
    let val: u64 = u64::MAX;
    oxicode::encode_to_file(&val, &path).expect("encode u64::MAX to file");
    let decoded: u64 = oxicode::decode_from_file(&path).expect("decode u64::MAX from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 20: f64::PI roundtrip (bit-exact)
#[test]
fn test_adv10_20_f64_pi_bit_exact_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_20.bin", std::process::id()));
    let val: f64 = std::f64::consts::PI;
    oxicode::encode_to_file(&val, &path).expect("encode f64::PI to file");
    let decoded: f64 = oxicode::decode_from_file(&path).expect("decode f64::PI from file");
    assert_eq!(
        val.to_bits(),
        decoded.to_bits(),
        "f64 PI must be bit-exact after roundtrip"
    );
    std::fs::remove_file(&path).ok();
}

// Test 21: Nested struct with all ExperimentStatus variants in Vec
#[test]
fn test_adv10_21_all_experiment_status_variants_in_vec() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_21.bin", std::process::id()));
    let statuses: Vec<ExperimentStatus> = vec![
        ExperimentStatus::Pending,
        ExperimentStatus::Running { progress_pct: 0 },
        ExperimentStatus::Running { progress_pct: 50 },
        ExperimentStatus::Running { progress_pct: 100 },
        ExperimentStatus::Complete { result_id: 1 },
        ExperimentStatus::Complete {
            result_id: u64::MAX,
        },
        ExperimentStatus::Failed {
            error: "out of memory".to_string(),
        },
        ExperimentStatus::Failed {
            error: "timeout after 3600s".to_string(),
        },
    ];
    oxicode::encode_to_file(&statuses, &path)
        .expect("encode Vec<ExperimentStatus> all variants to file");
    let decoded: Vec<ExperimentStatus> = oxicode::decode_from_file(&path)
        .expect("decode Vec<ExperimentStatus> all variants from file");
    assert_eq!(statuses.len(), decoded.len());
    assert_eq!(statuses, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 22: File size matches encode_to_vec length
#[test]
fn test_adv10_22_file_size_matches_encode_to_vec_length() {
    let path = temp_dir().join(format!("oxicode_adv10_{}_22.bin", std::process::id()));
    let exp = Experiment {
        id: 7777,
        name: "size_verification_experiment".to_string(),
        parameters: vec![0.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0],
        tags: vec!["fibonacci".to_string(), "size-check".to_string()],
        success: true,
    };
    oxicode::encode_to_file(&exp, &path).expect("encode Experiment to file for size check");

    let metadata = std::fs::metadata(&path).expect("get file metadata for size check");
    let vec_bytes = oxicode::encode_to_vec(&exp).expect("encode Experiment to vec for size check");

    assert_eq!(
        metadata.len() as usize,
        vec_bytes.len(),
        "file size must equal encode_to_vec byte length"
    );
    std::fs::remove_file(&path).ok();
}
