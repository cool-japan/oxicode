//! Advanced file I/O integration tests for OxiCode — set 3.
//!
//! 22 top-level `#[test]` functions — no module wrapper, no unwrap().

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
use std::collections::HashMap;
use std::env::temp_dir;

// ── shared helper types ───────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllFieldStruct {
    a_u8: u8,
    b_i16: i16,
    c_u32: u32,
    d_i64: i64,
    e_f32: f32,
    f_f64: f64,
    g_bool: bool,
    h_string: String,
    i_opt: Option<u64>,
    j_vec: Vec<u32>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MultiVariantEnum {
    Unit,
    Newtype(i32),
    Tuple(u8, u8, u8),
    Struct { x: f32, y: f32 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LargePayloadStruct {
    id: u64,
    payload: Vec<u8>,
    label: String,
    flags: Vec<bool>,
}

// ── test 1: write/read HashMap<String, u32> to file ──────────────────────────

#[test]
fn test_fio3_hashmap_string_u32_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_1.bin", std::process::id()));
    let mut value: HashMap<String, u32> = HashMap::new();
    value.insert("alpha".to_string(), 1);
    value.insert("beta".to_string(), 2);
    value.insert("gamma".to_string(), 3);
    value.insert("delta".to_string(), 100);

    oxicode::encode_to_file(&value, &path).expect("encode HashMap<String,u32> to file");
    let decoded: HashMap<String, u32> =
        oxicode::decode_from_file(&path).expect("decode HashMap<String,u32> from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 2: write/read Vec<Vec<u8>> nested to file ───────────────────────────

#[test]
fn test_fio3_nested_vec_u8_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_2.bin", std::process::id()));
    let value: Vec<Vec<u8>> = vec![
        vec![1, 2, 3],
        vec![],
        vec![10, 20, 30, 40, 50],
        vec![255, 128, 0],
    ];

    oxicode::encode_to_file(&value, &path).expect("encode Vec<Vec<u8>> to file");
    let decoded: Vec<Vec<u8>> =
        oxicode::decode_from_file(&path).expect("decode Vec<Vec<u8>> from file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 3: write/read struct with all field types to file ───────────────────

#[test]
fn test_fio3_all_field_struct_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_3.bin", std::process::id()));
    let value = AllFieldStruct {
        a_u8: 200,
        b_i16: -30000,
        c_u32: 0xCAFE_BABE,
        d_i64: i64::MIN / 2,
        e_f32: std::f32::consts::PI,
        f_f64: std::f64::consts::E,
        g_bool: true,
        h_string: "all-fields-test".to_string(),
        i_opt: Some(0xDEAD_BEEF_u64),
        j_vec: vec![10, 20, 30, 40, 50],
    };

    oxicode::encode_to_file(&value, &path).expect("encode AllFieldStruct to file");
    let decoded: AllFieldStruct =
        oxicode::decode_from_file(&path).expect("decode AllFieldStruct from file");

    assert_eq!(value.a_u8, decoded.a_u8);
    assert_eq!(value.b_i16, decoded.b_i16);
    assert_eq!(value.c_u32, decoded.c_u32);
    assert_eq!(value.d_i64, decoded.d_i64);
    assert_eq!(value.e_f32.to_bits(), decoded.e_f32.to_bits());
    assert_eq!(value.f_f64.to_bits(), decoded.f_f64.to_bits());
    assert_eq!(value.g_bool, decoded.g_bool);
    assert_eq!(value.h_string, decoded.h_string);
    assert_eq!(value.i_opt, decoded.i_opt);
    assert_eq!(value.j_vec, decoded.j_vec);
    std::fs::remove_file(&path).ok();
}

// ── test 4: write/read enum with all variants to file ────────────────────────

#[test]
fn test_fio3_enum_all_variants_roundtrip() {
    let pid = std::process::id();
    let base = format!("oxicode_fio3_{}_4", pid);

    let variants: Vec<(MultiVariantEnum, &str)> = vec![
        (MultiVariantEnum::Unit, "unit"),
        (MultiVariantEnum::Newtype(-42), "newtype"),
        (MultiVariantEnum::Tuple(10, 20, 30), "tuple"),
        (MultiVariantEnum::Struct { x: 1.5, y: -2.5 }, "struct"),
    ];

    for (variant, suffix) in variants {
        let path = temp_dir().join(format!("{}_{}.bin", base, suffix));
        oxicode::encode_to_file(&variant, &path).expect("encode MultiVariantEnum variant to file");
        let decoded: MultiVariantEnum =
            oxicode::decode_from_file(&path).expect("decode MultiVariantEnum variant from file");
        assert_eq!(variant, decoded);
        std::fs::remove_file(&path).ok();
    }
}

// ── test 5: write/read None Option<String> to file ───────────────────────────

#[test]
fn test_fio3_option_none_string_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_5.bin", std::process::id()));
    let value: Option<String> = None;

    oxicode::encode_to_file(&value, &path).expect("encode Option<String> None to file");
    let decoded: Option<String> =
        oxicode::decode_from_file(&path).expect("decode Option<String> None from file");

    assert_eq!(value, decoded);
    assert!(decoded.is_none());
    std::fs::remove_file(&path).ok();
}

// ── test 6: write 3 different values to 3 files, read all 3 back ─────────────

#[test]
fn test_fio3_three_files_independent_roundtrip() {
    let pid = std::process::id();
    let path_a = temp_dir().join(format!("oxicode_fio3_{}_6a.bin", pid));
    let path_b = temp_dir().join(format!("oxicode_fio3_{}_6b.bin", pid));
    let path_c = temp_dir().join(format!("oxicode_fio3_{}_6c.bin", pid));

    let val_a: u32 = 111_111;
    let val_b: String = "second-file-value".to_string();
    let val_c: Vec<i32> = vec![-1, 0, 1, 2, 3];

    oxicode::encode_to_file(&val_a, &path_a).expect("encode val_a to file A");
    oxicode::encode_to_file(&val_b, &path_b).expect("encode val_b to file B");
    oxicode::encode_to_file(&val_c, &path_c).expect("encode val_c to file C");

    let read_a: u32 = oxicode::decode_from_file(&path_a).expect("decode val_a from file A");
    let read_b: String = oxicode::decode_from_file(&path_b).expect("decode val_b from file B");
    let read_c: Vec<i32> = oxicode::decode_from_file(&path_c).expect("decode val_c from file C");

    assert_eq!(val_a, read_a);
    assert_eq!(val_b, read_b);
    assert_eq!(val_c, read_c);

    std::fs::remove_file(&path_a).ok();
    std::fs::remove_file(&path_b).ok();
    std::fs::remove_file(&path_c).ok();
}

// ── test 7: file written with standard config, read with same config ──────────

#[test]
fn test_fio3_standard_config_write_read() {
    use oxicode::config;

    let path = temp_dir().join(format!("oxicode_fio3_{}_7.bin", std::process::id()));
    let cfg = config::standard();
    let value: Vec<u64> = vec![0, 1, u64::MAX / 2, u64::MAX];

    oxicode::encode_to_file_with_config(&value, &path, cfg)
        .expect("encode Vec<u64> with standard config");
    let decoded: Vec<u64> = oxicode::decode_from_file_with_config(&path, cfg)
        .expect("decode Vec<u64> with standard config");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 8: file written with fixed-int config, read with fixed-int config ────

#[test]
fn test_fio3_fixed_int_config_write_read() {
    use oxicode::config;

    let path = temp_dir().join(format!("oxicode_fio3_{}_8.bin", std::process::id()));
    let cfg = config::standard().with_fixed_int_encoding();
    let value: i32 = -0x1234_5678_i32;

    oxicode::encode_to_file_with_config(&value, &path, cfg)
        .expect("encode i32 with fixed-int config");
    let decoded: i32 = oxicode::decode_from_file_with_config(&path, cfg)
        .expect("decode i32 with fixed-int config");

    assert_eq!(value, decoded);

    // With fixed-int encoding, i32 takes exactly 4 bytes
    let raw = std::fs::read(&path).expect("read raw fixed-int file");
    assert_eq!(raw.len(), 4, "fixed-int i32 must occupy exactly 4 bytes");

    std::fs::remove_file(&path).ok();
}

// ── test 9: file written with big-endian config, verify byte order ────────────

#[test]
fn test_fio3_big_endian_config_verify_bytes() {
    use oxicode::config;

    let path = temp_dir().join(format!("oxicode_fio3_{}_9.bin", std::process::id()));
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value: u32 = 0xAABB_CCDD_u32;

    oxicode::encode_to_file_with_config(&value, &path, cfg)
        .expect("encode u32 with big-endian config");
    let decoded: u32 = oxicode::decode_from_file_with_config(&path, cfg)
        .expect("decode u32 with big-endian config");

    assert_eq!(value, decoded);

    let raw = std::fs::read(&path).expect("read raw big-endian file");
    assert_eq!(
        raw,
        [0xAA, 0xBB, 0xCC, 0xDD],
        "big-endian bytes must be in network order"
    );

    std::fs::remove_file(&path).ok();
}

// ── test 10: file size matches encode_to_vec length ──────────────────────────

#[test]
fn test_fio3_file_size_matches_encode_to_vec() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_10.bin", std::process::id()));
    let value = AllFieldStruct {
        a_u8: 7,
        b_i16: 1000,
        c_u32: 42,
        d_i64: -999,
        e_f32: 0.0,
        f_f64: 1.0,
        g_bool: false,
        h_string: "size-check-struct".to_string(),
        i_opt: None,
        j_vec: vec![1, 2, 3],
    };

    oxicode::encode_to_file(&value, &path).expect("encode AllFieldStruct for size check");

    let file_bytes = std::fs::read(&path).expect("read file bytes");
    let vec_bytes = oxicode::encode_to_vec(&value).expect("encode_to_vec for size check");

    assert_eq!(
        file_bytes.len(),
        vec_bytes.len(),
        "file byte count must equal encode_to_vec byte count"
    );
    assert_eq!(
        file_bytes, vec_bytes,
        "file bytes must be identical to encode_to_vec bytes"
    );

    std::fs::remove_file(&path).ok();
}

// ── test 11: overwrite file — second write replaces first content ─────────────

#[test]
fn test_fio3_overwrite_second_write_wins() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_11.bin", std::process::id()));
    let first: String = "first-content".to_string();
    let second: String = "second-content-longer-replacement".to_string();

    oxicode::encode_to_file(&first, &path).expect("first write to file");
    let size_after_first = std::fs::metadata(&path)
        .expect("stat file after first write")
        .len();

    oxicode::encode_to_file(&second, &path).expect("second (overwrite) write to file");
    let size_after_second = std::fs::metadata(&path)
        .expect("stat file after second write")
        .len();

    // Second value is longer, so file should be larger
    assert!(
        size_after_second > size_after_first,
        "overwritten file must reflect longer second value"
    );

    let decoded: String = oxicode::decode_from_file(&path).expect("decode after overwrite");
    assert_eq!(
        second, decoded,
        "decoded value must equal second written value"
    );

    std::fs::remove_file(&path).ok();
}

// ── test 12: write Vec<u8> of 1000 bytes, read back ──────────────────────────

#[test]
fn test_fio3_vec_u8_1000_bytes_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_12.bin", std::process::id()));
    let value: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    assert_eq!(value.len(), 1000);

    oxicode::encode_to_file(&value, &path).expect("encode 1000-byte Vec<u8> to file");
    let decoded: Vec<u8> =
        oxicode::decode_from_file(&path).expect("decode 1000-byte Vec<u8> from file");

    assert_eq!(value, decoded);
    assert_eq!(decoded.len(), 1000);
    std::fs::remove_file(&path).ok();
}

// ── test 13: write large struct, verify file size is non-trivial ──────────────

#[test]
fn test_fio3_large_struct_file_size() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_13.bin", std::process::id()));
    let value = LargePayloadStruct {
        id: 0xFEED_FACE_CAFE_BABE_u64,
        payload: (0u8..=255).cycle().take(5000).collect(),
        label: "L".repeat(500),
        flags: std::iter::repeat([true, false, true, true, false].as_slice())
            .take(200)
            .flat_map(|s| s.iter().copied())
            .collect(),
    };

    oxicode::encode_to_file(&value, &path).expect("encode LargePayloadStruct to file");

    let file_size = std::fs::metadata(&path)
        .expect("stat large struct file")
        .len();

    // Payload alone is 5000 bytes; label is 500 bytes; file must be well over 5000 bytes
    assert!(
        file_size > 5000,
        "large struct file must exceed 5000 bytes, got {file_size}"
    );

    let decoded: LargePayloadStruct =
        oxicode::decode_from_file(&path).expect("decode LargePayloadStruct from file");
    assert_eq!(value.id, decoded.id);
    assert_eq!(value.payload.len(), decoded.payload.len());
    assert_eq!(value.label.len(), decoded.label.len());
    assert_eq!(value.flags.len(), decoded.flags.len());

    std::fs::remove_file(&path).ok();
}

// ── test 14: write then read same file twice (idempotent reads) ───────────────

#[test]
fn test_fio3_read_same_file_twice() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_14.bin", std::process::id()));
    let value: Vec<i64> = vec![i64::MIN, -1, 0, 1, i64::MAX];

    oxicode::encode_to_file(&value, &path).expect("encode Vec<i64> to file");

    let first_read: Vec<i64> = oxicode::decode_from_file(&path).expect("first decode of Vec<i64>");
    let second_read: Vec<i64> =
        oxicode::decode_from_file(&path).expect("second decode of Vec<i64>");

    assert_eq!(value, first_read);
    assert_eq!(value, second_read);
    assert_eq!(first_read, second_read);

    std::fs::remove_file(&path).ok();
}

// ── test 15: temp file cleanup — process ID in filename avoids conflicts ──────

#[test]
fn test_fio3_temp_file_pid_in_name() {
    let pid = std::process::id();
    let path = temp_dir().join(format!("oxicode_fio3_{}_15.bin", pid));

    // Ensure path contains the process ID
    let name = path
        .file_name()
        .expect("path has a filename")
        .to_string_lossy();
    assert!(
        name.contains(&pid.to_string()),
        "temp filename must contain process ID"
    );

    let value: u64 = pid as u64;
    oxicode::encode_to_file(&value, &path).expect("encode pid value to temp file");
    let decoded: u64 = oxicode::decode_from_file(&path).expect("decode pid value from temp file");

    assert_eq!(value, decoded);
    std::fs::remove_file(&path).expect("cleanup temp file");

    // Verify file no longer exists after removal
    assert!(
        !path.exists(),
        "temp file must not exist after explicit cleanup"
    );
}

// ── test 16: write u128::MAX to file, read back ───────────────────────────────

#[test]
fn test_fio3_u128_max_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_16.bin", std::process::id()));
    let value: u128 = u128::MAX;

    oxicode::encode_to_file(&value, &path).expect("encode u128::MAX to file");
    let decoded: u128 = oxicode::decode_from_file(&path).expect("decode u128::MAX from file");

    assert_eq!(value, decoded, "u128::MAX must round-trip exactly");
    std::fs::remove_file(&path).ok();
}

// ── test 17: write i128::MIN to file, read back ───────────────────────────────

#[test]
fn test_fio3_i128_min_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_17.bin", std::process::id()));
    let value: i128 = i128::MIN;

    oxicode::encode_to_file(&value, &path).expect("encode i128::MIN to file");
    let decoded: i128 = oxicode::decode_from_file(&path).expect("decode i128::MIN from file");

    assert_eq!(value, decoded, "i128::MIN must round-trip exactly");
    std::fs::remove_file(&path).ok();
}

// ── test 18: write empty Vec<String> to file, read back ──────────────────────

#[test]
fn test_fio3_empty_vec_string_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_18.bin", std::process::id()));
    let value: Vec<String> = Vec::new();

    oxicode::encode_to_file(&value, &path).expect("encode empty Vec<String> to file");
    let decoded: Vec<String> =
        oxicode::decode_from_file(&path).expect("decode empty Vec<String> from file");

    assert_eq!(value, decoded);
    assert!(decoded.is_empty(), "decoded Vec<String> must be empty");
    std::fs::remove_file(&path).ok();
}

// ── test 19: write f64::NAN to file, read back (NaN bit pattern preserved) ───

#[test]
fn test_fio3_f64_nan_bit_pattern_preserved() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_19.bin", std::process::id()));
    let value: f64 = f64::NAN;

    oxicode::encode_to_file(&value, &path).expect("encode f64::NAN to file");
    let decoded: f64 = oxicode::decode_from_file(&path).expect("decode f64::NAN from file");

    // NaN != NaN by IEEE 754, so compare bit patterns directly
    assert_eq!(
        value.to_bits(),
        decoded.to_bits(),
        "f64::NAN bit pattern must be preserved across file round-trip"
    );
    assert!(decoded.is_nan(), "decoded f64 must be NaN");
    std::fs::remove_file(&path).ok();
}

// ── test 20: sequential encode via encode_into_std_write ─────────────────────

#[test]
fn test_fio3_sequential_encode_into_std_write() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_20.bin", std::process::id()));
    let cfg = oxicode::config::standard();

    let values: Vec<u32> = vec![10, 20, 30, 40, 50];

    {
        let mut file = std::fs::File::create(&path).expect("create file for sequential encode");
        for &v in &values {
            oxicode::encode_into_std_write(v, &mut file, cfg)
                .expect("encode_into_std_write sequential value");
        }
    }

    // Read back each value sequentially
    let file = std::fs::File::open(&path).expect("open file for sequential decode");
    let mut reader = std::io::BufReader::new(file);
    let mut decoded_values: Vec<u32> = Vec::with_capacity(values.len());
    for _ in 0..values.len() {
        let v: u32 = oxicode::decode_from_std_read(&mut reader, cfg)
            .expect("decode_from_std_read sequential value");
        decoded_values.push(v);
    }

    assert_eq!(
        values, decoded_values,
        "sequential encoded values must match"
    );
    std::fs::remove_file(&path).ok();
}

// ── test 21: sequential decode via decode_from_std_read ──────────────────────

#[test]
fn test_fio3_sequential_decode_from_std_read() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_21.bin", std::process::id()));
    let cfg = oxicode::config::standard();

    // Write heterogeneous values sequentially: u8, u16, u32, u64
    {
        let mut file = std::fs::File::create(&path).expect("create file for heterogeneous encode");
        oxicode::encode_into_std_write(0xAA_u8, &mut file, cfg).expect("encode u8 sequentially");
        oxicode::encode_into_std_write(0xBBCC_u16, &mut file, cfg)
            .expect("encode u16 sequentially");
        oxicode::encode_into_std_write(0xDDEE_FF00_u32, &mut file, cfg)
            .expect("encode u32 sequentially");
        oxicode::encode_into_std_write(0x1122_3344_5566_7788_u64, &mut file, cfg)
            .expect("encode u64 sequentially");
    }

    let file = std::fs::File::open(&path).expect("open file for heterogeneous decode");
    let mut reader = std::io::BufReader::new(file);

    let r_u8: u8 =
        oxicode::decode_from_std_read(&mut reader, cfg).expect("decode u8 from sequential stream");
    let r_u16: u16 =
        oxicode::decode_from_std_read(&mut reader, cfg).expect("decode u16 from sequential stream");
    let r_u32: u32 =
        oxicode::decode_from_std_read(&mut reader, cfg).expect("decode u32 from sequential stream");
    let r_u64: u64 =
        oxicode::decode_from_std_read(&mut reader, cfg).expect("decode u64 from sequential stream");

    assert_eq!(r_u8, 0xAA_u8);
    assert_eq!(r_u16, 0xBBCC_u16);
    assert_eq!(r_u32, 0xDDEE_FF00_u32);
    assert_eq!(r_u64, 0x1122_3344_5566_7788_u64);

    std::fs::remove_file(&path).ok();
}

// ── test 22: write struct, delete file, attempt read returns error ────────────

#[test]
fn test_fio3_read_after_delete_returns_error() {
    let path = temp_dir().join(format!("oxicode_fio3_{}_22.bin", std::process::id()));
    let value = AllFieldStruct {
        a_u8: 1,
        b_i16: 2,
        c_u32: 3,
        d_i64: 4,
        e_f32: 5.0,
        f_f64: 6.0,
        g_bool: false,
        h_string: "delete-me".to_string(),
        i_opt: None,
        j_vec: vec![],
    };

    oxicode::encode_to_file(&value, &path).expect("encode AllFieldStruct before deletion");
    assert!(path.exists(), "file must exist before deletion");

    std::fs::remove_file(&path).expect("delete file");
    assert!(!path.exists(), "file must not exist after deletion");

    let result = oxicode::decode_from_file::<AllFieldStruct>(&path);
    assert!(
        result.is_err(),
        "decode_from_file must return Err after file has been deleted"
    );
}
