//! Regeneration harness for the committed golden-vector corpus.
//!
//! This test is `#[ignore]`d by default: the corpus in
//! `compatibility/src/golden_vectors.rs` is committed data and must not
//! change silently. Run it explicitly (`cargo test -p oxicode_compatibility
//! --test regen_golden_vectors -- --ignored --nocapture`) whenever the corpus
//! needs to be regenerated (e.g. a new type/config/value is added to the
//! matrix), then diff the printed output against
//! `compatibility/src/golden_vectors.rs` and update that file by hand.
//!
//! This is the ONLY place in the crate that is allowed to depend on the live
//! `bincode` crate as an encoding oracle at test-run time; every other test
//! in this crate either cross-checks bincode directly (the existing suites in
//! `src/lib.rs`) or checks against the already-committed corpus
//! (`tests/golden_vectors_test.rs`), never regenerating it.

// 3.14159 is deliberately an approximation of pi (not `std::f32::consts::PI`
// itself): it must match the literal baked into the committed corpus.
#![allow(clippy::approx_constant)]

use bincode::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
    Unit,
}

fn fmt_bytes(bytes: &[u8]) -> String {
    let mut s = String::from("&[");
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 {
            s.push_str(", ");
        }
        s.push_str(&format!("0x{b:02x}"));
    }
    s.push(']');
    s
}

fn gen_for_config<C: bincode::config::Config + Copy>(cfg: C) {
    macro_rules! emit {
        ($name:expr, $v:expr) => {
            println!(
                "    (\"{}\", {}),",
                $name,
                fmt_bytes(&bincode::encode_to_vec($v, cfg).expect("encode failed"))
            );
        };
    }

    emit!("u8_0", 0u8);
    emit!("u8_max", u8::MAX);
    emit!("u16_0", 0u16);
    emit!("u16_max", u16::MAX);
    emit!("u32_250", 250u32);
    emit!("u32_251", 251u32);
    emit!("u32_65535", 65535u32);
    emit!("u32_65536", 65536u32);
    emit!("u32_max", u32::MAX);
    emit!("u64_0", 0u64);
    emit!("u64_250", 250u64);
    emit!("u64_251", 251u64);
    emit!("u64_65535", 65535u64);
    emit!("u64_65536", 65536u64);
    emit!("u64_4294967295", 4294967295u64);
    emit!("u64_4294967296", 4294967296u64);
    emit!("u64_max", u64::MAX);
    emit!("u128_max", u128::MAX);
    emit!("u128_250", 250u128);
    emit!("u128_251", 251u128);

    emit!("i8_min", i8::MIN);
    emit!("i8_max", i8::MAX);
    emit!("i16_min", i16::MIN);
    emit!("i16_max", i16::MAX);
    emit!("i32_min", i32::MIN);
    emit!("i32_max", i32::MAX);
    emit!("i64_0", 0i64);
    emit!("i64_neg1", -1i64);
    emit!("i64_1", 1i64);
    emit!("i64_neg1000", -1000i64);
    emit!("i64_1000", 1000i64);
    emit!("i64_min", i64::MIN);
    emit!("i64_max", i64::MAX);
    emit!("i128_min", i128::MIN);
    emit!("i128_max", i128::MAX);

    emit!("bool_true", true);
    emit!("bool_false", false);

    emit!("f32_pi", 3.14159_f32);
    emit!("f32_neg_zero", -0.0_f32);
    emit!("f32_infinity", f32::INFINITY);
    emit!("f32_nan", f32::NAN);
    emit!("f64_pi", std::f64::consts::PI);
    emit!("f64_neg_zero", -0.0_f64);
    emit!("f64_infinity", f64::INFINITY);
    emit!("f64_nan", f64::NAN);

    emit!("char_ascii", 'a');
    emit!("char_2byte", '\u{e9}');
    emit!("char_3byte", '\u{4e2d}');
    emit!("char_4byte", '\u{1f980}');
    emit!("char_max", '\u{10ffff}');
    emit!("char_nul", '\0');

    emit!("string_empty", String::new());
    emit!("string_hello_crab", "Hello, bincode! \u{1f980}".to_string());

    emit!("vec_u32_empty", Vec::<u32>::new());
    emit!("vec_u32_12345", vec![1u32, 2, 3, 4, 5]);

    emit!("tuple_u32_string_bool", (42u32, "test".to_string(), true));

    let some_val: Option<u64> = Some(999);
    let none_val: Option<u64> = None;
    emit!("option_some_u64_999", some_val);
    emit!("option_none_u64", none_val);

    let ok_val: Result<u32, String> = Ok(5);
    let err_val: Result<u32, String> = Err("bad".to_string());
    emit!("result_ok_u32_5", ok_val);
    emit!("result_err_string_bad", err_val);

    let mut btm: std::collections::BTreeMap<u32, String> = std::collections::BTreeMap::new();
    btm.insert(1, "a".to_string());
    btm.insert(2, "b".to_string());
    emit!("btreemap_u32_string", btm);

    let mut bts: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();
    bts.insert(1);
    bts.insert(2);
    bts.insert(3);
    emit!("btreeset_u32", bts);

    emit!("struct_point", Point { x: 10, y: -20 });
    emit!("enum_unit_shape", Shape::Unit);
    emit!("enum_tuple_shape", Shape::Circle(2.5));
    emit!("enum_struct_shape", Shape::Rect { w: 3.0, h: 4.0 });
}

/// Prints a fresh copy of the four golden-vector tables from a live bincode
/// 2.0.1 oracle. Ignored by default — see module docs.
#[test]
#[ignore = "regeneration harness: run manually with --ignored --nocapture and diff against src/golden_vectors.rs"]
fn regenerate_golden_vectors() {
    println!("pub const LE_VARINT: &[(&str, &[u8])] = &[");
    gen_for_config(bincode::config::standard());
    println!("];\n");

    println!("pub const LE_FIXINT: &[(&str, &[u8])] = &[");
    gen_for_config(bincode::config::legacy());
    println!("];\n");

    println!("pub const BE_VARINT: &[(&str, &[u8])] = &[");
    gen_for_config(bincode::config::standard().with_big_endian());
    println!("];\n");

    println!("pub const BE_FIXINT: &[(&str, &[u8])] = &[");
    gen_for_config(bincode::config::legacy().with_big_endian());
    println!("];");
}
