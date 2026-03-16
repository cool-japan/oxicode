//! Struct layout stability and wire format invariant tests.
//!
//! Verifies that the binary encoding of structs follows strict wire format rules:
//! fields are encoded in declaration order, names are not encoded, and the byte
//! layout is the exact concatenation of each field's individual encoding.

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
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ---------------------------------------------------------------------------
// Shared struct definitions used across multiple tests
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Single {
    value: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Double {
    a: u32,
    b: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Triple {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithString {
    id: u32,
    name: String,
}

// ---------------------------------------------------------------------------
// Test 1: Struct with 1 field encodes to same bytes as that field alone
// ---------------------------------------------------------------------------

#[test]
fn test_01_single_field_struct_bytes_equal_field_alone() {
    let s = Single { value: 77 };
    let struct_bytes = encode_to_vec(&s).expect("encode Single failed");
    let field_bytes = encode_to_vec(&77u32).expect("encode u32 alone failed");
    assert_eq!(
        struct_bytes, field_bytes,
        "Single {{ value: 77 }} must produce identical bytes to bare u32(77)"
    );
}

// ---------------------------------------------------------------------------
// Test 2: Struct with 2 fields: bytes == field1_bytes + field2_bytes
// ---------------------------------------------------------------------------

#[test]
fn test_02_two_field_struct_bytes_equal_fields_concatenated() {
    let s = Double { a: 10, b: 20 };
    let struct_bytes = encode_to_vec(&s).expect("encode Double failed");

    let bytes_a = encode_to_vec(&10u32).expect("encode field a");
    let bytes_b = encode_to_vec(&20u32).expect("encode field b");
    let mut expected = bytes_a;
    expected.extend_from_slice(&bytes_b);

    assert_eq!(
        struct_bytes, expected,
        "Double {{ a: 10, b: 20 }} must equal bytes(a) ++ bytes(b)"
    );
}

// ---------------------------------------------------------------------------
// Test 3: Struct with 3 fields: bytes == field1 + field2 + field3
// ---------------------------------------------------------------------------

#[test]
fn test_03_three_field_struct_bytes_equal_fields_concatenated() {
    let s = Triple { x: 1, y: 2, z: 3 };
    let struct_bytes = encode_to_vec(&s).expect("encode Triple failed");

    let bytes_x = encode_to_vec(&1u32).expect("encode field x");
    let bytes_y = encode_to_vec(&2u32).expect("encode field y");
    let bytes_z = encode_to_vec(&3u32).expect("encode field z");
    let mut expected = bytes_x;
    expected.extend_from_slice(&bytes_y);
    expected.extend_from_slice(&bytes_z);

    assert_eq!(
        struct_bytes, expected,
        "Triple {{ x:1, y:2, z:3 }} must equal bytes(x) ++ bytes(y) ++ bytes(z)"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Field order matters: struct {a, b} != struct {b, a} for different values
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct OrderAB {
    a: u32,
    b: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OrderBA {
    b: u32,
    a: u32,
}

#[test]
fn test_04_field_order_matters_different_values_different_bytes() {
    // Use different values so that order can be distinguished
    let ab = OrderAB { a: 1, b: 2 };
    let ba = OrderBA { b: 1, a: 2 };

    let bytes_ab = encode_to_vec(&ab).expect("encode OrderAB");
    let bytes_ba = encode_to_vec(&ba).expect("encode OrderBA");

    // To produce truly different bytes, we need different field-order structs with swapped values.
    // Use values where the varint encodings have different lengths (5 = 1 byte, 300 = 3 bytes):
    let ab_v = OrderAB { a: 5, b: 300 }; // encodes a=5 first, b=300 second
    let ba_v = OrderBA { b: 300, a: 5 }; // encodes b=300 first (declaration order), a=5 second

    let bytes_ab_v = encode_to_vec(&ab_v).expect("encode ab_v");
    let bytes_ba_v = encode_to_vec(&ba_v).expect("encode ba_v");

    assert_ne!(
        bytes_ab_v, bytes_ba_v,
        "struct{{a:5, b:300}} must differ from struct where b=300 is encoded first"
    );

    // Also verify ab with equal values produces same bytes as ba (names don't matter)
    let ab_eq = OrderAB { a: 7, b: 7 };
    let ba_eq = OrderBA { b: 7, a: 7 };
    assert_eq!(
        encode_to_vec(&ab_eq).expect("encode ab_eq"),
        encode_to_vec(&ba_eq).expect("encode ba_eq"),
        "when all field values are equal, field order doesn't change bytes"
    );
    drop(bytes_ab);
    drop(bytes_ba);
}

// ---------------------------------------------------------------------------
// Test 5: Changing a field value changes the encoded bytes
// ---------------------------------------------------------------------------

#[test]
fn test_05_changing_field_value_changes_encoded_bytes() {
    let original = Single { value: 100 };
    let modified = Single { value: 101 };

    let bytes_orig = encode_to_vec(&original).expect("encode original Single");
    let bytes_mod = encode_to_vec(&modified).expect("encode modified Single");

    assert_ne!(
        bytes_orig, bytes_mod,
        "changing field value from 100 to 101 must change encoded bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Adding a unit struct field (ZST) doesn't change wire format
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct UnitZst;

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithZst {
    value: u32,
    marker: UnitZst,
}

#[test]
fn test_06_unit_struct_zst_field_does_not_change_wire_format() {
    let with_zst = WithZst {
        value: 42,
        marker: UnitZst,
    };
    let without_zst = Single { value: 42 };

    let bytes_with = encode_to_vec(&with_zst).expect("encode WithZst");
    let bytes_without = encode_to_vec(&without_zst).expect("encode Single (without ZST)");

    // UnitZst encodes to 0 bytes, so WithZst and Single with same u32 value must match
    assert_eq!(
        bytes_with, bytes_without,
        "adding a ZST (unit struct) field must not change the wire format"
    );
}

// ---------------------------------------------------------------------------
// Test 7: Two structs with same layout but different names encode identically
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct StructFoo {
    x: u32,
    y: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct StructBar {
    p: u32,
    q: u64,
}

#[test]
fn test_07_same_layout_different_names_identical_bytes() {
    let foo = StructFoo { x: 55, y: 9999 };
    let bar = StructBar { p: 55, q: 9999 };

    let bytes_foo = encode_to_vec(&foo).expect("encode StructFoo");
    let bytes_bar = encode_to_vec(&bar).expect("encode StructBar");

    assert_eq!(
        bytes_foo, bytes_bar,
        "structs with same field types and values but different names must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Struct with String field: bytes == varint(len) + utf8_bytes (after id field)
// ---------------------------------------------------------------------------

#[test]
fn test_08_struct_string_field_bytes_equal_varint_len_plus_utf8() {
    let s = WithString {
        id: 0,
        name: "hello".to_string(),
    };
    let struct_bytes = encode_to_vec(&s).expect("encode WithString");

    // id=0 encodes as varint [0x00]; "hello" encodes as [0x05, h, e, l, l, o]
    let id_bytes = encode_to_vec(&0u32).expect("encode id");
    let name_bytes = encode_to_vec(&"hello".to_string()).expect("encode name string");

    let mut expected = id_bytes;
    expected.extend_from_slice(&name_bytes);

    assert_eq!(
        struct_bytes, expected,
        "WithString bytes must be bytes(id) ++ varint(len) ++ utf8(name)"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Struct with Vec<u8> field: bytes == varint(len) + elements
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithVecU8 {
    tag: u8,
    data: Vec<u8>,
}

#[test]
fn test_09_struct_vec_u8_field_bytes_equal_varint_len_plus_elements() {
    let s = WithVecU8 {
        tag: 1,
        data: vec![10u8, 20u8, 30u8],
    };
    let struct_bytes = encode_to_vec(&s).expect("encode WithVecU8");

    let tag_bytes = encode_to_vec(&1u8).expect("encode tag");
    let data_bytes = encode_to_vec(&vec![10u8, 20u8, 30u8]).expect("encode data vec");

    let mut expected = tag_bytes;
    expected.extend_from_slice(&data_bytes);

    assert_eq!(
        struct_bytes, expected,
        "WithVecU8 bytes must be bytes(tag) ++ varint(len) ++ element_bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Struct with bool field true: that byte is 1
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithBool {
    flag: bool,
}

#[test]
fn test_10_struct_bool_field_true_encodes_as_byte_one() {
    let s = WithBool { flag: true };
    let bytes = encode_to_vec(&s).expect("encode WithBool(true)");

    assert_eq!(
        bytes.len(),
        1,
        "struct with single bool field must be 1 byte"
    );
    assert_eq!(
        bytes[0], 0x01u8,
        "bool true in struct must encode as byte value 1"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Struct with bool field false: that byte is 0
// ---------------------------------------------------------------------------

#[test]
fn test_11_struct_bool_field_false_encodes_as_byte_zero() {
    let s = WithBool { flag: false };
    let bytes = encode_to_vec(&s).expect("encode WithBool(false)");

    assert_eq!(
        bytes.len(),
        1,
        "struct with single bool field must be 1 byte"
    );
    assert_eq!(
        bytes[0], 0x00u8,
        "bool false in struct must encode as byte value 0"
    );
}

// ---------------------------------------------------------------------------
// Test 12: Struct with Option<u32> None: that portion is just [0]
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithOptionU32 {
    opt: Option<u32>,
}

#[test]
fn test_12_struct_option_u32_none_encodes_as_single_zero_byte() {
    let s = WithOptionU32 { opt: None };
    let bytes = encode_to_vec(&s).expect("encode WithOptionU32(None)");

    assert_eq!(
        bytes,
        vec![0x00u8],
        "Option<u32> None in struct must encode as exactly [0x00]"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Struct with Option<u32> Some(42): that portion is [1, 42]
// ---------------------------------------------------------------------------

#[test]
fn test_13_struct_option_u32_some_42_encodes_as_one_then_42() {
    let s = WithOptionU32 { opt: Some(42) };
    let bytes = encode_to_vec(&s).expect("encode WithOptionU32(Some(42))");

    // Some discriminant = 0x01, then varint(42) = 0x2A (42 <= 250, 1 byte)
    assert_eq!(
        bytes,
        vec![0x01u8, 0x2Au8],
        "Option<u32> Some(42) must encode as [0x01, 0x2A] (discriminant + varint(42))"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Enum unit variant encodes as discriminant only
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum Color {
    Red,
    Green,
    Blue,
}

#[test]
fn test_14_enum_unit_variant_encodes_as_discriminant_only() {
    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;

    let bytes_red = encode_to_vec(&red).expect("encode Color::Red");
    let bytes_green = encode_to_vec(&green).expect("encode Color::Green");
    let bytes_blue = encode_to_vec(&blue).expect("encode Color::Blue");

    // Unit variants encode as varint discriminant: 0, 1, 2
    assert_eq!(bytes_red, vec![0x00u8], "Color::Red must encode as [0x00]");
    assert_eq!(
        bytes_green,
        vec![0x01u8],
        "Color::Green must encode as [0x01]"
    );
    assert_eq!(
        bytes_blue,
        vec![0x02u8],
        "Color::Blue must encode as [0x02]"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Enum tuple variant encodes as discriminant + field bytes
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum Payload {
    Empty,
    WithU32(u32),
    WithTwo(u8, u8),
}

#[test]
fn test_15_enum_tuple_variant_encodes_as_discriminant_plus_field_bytes() {
    let variant = Payload::WithU32(100);
    let bytes = encode_to_vec(&variant).expect("encode Payload::WithU32(100)");

    // discriminant for WithU32 = 1 (varint [0x01]) + varint(100) = [0x64]
    let expected = vec![0x01u8, 0x64u8];
    assert_eq!(
        bytes, expected,
        "Payload::WithU32(100) must encode as [discriminant=1, varint(100)=0x64]"
    );

    // Verify WithTwo variant: discriminant=2, then two raw u8 bytes
    let two = Payload::WithTwo(7, 8);
    let bytes_two = encode_to_vec(&two).expect("encode Payload::WithTwo(7,8)");
    // discriminant=2 [0x02], u8(7)=[0x07], u8(8)=[0x08]
    assert_eq!(
        bytes_two,
        vec![0x02u8, 0x07u8, 0x08u8],
        "Payload::WithTwo(7,8) must encode as [0x02, 0x07, 0x08]"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Struct total encoded length equals sum of all field lengths
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct ThreeFields {
    a: u8,
    b: u32,
    c: u64,
}

#[test]
fn test_16_struct_total_length_equals_sum_of_field_lengths() {
    let s = ThreeFields {
        a: 5,
        b: 1000,
        c: u64::MAX,
    };
    let struct_bytes = encode_to_vec(&s).expect("encode ThreeFields");

    let len_a = encode_to_vec(&5u8).expect("encode a").len();
    let len_b = encode_to_vec(&1000u32).expect("encode b").len();
    let len_c = encode_to_vec(&u64::MAX).expect("encode c").len();

    assert_eq!(
        struct_bytes.len(),
        len_a + len_b + len_c,
        "struct total length must equal sum of all individual field lengths"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Nested struct encodes as flat sequence
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Inner17 {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Outer17 {
    prefix: u8,
    inner: Inner17,
    suffix: u8,
}

#[test]
fn test_17_nested_struct_encodes_as_flat_sequence() {
    let s = Outer17 {
        prefix: 1,
        inner: Inner17 { x: 2, y: 3 },
        suffix: 4,
    };
    let struct_bytes = encode_to_vec(&s).expect("encode Outer17");

    // Flat encoding: bytes(prefix=1) ++ bytes(x=2) ++ bytes(y=3) ++ bytes(suffix=4)
    let mut expected = encode_to_vec(&1u8).expect("encode prefix");
    expected.extend_from_slice(&encode_to_vec(&2u32).expect("encode x"));
    expected.extend_from_slice(&encode_to_vec(&3u32).expect("encode y"));
    expected.extend_from_slice(&encode_to_vec(&4u8).expect("encode suffix"));

    assert_eq!(
        struct_bytes, expected,
        "nested struct must encode as flat concatenation of all leaf fields"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Large struct with 10 fields: all fields preserved after roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct TenFields {
    f0: u8,
    f1: u16,
    f2: u32,
    f3: u64,
    f4: i8,
    f5: i32,
    f6: bool,
    f7: u8,
    f8: u32,
    f9: u64,
}

#[test]
fn test_18_large_struct_ten_fields_all_preserved_after_roundtrip() {
    let s = TenFields {
        f0: 0xFF,
        f1: 0xABCD,
        f2: 0x12345678,
        f3: u64::MAX / 2,
        f4: i8::MIN,
        f5: i32::MAX,
        f6: true,
        f7: 0,
        f8: 1,
        f9: 0,
    };
    let encoded = encode_to_vec(&s).expect("encode TenFields");
    let (decoded, consumed): (TenFields, usize) =
        decode_from_slice(&encoded).expect("decode TenFields");

    assert_eq!(
        decoded, s,
        "all 10 fields must be preserved after roundtrip"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "all encoded bytes must be consumed"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Struct field with u64::MAX encodes to 9 bytes (0xFD marker + 8 LE bytes)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithU64Max {
    val: u64,
}

#[test]
fn test_19_struct_u64_max_field_encodes_to_nine_bytes() {
    let s = WithU64Max { val: u64::MAX };
    let bytes = encode_to_vec(&s).expect("encode WithU64Max");

    // u64::MAX encodes as [0xFD, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF] = 9 bytes
    assert_eq!(
        bytes.len(),
        9,
        "u64::MAX must encode to exactly 9 bytes (varint marker 0xFD + 8 LE bytes)"
    );
    assert_eq!(
        bytes[0], 0xFD,
        "first byte of u64::MAX encoding must be 0xFD varint marker"
    );
    assert_eq!(
        &bytes[1..],
        &[0xFFu8; 8],
        "remaining 8 bytes of u64::MAX encoding must all be 0xFF"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Struct with same fields in different order produces different bytes
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct FieldsAB {
    small: u8,
    large: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FieldsBA {
    large: u32,
    small: u8,
}

#[test]
fn test_20_same_fields_different_declaration_order_different_bytes() {
    // Use values where field type sizes differ (u8 vs u32 with value > 250)
    let ab = FieldsAB {
        small: 3,
        large: 1000,
    };
    let ba = FieldsBA {
        large: 1000,
        small: 3,
    };

    let bytes_ab = encode_to_vec(&ab).expect("encode FieldsAB");
    let bytes_ba = encode_to_vec(&ba).expect("encode FieldsBA");

    // FieldsAB: bytes(u8=3) ++ bytes(u32=1000) = [0x03][0xFB, 0xE8, 0x03]
    // FieldsBA: bytes(u32=1000) ++ bytes(u8=3)  = [0xFB, 0xE8, 0x03][0x03]
    assert_ne!(
        bytes_ab, bytes_ba,
        "different declaration order of fields with different sizes must produce different bytes"
    );
    assert_eq!(
        bytes_ab.len(),
        bytes_ba.len(),
        "total byte length must be the same regardless of field order"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Struct with String field: the string bytes appear consecutively in output
// ---------------------------------------------------------------------------

#[test]
fn test_21_struct_string_field_bytes_appear_consecutively_in_output() {
    let s = WithString {
        id: 1,
        name: "oxicode".to_string(),
    };
    let struct_bytes = encode_to_vec(&s).expect("encode WithString");

    // "oxicode" as UTF-8
    let utf8: &[u8] = b"oxicode";

    // Find the UTF-8 bytes as a contiguous subsequence in the encoding
    let found = struct_bytes
        .windows(utf8.len())
        .any(|window| window == utf8);

    assert!(
        found,
        "UTF-8 bytes of name field 'oxicode' must appear consecutively in struct encoding"
    );

    // Also verify the length prefix (varint(7) = [0x07]) appears just before the UTF-8 bytes
    let varint_len = 0x07u8;
    let pos = struct_bytes
        .windows(utf8.len())
        .position(|window| window == utf8)
        .expect("utf8 bytes must be present");
    assert!(pos > 0, "there must be bytes before the string content");
    assert_eq!(
        struct_bytes[pos - 1],
        varint_len,
        "byte immediately before string content must be varint(len)=0x07"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Struct roundtrip: encode -> decode -> all fields match original
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct FullRoundtrip {
    id: u32,
    name: String,
    score: f64,
    active: bool,
    tags: Vec<u8>,
    maybe_ref: Option<u32>,
}

#[test]
fn test_22_struct_roundtrip_all_fields_match_original() {
    let original = FullRoundtrip {
        id: 42,
        name: "struct-evolution".to_string(),
        score: 3.14159265358979,
        active: true,
        tags: vec![1u8, 2u8, 3u8, 4u8, 5u8],
        maybe_ref: Some(999),
    };

    let encoded = encode_to_vec(&original).expect("encode FullRoundtrip");
    let (decoded, consumed): (FullRoundtrip, usize) =
        decode_from_slice(&encoded).expect("decode FullRoundtrip");

    assert_eq!(
        decoded.id, original.id,
        "id field must match after roundtrip"
    );
    assert_eq!(
        decoded.name, original.name,
        "name field must match after roundtrip"
    );
    assert_eq!(
        decoded.score, original.score,
        "score field must match after roundtrip"
    );
    assert_eq!(
        decoded.active, original.active,
        "active field must match after roundtrip"
    );
    assert_eq!(
        decoded.tags, original.tags,
        "tags field must match after roundtrip"
    );
    assert_eq!(
        decoded.maybe_ref, original.maybe_ref,
        "maybe_ref field must match after roundtrip"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "all encoded bytes must be consumed"
    );

    // Also verify with a config variant for completeness
    let cfg = config::standard();
    let enc2 = encode_to_vec_with_config(&original, cfg).expect("encode with config");
    let (decoded2, _): (FullRoundtrip, usize) =
        decode_from_slice_with_config(&enc2, cfg).expect("decode with config");
    assert_eq!(
        decoded2, original,
        "struct must roundtrip correctly with explicit standard config"
    );
}
