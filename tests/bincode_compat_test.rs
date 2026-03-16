//! Wire format compatibility tests with bincode v2
//!
//! Verifies that oxicode produces identical bytes to bincode v2 for a wide
//! range of types and configurations. Tests in compatibility/src/lib.rs cover
//! the basics; this file goes deeper: more types, edge cases, cross-decode,
//! and byte-level assertions.

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
use std::collections::{BTreeMap, BTreeSet, LinkedList, VecDeque};
use std::io::Cursor;

// ---------------------------------------------------------------------------
// Shared derive structs/enums (derive both crates' traits on the same type)
// ---------------------------------------------------------------------------

#[derive(
    Debug, PartialEq, Clone, bincode::Encode, bincode::Decode, oxicode::Encode, oxicode::Decode,
)]
struct SharedSimple {
    id: u32,
    name: String,
    flag: bool,
}

#[derive(
    Debug, PartialEq, Clone, bincode::Encode, bincode::Decode, oxicode::Encode, oxicode::Decode,
)]
struct SharedInner {
    x: i32,
    y: i32,
}

#[derive(
    Debug, PartialEq, Clone, bincode::Encode, bincode::Decode, oxicode::Encode, oxicode::Decode,
)]
struct SharedOuter {
    inner: SharedInner,
    data: Vec<u8>,
    label: String,
}

#[derive(
    Debug, PartialEq, Clone, bincode::Encode, bincode::Decode, oxicode::Encode, oxicode::Decode,
)]
struct SharedWithVecF64 {
    id: u32,
    name: String,
    values: Vec<f64>,
    flag: bool,
}

#[derive(
    Debug, PartialEq, Clone, bincode::Encode, bincode::Decode, oxicode::Encode, oxicode::Decode,
)]
enum SharedEnum {
    Unit,
    Newtype(u64),
    Tuple(i32, i32),
    Struct { name: String, count: u32 },
}

// ---------------------------------------------------------------------------
// Helper macro
// ---------------------------------------------------------------------------

macro_rules! assert_bytes_eq {
    ($oxi_bytes:expr, $bin_bytes:expr, $label:expr) => {
        assert_eq!(
            $oxi_bytes, $bin_bytes,
            "Byte mismatch for {}: oxi={:?} bin={:?}",
            $label, $oxi_bytes, $bin_bytes
        );
    };
}

// ---------------------------------------------------------------------------
// 1. All primitive integer types
// ---------------------------------------------------------------------------

#[test]
fn test_compat_all_integer_types() {
    let bin_std = bincode::config::standard();

    macro_rules! check {
        ($ty:ty, $val:expr) => {{
            let v: $ty = $val;
            let oxi = oxicode::encode_to_vec(&v).expect(concat!("oxi encode ", stringify!($ty)));
            let bin =
                bincode::encode_to_vec(&v, bin_std).expect(concat!("bin encode ", stringify!($ty)));
            assert_bytes_eq!(oxi, bin, concat!(stringify!($ty), "=", stringify!($val)));
        }};
    }

    check!(u8, 0u8);
    check!(u8, 127u8);
    check!(u8, 255u8);
    check!(u16, 0u16);
    check!(u16, 250u16);
    check!(u16, 251u16);
    check!(u16, 1000u16);
    check!(u16, u16::MAX);
    check!(u32, 0u32);
    check!(u32, 250u32);
    check!(u32, 251u32);
    check!(u32, 65535u32);
    check!(u32, 65536u32);
    check!(u32, u32::MAX);
    check!(u64, 0u64);
    check!(u64, 250u64);
    check!(u64, u64::MAX / 2);
    check!(u64, u64::MAX);
    check!(u128, 0u128);
    check!(u128, u128::MAX);
    check!(i8, 0i8);
    check!(i8, -1i8);
    check!(i8, i8::MIN);
    check!(i8, i8::MAX);
    check!(i16, 0i16);
    check!(i16, -500i16);
    check!(i16, i16::MIN);
    check!(i16, i16::MAX);
    check!(i32, 0i32);
    check!(i32, -12345i32);
    check!(i32, i32::MIN);
    check!(i32, i32::MAX);
    check!(i64, 0i64);
    check!(i64, -1i64);
    check!(i64, i64::MIN);
    check!(i64, i64::MAX);
    check!(i128, 0i128);
    check!(i128, i128::MIN);
    check!(i128, i128::MAX);
    check!(bool, true);
    check!(bool, false);
}

#[test]
fn test_compat_float_types() {
    let bin_std = bincode::config::standard();

    let f32_vals: &[f32] = &[
        0.0,
        1.0,
        -1.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::MAX,
        f32::MIN_POSITIVE,
    ];
    for &v in f32_vals {
        let oxi = oxicode::encode_to_vec(&v).expect("oxi f32");
        let bin = bincode::encode_to_vec(v, bin_std).expect("bin f32");
        assert_bytes_eq!(oxi, bin, format!("f32={v}"));
    }

    let f64_vals: &[f64] = &[
        0.0,
        1.0,
        -1.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MAX,
        f64::MIN_POSITIVE,
    ];
    for &v in f64_vals {
        let oxi = oxicode::encode_to_vec(&v).expect("oxi f64");
        let bin = bincode::encode_to_vec(v, bin_std).expect("bin f64");
        assert_bytes_eq!(oxi, bin, format!("f64={v}"));
    }

    // NaN: bytes must match even though NaN != NaN
    let nan_f32 = f32::NAN;
    let oxi_nan32 = oxicode::encode_to_vec(&nan_f32).expect("oxi f32 NaN");
    let bin_nan32 = bincode::encode_to_vec(nan_f32, bin_std).expect("bin f32 NaN");
    assert_bytes_eq!(oxi_nan32, bin_nan32, "f32::NAN bytes");

    let nan_f64 = f64::NAN;
    let oxi_nan64 = oxicode::encode_to_vec(&nan_f64).expect("oxi f64 NaN");
    let bin_nan64 = bincode::encode_to_vec(nan_f64, bin_std).expect("bin f64 NaN");
    assert_bytes_eq!(oxi_nan64, bin_nan64, "f64::NAN bytes");
}

// ---------------------------------------------------------------------------
// 2. String variants
// ---------------------------------------------------------------------------

#[test]
fn test_compat_strings() {
    let bin_std = bincode::config::standard();

    let strings: &[&str] = &[
        "",
        "hello",
        "Hello, World! 🌍",
        "中文测试",
        "عربي",
        "日本語テスト",
        "line1\nline2\ttabbed",
    ];

    for &s in strings {
        let oxi = oxicode::encode_to_vec(&s).expect("oxi str");
        let bin = bincode::encode_to_vec(s, bin_std).expect("bin str");
        assert_bytes_eq!(oxi, bin, format!("str={s:?}"));

        // cross-decode
        let (decoded, _): (String, _) =
            oxicode::decode_from_slice(&bin).expect("oxi decode bin str");
        assert_eq!(s, decoded.as_str());
    }
}

#[test]
fn test_compat_large_string() {
    let bin_std = bincode::config::standard();
    let large: String = "x".repeat(100_000);
    let oxi = oxicode::encode_to_vec(&large).expect("oxi large str");
    let bin = bincode::encode_to_vec(&large, bin_std).expect("bin large str");
    assert_bytes_eq!(oxi, bin, "large string 100k");
}

// ---------------------------------------------------------------------------
// 3. Collection types
// ---------------------------------------------------------------------------

#[test]
fn test_compat_vec_u8() {
    let bin_std = bincode::config::standard();
    let cases: &[Vec<u8>] = &[vec![], vec![0], vec![1, 2, 3], vec![255; 100]];
    for v in cases {
        let oxi = oxicode::encode_to_vec(v).expect("oxi vec<u8>");
        let bin = bincode::encode_to_vec(v, bin_std).expect("bin vec<u8>");
        assert_bytes_eq!(oxi, bin, format!("Vec<u8> len={}", v.len()));
    }
}

#[test]
fn test_compat_vec_i32() {
    let bin_std = bincode::config::standard();
    let v: Vec<i32> = vec![-1000, 0, 1, 1000, i32::MIN, i32::MAX];
    let oxi = oxicode::encode_to_vec(&v).expect("oxi vec<i32>");
    let bin = bincode::encode_to_vec(&v, bin_std).expect("bin vec<i32>");
    assert_bytes_eq!(oxi, bin, "Vec<i32>");
}

#[test]
fn test_compat_vec_string() {
    let bin_std = bincode::config::standard();
    let v: Vec<String> = vec!["alpha".into(), "beta".into(), "gamma δ".into()];
    let oxi = oxicode::encode_to_vec(&v).expect("oxi vec<String>");
    let bin = bincode::encode_to_vec(&v, bin_std).expect("bin vec<String>");
    assert_bytes_eq!(oxi, bin, "Vec<String>");
}

#[test]
fn test_compat_vec_vec_u8() {
    let bin_std = bincode::config::standard();
    let v: Vec<Vec<u8>> = vec![vec![1, 2], vec![], vec![3, 4, 5]];
    let oxi = oxicode::encode_to_vec(&v).expect("oxi vec<vec<u8>>");
    let bin = bincode::encode_to_vec(&v, bin_std).expect("bin vec<vec<u8>>");
    assert_bytes_eq!(oxi, bin, "Vec<Vec<u8>>");
}

#[test]
fn test_compat_large_vec() {
    let bin_std = bincode::config::standard();
    let v: Vec<u32> = (0u32..10_000).collect();
    let oxi = oxicode::encode_to_vec(&v).expect("oxi large vec");
    let bin = bincode::encode_to_vec(&v, bin_std).expect("bin large vec");
    assert_bytes_eq!(oxi, bin, "Vec<u32> 10k");
}

#[test]
fn test_compat_btreemap() {
    let bin_std = bincode::config::standard();
    let mut m: BTreeMap<String, u32> = BTreeMap::new();
    m.insert("alpha".into(), 1);
    m.insert("beta".into(), 2);
    m.insert("gamma".into(), 3);
    let oxi = oxicode::encode_to_vec(&m).expect("oxi btreemap");
    let bin = bincode::encode_to_vec(&m, bin_std).expect("bin btreemap");
    assert_bytes_eq!(oxi, bin, "BTreeMap<String,u32>");

    // cross-decode
    let (decoded, _): (BTreeMap<String, u32>, _) =
        oxicode::decode_from_slice(&bin).expect("oxi decode bin btreemap");
    assert_eq!(m, decoded);
}

#[test]
fn test_compat_btreeset() {
    let bin_std = bincode::config::standard();
    let s: BTreeSet<u32> = [10u32, 20, 30, 40, 50].into_iter().collect();
    let oxi = oxicode::encode_to_vec(&s).expect("oxi btreeset");
    let bin = bincode::encode_to_vec(&s, bin_std).expect("bin btreeset");
    assert_bytes_eq!(oxi, bin, "BTreeSet<u32>");
}

#[test]
fn test_compat_vecdeque() {
    let bin_std = bincode::config::standard();
    let d: VecDeque<u32> = [1u32, 2, 3, 4, 5].into_iter().collect();
    let oxi = oxicode::encode_to_vec(&d).expect("oxi vecdeque");
    let bin = bincode::encode_to_vec(&d, bin_std).expect("bin vecdeque");
    assert_bytes_eq!(oxi, bin, "VecDeque<u32>");
}

#[test]
fn test_compat_linked_list() {
    // bincode v2 does not implement Encode for LinkedList; we verify oxicode
    // roundtrips correctly and produces sane output.
    let ll: LinkedList<u32> = [1u32, 2, 3].into_iter().collect();
    let oxi = oxicode::encode_to_vec(&ll).expect("oxi linkedlist encode");
    let (decoded, _): (LinkedList<u32>, _) =
        oxicode::decode_from_slice(&oxi).expect("oxi linkedlist decode");
    assert_eq!(ll, decoded, "LinkedList<u32> roundtrip");
}

// ---------------------------------------------------------------------------
// 4. Nested / complex types
// ---------------------------------------------------------------------------

#[test]
fn test_compat_nested_struct() {
    let bin_std = bincode::config::standard();
    let val = SharedOuter {
        inner: SharedInner { x: -99, y: 42 },
        data: vec![0xDE, 0xAD, 0xBE, 0xEF],
        label: "nested".into(),
    };
    let oxi = oxicode::encode_to_vec(&val).expect("oxi nested struct");
    let bin = bincode::encode_to_vec(&val, bin_std).expect("bin nested struct");
    assert_bytes_eq!(oxi, bin, "SharedOuter");

    // cross-decode
    let (decoded, _): (SharedOuter, _) =
        oxicode::decode_from_slice(&bin).expect("oxi decode nested");
    assert_eq!(val, decoded);
}

#[test]
fn test_compat_deeply_nested_vec() {
    let bin_std = bincode::config::standard();
    let val: Vec<Vec<Vec<u32>>> = vec![
        vec![vec![1, 2], vec![3]],
        vec![vec![]],
        vec![vec![4, 5, 6], vec![7, 8]],
    ];
    let oxi = oxicode::encode_to_vec(&val).expect("oxi deep vec");
    let bin = bincode::encode_to_vec(&val, bin_std).expect("bin deep vec");
    assert_bytes_eq!(oxi, bin, "Vec<Vec<Vec<u32>>>");
}

#[test]
fn test_compat_option_nested() {
    let bin_std = bincode::config::standard();
    let val: Option<Vec<Option<i64>>> = Some(vec![Some(1), None, Some(-100)]);
    let oxi = oxicode::encode_to_vec(&val).expect("oxi option nested");
    let bin = bincode::encode_to_vec(&val, bin_std).expect("bin option nested");
    assert_bytes_eq!(oxi, bin, "Option<Vec<Option<i64>>>");

    let val2: Option<Vec<Option<i64>>> = None;
    let oxi2 = oxicode::encode_to_vec(&val2).expect("oxi option none");
    let bin2 = bincode::encode_to_vec(&val2, bin_std).expect("bin option none");
    assert_bytes_eq!(oxi2, bin2, "Option<Vec<Option<i64>>> None");
}

// ---------------------------------------------------------------------------
// 5. Enum variants comprehensive
// ---------------------------------------------------------------------------

#[test]
fn test_compat_enum_all_variants() {
    let bin_std = bincode::config::standard();

    let variants = &[
        SharedEnum::Unit,
        SharedEnum::Newtype(u64::MAX),
        SharedEnum::Tuple(-99, 42),
        SharedEnum::Struct {
            name: "hello".into(),
            count: 7,
        },
    ];

    for variant in variants {
        let oxi = oxicode::encode_to_vec(variant).expect("oxi enum");
        let bin = bincode::encode_to_vec(variant, bin_std).expect("bin enum");
        assert_bytes_eq!(oxi, bin, format!("SharedEnum::{variant:?}"));
    }
}

// ---------------------------------------------------------------------------
// 6. Cross-decode with shared derive types
// ---------------------------------------------------------------------------

#[test]
fn test_cross_decode_simple_struct() {
    let orig = SharedSimple {
        id: 42,
        name: "cross-test".into(),
        flag: true,
    };

    // encode with oxicode, decode with bincode
    let oxi_bytes = oxicode::encode_to_vec(&orig).expect("oxi encode");
    let (bin_decoded, _): (SharedSimple, _) =
        bincode::decode_from_slice(&oxi_bytes, bincode::config::standard())
            .expect("bin decode oxi bytes");
    assert_eq!(orig, bin_decoded);

    // encode with bincode, decode with oxicode
    let bin_bytes = bincode::encode_to_vec(&orig, bincode::config::standard()).expect("bin encode");
    let (oxi_decoded, _): (SharedSimple, _) =
        oxicode::decode_from_slice(&bin_bytes).expect("oxi decode bin bytes");
    assert_eq!(orig, oxi_decoded);
}

#[test]
fn test_cross_decode_rich_struct() {
    let orig = SharedWithVecF64 {
        id: 999,
        name: "rich struct 🦀".into(),
        values: vec![1.0, 2.5, -3.5, f64::MAX],
        flag: false,
    };

    let oxi_bytes = oxicode::encode_to_vec(&orig).expect("oxi encode");
    let (bin_decoded, _): (SharedWithVecF64, _) =
        bincode::decode_from_slice(&oxi_bytes, bincode::config::standard()).expect("bin decode");
    assert_eq!(orig, bin_decoded);

    let bin_bytes = bincode::encode_to_vec(&orig, bincode::config::standard()).expect("bin encode");
    let (oxi_decoded, _): (SharedWithVecF64, _) =
        oxicode::decode_from_slice(&bin_bytes).expect("oxi decode");
    assert_eq!(orig, oxi_decoded);
}

#[test]
fn test_cross_decode_enum() {
    let variants = &[
        SharedEnum::Unit,
        SharedEnum::Newtype(12345),
        SharedEnum::Tuple(0, -1),
        SharedEnum::Struct {
            name: "test".into(),
            count: 0,
        },
    ];

    for variant in variants {
        // oxi -> bin
        let oxi_bytes = oxicode::encode_to_vec(variant).expect("oxi encode enum");
        let (bin_decoded, _): (SharedEnum, _) =
            bincode::decode_from_slice(&oxi_bytes, bincode::config::standard())
                .expect("bin decode enum");
        assert_eq!(variant, &bin_decoded);

        // bin -> oxi
        let bin_bytes =
            bincode::encode_to_vec(variant, bincode::config::standard()).expect("bin encode enum");
        let (oxi_decoded, _): (SharedEnum, _) =
            oxicode::decode_from_slice(&bin_bytes).expect("oxi decode enum");
        assert_eq!(variant, &oxi_decoded);
    }
}

// ---------------------------------------------------------------------------
// 7. Config compatibility matrix
// ---------------------------------------------------------------------------

#[test]
fn test_config_standard_matches() {
    let val = 12345u32;
    let oxi = oxicode::encode_to_vec_with_config(&val, oxicode::config::standard())
        .expect("oxi standard");
    let bin = bincode::encode_to_vec(val, bincode::config::standard()).expect("bin standard");
    assert_bytes_eq!(oxi, bin, "standard config u32");
}

#[test]
fn test_config_legacy_matches() {
    let val = 99999u32;
    let oxi =
        oxicode::encode_to_vec_with_config(&val, oxicode::config::legacy()).expect("oxi legacy");
    let bin = bincode::encode_to_vec(val, bincode::config::legacy()).expect("bin legacy");
    assert_bytes_eq!(oxi, bin, "legacy config u32");
}

#[test]
fn test_config_big_endian_matches() {
    let val = 0xDEADBEEFu32;
    let oxi =
        oxicode::encode_to_vec_with_config(&val, oxicode::config::standard().with_big_endian())
            .expect("oxi big-endian");
    let bin = bincode::encode_to_vec(val, bincode::config::standard().with_big_endian())
        .expect("bin big-endian");
    assert_bytes_eq!(oxi, bin, "big-endian config u32");
}

#[test]
fn test_config_fixed_int_matches() {
    let val = 42u32;
    let oxi = oxicode::encode_to_vec_with_config(
        &val,
        oxicode::config::standard().with_fixed_int_encoding(),
    )
    .expect("oxi fixed-int");
    let bin = bincode::encode_to_vec(val, bincode::config::standard().with_fixed_int_encoding())
        .expect("bin fixed-int");
    assert_bytes_eq!(oxi, bin, "fixed-int config u32");
}

#[test]
fn test_config_legacy_big_endian_matches() {
    let val = 0xABCDu16;
    let oxi = oxicode::encode_to_vec_with_config(&val, oxicode::config::legacy().with_big_endian())
        .expect("oxi legacy big-endian");
    let bin = bincode::encode_to_vec(val, bincode::config::legacy().with_big_endian())
        .expect("bin legacy big-endian");
    assert_bytes_eq!(oxi, bin, "legacy big-endian config u16");
}

#[test]
fn test_config_matrix_various_types() {
    // Exercise the matrix with a struct across multiple configs
    let val = SharedSimple {
        id: 7,
        name: "cfg".into(),
        flag: true,
    };

    let oxi_std = oxicode::encode_to_vec_with_config(&val, oxicode::config::standard())
        .expect("oxi std struct");
    let bin_std =
        bincode::encode_to_vec(&val, bincode::config::standard()).expect("bin std struct");
    assert_bytes_eq!(oxi_std, bin_std, "standard config struct");

    let oxi_fix = oxicode::encode_to_vec_with_config(
        &val,
        oxicode::config::standard().with_fixed_int_encoding(),
    )
    .expect("oxi fix struct");
    let bin_fix = bincode::encode_to_vec(
        val.clone(),
        bincode::config::standard().with_fixed_int_encoding(),
    )
    .expect("bin fix struct");
    assert_bytes_eq!(oxi_fix, bin_fix, "fixed-int config struct");
}

// ---------------------------------------------------------------------------
// 8. Byte-level verification (exact expected bytes)
// ---------------------------------------------------------------------------

#[test]
fn test_byte_level_u32_42_varint() {
    // 42 < 251, so standard varint encodes as single byte [42]
    let oxi = oxicode::encode_to_vec(&42u32).expect("oxi u32 42");
    assert_eq!(oxi, vec![42u8], "42u32 should encode as single byte [42]");
    let bin = bincode::encode_to_vec(42u32, bincode::config::standard()).expect("bin u32 42");
    assert_eq!(bin, vec![42u8]);
    assert_bytes_eq!(oxi, bin, "u32=42 byte-level");
}

#[test]
fn test_byte_level_bool() {
    let oxi_true = oxicode::encode_to_vec(&true).expect("oxi true");
    let oxi_false = oxicode::encode_to_vec(&false).expect("oxi false");
    assert_eq!(oxi_true, vec![1u8], "true should be [1]");
    assert_eq!(oxi_false, vec![0u8], "false should be [0]");

    let bin_true = bincode::encode_to_vec(true, bincode::config::standard()).expect("bin true");
    let bin_false = bincode::encode_to_vec(false, bincode::config::standard()).expect("bin false");
    assert_bytes_eq!(oxi_true, bin_true, "bool true");
    assert_bytes_eq!(oxi_false, bin_false, "bool false");
}

#[test]
fn test_byte_level_empty_string() {
    // empty string: varint length=0 → [0]
    let s: &str = "";
    let oxi = oxicode::encode_to_vec(&s.to_string()).expect("oxi empty str");
    assert_eq!(oxi, vec![0u8], "empty string should be [0]");
    let bin = bincode::encode_to_vec(s, bincode::config::standard()).expect("bin empty str");
    assert_bytes_eq!(oxi, bin, "empty string byte-level");
}

#[test]
fn test_byte_level_short_string() {
    // "hi" → varint(2) + b'h' + b'i' = [2, 104, 105]
    let s: &str = "hi";
    let oxi = oxicode::encode_to_vec(&s.to_string()).expect("oxi 'hi'");
    assert_eq!(oxi, vec![2u8, b'h', b'i'], "'hi' should be [2, 'h', 'i']");
    let bin = bincode::encode_to_vec(s, bincode::config::standard()).expect("bin 'hi'");
    assert_bytes_eq!(oxi, bin, "'hi' byte-level");
}

#[test]
fn test_byte_level_u32_large_varint() {
    // 1000u32: > 250, so bincode/oxicode standard uses 251 prefix + u16 LE
    // expected: [251, 0xE8, 0x03] (251 = u16 marker, 1000 = 0x03E8 LE → [0xE8, 0x03])
    let oxi = oxicode::encode_to_vec(&1000u32).expect("oxi u32 1000");
    let bin = bincode::encode_to_vec(1000u32, bincode::config::standard()).expect("bin u32 1000");
    assert_bytes_eq!(oxi, bin, "u32=1000 varint byte-level");
}

#[test]
fn test_byte_level_none_option() {
    let val: Option<u32> = None;
    let oxi = oxicode::encode_to_vec(&val).expect("oxi None");
    let bin = bincode::encode_to_vec(val, bincode::config::standard()).expect("bin None");
    // None encodes as [0]
    assert_eq!(oxi, vec![0u8], "Option::None should encode as [0]");
    assert_bytes_eq!(oxi, bin, "Option::None byte-level");
}

// ---------------------------------------------------------------------------
// 9. Roundtrip with std::io (encode_into_std_write / decode_from_std_read)
// ---------------------------------------------------------------------------

#[test]
fn test_std_io_roundtrip_simple() {
    let orig = SharedSimple {
        id: 100,
        name: "io-test".into(),
        flag: false,
    };
    let cfg = oxicode::config::standard();

    let mut buf: Vec<u8> = Vec::new();
    oxicode::encode_into_std_write(orig.clone(), &mut buf, cfg).expect("encode_into_std_write");

    let cursor = Cursor::new(&buf);
    let decoded: SharedSimple =
        oxicode::decode_from_std_read(cursor, cfg).expect("decode_from_std_read");
    assert_eq!(orig, decoded);

    // compare bytes with bincode
    let bin_bytes = bincode::encode_to_vec(&orig, bincode::config::standard()).expect("bin encode");
    assert_bytes_eq!(buf, bin_bytes, "std_io roundtrip bytes");
}

#[test]
fn test_std_io_roundtrip_vec() {
    let orig: Vec<i64> = vec![-1, 0, 1, i64::MIN, i64::MAX];
    let cfg = oxicode::config::standard();

    let mut buf: Vec<u8> = Vec::new();
    oxicode::encode_into_std_write(orig.clone(), &mut buf, cfg).expect("encode vec into std write");

    let cursor = Cursor::new(&buf);
    let decoded: Vec<i64> =
        oxicode::decode_from_std_read(cursor, cfg).expect("decode vec from std read");
    assert_eq!(orig, decoded);

    // compare with bincode
    let bin_bytes =
        bincode::encode_to_vec(&orig, bincode::config::standard()).expect("bin encode vec");
    assert_bytes_eq!(buf, bin_bytes, "std_io vec roundtrip bytes");
}

#[test]
fn test_std_io_roundtrip_with_legacy_config() {
    let orig = SharedOuter {
        inner: SharedInner { x: 10, y: -10 },
        data: vec![0xCA, 0xFE],
        label: "legacy-io".into(),
    };
    let cfg = oxicode::config::legacy();

    let mut buf: Vec<u8> = Vec::new();
    oxicode::encode_into_std_write(orig.clone(), &mut buf, cfg)
        .expect("encode legacy into std write");

    let cursor = Cursor::new(&buf);
    let decoded: SharedOuter =
        oxicode::decode_from_std_read(cursor, cfg).expect("decode legacy from std read");
    assert_eq!(orig, decoded);

    // compare with bincode legacy
    let bin_bytes =
        bincode::encode_to_vec(&orig, bincode::config::legacy()).expect("bin legacy encode");
    assert_bytes_eq!(buf, bin_bytes, "std_io legacy roundtrip bytes");
}

// ---------------------------------------------------------------------------
// 10. Edge cases
// ---------------------------------------------------------------------------

#[test]
fn test_edge_case_u128_extremes() {
    let bin_std = bincode::config::standard();
    for val in [0u128, u128::MAX, u128::MAX / 2] {
        let oxi = oxicode::encode_to_vec(&val).expect("oxi u128");
        let bin = bincode::encode_to_vec(val, bin_std).expect("bin u128");
        assert_bytes_eq!(oxi, bin, format!("u128={val}"));
    }
}

#[test]
fn test_edge_case_i128_extremes() {
    let bin_std = bincode::config::standard();
    for val in [0i128, i128::MIN, i128::MAX, -1i128, 1i128] {
        let oxi = oxicode::encode_to_vec(&val).expect("oxi i128");
        let bin = bincode::encode_to_vec(val, bin_std).expect("bin i128");
        assert_bytes_eq!(oxi, bin, format!("i128={val}"));
    }
}

#[test]
fn test_edge_case_empty_collections() {
    let bin_std = bincode::config::standard();

    let empty_vec: Vec<u32> = vec![];
    let oxi = oxicode::encode_to_vec(&empty_vec).expect("oxi empty vec");
    let bin = bincode::encode_to_vec(&empty_vec, bin_std).expect("bin empty vec");
    assert_bytes_eq!(oxi, bin, "empty Vec<u32>");

    let empty_map: BTreeMap<String, u32> = BTreeMap::new();
    let oxi_m = oxicode::encode_to_vec(&empty_map).expect("oxi empty map");
    let bin_m = bincode::encode_to_vec(&empty_map, bin_std).expect("bin empty map");
    assert_bytes_eq!(oxi_m, bin_m, "empty BTreeMap");

    let empty_set: BTreeSet<u32> = BTreeSet::new();
    let oxi_s = oxicode::encode_to_vec(&empty_set).expect("oxi empty set");
    let bin_s = bincode::encode_to_vec(&empty_set, bin_std).expect("bin empty set");
    assert_bytes_eq!(oxi_s, bin_s, "empty BTreeSet");
}

#[test]
fn test_edge_case_f32_special_values() {
    let bin_std = bincode::config::standard();
    // Byte equality for special float values
    for val in [
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
    ] {
        let oxi = oxicode::encode_to_vec(&val).expect("oxi f32 special");
        let bin = bincode::encode_to_vec(val, bin_std).expect("bin f32 special");
        assert_bytes_eq!(oxi, bin, format!("f32 special={val}"));
    }
}

#[test]
fn test_edge_case_f64_special_values() {
    let bin_std = bincode::config::standard();
    for val in [
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
    ] {
        let oxi = oxicode::encode_to_vec(&val).expect("oxi f64 special");
        let bin = bincode::encode_to_vec(val, bin_std).expect("bin f64 special");
        assert_bytes_eq!(oxi, bin, format!("f64 special={val}"));
    }
}

#[test]
fn test_edge_case_zero_length_array() {
    let bin_std = bincode::config::standard();
    let arr: [u32; 0] = [];
    let oxi = oxicode::encode_to_vec(&arr).expect("oxi [u32;0]");
    let bin = bincode::encode_to_vec(arr, bin_std).expect("bin [u32;0]");
    assert_bytes_eq!(oxi, bin, "[u32;0]");
}

#[test]
fn test_edge_case_fixed_size_arrays() {
    let bin_std = bincode::config::standard();

    let arr4: [u32; 4] = [1, 2, 3, 4];
    let oxi4 = oxicode::encode_to_vec(&arr4).expect("oxi [u32;4]");
    let bin4 = bincode::encode_to_vec(arr4, bin_std).expect("bin [u32;4]");
    assert_bytes_eq!(oxi4, bin4, "[u32;4]");

    let arr8: [u8; 8] = [0xFF; 8];
    let oxi8 = oxicode::encode_to_vec(&arr8).expect("oxi [u8;8]");
    let bin8 = bincode::encode_to_vec(arr8, bin_std).expect("bin [u8;8]");
    assert_bytes_eq!(oxi8, bin8, "[u8;8]");
}

#[test]
fn test_edge_case_deeply_nested_option() {
    let bin_std = bincode::config::standard();
    let val: Option<Option<Option<u32>>> = Some(Some(Some(42)));
    let oxi = oxicode::encode_to_vec(&val).expect("oxi triple option some");
    let bin = bincode::encode_to_vec(val, bin_std).expect("bin triple option some");
    assert_bytes_eq!(oxi, bin, "Option<Option<Option<u32>>> Some");

    let val_none: Option<Option<Option<u32>>> = Some(None);
    let oxi_none = oxicode::encode_to_vec(&val_none).expect("oxi triple option inner none");
    let bin_none = bincode::encode_to_vec(val_none, bin_std).expect("bin triple option inner none");
    assert_bytes_eq!(oxi_none, bin_none, "Option<Option<Option<u32>>> Some(None)");
}

#[test]
fn test_edge_case_large_btreemap() {
    let bin_std = bincode::config::standard();
    let m: BTreeMap<u32, u64> = (0u32..500).map(|i| (i, i as u64 * 1000)).collect();
    let oxi = oxicode::encode_to_vec(&m).expect("oxi large btreemap");
    let bin = bincode::encode_to_vec(&m, bin_std).expect("bin large btreemap");
    assert_bytes_eq!(oxi, bin, "large BTreeMap<u32,u64> 500 entries");
}
