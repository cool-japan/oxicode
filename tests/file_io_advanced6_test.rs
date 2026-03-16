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
use oxicode::{config, Decode, Encode};
use std::env::temp_dir;

#[derive(Debug, PartialEq, Encode, Decode)]
struct Record {
    id: u64,
    name: String,
    values: Vec<f64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Batch {
    records: Vec<Record>,
    priority: Priority,
    timestamp: u64,
}

// Test 1: Record to file roundtrip
#[test]
fn test_fio6_record_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_01.bin", std::process::id()));
    let original = Record {
        id: 1001,
        name: "alpha".to_string(),
        values: vec![1.0, 2.5, 3.14],
    };
    oxicode::encode_to_file(&original, &path).expect("encode_to_file failed");
    let decoded: Record = oxicode::decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 2: Vec<Record> to file roundtrip
#[test]
fn test_fio6_vec_record_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_02.bin", std::process::id()));
    let original: Vec<Record> = vec![
        Record {
            id: 1,
            name: "first".to_string(),
            values: vec![0.1, 0.2],
        },
        Record {
            id: 2,
            name: "second".to_string(),
            values: vec![1.0, 2.0, 3.0],
        },
        Record {
            id: 3,
            name: "third".to_string(),
            values: vec![],
        },
    ];
    oxicode::encode_to_file(&original, &path).expect("encode_to_file Vec<Record> failed");
    let decoded: Vec<Record> =
        oxicode::decode_from_file(&path).expect("decode_from_file Vec<Record> failed");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 3: Priority::Low to file roundtrip
#[test]
fn test_fio6_priority_low_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_03.bin", std::process::id()));
    let original = Priority::Low;
    oxicode::encode_to_file(&original, &path).expect("encode Priority::Low failed");
    let decoded: Priority = oxicode::decode_from_file(&path).expect("decode Priority::Low failed");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 4: Priority::Critical to file roundtrip
#[test]
fn test_fio6_priority_critical_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_04.bin", std::process::id()));
    let original = Priority::Critical;
    oxicode::encode_to_file(&original, &path).expect("encode Priority::Critical failed");
    let decoded: Priority =
        oxicode::decode_from_file(&path).expect("decode Priority::Critical failed");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 5: Batch to file roundtrip
#[test]
fn test_fio6_batch_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_05.bin", std::process::id()));
    let original = Batch {
        records: vec![
            Record {
                id: 10,
                name: "r1".to_string(),
                values: vec![9.9, 8.8],
            },
            Record {
                id: 20,
                name: "r2".to_string(),
                values: vec![7.7],
            },
        ],
        priority: Priority::High,
        timestamp: 1_700_000_000,
    };
    oxicode::encode_to_file(&original, &path).expect("encode Batch failed");
    let decoded: Batch = oxicode::decode_from_file(&path).expect("decode Batch failed");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 6: u64::MAX to file roundtrip
#[test]
fn test_fio6_u64_max_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_06.bin", std::process::id()));
    let original: u64 = u64::MAX;
    oxicode::encode_to_file(&original, &path).expect("encode u64::MAX failed");
    let decoded: u64 = oxicode::decode_from_file(&path).expect("decode u64::MAX failed");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 7: i64::MIN to file roundtrip
#[test]
fn test_fio6_i64_min_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_07.bin", std::process::id()));
    let original: i64 = i64::MIN;
    oxicode::encode_to_file(&original, &path).expect("encode i64::MIN failed");
    let decoded: i64 = oxicode::decode_from_file(&path).expect("decode i64::MIN failed");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 8: f64::NAN — encode then decode, check bit identity via to_bits()
#[test]
fn test_fio6_f64_nan_bit_identity() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_08.bin", std::process::id()));
    let original: f64 = f64::NAN;
    oxicode::encode_to_file(&original, &path).expect("encode f64::NAN failed");
    let decoded: f64 = oxicode::decode_from_file(&path).expect("decode f64::NAN failed");
    assert_eq!(
        original.to_bits(),
        decoded.to_bits(),
        "NaN bit representation must be preserved"
    );
    std::fs::remove_file(&path).ok();
}

// Test 9: Empty Vec<Record> to file
#[test]
fn test_fio6_empty_vec_record() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_09.bin", std::process::id()));
    let original: Vec<Record> = Vec::new();
    oxicode::encode_to_file(&original, &path).expect("encode empty Vec<Record> failed");
    let decoded: Vec<Record> =
        oxicode::decode_from_file(&path).expect("decode empty Vec<Record> failed");
    assert!(decoded.is_empty(), "decoded empty vec must be empty");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 10: Vec<String> 20 strings to file
#[test]
fn test_fio6_vec_20_strings_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_10.bin", std::process::id()));
    let original: Vec<String> = (0..20).map(|i| format!("string_{:03}", i)).collect();
    oxicode::encode_to_file(&original, &path).expect("encode Vec<String> failed");
    let decoded: Vec<String> = oxicode::decode_from_file(&path).expect("decode Vec<String> failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 20);
    std::fs::remove_file(&path).ok();
}

// Test 11: bool true/false to file
#[test]
fn test_fio6_bool_roundtrip() {
    let path_true = temp_dir().join(format!("oxicode_fio6_{}_test_11a.bin", std::process::id()));
    let path_false = temp_dir().join(format!("oxicode_fio6_{}_test_11b.bin", std::process::id()));
    oxicode::encode_to_file(&true, &path_true).expect("encode true failed");
    oxicode::encode_to_file(&false, &path_false).expect("encode false failed");
    let decoded_true: bool = oxicode::decode_from_file(&path_true).expect("decode true failed");
    let decoded_false: bool = oxicode::decode_from_file(&path_false).expect("decode false failed");
    assert!(decoded_true, "decoded true must be true");
    assert!(!decoded_false, "decoded false must be false");
    std::fs::remove_file(&path_true).ok();
    std::fs::remove_file(&path_false).ok();
}

// Test 12: Option<Record> Some to file
#[test]
fn test_fio6_option_record_some_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_12.bin", std::process::id()));
    let original: Option<Record> = Some(Record {
        id: 42,
        name: "some_record".to_string(),
        values: vec![3.14, 2.71],
    });
    oxicode::encode_to_file(&original, &path).expect("encode Option<Record> Some failed");
    let decoded: Option<Record> =
        oxicode::decode_from_file(&path).expect("decode Option<Record> Some failed");
    assert_eq!(original, decoded);
    assert!(decoded.is_some(), "decoded must be Some");
    std::fs::remove_file(&path).ok();
}

// Test 13: Option<Record> None to file
#[test]
fn test_fio6_option_record_none_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_13.bin", std::process::id()));
    let original: Option<Record> = None;
    oxicode::encode_to_file(&original, &path).expect("encode Option<Record> None failed");
    let decoded: Option<Record> =
        oxicode::decode_from_file(&path).expect("decode Option<Record> None failed");
    assert_eq!(original, decoded);
    assert!(decoded.is_none(), "decoded must be None");
    std::fs::remove_file(&path).ok();
}

// Test 14: Fixed int config with u32 to file (verify 4 bytes)
#[test]
fn test_fio6_fixed_int_config_u32_file_size() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_14.bin", std::process::id()));
    let original: u32 = 12345;
    let cfg = config::standard().with_fixed_int_encoding();
    oxicode::encode_to_file_with_config(&original, &path, cfg)
        .expect("encode u32 with fixed int config failed");
    let file_size = std::fs::metadata(&path).expect("metadata failed").len();
    assert_eq!(
        file_size, 4,
        "u32 with fixed int encoding must be exactly 4 bytes"
    );
    let decoded: u32 =
        oxicode::decode_from_file_with_config(&path, cfg).expect("decode u32 fixed int failed");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 15: Big endian config to file
#[test]
fn test_fio6_big_endian_config_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_15.bin", std::process::id()));
    let original = Record {
        id: 999,
        name: "big_endian".to_string(),
        values: vec![1.1, 2.2, 3.3],
    };
    let cfg = config::standard().with_big_endian();
    oxicode::encode_to_file_with_config(&original, &path, cfg)
        .expect("encode Record big endian failed");
    let decoded: Record =
        oxicode::decode_from_file_with_config(&path, cfg).expect("decode Record big endian failed");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 16: Sequential encode two Records then decode both
#[test]
fn test_fio6_sequential_encode_two_records_decode_both() {
    let path1 = temp_dir().join(format!("oxicode_fio6_{}_test_16a.bin", std::process::id()));
    let path2 = temp_dir().join(format!("oxicode_fio6_{}_test_16b.bin", std::process::id()));
    let rec1 = Record {
        id: 101,
        name: "rec_one".to_string(),
        values: vec![0.5, 1.5],
    };
    let rec2 = Record {
        id: 202,
        name: "rec_two".to_string(),
        values: vec![2.5, 3.5, 4.5],
    };
    oxicode::encode_to_file(&rec1, &path1).expect("encode rec1 failed");
    oxicode::encode_to_file(&rec2, &path2).expect("encode rec2 failed");
    let decoded1: Record = oxicode::decode_from_file(&path1).expect("decode rec1 failed");
    let decoded2: Record = oxicode::decode_from_file(&path2).expect("decode rec2 failed");
    assert_eq!(rec1, decoded1);
    assert_eq!(rec2, decoded2);
    std::fs::remove_file(&path1).ok();
    std::fs::remove_file(&path2).ok();
}

// Test 17: File size matches encode_to_vec length
#[test]
fn test_fio6_file_size_matches_encode_to_vec_length() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_17.bin", std::process::id()));
    let original = Batch {
        records: vec![Record {
            id: 77,
            name: "size_check".to_string(),
            values: vec![1.0, 2.0],
        }],
        priority: Priority::Normal,
        timestamp: 12345678,
    };
    let vec_bytes = oxicode::encode_to_vec(&original).expect("encode_to_vec for size check failed");
    oxicode::encode_to_file(&original, &path).expect("encode_to_file for size check failed");
    let file_size = std::fs::metadata(&path)
        .expect("metadata for size check failed")
        .len();
    assert_eq!(
        file_size as usize,
        vec_bytes.len(),
        "file size must equal encode_to_vec length"
    );
    std::fs::remove_file(&path).ok();
}

// Test 18: Overwrite: write first value, then second, decode yields second
#[test]
fn test_fio6_overwrite_yields_second_value() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_18.bin", std::process::id()));
    let first = Record {
        id: 1,
        name: "first_value".to_string(),
        values: vec![1.0],
    };
    let second = Record {
        id: 2,
        name: "second_value".to_string(),
        values: vec![2.0, 3.0],
    };
    oxicode::encode_to_file(&first, &path).expect("encode first value failed");
    oxicode::encode_to_file(&second, &path).expect("encode second value (overwrite) failed");
    let decoded: Record = oxicode::decode_from_file(&path).expect("decode after overwrite failed");
    assert_eq!(
        second, decoded,
        "decoded must be the second (overwriting) value"
    );
    std::fs::remove_file(&path).ok();
}

// Test 19: Decode from non-existent file returns error
#[test]
fn test_fio6_decode_nonexistent_file_returns_error() {
    let path = temp_dir().join(format!(
        "oxicode_fio6_{}_test_19_nonexistent_should_not_exist.bin",
        std::process::id()
    ));
    // Ensure it does not exist
    std::fs::remove_file(&path).ok();
    let result = oxicode::decode_from_file::<Record>(&path);
    assert!(
        result.is_err(),
        "decoding from non-existent file must return an error"
    );
}

// Test 20: Large Batch (100 records) to file roundtrip
#[test]
fn test_fio6_large_batch_100_records_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_20.bin", std::process::id()));
    let records: Vec<Record> = (0u64..100)
        .map(|i| Record {
            id: i,
            name: format!("record_{:04}", i),
            values: (0..10).map(|j| i as f64 * 10.0 + j as f64).collect(),
        })
        .collect();
    let original = Batch {
        records,
        priority: Priority::Critical,
        timestamp: 9_999_999_999,
    };
    oxicode::encode_to_file(&original, &path).expect("encode large Batch failed");
    let decoded: Batch = oxicode::decode_from_file(&path).expect("decode large Batch failed");
    assert_eq!(original.records.len(), decoded.records.len());
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 21: Vec<u8> 10000 bytes to file roundtrip
#[test]
fn test_fio6_vec_u8_10000_bytes_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_21.bin", std::process::id()));
    let original: Vec<u8> = (0u8..=255).cycle().take(10_000).collect();
    oxicode::encode_to_file(&original, &path).expect("encode Vec<u8> 10000 bytes failed");
    let decoded: Vec<u8> =
        oxicode::decode_from_file(&path).expect("decode Vec<u8> 10000 bytes failed");
    assert_eq!(original.len(), decoded.len());
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// Test 22: Vec<Priority> all variants to file roundtrip
#[test]
fn test_fio6_vec_priority_all_variants_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio6_{}_test_22.bin", std::process::id()));
    let original: Vec<Priority> = vec![
        Priority::Low,
        Priority::Normal,
        Priority::High,
        Priority::Critical,
        Priority::Normal,
        Priority::Low,
        Priority::Critical,
        Priority::High,
    ];
    oxicode::encode_to_file(&original, &path).expect("encode Vec<Priority> failed");
    let decoded: Vec<Priority> =
        oxicode::decode_from_file(&path).expect("decode Vec<Priority> failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 8);
    std::fs::remove_file(&path).ok();
}
