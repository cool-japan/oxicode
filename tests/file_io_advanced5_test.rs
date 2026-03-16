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

// ── Shared types ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllPrimitives {
    a: i8,
    b: i16,
    c: i32,
    d: i64,
    e: i128,
    f: u8,
    g: u16,
    h: u32,
    i: u64,
    j: u128,
    k: f32,
    l: f64,
    m: bool,
    n: char,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MultiVariant {
    Unit,
    Newtype(u64),
    Pair(i32, i32),
    Named { label: String, value: f64 },
}

// ── Test 1: i8 roundtrip through file ────────────────────────────────────────

#[test]
fn test_file_io_i8_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test1_{}.bin", std::process::id()));
    let original: i8 = -42;
    oxicode::encode_to_file(&original, &path).expect("encode i8 to file");
    let decoded: i8 = oxicode::decode_from_file(&path).expect("decode i8 from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 2: i16 roundtrip through file ───────────────────────────────────────

#[test]
fn test_file_io_i16_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test2_{}.bin", std::process::id()));
    let original: i16 = -32000;
    oxicode::encode_to_file(&original, &path).expect("encode i16 to file");
    let decoded: i16 = oxicode::decode_from_file(&path).expect("decode i16 from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 3: i32 roundtrip through file ───────────────────────────────────────

#[test]
fn test_file_io_i32_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test3_{}.bin", std::process::id()));
    let original: i32 = -2_000_000_000;
    oxicode::encode_to_file(&original, &path).expect("encode i32 to file");
    let decoded: i32 = oxicode::decode_from_file(&path).expect("decode i32 from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 4: i64 roundtrip through file ───────────────────────────────────────

#[test]
fn test_file_io_i64_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test4_{}.bin", std::process::id()));
    let original: i64 = i64::MIN;
    oxicode::encode_to_file(&original, &path).expect("encode i64 to file");
    let decoded: i64 = oxicode::decode_from_file(&path).expect("decode i64 from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 5: i128 roundtrip through file ──────────────────────────────────────

#[test]
fn test_file_io_i128_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test5_{}.bin", std::process::id()));
    let original: i128 = i128::MIN / 2 + 1;
    oxicode::encode_to_file(&original, &path).expect("encode i128 to file");
    let decoded: i128 = oxicode::decode_from_file(&path).expect("decode i128 from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 6: u8 roundtrip through file ────────────────────────────────────────

#[test]
fn test_file_io_u8_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test6_{}.bin", std::process::id()));
    let original: u8 = u8::MAX;
    oxicode::encode_to_file(&original, &path).expect("encode u8 to file");
    let decoded: u8 = oxicode::decode_from_file(&path).expect("decode u8 from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 7: char roundtrip through file ──────────────────────────────────────

#[test]
fn test_file_io_char_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test7_{}.bin", std::process::id()));
    let original: char = '⚡';
    oxicode::encode_to_file(&original, &path).expect("encode char to file");
    let decoded: char = oxicode::decode_from_file(&path).expect("decode char from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 8: f32::PI roundtrip through file (bit-exact) ───────────────────────

#[test]
fn test_file_io_f32_pi_roundtrip_bit_exact() {
    let path = temp_dir().join(format!("oxicode_fio5_test8_{}.bin", std::process::id()));
    let original: f32 = std::f32::consts::PI;
    oxicode::encode_to_file(&original, &path).expect("encode f32::PI to file");
    let decoded: f32 = oxicode::decode_from_file(&path).expect("decode f32::PI from file");
    assert_eq!(
        original.to_bits(),
        decoded.to_bits(),
        "f32::PI must be bit-exact after roundtrip"
    );
    std::fs::remove_file(&path).ok();
}

// ── Test 9: Tuple (u32, String) roundtrip through file ───────────────────────

#[test]
fn test_file_io_tuple_u32_string_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test9_{}.bin", std::process::id()));
    let original: (u32, String) = (99, "tuple-value".to_string());
    oxicode::encode_to_file(&original, &path).expect("encode (u32, String) to file");
    let decoded: (u32, String) =
        oxicode::decode_from_file(&path).expect("decode (u32, String) from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 10: Vec<bool> roundtrip through file ─────────────────────────────────

#[test]
fn test_file_io_vec_bool_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test10_{}.bin", std::process::id()));
    let original: Vec<bool> = vec![true, false, true, true, false, false, true];
    oxicode::encode_to_file(&original, &path).expect("encode Vec<bool> to file");
    let decoded: Vec<bool> = oxicode::decode_from_file(&path).expect("decode Vec<bool> from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 11: Vec<(u32, String)> roundtrip through file ───────────────────────

#[test]
fn test_file_io_vec_tuple_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test11_{}.bin", std::process::id()));
    let original: Vec<(u32, String)> = vec![
        (1, "alpha".to_string()),
        (2, "beta".to_string()),
        (3, "gamma".to_string()),
    ];
    oxicode::encode_to_file(&original, &path).expect("encode Vec<(u32,String)> to file");
    let decoded: Vec<(u32, String)> =
        oxicode::decode_from_file(&path).expect("decode Vec<(u32,String)> from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 12: Struct with all primitive types roundtrip through file ───────────

#[test]
fn test_file_io_all_primitives_struct_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test12_{}.bin", std::process::id()));
    let original = AllPrimitives {
        a: -1,
        b: -512,
        c: -100_000,
        d: -1_000_000_000,
        e: -1_000_000_000_000_000,
        f: 255,
        g: 65535,
        h: 4_294_967_295,
        i: u64::MAX / 2,
        j: u128::MAX / 3,
        k: std::f32::consts::E,
        l: std::f64::consts::SQRT_2,
        m: true,
        n: 'Z',
    };
    oxicode::encode_to_file(&original, &path).expect("encode AllPrimitives to file");
    let decoded: AllPrimitives =
        oxicode::decode_from_file(&path).expect("decode AllPrimitives from file");
    assert_eq!(original.a, decoded.a);
    assert_eq!(original.b, decoded.b);
    assert_eq!(original.c, decoded.c);
    assert_eq!(original.d, decoded.d);
    assert_eq!(original.e, decoded.e);
    assert_eq!(original.f, decoded.f);
    assert_eq!(original.g, decoded.g);
    assert_eq!(original.h, decoded.h);
    assert_eq!(original.i, decoded.i);
    assert_eq!(original.j, decoded.j);
    assert_eq!(original.k.to_bits(), decoded.k.to_bits());
    assert_eq!(original.l.to_bits(), decoded.l.to_bits());
    assert_eq!(original.m, decoded.m);
    assert_eq!(original.n, decoded.n);
    std::fs::remove_file(&path).ok();
}

// ── Test 13: Enum with multiple variants roundtrip through file ───────────────

#[test]
fn test_file_io_enum_multi_variant_roundtrip() {
    let path_unit = temp_dir().join(format!("oxicode_fio5_test13a_{}.bin", std::process::id()));
    let path_newtype = temp_dir().join(format!("oxicode_fio5_test13b_{}.bin", std::process::id()));
    let path_pair = temp_dir().join(format!("oxicode_fio5_test13c_{}.bin", std::process::id()));
    let path_named = temp_dir().join(format!("oxicode_fio5_test13d_{}.bin", std::process::id()));

    let unit = MultiVariant::Unit;
    oxicode::encode_to_file(&unit, &path_unit).expect("encode Unit variant");
    let decoded_unit: MultiVariant =
        oxicode::decode_from_file(&path_unit).expect("decode Unit variant");
    assert_eq!(unit, decoded_unit);

    let newtype = MultiVariant::Newtype(42_000);
    oxicode::encode_to_file(&newtype, &path_newtype).expect("encode Newtype variant");
    let decoded_newtype: MultiVariant =
        oxicode::decode_from_file(&path_newtype).expect("decode Newtype variant");
    assert_eq!(newtype, decoded_newtype);

    let pair = MultiVariant::Pair(-7, 13);
    oxicode::encode_to_file(&pair, &path_pair).expect("encode Pair variant");
    let decoded_pair: MultiVariant =
        oxicode::decode_from_file(&path_pair).expect("decode Pair variant");
    assert_eq!(pair, decoded_pair);

    let named = MultiVariant::Named {
        label: "oxicode".to_string(),
        value: std::f64::consts::PI,
    };
    oxicode::encode_to_file(&named, &path_named).expect("encode Named variant");
    let decoded_named: MultiVariant =
        oxicode::decode_from_file(&path_named).expect("decode Named variant");
    assert_eq!(named, decoded_named);

    std::fs::remove_file(&path_unit).ok();
    std::fs::remove_file(&path_newtype).ok();
    std::fs::remove_file(&path_pair).ok();
    std::fs::remove_file(&path_named).ok();
}

// ── Test 14: encode_to_file with fixed_int_encoding config roundtrip ──────────

#[test]
fn test_file_io_fixed_int_encoding_config_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test14_{}.bin", std::process::id()));
    let cfg = oxicode::config::standard().with_fixed_int_encoding();
    let original: u64 = 12_345_678_901;
    oxicode::encode_to_file_with_config(&original, &path, cfg).expect("encode with fixed_int cfg");
    let decoded: u64 =
        oxicode::decode_from_file_with_config(&path, cfg).expect("decode with fixed_int cfg");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 15: encode_to_file with big_endian config — verify raw bytes ─────────

#[test]
fn test_file_io_big_endian_config_raw_bytes_verification() {
    let path = temp_dir().join(format!("oxicode_fio5_test15_{}.bin", std::process::id()));
    let cfg = oxicode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let original: u32 = 0x0102_0304;

    oxicode::encode_to_file_with_config(&original, &path, cfg)
        .expect("encode u32 with big_endian fixed_int cfg");

    let raw_bytes = std::fs::read(&path).expect("read raw bytes from file");
    // With big-endian fixed-int encoding, u32 0x01020304 must appear as [0x01, 0x02, 0x03, 0x04]
    assert!(
        raw_bytes.windows(4).any(|w| w == [0x01, 0x02, 0x03, 0x04]),
        "big-endian bytes not found in file: {:?}",
        raw_bytes
    );

    let decoded: u32 =
        oxicode::decode_from_file_with_config(&path, cfg).expect("decode with big_endian cfg");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 16: Overwrite file multiple times — last write wins ──────────────────

#[test]
fn test_file_io_overwrite_multiple_times_last_write_wins() {
    let path = temp_dir().join(format!("oxicode_fio5_test16_{}.bin", std::process::id()));

    for val in [100u32, 200u32, 300u32, 400u32, 999u32] {
        oxicode::encode_to_file(&val, &path).expect("encode overwrite pass");
    }

    let decoded: u32 = oxicode::decode_from_file(&path).expect("decode after multiple overwrites");
    assert_eq!(999u32, decoded, "only the last write should survive");
    std::fs::remove_file(&path).ok();
}

// ── Test 17: decode from nonexistent file returns Err ────────────────────────

#[test]
fn test_file_io_decode_nonexistent_file_returns_err() {
    let path = temp_dir().join(format!(
        "oxicode_fio5_test17_nonexistent_{}.bin",
        std::process::id()
    ));
    // Guarantee the file does not exist
    std::fs::remove_file(&path).ok();

    let result = oxicode::decode_from_file::<u64>(&path);
    assert!(
        result.is_err(),
        "decoding a nonexistent file must return Err"
    );
}

// ── Test 18: File size matches encode_to_vec output for complex struct ────────

#[test]
fn test_file_io_file_size_matches_encode_to_vec() {
    let path = temp_dir().join(format!("oxicode_fio5_test18_{}.bin", std::process::id()));

    let original = AllPrimitives {
        a: 127,
        b: 256,
        c: 1_000_000,
        d: -9_999_999,
        e: 1,
        f: 0,
        g: 1,
        h: 2,
        i: 3,
        j: 4,
        k: 1.0_f32,
        l: 2.0_f64,
        m: false,
        n: 'A',
    };

    oxicode::encode_to_file(&original, &path).expect("encode AllPrimitives for size check");
    let file_bytes = std::fs::read(&path).expect("read file bytes");
    let vec_bytes =
        oxicode::encode_to_vec(&original).expect("encode AllPrimitives to vec for comparison");

    assert_eq!(
        file_bytes.len(),
        vec_bytes.len(),
        "file size must match encode_to_vec length"
    );
    assert_eq!(
        file_bytes, vec_bytes,
        "file contents must exactly match encode_to_vec output"
    );
    std::fs::remove_file(&path).ok();
}

// ── Test 19: Vec<Vec<u8>> roundtrip through file ──────────────────────────────

#[test]
fn test_file_io_vec_of_vec_u8_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test19_{}.bin", std::process::id()));
    let original: Vec<Vec<u8>> = vec![
        vec![0, 1, 2, 3],
        vec![],
        vec![255, 254, 253],
        (0u8..=127).collect(),
    ];
    oxicode::encode_to_file(&original, &path).expect("encode Vec<Vec<u8>> to file");
    let decoded: Vec<Vec<u8>> =
        oxicode::decode_from_file(&path).expect("decode Vec<Vec<u8>> from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 20: Option<Vec<String>> Some roundtrip through file ─────────────────

#[test]
fn test_file_io_option_vec_string_some_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test20_{}.bin", std::process::id()));
    let original: Option<Vec<String>> = Some(vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
    ]);
    oxicode::encode_to_file(&original, &path).expect("encode Option<Vec<String>> Some to file");
    let decoded: Option<Vec<String>> =
        oxicode::decode_from_file(&path).expect("decode Option<Vec<String>> from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// ── Test 21: u64::MAX roundtrip through file ──────────────────────────────────

#[test]
fn test_file_io_u64_max_roundtrip() {
    let path = temp_dir().join(format!("oxicode_fio5_test21_{}.bin", std::process::id()));
    let original: u64 = u64::MAX;
    oxicode::encode_to_file(&original, &path).expect("encode u64::MAX to file");
    let decoded: u64 = oxicode::decode_from_file(&path).expect("decode u64::MAX from file");
    assert_eq!(
        original, decoded,
        "u64::MAX must survive file roundtrip exactly"
    );
    std::fs::remove_file(&path).ok();
}

// ── Test 22: Sequential write of two values, decode both sequentially ─────────

#[test]
fn test_file_io_sequential_write_decode_two_values() {
    use std::io::Write;

    let path = temp_dir().join(format!("oxicode_fio5_test22_{}.bin", std::process::id()));

    // Encode each value into a separate byte vector, then concatenate into one file
    let first_val: u32 = 0xDEAD_BEEF;
    let second_val: String = "sequential-decode".to_string();

    let first_bytes = oxicode::encode_to_vec(&first_val).expect("encode first value to vec");
    let second_bytes = oxicode::encode_to_vec(&second_val).expect("encode second value to vec");

    let mut file = std::fs::File::create(&path).expect("create sequential test file");
    file.write_all(&first_bytes).expect("write first bytes");
    file.write_all(&second_bytes).expect("write second bytes");
    drop(file);

    // Decode both values sequentially using a BufReader
    let mut reader =
        std::io::BufReader::new(std::fs::File::open(&path).expect("open sequential test file"));

    let decoded_first: u32 =
        oxicode::decode_from_std_read(&mut reader, oxicode::config::standard())
            .expect("decode first u32 sequentially");
    let decoded_second: String =
        oxicode::decode_from_std_read(&mut reader, oxicode::config::standard())
            .expect("decode second String sequentially");

    assert_eq!(first_val, decoded_first, "first sequential value mismatch");
    assert_eq!(
        second_val, decoded_second,
        "second sequential value mismatch"
    );
    std::fs::remove_file(&path).ok();
}
