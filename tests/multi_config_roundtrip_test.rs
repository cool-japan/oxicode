//! Multi-config roundtrip tests for oxicode.
//!
//! 22 tests verifying roundtrip correctness across multiple config combinations
//! using structs, enums, primitives, collections, and nested types.
//! Each test uses `encode_to_vec_with_config` / `decode_from_slice_with_config`.

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
use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config, Decode, Encode};
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Shared type definitions
// ---------------------------------------------------------------------------

/// A complex struct covering multiple field types.
#[derive(Debug, PartialEq, Encode, Decode)]
struct ComplexStruct {
    id: u32,
    name: String,
    tags: Vec<u8>,
    active: bool,
}

/// Struct with only u32 fields — useful for fixed-int size assertions.
#[derive(Debug, PartialEq, Encode, Decode)]
struct U32Fields {
    a: u32,
    b: u32,
    c: u32,
}

/// Struct covering every primitive integer type.
#[derive(Debug, PartialEq, Encode, Decode)]
struct AllInts {
    u8_val: u8,
    u16_val: u16,
    u32_val: u32,
    u64_val: u64,
    i8_val: i8,
    i16_val: i16,
    i32_val: i32,
    i64_val: i64,
}

/// Struct holding f32 and f64.
#[derive(Debug, PartialEq, Encode, Decode)]
struct Floats {
    f32_val: f32,
    f64_val: f64,
}

/// Nested struct: outer contains an inner value.
#[derive(Debug, PartialEq, Encode, Decode)]
struct Inner {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Outer {
    inner: Inner,
    label: String,
}

/// Simple enum for roundtrip tests.
#[derive(Debug, PartialEq, Encode, Decode)]
enum Command {
    Quit,
    Move { dx: i32, dy: i32 },
    Print(String),
}

/// Struct that wraps an Option<u32>.
#[derive(Debug, PartialEq, Encode, Decode)]
struct MaybeU32 {
    value: Option<u32>,
}

// ---------------------------------------------------------------------------
// Test 1 — Standard config roundtrip of a complex struct
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_01_standard_complex_struct() {
    let cfg = config::standard();
    let original = ComplexStruct {
        id: 42,
        name: String::from("oxicode"),
        tags: vec![1, 2, 3, 255],
        active: true,
    };
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_01: encode");
    let (decoded, consumed): (ComplexStruct, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_01: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 2 — Fixed-int encoding roundtrip of struct with u32 fields
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_02_fixed_int_u32_struct() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = U32Fields {
        a: 0,
        b: 65535,
        c: u32::MAX,
    };
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_02: encode");
    // Three u32 fields at 4 bytes each = 12 bytes minimum (no length prefixes needed)
    assert!(
        bytes.len() >= 12,
        "test_roundtrip_02: expected at least 12 bytes, got {}",
        bytes.len()
    );
    let (decoded, consumed): (U32Fields, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_02: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 3 — Big-endian fixed-int roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_03_big_endian_fixed_int() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let original = U32Fields {
        a: 0x0102_0304,
        b: 0xDEAD_BEEF,
        c: 1,
    };
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_03: encode");
    // First u32 = 0x01020304 must appear in big-endian byte order
    assert_eq!(
        &bytes[0..4],
        &[0x01, 0x02, 0x03, 0x04],
        "test_roundtrip_03: first u32 must be big-endian"
    );
    let (decoded, consumed): (U32Fields, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_03: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 4 — Little-endian fixed-int roundtrip (default endianness)
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_04_little_endian_fixed_int() {
    let cfg = config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();
    let original = U32Fields {
        a: 0x0102_0304,
        b: 0xDEAD_BEEF,
        c: 256,
    };
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_04: encode");
    // First u32 = 0x01020304 in little-endian: [0x04, 0x03, 0x02, 0x01]
    assert_eq!(
        &bytes[0..4],
        &[0x04, 0x03, 0x02, 0x01],
        "test_roundtrip_04: first u32 must be little-endian"
    );
    let (decoded, consumed): (U32Fields, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_04: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 5 — with_limit config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_05_with_limit_config() {
    let cfg = config::standard().with_limit::<512>();
    // with_limit::<512>() sets limit; encoding should succeed within limit
    let original = ComplexStruct {
        id: 7,
        name: String::from("limit-test"),
        tags: vec![10, 20, 30],
        active: false,
    };
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_05: encode");
    assert!(
        bytes.len() <= 512,
        "test_roundtrip_05: encoded size {} exceeds limit 512",
        bytes.len()
    );
    let (decoded, consumed): (ComplexStruct, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_05: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 6 — Standard config + large Vec
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_06_standard_large_vec() {
    let cfg = config::standard();
    let original: Vec<u32> = (0u32..1024).collect();
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_06: encode");
    let (decoded, consumed): (Vec<u32>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_06: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 7 — Fixed-int config with u64::MAX
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_07_fixed_int_u64_max() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: u64 = u64::MAX;
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_07: encode");
    assert_eq!(
        bytes.len(),
        8,
        "test_roundtrip_07: u64::MAX with fixed encoding must be 8 bytes"
    );
    // Little-endian u64::MAX = all 0xFF bytes
    assert_eq!(
        bytes,
        &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
        "test_roundtrip_07: u64::MAX in LE must be [FF;8]"
    );
    let (decoded, consumed): (u64, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_07: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 8);
}

// ---------------------------------------------------------------------------
// Test 8 — Fixed-int config write_fixed_array_length — fixed-int encodes array
//          length prefix as u64 (8 bytes) rather than varint (1 byte)
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_08_fixed_int_array_length_prefix() {
    let cfg_fixed = config::standard().with_fixed_int_encoding();
    let cfg_varint = config::standard().with_variable_int_encoding();

    // A Vec<u8> of 5 elements: payload = 5 bytes
    let original: Vec<u8> = vec![10, 20, 30, 40, 50];

    let fixed_bytes =
        encode_to_vec_with_config(&original, cfg_fixed).expect("test_roundtrip_08: fixed encode");
    let varint_bytes =
        encode_to_vec_with_config(&original, cfg_varint).expect("test_roundtrip_08: varint encode");

    // fixed-int: 8-byte length prefix + 5 bytes payload = 13 bytes
    assert_eq!(
        fixed_bytes.len(),
        13,
        "test_roundtrip_08: fixed-int Vec<u8>(5) must be 13 bytes"
    );
    // varint: 1-byte length prefix (5 < 251) + 5 bytes payload = 6 bytes
    assert_eq!(
        varint_bytes.len(),
        6,
        "test_roundtrip_08: varint Vec<u8>(5) must be 6 bytes"
    );

    let (decoded_fixed, _): (Vec<u8>, usize) =
        decode_from_slice_with_config(&fixed_bytes, cfg_fixed)
            .expect("test_roundtrip_08: fixed decode");
    assert_eq!(decoded_fixed, original);

    let (decoded_varint, _): (Vec<u8>, usize) =
        decode_from_slice_with_config(&varint_bytes, cfg_varint)
            .expect("test_roundtrip_08: varint decode");
    assert_eq!(decoded_varint, original);
}

// ---------------------------------------------------------------------------
// Test 9 — Standard config for all integer types in one struct
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_09_standard_all_integer_types() {
    let cfg = config::standard();
    let original = AllInts {
        u8_val: 200,
        u16_val: 50000,
        u32_val: 3_000_000_000,
        u64_val: 10_000_000_000,
        i8_val: -100,
        i16_val: -30000,
        i32_val: -2_000_000_000,
        i64_val: -5_000_000_000,
    };
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_09: encode");
    let (decoded, consumed): (AllInts, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_09: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 10 — Fixed-int config for all integer types
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_10_fixed_int_all_integer_types() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = AllInts {
        u8_val: 1,
        u16_val: 2,
        u32_val: 3,
        u64_val: 4,
        i8_val: -1,
        i16_val: -2,
        i32_val: -3,
        i64_val: -4,
    };
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_10: encode");
    // u8=1, u16=2, u32=4, u64=8, i8=1, i16=2, i32=4, i64=8 → total = 30 bytes
    assert_eq!(
        bytes.len(),
        30,
        "test_roundtrip_10: AllInts with fixed encoding must be 30 bytes"
    );
    let (decoded, consumed): (AllInts, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_10: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 30);
}

// ---------------------------------------------------------------------------
// Test 11 — Roundtrip with Vec<String> using standard config
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_11_vec_string_standard() {
    let cfg = config::standard();
    let original: Vec<String> = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma delta"),
        String::from(""),
    ];
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_11: encode");
    let (decoded, consumed): (Vec<String>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_11: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 12 — Roundtrip with BTreeMap<u32, String> standard config
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_12_btreemap_u32_string_standard() {
    let cfg = config::standard();
    let mut original: BTreeMap<u32, String> = BTreeMap::new();
    original.insert(1, String::from("one"));
    original.insert(10, String::from("ten"));
    original.insert(100, String::from("hundred"));
    original.insert(1000, String::from("thousand"));

    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_12: encode");
    let (decoded, consumed): (BTreeMap<u32, String>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_12: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 13 — Roundtrip with nested struct standard config
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_13_nested_struct_standard() {
    let cfg = config::standard();
    let original = Outer {
        inner: Inner { x: -10, y: 999 },
        label: String::from("nested-roundtrip"),
    };
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_13: encode");
    let (decoded, consumed): (Outer, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_13: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 14 — Roundtrip with enum standard config
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_14_enum_standard_config() {
    let cfg = config::standard();

    let variants: Vec<Command> = vec![
        Command::Quit,
        Command::Move { dx: 5, dy: -3 },
        Command::Print(String::from("hello from enum")),
    ];
    for variant in &variants {
        let bytes = encode_to_vec_with_config(variant, cfg).expect("test_roundtrip_14: encode");
        let (decoded, consumed): (Command, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_14: decode");
        assert_eq!(&decoded, variant);
        assert_eq!(consumed, bytes.len());
    }
}

// ---------------------------------------------------------------------------
// Test 15 — Fixed-int config with Option<u32>
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_15_fixed_int_option_u32() {
    let cfg = config::standard().with_fixed_int_encoding();

    let some_val = MaybeU32 { value: Some(123) };
    let bytes_some =
        encode_to_vec_with_config(&some_val, cfg).expect("test_roundtrip_15: encode Some");
    let (decoded_some, consumed_some): (MaybeU32, usize) =
        decode_from_slice_with_config(&bytes_some, cfg).expect("test_roundtrip_15: decode Some");
    assert_eq!(decoded_some, some_val);
    assert_eq!(consumed_some, bytes_some.len());

    let none_val = MaybeU32 { value: None };
    let bytes_none =
        encode_to_vec_with_config(&none_val, cfg).expect("test_roundtrip_15: encode None");
    let (decoded_none, consumed_none): (MaybeU32, usize) =
        decode_from_slice_with_config(&bytes_none, cfg).expect("test_roundtrip_15: decode None");
    assert_eq!(decoded_none, none_val);
    assert_eq!(consumed_none, bytes_none.len());

    // None must encode smaller than Some(123) (no payload for None)
    assert!(
        bytes_none.len() < bytes_some.len(),
        "test_roundtrip_15: None must encode to fewer bytes than Some(123)"
    );
}

// ---------------------------------------------------------------------------
// Test 16 — Standard config Vec<Option<u32>>
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_16_vec_option_u32_standard() {
    let cfg = config::standard();
    let original: Vec<Option<u32>> = vec![Some(1), None, Some(u32::MAX), None, Some(0)];
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_16: encode");
    let (decoded, consumed): (Vec<Option<u32>>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_16: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 17 — Config with all options set roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_17_all_options_combined() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding()
        .with_limit::<4096>();

    // with_limit::<4096>() sets limit; encoding should succeed within limit

    let original = AllInts {
        u8_val: 0xFF,
        u16_val: 0xABCD,
        u32_val: 0xDEAD_BEEF,
        u64_val: 0x0102_0304_0506_0708,
        i8_val: i8::MIN,
        i16_val: i16::MAX,
        i32_val: i32::MIN,
        i64_val: i64::MAX,
    };
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_17: encode");
    assert!(
        bytes.len() <= 4096,
        "test_roundtrip_17: encoded size {} must not exceed limit 4096",
        bytes.len()
    );
    // In big-endian: u8 then u16=0xABCD → bytes[1..3] = [AB, CD]
    assert_eq!(
        &bytes[1..3],
        &[0xAB, 0xCD],
        "test_roundtrip_17: u16=0xABCD must appear as [AB, CD] in big-endian"
    );
    let (decoded, consumed): (AllInts, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_17: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 18 — Standard config with f32/f64 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_18_standard_floats() {
    let cfg = config::standard();
    let original = Floats {
        f32_val: core::f32::consts::PI,
        f64_val: core::f64::consts::E,
    };
    let bytes = encode_to_vec_with_config(&original, cfg).expect("test_roundtrip_18: encode");
    // f32 = 4 bytes, f64 = 8 bytes → total 12 bytes (floats are always fixed-width)
    assert_eq!(
        bytes.len(),
        12,
        "test_roundtrip_18: Floats struct must encode to 12 bytes"
    );
    let (decoded, consumed): (Floats, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("test_roundtrip_18: decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 12);
}

// ---------------------------------------------------------------------------
// Test 19 — Fixed-int config with boolean values
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_19_fixed_int_booleans() {
    let cfg = config::standard().with_fixed_int_encoding();

    // Booleans are always 1 byte regardless of int encoding
    let true_val: bool = true;
    let false_val: bool = false;

    let bytes_true =
        encode_to_vec_with_config(&true_val, cfg).expect("test_roundtrip_19: encode true");
    let bytes_false =
        encode_to_vec_with_config(&false_val, cfg).expect("test_roundtrip_19: encode false");

    assert_eq!(
        bytes_true.len(),
        1,
        "test_roundtrip_19: bool must always be 1 byte"
    );
    assert_eq!(
        bytes_false.len(),
        1,
        "test_roundtrip_19: bool must always be 1 byte"
    );
    assert_ne!(
        bytes_true, bytes_false,
        "test_roundtrip_19: true and false must encode differently"
    );

    let (decoded_true, _): (bool, usize) =
        decode_from_slice_with_config(&bytes_true, cfg).expect("test_roundtrip_19: decode true");
    let (decoded_false, _): (bool, usize) =
        decode_from_slice_with_config(&bytes_false, cfg).expect("test_roundtrip_19: decode false");

    assert!(decoded_true, "test_roundtrip_19: decoded true must be true");
    assert!(
        !decoded_false,
        "test_roundtrip_19: decoded false must be false"
    );
}

// ---------------------------------------------------------------------------
// Test 20 — Encoded bytes differ between standard vs fixed-int config for u32 = 1
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_20_standard_vs_fixed_int_bytes_differ_u32_1() {
    let standard_cfg = config::standard().with_variable_int_encoding();
    let fixed_cfg = config::standard().with_fixed_int_encoding();
    let value: u32 = 1;

    let standard_bytes = encode_to_vec_with_config(&value, standard_cfg)
        .expect("test_roundtrip_20: standard encode");
    let fixed_bytes =
        encode_to_vec_with_config(&value, fixed_cfg).expect("test_roundtrip_20: fixed encode");

    // standard (varint): u32(1) = 1 byte; fixed: u32 = always 4 bytes
    assert_eq!(
        standard_bytes.len(),
        1,
        "test_roundtrip_20: varint u32(1) must be 1 byte"
    );
    assert_eq!(
        fixed_bytes.len(),
        4,
        "test_roundtrip_20: fixed u32(1) must be 4 bytes"
    );
    assert_ne!(
        standard_bytes, fixed_bytes,
        "test_roundtrip_20: standard and fixed-int configs must produce different bytes for u32=1"
    );

    // Both must still roundtrip correctly
    let (decoded_std, _): (u32, usize) =
        decode_from_slice_with_config(&standard_bytes, standard_cfg)
            .expect("test_roundtrip_20: standard decode");
    let (decoded_fixed, _): (u32, usize) = decode_from_slice_with_config(&fixed_bytes, fixed_cfg)
        .expect("test_roundtrip_20: fixed decode");

    assert_eq!(decoded_std, value);
    assert_eq!(decoded_fixed, value);
}

// ---------------------------------------------------------------------------
// Test 21 — Roundtrip with tuple types standard config
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_21_tuple_types_standard() {
    let cfg = config::standard();

    // 2-tuple
    let t2: (u32, String) = (99, String::from("hello"));
    let bytes2 = encode_to_vec_with_config(&t2, cfg).expect("test_roundtrip_21: encode t2");
    let (decoded2, consumed2): ((u32, String), usize) =
        decode_from_slice_with_config(&bytes2, cfg).expect("test_roundtrip_21: decode t2");
    assert_eq!(decoded2, t2);
    assert_eq!(consumed2, bytes2.len());

    // 3-tuple
    let t3: (bool, i64, f32) = (true, -987654321, std::f32::consts::PI);
    let bytes3 = encode_to_vec_with_config(&t3, cfg).expect("test_roundtrip_21: encode t3");
    let (decoded3, consumed3): ((bool, i64, f32), usize) =
        decode_from_slice_with_config(&bytes3, cfg).expect("test_roundtrip_21: decode t3");
    assert_eq!(decoded3, t3);
    assert_eq!(consumed3, bytes3.len());

    // 4-tuple with Option
    let t4: (u8, u16, u32, Option<u64>) = (1, 2, 3, Some(u64::MAX));
    let bytes4 = encode_to_vec_with_config(&t4, cfg).expect("test_roundtrip_21: encode t4");
    let (decoded4, consumed4): ((u8, u16, u32, Option<u64>), usize) =
        decode_from_slice_with_config(&bytes4, cfg).expect("test_roundtrip_21: decode t4");
    assert_eq!(decoded4, t4);
    assert_eq!(consumed4, bytes4.len());
}

// ---------------------------------------------------------------------------
// Test 22 — Nested collection roundtrip standard config
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_22_nested_collections_standard() {
    let cfg = config::standard();

    // Vec<Vec<u32>>: a 2-level nested vector
    let nested_vec: Vec<Vec<u32>> = vec![
        vec![1, 2, 3],
        vec![],
        vec![100, 200, 300, 400],
        vec![u32::MAX],
    ];
    let bytes_vv =
        encode_to_vec_with_config(&nested_vec, cfg).expect("test_roundtrip_22: encode Vec<Vec>");
    let (decoded_vv, consumed_vv): (Vec<Vec<u32>>, usize) =
        decode_from_slice_with_config(&bytes_vv, cfg).expect("test_roundtrip_22: decode Vec<Vec>");
    assert_eq!(decoded_vv, nested_vec);
    assert_eq!(consumed_vv, bytes_vv.len());

    // BTreeMap<String, Vec<u32>>: map values are collections
    let mut map_of_vecs: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    map_of_vecs.insert(String::from("a"), vec![1, 2]);
    map_of_vecs.insert(String::from("b"), vec![3, 4, 5]);
    map_of_vecs.insert(String::from("empty"), vec![]);

    let bytes_mv = encode_to_vec_with_config(&map_of_vecs, cfg)
        .expect("test_roundtrip_22: encode BTreeMap<String,Vec>");
    let (decoded_mv, consumed_mv): (BTreeMap<String, Vec<u32>>, usize) =
        decode_from_slice_with_config(&bytes_mv, cfg)
            .expect("test_roundtrip_22: decode BTreeMap<String,Vec>");
    assert_eq!(decoded_mv, map_of_vecs);
    assert_eq!(consumed_mv, bytes_mv.len());

    // Vec<Option<Vec<u8>>>: options wrapping inner vecs
    let opt_vecs: Vec<Option<Vec<u8>>> = vec![
        Some(vec![0xFF, 0x00]),
        None,
        Some(vec![]),
        Some(vec![1, 2, 3]),
    ];
    let bytes_ov = encode_to_vec_with_config(&opt_vecs, cfg)
        .expect("test_roundtrip_22: encode Vec<Option<Vec<u8>>>");
    let (decoded_ov, consumed_ov): (Vec<Option<Vec<u8>>>, usize) =
        decode_from_slice_with_config(&bytes_ov, cfg)
            .expect("test_roundtrip_22: decode Vec<Option<Vec<u8>>>");
    assert_eq!(decoded_ov, opt_vecs);
    assert_eq!(consumed_ov, bytes_ov.len());
}
