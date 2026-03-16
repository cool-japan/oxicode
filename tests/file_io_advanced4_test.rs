//! Advanced file I/O integration tests for OxiCode — set 4.
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
use oxicode::{
    config, decode_from_file, decode_from_file_with_config, decode_from_std_read,
    encode_into_std_write, encode_to_file, encode_to_file_with_config, Decode, Encode,
};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env::temp_dir;

// ── shared helper type ─────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Event4 {
    id: u32,
    name: String,
    timestamp: u64,
}

// ── test 1: Event4 struct to file and back ────────────────────────────────────

#[test]
fn test_fio4_event4_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_1.bin", std::process::id()));
    let value = Event4 {
        id: 42,
        name: "login".to_string(),
        timestamp: 1_700_000_000,
    };

    encode_to_file(&value, &path).expect("encode Event4 to file");
    let decoded: Event4 = decode_from_file(&path).expect("decode Event4 from file");

    assert_eq!(value.id, decoded.id);
    assert_eq!(value.name, decoded.name);
    assert_eq!(value.timestamp, decoded.timestamp);
    std::fs::remove_file(&path).ok();
}

// ── test 2: Vec<Event4> to file and back ──────────────────────────────────────

#[test]
fn test_fio4_vec_event4_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_2.bin", std::process::id()));
    let value: Vec<Event4> = vec![
        Event4 {
            id: 1,
            name: "start".to_string(),
            timestamp: 1_000,
        },
        Event4 {
            id: 2,
            name: "stop".to_string(),
            timestamp: 2_000,
        },
        Event4 {
            id: 3,
            name: "restart".to_string(),
            timestamp: 3_000,
        },
    ];

    encode_to_file(&value, &path).expect("encode Vec<Event4> to file");
    let decoded: Vec<Event4> = decode_from_file(&path).expect("decode Vec<Event4> from file");

    assert_eq!(value.len(), decoded.len());
    for (orig, dec) in value.iter().zip(decoded.iter()) {
        assert_eq!(orig.id, dec.id);
        assert_eq!(orig.name, dec.name);
        assert_eq!(orig.timestamp, dec.timestamp);
    }
    std::fs::remove_file(&path).ok();
}

// ── test 3: HashMap<String, Event4> to file and back ─────────────────────────

#[test]
fn test_fio4_hashmap_string_event4_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_3.bin", std::process::id()));
    let mut value: HashMap<String, Event4> = HashMap::new();
    value.insert(
        "alpha".to_string(),
        Event4 {
            id: 10,
            name: "alpha_event".to_string(),
            timestamp: 111,
        },
    );
    value.insert(
        "beta".to_string(),
        Event4 {
            id: 20,
            name: "beta_event".to_string(),
            timestamp: 222,
        },
    );
    value.insert(
        "gamma".to_string(),
        Event4 {
            id: 30,
            name: "gamma_event".to_string(),
            timestamp: 333,
        },
    );

    encode_to_file(&value, &path).expect("encode HashMap<String, Event4> to file");
    let decoded: HashMap<String, Event4> =
        decode_from_file(&path).expect("decode HashMap<String, Event4> from file");

    assert_eq!(value.len(), decoded.len());
    for (k, v) in &value {
        let d = decoded.get(k).expect("key must be present in decoded map");
        assert_eq!(v.id, d.id);
        assert_eq!(v.name, d.name);
        assert_eq!(v.timestamp, d.timestamp);
    }
    std::fs::remove_file(&path).ok();
}

// ── test 4: Multiple types sequentially in same file ──────────────────────────

#[test]
fn test_fio4_multiple_types_sequential_write_read() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_4.bin", std::process::id()));
    let cfg = config::standard();

    let val_u32: u32 = 0xDEAD_BEEF;
    let val_str: String = "sequential-multi-type".to_string();
    let val_event = Event4 {
        id: 99,
        name: "seq_event".to_string(),
        timestamp: 9_999_999,
    };

    {
        let mut file = std::fs::File::create(&path).expect("create file for multi-type sequential");
        encode_into_std_write(val_u32, &mut file, cfg).expect("encode u32 into std_write");
        encode_into_std_write(val_str.clone(), &mut file, cfg)
            .expect("encode String into std_write");
        encode_into_std_write(val_event.id, &mut file, cfg)
            .expect("encode Event4.id into std_write");
        encode_into_std_write(val_event.name.clone(), &mut file, cfg)
            .expect("encode Event4.name into std_write");
        encode_into_std_write(val_event.timestamp, &mut file, cfg)
            .expect("encode Event4.timestamp into std_write");
    }

    let file = std::fs::File::open(&path).expect("open file for multi-type sequential decode");
    let mut reader = std::io::BufReader::new(file);

    let r_u32: u32 = decode_from_std_read(&mut reader, cfg).expect("decode u32 from std_read");
    let r_str: String =
        decode_from_std_read(&mut reader, cfg).expect("decode String from std_read");
    let r_id: u32 = decode_from_std_read(&mut reader, cfg).expect("decode Event4.id from std_read");
    let r_name: String =
        decode_from_std_read(&mut reader, cfg).expect("decode Event4.name from std_read");
    let r_ts: u64 =
        decode_from_std_read(&mut reader, cfg).expect("decode Event4.timestamp from std_read");

    assert_eq!(r_u32, val_u32);
    assert_eq!(r_str, val_str);
    assert_eq!(r_id, val_event.id);
    assert_eq!(r_name, val_event.name);
    assert_eq!(r_ts, val_event.timestamp);
    std::fs::remove_file(&path).ok();
}

// ── test 5: File size matches encode_to_vec length for Vec<u8> of 100 bytes ───

#[test]
fn test_fio4_file_size_matches_vec_encoding_100_bytes() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_5.bin", std::process::id()));
    let value: Vec<u8> = (0u8..100).collect();
    assert_eq!(value.len(), 100);

    encode_to_file(&value, &path).expect("encode 100-byte Vec<u8> to file");

    let file_bytes = std::fs::read(&path).expect("read raw file bytes");
    let vec_bytes = oxicode::encode_to_vec(&value).expect("encode_to_vec 100-byte Vec<u8>");

    assert_eq!(
        file_bytes.len(),
        vec_bytes.len(),
        "file size must equal encode_to_vec length"
    );
    assert_eq!(
        file_bytes, vec_bytes,
        "file bytes must be identical to encode_to_vec bytes"
    );
    std::fs::remove_file(&path).ok();
}

// ── test 6: Writing to subdirectory of temp_dir ────────────────────────────────

#[test]
fn test_fio4_write_to_subdirectory() {
    let subdir = temp_dir().join(format!("oxicode_fio4_subdir_{}", std::process::id()));
    std::fs::create_dir_all(&subdir).expect("create subdirectory in temp_dir");

    let path = subdir.join("event4_subdir.bin");
    let value = Event4 {
        id: 77,
        name: "subdir_test".to_string(),
        timestamp: 7_777,
    };

    encode_to_file(&value, &path).expect("encode Event4 to subdirectory file");
    let decoded: Event4 = decode_from_file(&path).expect("decode Event4 from subdirectory file");

    assert_eq!(value.id, decoded.id);
    assert_eq!(value.name, decoded.name);
    assert_eq!(value.timestamp, decoded.timestamp);

    std::fs::remove_file(&path).ok();
    std::fs::remove_dir(&subdir).ok();
}

// ── test 7: Read after write — file exists and has correct content ─────────────

#[test]
fn test_fio4_read_after_write_file_exists_correct_content() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_7.bin", std::process::id()));
    let value = Event4 {
        id: 555,
        name: "read_after_write".to_string(),
        timestamp: 55_555,
    };

    encode_to_file(&value, &path).expect("encode Event4 for read-after-write check");
    assert!(path.exists(), "file must exist after write");

    let file_bytes = std::fs::read(&path).expect("read raw bytes after write");
    assert!(
        !file_bytes.is_empty(),
        "file must have non-zero bytes after write"
    );

    let decoded: Event4 =
        decode_from_file(&path).expect("decode Event4 for read-after-write check");
    assert_eq!(value.id, decoded.id);
    assert_eq!(value.name, decoded.name);
    assert_eq!(value.timestamp, decoded.timestamp);
    std::fs::remove_file(&path).ok();
}

// ── test 8: encode_to_file with fixed-int config ──────────────────────────────

#[test]
fn test_fio4_encode_to_file_fixed_int_config() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_8.bin", std::process::id()));
    let cfg = config::standard().with_fixed_int_encoding();
    let value: u32 = 0x0102_0304_u32;

    encode_to_file_with_config(&value, &path, cfg)
        .expect("encode u32 with fixed-int config to file");

    let file_bytes = std::fs::read(&path).expect("read raw fixed-int file");
    // With fixed-int encoding, u32 takes exactly 4 bytes
    assert_eq!(
        file_bytes.len(),
        4,
        "fixed-int u32 must occupy exactly 4 bytes"
    );
    std::fs::remove_file(&path).ok();
}

// ── test 9: decode_from_file with fixed-int config ────────────────────────────

#[test]
fn test_fio4_decode_from_file_fixed_int_config() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_9.bin", std::process::id()));
    let cfg = config::standard().with_fixed_int_encoding();
    let value: i64 = -0x1234_5678_9ABC_DEF0_i64;

    encode_to_file_with_config(&value, &path, cfg)
        .expect("encode i64 with fixed-int config to file");
    let decoded: i64 = decode_from_file_with_config(&path, cfg)
        .expect("decode i64 with fixed-int config from file");

    assert_eq!(
        value, decoded,
        "i64 with fixed-int config must round-trip exactly"
    );
    std::fs::remove_file(&path).ok();
}

// ── test 10: encode with big-endian + fixed-int config, verify raw bytes ──────

#[test]
fn test_fio4_big_endian_fixed_int_verify_raw_bytes() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_10.bin", std::process::id()));
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value: u32 = 0x11_22_33_44_u32;

    encode_to_file_with_config(&value, &path, cfg)
        .expect("encode u32 with big-endian + fixed-int to file");
    let decoded: u32 = decode_from_file_with_config(&path, cfg)
        .expect("decode u32 with big-endian + fixed-int from file");

    assert_eq!(
        value, decoded,
        "u32 must round-trip with big-endian + fixed-int"
    );

    let raw = std::fs::read(&path).expect("read raw big-endian fixed-int file");
    assert_eq!(
        raw,
        [0x11, 0x22, 0x33, 0x44],
        "big-endian bytes must be in network order"
    );
    std::fs::remove_file(&path).ok();
}

// ── test 11: Overwrite existing file — second encode replaces first ───────────

#[test]
fn test_fio4_overwrite_existing_file_second_replaces_first() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_11.bin", std::process::id()));

    let first = Event4 {
        id: 1,
        name: "first".to_string(),
        timestamp: 100,
    };
    let second = Event4 {
        id: 2,
        name: "second_event_longer_name".to_string(),
        timestamp: 200,
    };

    encode_to_file(&first, &path).expect("first encode Event4 to file");
    let first_size = std::fs::metadata(&path)
        .expect("stat after first write")
        .len();

    encode_to_file(&second, &path).expect("second encode Event4 to file (overwrite)");
    let second_size = std::fs::metadata(&path)
        .expect("stat after second write")
        .len();

    // second value has longer name, so file should be larger
    assert!(
        second_size > first_size,
        "overwritten file must reflect larger second value"
    );

    let decoded: Event4 = decode_from_file(&path).expect("decode after overwrite");
    assert_eq!(second.id, decoded.id);
    assert_eq!(second.name, decoded.name);
    assert_eq!(second.timestamp, decoded.timestamp);
    std::fs::remove_file(&path).ok();
}

// ── test 12: Option<Event4> Some to file and back ─────────────────────────────

#[test]
fn test_fio4_option_event4_some_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_12.bin", std::process::id()));
    let value: Option<Event4> = Some(Event4 {
        id: 88,
        name: "some_event".to_string(),
        timestamp: 8_888,
    });

    encode_to_file(&value, &path).expect("encode Option<Event4> Some to file");
    let decoded: Option<Event4> =
        decode_from_file(&path).expect("decode Option<Event4> Some from file");

    assert!(decoded.is_some(), "decoded Option<Event4> must be Some");
    let d = decoded.expect("Some value must be present after roundtrip");
    let orig = value.expect("original value must be Some");
    assert_eq!(orig.id, d.id);
    assert_eq!(orig.name, d.name);
    assert_eq!(orig.timestamp, d.timestamp);
    std::fs::remove_file(&path).ok();
}

// ── test 13: Option<Event4> None to file and back ─────────────────────────────

#[test]
fn test_fio4_option_event4_none_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_13.bin", std::process::id()));
    let value: Option<Event4> = None;

    encode_to_file(&value, &path).expect("encode Option<Event4> None to file");
    let decoded: Option<Event4> =
        decode_from_file(&path).expect("decode Option<Event4> None from file");

    assert!(decoded.is_none(), "decoded Option<Event4> must be None");
    std::fs::remove_file(&path).ok();
}

// ── test 14: Empty Vec<u8> to file and back ───────────────────────────────────

#[test]
fn test_fio4_empty_vec_u8_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_14.bin", std::process::id()));
    let value: Vec<u8> = Vec::new();

    encode_to_file(&value, &path).expect("encode empty Vec<u8> to file");
    let decoded: Vec<u8> = decode_from_file(&path).expect("decode empty Vec<u8> from file");

    assert_eq!(value, decoded);
    assert!(decoded.is_empty(), "decoded Vec<u8> must be empty");
    std::fs::remove_file(&path).ok();
}

// ── test 15: u128::MAX to file and back ───────────────────────────────────────

#[test]
fn test_fio4_u128_max_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_15.bin", std::process::id()));
    let value: u128 = u128::MAX;

    encode_to_file(&value, &path).expect("encode u128::MAX to file");
    let decoded: u128 = decode_from_file(&path).expect("decode u128::MAX from file");

    assert_eq!(value, decoded, "u128::MAX must round-trip exactly via file");
    std::fs::remove_file(&path).ok();
}

// ── test 16: i128::MIN to file and back ───────────────────────────────────────

#[test]
fn test_fio4_i128_min_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_16.bin", std::process::id()));
    let value: i128 = i128::MIN;

    encode_to_file(&value, &path).expect("encode i128::MIN to file");
    let decoded: i128 = decode_from_file(&path).expect("decode i128::MIN from file");

    assert_eq!(value, decoded, "i128::MIN must round-trip exactly via file");
    std::fs::remove_file(&path).ok();
}

// ── test 17: f64::NAN bit-exact roundtrip via file ────────────────────────────

#[test]
fn test_fio4_f64_nan_bit_exact_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_17.bin", std::process::id()));
    let value: f64 = f64::NAN;

    encode_to_file(&value, &path).expect("encode f64::NAN to file");
    let decoded: f64 = decode_from_file(&path).expect("decode f64::NAN from file");

    // NaN != NaN by IEEE 754; compare bit patterns directly for exact roundtrip
    assert_eq!(
        value.to_bits(),
        decoded.to_bits(),
        "f64::NAN bit pattern must be preserved through file round-trip"
    );
    assert!(decoded.is_nan(), "decoded f64 must still be NaN");
    std::fs::remove_file(&path).ok();
}

// ── test 18: Very long string (1000 chars) to file and back ───────────────────

#[test]
fn test_fio4_very_long_string_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_18.bin", std::process::id()));
    let value: String = "X".repeat(1000);
    assert_eq!(value.len(), 1000);

    encode_to_file(&value, &path).expect("encode 1000-char String to file");
    let decoded: String = decode_from_file(&path).expect("decode 1000-char String from file");

    assert_eq!(value, decoded);
    assert_eq!(decoded.len(), 1000);
    std::fs::remove_file(&path).ok();
}

// ── test 19: bool true/false to separate files, read back ─────────────────────

#[test]
fn test_fio4_bool_true_false_separate_files() {
    let pid = std::process::id();
    let path_true = temp_dir().join(format!("oxicode_fio4_{}_19a.bin", pid));
    let path_false = temp_dir().join(format!("oxicode_fio4_{}_19b.bin", pid));

    encode_to_file(&true, &path_true).expect("encode bool true to file");
    encode_to_file(&false, &path_false).expect("encode bool false to file");

    let decoded_true: bool = decode_from_file(&path_true).expect("decode bool true from file");
    let decoded_false: bool = decode_from_file(&path_false).expect("decode bool false from file");

    assert!(decoded_true, "decoded bool from 'true' file must be true");
    assert!(
        !decoded_false,
        "decoded bool from 'false' file must be false"
    );

    std::fs::remove_file(&path_true).ok();
    std::fs::remove_file(&path_false).ok();
}

// ── test 20: Sequential: write 10 u32 values, read all 10 back in order ───────

#[test]
fn test_fio4_sequential_10_u32_write_read_in_order() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_20.bin", std::process::id()));
    let cfg = config::standard();
    let values: Vec<u32> = (0..10).map(|i| i * 111 + 1).collect();

    {
        let mut file = std::fs::File::create(&path).expect("create file for 10 sequential u32");
        for &v in &values {
            encode_into_std_write(v, &mut file, cfg).expect("encode u32 sequentially");
        }
    }

    let file = std::fs::File::open(&path).expect("open file for sequential u32 decode");
    let mut reader = std::io::BufReader::new(file);
    let mut decoded_values: Vec<u32> = Vec::with_capacity(10);
    for _ in 0..10 {
        let v: u32 =
            decode_from_std_read(&mut reader, cfg).expect("decode u32 from sequential stream");
        decoded_values.push(v);
    }

    assert_eq!(
        values, decoded_values,
        "10 sequential u32 values must be read back in order"
    );
    std::fs::remove_file(&path).ok();
}

// ── test 21: File cleanup — after remove_file, decode_from_file returns Err ───

#[test]
fn test_fio4_file_cleanup_decode_returns_err_after_remove() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_21.bin", std::process::id()));
    let value = Event4 {
        id: 0,
        name: "to_be_deleted".to_string(),
        timestamp: 0,
    };

    encode_to_file(&value, &path).expect("encode Event4 before file removal");
    assert!(path.exists(), "file must exist before removal");

    std::fs::remove_file(&path).expect("remove temp file");
    assert!(!path.exists(), "file must not exist after removal");

    let result = decode_from_file::<Event4>(&path);
    assert!(
        result.is_err(),
        "decode_from_file must return Err after file has been removed"
    );
}

// ── test 22: BTreeMap<u32, String> to file and back ──────────────────────────

#[test]
fn test_fio4_btreemap_u32_string_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio4_{}_22.bin", std::process::id()));
    let mut value: BTreeMap<u32, String> = BTreeMap::new();
    value.insert(1, "one".to_string());
    value.insert(2, "two".to_string());
    value.insert(3, "three".to_string());
    value.insert(100, "hundred".to_string());
    value.insert(0, "zero".to_string());

    encode_to_file(&value, &path).expect("encode BTreeMap<u32, String> to file");
    let decoded: BTreeMap<u32, String> =
        decode_from_file(&path).expect("decode BTreeMap<u32, String> from file");

    assert_eq!(
        value, decoded,
        "BTreeMap<u32, String> must round-trip exactly via file"
    );
    // Verify ordering is preserved (BTreeMap iterates in key order)
    let orig_keys: Vec<u32> = value.keys().copied().collect();
    let dec_keys: Vec<u32> = decoded.keys().copied().collect();
    assert_eq!(
        orig_keys, dec_keys,
        "BTreeMap keys must be in sorted order after round-trip"
    );
    std::fs::remove_file(&path).ok();
}
