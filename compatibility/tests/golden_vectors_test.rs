//! Byte-for-byte assertions of `oxicode`'s encoder against the committed
//! golden-vector corpus in `oxicode_compatibility::golden_vectors`.
//!
//! Unlike the tests in `src/lib.rs`, this file never calls into the live
//! `bincode` crate: every expected byte sequence was generated once, offline,
//! from bincode 2.0.1 and is now hard-coded data (see
//! `compatibility/src/golden_vectors.rs`). This is what lets the compatibility
//! guarantee for these types be checked even on hosts/targets where running
//! the actual `bincode` crate as a live oracle is impractical (e.g. cross
//! builds), and keeps the assertions reproducible byte-for-byte.

// 3.14159 is deliberately an approximation of pi (not `std::f32::consts::PI`
// itself): the golden-vector corpus locks in this exact literal's bit
// pattern, generated once from bincode 2.0.1.
#![allow(clippy::approx_constant)]

use oxicode_compatibility::golden_vectors::{lookup, BE_FIXINT, BE_VARINT, LE_FIXINT, LE_VARINT};

#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
    Unit,
}

/// Encode every entry of the corpus under a given oxicode config and assert
/// the bytes match the committed golden vector exactly.
macro_rules! assert_all_golden {
    ($table:expr, $config:expr) => {{
        let config = $config;
        let table = $table;

        assert_eq!(
            oxicode::encode_to_vec_with_config(&0u8, config).unwrap(),
            lookup(table, "u8_0")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&u8::MAX, config).unwrap(),
            lookup(table, "u8_max")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&0u16, config).unwrap(),
            lookup(table, "u16_0")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&u16::MAX, config).unwrap(),
            lookup(table, "u16_max")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&250u32, config).unwrap(),
            lookup(table, "u32_250")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&251u32, config).unwrap(),
            lookup(table, "u32_251")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&65535u32, config).unwrap(),
            lookup(table, "u32_65535")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&65536u32, config).unwrap(),
            lookup(table, "u32_65536")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&u32::MAX, config).unwrap(),
            lookup(table, "u32_max")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&0u64, config).unwrap(),
            lookup(table, "u64_0")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&250u64, config).unwrap(),
            lookup(table, "u64_250")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&251u64, config).unwrap(),
            lookup(table, "u64_251")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&65535u64, config).unwrap(),
            lookup(table, "u64_65535")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&65536u64, config).unwrap(),
            lookup(table, "u64_65536")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&4294967295u64, config).unwrap(),
            lookup(table, "u64_4294967295")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&4294967296u64, config).unwrap(),
            lookup(table, "u64_4294967296")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&u64::MAX, config).unwrap(),
            lookup(table, "u64_max")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&u128::MAX, config).unwrap(),
            lookup(table, "u128_max")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&250u128, config).unwrap(),
            lookup(table, "u128_250")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&251u128, config).unwrap(),
            lookup(table, "u128_251")
        );

        assert_eq!(
            oxicode::encode_to_vec_with_config(&i8::MIN, config).unwrap(),
            lookup(table, "i8_min")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&i8::MAX, config).unwrap(),
            lookup(table, "i8_max")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&i16::MIN, config).unwrap(),
            lookup(table, "i16_min")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&i16::MAX, config).unwrap(),
            lookup(table, "i16_max")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&i32::MIN, config).unwrap(),
            lookup(table, "i32_min")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&i32::MAX, config).unwrap(),
            lookup(table, "i32_max")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&0i64, config).unwrap(),
            lookup(table, "i64_0")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&(-1i64), config).unwrap(),
            lookup(table, "i64_neg1")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&1i64, config).unwrap(),
            lookup(table, "i64_1")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&(-1000i64), config).unwrap(),
            lookup(table, "i64_neg1000")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&1000i64, config).unwrap(),
            lookup(table, "i64_1000")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&i64::MIN, config).unwrap(),
            lookup(table, "i64_min")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&i64::MAX, config).unwrap(),
            lookup(table, "i64_max")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&i128::MIN, config).unwrap(),
            lookup(table, "i128_min")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&i128::MAX, config).unwrap(),
            lookup(table, "i128_max")
        );

        assert_eq!(
            oxicode::encode_to_vec_with_config(&true, config).unwrap(),
            lookup(table, "bool_true")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&false, config).unwrap(),
            lookup(table, "bool_false")
        );

        assert_eq!(
            oxicode::encode_to_vec_with_config(&3.14159_f32, config).unwrap(),
            lookup(table, "f32_pi")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&(-0.0_f32), config).unwrap(),
            lookup(table, "f32_neg_zero")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&f32::INFINITY, config).unwrap(),
            lookup(table, "f32_infinity")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&f32::NAN, config).unwrap(),
            lookup(table, "f32_nan")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&std::f64::consts::PI, config).unwrap(),
            lookup(table, "f64_pi")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&(-0.0_f64), config).unwrap(),
            lookup(table, "f64_neg_zero")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&f64::INFINITY, config).unwrap(),
            lookup(table, "f64_infinity")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&f64::NAN, config).unwrap(),
            lookup(table, "f64_nan")
        );

        assert_eq!(
            oxicode::encode_to_vec_with_config(&'a', config).unwrap(),
            lookup(table, "char_ascii")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&'\u{e9}', config).unwrap(),
            lookup(table, "char_2byte")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&'\u{4e2d}', config).unwrap(),
            lookup(table, "char_3byte")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&'\u{1f980}', config).unwrap(),
            lookup(table, "char_4byte")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&'\u{10ffff}', config).unwrap(),
            lookup(table, "char_max")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&'\0', config).unwrap(),
            lookup(table, "char_nul")
        );

        assert_eq!(
            oxicode::encode_to_vec_with_config(&String::new(), config).unwrap(),
            lookup(table, "string_empty")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&"Hello, bincode! \u{1f980}".to_string(), config)
                .unwrap(),
            lookup(table, "string_hello_crab")
        );

        assert_eq!(
            oxicode::encode_to_vec_with_config(&Vec::<u32>::new(), config).unwrap(),
            lookup(table, "vec_u32_empty")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&vec![1u32, 2, 3, 4, 5], config).unwrap(),
            lookup(table, "vec_u32_12345")
        );

        assert_eq!(
            oxicode::encode_to_vec_with_config(&(42u32, "test".to_string(), true), config).unwrap(),
            lookup(table, "tuple_u32_string_bool")
        );

        let some_val: Option<u64> = Some(999);
        let none_val: Option<u64> = None;
        assert_eq!(
            oxicode::encode_to_vec_with_config(&some_val, config).unwrap(),
            lookup(table, "option_some_u64_999")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&none_val, config).unwrap(),
            lookup(table, "option_none_u64")
        );

        let ok_val: Result<u32, String> = Ok(5);
        let err_val: Result<u32, String> = Err("bad".to_string());
        assert_eq!(
            oxicode::encode_to_vec_with_config(&ok_val, config).unwrap(),
            lookup(table, "result_ok_u32_5")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&err_val, config).unwrap(),
            lookup(table, "result_err_string_bad")
        );

        let mut btm: std::collections::BTreeMap<u32, String> = std::collections::BTreeMap::new();
        btm.insert(1, "a".to_string());
        btm.insert(2, "b".to_string());
        assert_eq!(
            oxicode::encode_to_vec_with_config(&btm, config).unwrap(),
            lookup(table, "btreemap_u32_string")
        );

        let mut bts: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();
        bts.insert(1);
        bts.insert(2);
        bts.insert(3);
        assert_eq!(
            oxicode::encode_to_vec_with_config(&bts, config).unwrap(),
            lookup(table, "btreeset_u32")
        );

        assert_eq!(
            oxicode::encode_to_vec_with_config(&Point { x: 10, y: -20 }, config).unwrap(),
            lookup(table, "struct_point")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&Shape::Unit, config).unwrap(),
            lookup(table, "enum_unit_shape")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&Shape::Circle(2.5), config).unwrap(),
            lookup(table, "enum_tuple_shape")
        );
        assert_eq!(
            oxicode::encode_to_vec_with_config(&Shape::Rect { w: 3.0, h: 4.0 }, config).unwrap(),
            lookup(table, "enum_struct_shape")
        );
    }};
}

#[test]
fn golden_vectors_le_varint_standard() {
    assert_all_golden!(LE_VARINT, oxicode::config::standard());
}

#[test]
fn golden_vectors_le_fixint_legacy() {
    assert_all_golden!(LE_FIXINT, oxicode::config::legacy());
}

#[test]
fn golden_vectors_be_varint() {
    assert_all_golden!(BE_VARINT, oxicode::config::standard().with_big_endian());
}

#[test]
fn golden_vectors_be_fixint() {
    assert_all_golden!(BE_FIXINT, oxicode::config::legacy().with_big_endian());
}

/// `standard().with_fixed_int_encoding()` and `legacy()` are the same
/// configuration (little-endian + fixed-int) in both bincode and oxicode;
/// lock that equivalence down explicitly so a future change to either
/// config's defaults would be caught here.
#[test]
fn fixed_int_config_matches_legacy_config() {
    let value = (42u32, "test".to_string(), true);
    let fixed_int_bytes = oxicode::encode_to_vec_with_config(
        &value,
        oxicode::config::standard().with_fixed_int_encoding(),
    )
    .unwrap();
    let legacy_bytes =
        oxicode::encode_to_vec_with_config(&value, oxicode::config::legacy()).unwrap();
    assert_eq!(fixed_int_bytes, legacy_bytes);
    assert_eq!(fixed_int_bytes, lookup(LE_FIXINT, "tuple_u32_string_bool"));
}
