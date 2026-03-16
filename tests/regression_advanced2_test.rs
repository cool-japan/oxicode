//! Regression and correctness tests that act as permanent documentation
//! of OxiCode's wire format and encoding behavior.
//!
//! These tests verify specific known behaviors and edge cases with hardcoded
//! expected byte sequences to detect any inadvertent format changes.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// Test 1: Zero values — 0u8, 0u16, 0u32, 0u64 all encode as single byte [0x00]
#[test]
fn test_zero_values_single_byte() {
    let enc_u8 = encode_to_vec(&0u8).expect("encode 0u8");
    let enc_u16 = encode_to_vec(&0u16).expect("encode 0u16");
    let enc_u32 = encode_to_vec(&0u32).expect("encode 0u32");
    let enc_u64 = encode_to_vec(&0u64).expect("encode 0u64");

    assert_eq!(enc_u8, vec![0x00u8], "0u8 must encode as [0x00]");
    assert_eq!(enc_u16, vec![0x00u8], "0u16 must encode as [0x00]");
    assert_eq!(enc_u32, vec![0x00u8], "0u32 must encode as [0x00]");
    assert_eq!(enc_u64, vec![0x00u8], "0u64 must encode as [0x00]");

    // Verify roundtrips
    let (v_u8, _): (u8, _) = decode_from_slice(&enc_u8).expect("decode 0u8");
    let (v_u16, _): (u16, _) = decode_from_slice(&enc_u16).expect("decode 0u16");
    let (v_u32, _): (u32, _) = decode_from_slice(&enc_u32).expect("decode 0u32");
    let (v_u64, _): (u64, _) = decode_from_slice(&enc_u64).expect("decode 0u64");
    assert_eq!(v_u8, 0u8);
    assert_eq!(v_u16, 0u16);
    assert_eq!(v_u32, 0u32);
    assert_eq!(v_u64, 0u64);
}

// Test 2: Value 250 (max 1-byte varint) encodes as [0xFA]
#[test]
fn test_value_250_single_byte_varint() {
    let enc_u8 = encode_to_vec(&250u8).expect("encode 250u8");
    let enc_u32 = encode_to_vec(&250u32).expect("encode 250u32");

    assert_eq!(enc_u8, vec![0xFAu8], "250u8 must encode as [0xFA]");
    assert_eq!(enc_u32, vec![0xFAu8], "250u32 must encode as [0xFA]");
    assert_eq!(enc_u8.len(), 1, "250 must fit in 1 byte");
    assert_eq!(enc_u32.len(), 1, "250 must fit in 1 byte");
}

// Test 3: Value 251 encodes as 3 bytes (varint marker + 2 bytes) for u16
#[test]
fn test_value_251_three_byte_varint() {
    let enc_u16 = encode_to_vec(&251u16).expect("encode 251u16");
    let enc_u32 = encode_to_vec(&251u32).expect("encode 251u32");

    assert_eq!(
        enc_u16.len(),
        3,
        "251u16 must encode as 3 bytes (marker + 2)"
    );
    assert_eq!(
        enc_u32.len(),
        3,
        "251u32 must encode as 3 bytes (marker + 2)"
    );
    // 251 decimal = 0xFB marker byte + little-endian u16 [0xFB, 0x00]
    assert_eq!(
        enc_u16,
        vec![0xFBu8, 0xFB, 0x00],
        "251u16 wire format must be [0xFB, 0xFB, 0x00]"
    );
    assert_eq!(
        enc_u32,
        vec![0xFBu8, 0xFB, 0x00],
        "251u32 wire format must be [0xFB, 0xFB, 0x00]"
    );

    let (v16, _): (u16, _) = decode_from_slice(&enc_u16).expect("decode 251u16");
    let (v32, _): (u32, _) = decode_from_slice(&enc_u32).expect("decode 251u32");
    assert_eq!(v16, 251u16);
    assert_eq!(v32, 251u32);
}

// Test 4: Value 65535 (u16::MAX) encodes as 3 bytes
#[test]
fn test_u16_max_three_bytes() {
    let enc_u16 = encode_to_vec(&65535u16).expect("encode u16::MAX");
    let enc_u32 = encode_to_vec(&65535u32).expect("encode 65535u32");

    assert_eq!(enc_u16.len(), 3, "u16::MAX must encode as 3 bytes");
    assert_eq!(enc_u32.len(), 3, "65535u32 must encode as 3 bytes");
    // 0xFB marker + LE u16 0xFFFF
    assert_eq!(
        enc_u16,
        vec![0xFBu8, 0xFF, 0xFF],
        "u16::MAX wire format must be [0xFB, 0xFF, 0xFF]"
    );
    assert_eq!(
        enc_u32,
        vec![0xFBu8, 0xFF, 0xFF],
        "65535u32 wire format must be [0xFB, 0xFF, 0xFF]"
    );

    let (v16, _): (u16, _) = decode_from_slice(&enc_u16).expect("decode u16::MAX");
    let (v32, _): (u32, _) = decode_from_slice(&enc_u32).expect("decode 65535u32");
    assert_eq!(v16, 65535u16);
    assert_eq!(v32, 65535u32);
}

// Test 5: Value 65536 (u32 only) encodes as 5 bytes
#[test]
fn test_value_65536_five_bytes() {
    let enc = encode_to_vec(&65536u32).expect("encode 65536u32");

    assert_eq!(
        enc.len(),
        5,
        "65536u32 must encode as 5 bytes (0xFC marker + 4 bytes)"
    );
    // 0xFC marker + LE u32 0x00010000
    assert_eq!(
        enc,
        vec![0xFCu8, 0x00, 0x00, 0x01, 0x00],
        "65536u32 wire format must be [0xFC, 0x00, 0x00, 0x01, 0x00]"
    );

    let (v, _): (u32, _) = decode_from_slice(&enc).expect("decode 65536u32");
    assert_eq!(v, 65536u32);
}

// Test 6: bool true = [0x01], bool false = [0x00]
#[test]
fn test_bool_encoding() {
    let enc_true = encode_to_vec(&true).expect("encode true");
    let enc_false = encode_to_vec(&false).expect("encode false");

    assert_eq!(enc_true, vec![0x01u8], "true must encode as [0x01]");
    assert_eq!(enc_false, vec![0x00u8], "false must encode as [0x00]");

    let (v_true, _): (bool, _) = decode_from_slice(&enc_true).expect("decode true");
    let (v_false, _): (bool, _) = decode_from_slice(&enc_false).expect("decode false");
    assert!(v_true, "decoded true must be true");
    assert!(!v_false, "decoded false must be false");
}

// Test 7: String "abc" encodes as: length(3) + 'a' + 'b' + 'c' = [0x03, 0x61, 0x62, 0x63]
#[test]
fn test_string_abc_encoding() {
    let enc = encode_to_vec(&"abc".to_string()).expect("encode \"abc\"");
    assert_eq!(
        enc,
        vec![0x03u8, 0x61, 0x62, 0x63],
        "\"abc\" must encode as [0x03, 0x61, 0x62, 0x63]"
    );

    let (v, _): (String, _) = decode_from_slice(&enc).expect("decode \"abc\"");
    assert_eq!(v, "abc");
}

// Test 8: Empty string encodes as [0x00] (length 0)
#[test]
fn test_empty_string_encoding() {
    let enc = encode_to_vec(&"".to_string()).expect("encode empty string");
    assert_eq!(enc, vec![0x00u8], "empty string must encode as [0x00]");

    let (v, _): (String, _) = decode_from_slice(&enc).expect("decode empty string");
    assert_eq!(v, "");
    assert_eq!(enc.len(), 1, "empty string must use 1 byte");
}

// Test 9: None Option<u32> encodes as [0x00]
#[test]
fn test_none_option_encoding() {
    let enc = encode_to_vec(&None::<u32>).expect("encode None<u32>");
    assert_eq!(enc, vec![0x00u8], "None<u32> must encode as [0x00]");

    let (v, _): (Option<u32>, _) = decode_from_slice(&enc).expect("decode None<u32>");
    assert_eq!(v, None);
}

// Test 10: Some(0u32) encodes as [0x01, 0x00]
#[test]
fn test_some_zero_encoding() {
    let enc = encode_to_vec(&Some(0u32)).expect("encode Some(0u32)");
    assert_eq!(
        enc,
        vec![0x01u8, 0x00],
        "Some(0u32) must encode as [0x01, 0x00]"
    );
    assert_eq!(enc.len(), 2, "Some(0u32) must use 2 bytes");

    let (v, _): (Option<u32>, _) = decode_from_slice(&enc).expect("decode Some(0u32)");
    assert_eq!(v, Some(0u32));
}

// Test 11: Unit struct encodes to 0 bytes
#[test]
fn test_unit_struct_zero_bytes() {
    #[derive(Encode, Decode)]
    struct UnitStruct;

    let enc = encode_to_vec(&UnitStruct).expect("encode UnitStruct");
    assert_eq!(enc.len(), 0, "unit struct must encode to 0 bytes");
    assert_eq!(enc, Vec::<u8>::new(), "unit struct encoding must be empty");
}

// Test 12: Tuple () (unit) encodes to 0 bytes
#[test]
fn test_unit_tuple_zero_bytes() {
    let enc = encode_to_vec(&()).expect("encode ()");
    assert_eq!(enc.len(), 0, "unit tuple () must encode to 0 bytes");
    assert_eq!(enc, Vec::<u8>::new(), "() encoding must be empty");
}

// Test 13: [u8; 4] encodes as exactly 4 bytes (no length prefix)
#[test]
fn test_fixed_array_no_length_prefix() {
    let arr: [u8; 4] = [1, 2, 3, 4];
    let enc = encode_to_vec(&arr).expect("encode [u8; 4]");

    assert_eq!(
        enc.len(),
        4,
        "[u8; 4] must encode as exactly 4 bytes (no length prefix)"
    );
    assert_eq!(
        enc,
        vec![1u8, 2, 3, 4],
        "[u8; 4] must encode elements directly"
    );

    let (v, _): ([u8; 4], _) = decode_from_slice(&enc).expect("decode [u8; 4]");
    assert_eq!(v, arr);
}

// Test 14: Vec<u8> with 4 elements: length prefix + 4 bytes = 5 bytes (if len<=250)
#[test]
fn test_vec_length_prefix() {
    let v: Vec<u8> = vec![1, 2, 3, 4];
    let enc = encode_to_vec(&v).expect("encode Vec<u8>(4)");

    assert_eq!(
        enc.len(),
        5,
        "Vec<u8> with 4 elements must encode as 5 bytes (1 len + 4 data)"
    );
    assert_eq!(enc[0], 4u8, "first byte must be the length prefix (4)");
    assert_eq!(
        enc,
        vec![4u8, 1, 2, 3, 4],
        "Vec<u8>(4) wire format must be [4, 1, 2, 3, 4]"
    );

    let (dv, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode Vec<u8>(4)");
    assert_eq!(dv, v);
}

// Test 15: Enum first variant (discriminant 0) encodes as [0x00]
#[test]
fn test_enum_first_variant_discriminant_zero() {
    #[derive(Encode, Decode, Debug, PartialEq)]
    enum TestEnum {
        First,
        Second(u32),
        Third { x: u8 },
    }

    let enc = encode_to_vec(&TestEnum::First).expect("encode TestEnum::First");
    assert_eq!(
        enc,
        vec![0x00u8],
        "first enum variant must encode as [0x00]"
    );

    let (v, _): (TestEnum, _) = decode_from_slice(&enc).expect("decode TestEnum::First");
    assert_eq!(v, TestEnum::First);
}

// Test 16: Nested struct: inner fields encoded consecutively, no separator
#[test]
fn test_nested_struct_consecutive_fields() {
    #[derive(Encode, Decode, Debug, PartialEq)]
    struct Inner {
        x: u8,
    }

    #[derive(Encode, Decode, Debug, PartialEq)]
    struct Outer {
        a: Inner,
        b: u8,
    }

    let o = Outer {
        a: Inner { x: 5 },
        b: 10,
    };
    let enc = encode_to_vec(&o).expect("encode Outer");

    // Fields encoded consecutively: Inner.x=5 then b=10, no separator
    assert_eq!(
        enc,
        vec![5u8, 10],
        "nested struct must encode as [5, 10] with no separator"
    );
    assert_eq!(enc.len(), 2, "nested struct must use exactly 2 bytes");

    let (v, _): (Outer, _) = decode_from_slice(&enc).expect("decode Outer");
    assert_eq!(v, o);
}

// Test 17: Verify re-encode: encode -> decode -> encode produces same bytes
#[test]
fn test_reencode_produces_same_bytes() {
    #[derive(Encode, Decode, Debug, PartialEq)]
    struct Sample {
        id: u32,
        name: String,
        flag: bool,
    }

    let original = Sample {
        id: 42,
        name: "hello".to_string(),
        flag: true,
    };

    let enc1 = encode_to_vec(&original).expect("first encode");
    let (decoded, _): (Sample, _) = decode_from_slice(&enc1).expect("decode");
    let enc2 = encode_to_vec(&decoded).expect("second encode");

    assert_eq!(
        enc1, enc2,
        "re-encoding a decoded value must produce identical bytes"
    );
    assert_eq!(decoded, original, "decoded value must equal original");
}

// Test 18: Wire format stability: [0x2A] decodes as u8 = 42 (0x2A = 42 decimal)
#[test]
fn test_wire_stability_u8_42() {
    // Hardcoded wire bytes
    let wire: Vec<u8> = vec![0x2A];
    let (val, consumed): (u8, _) = decode_from_slice(&wire).expect("decode [0x2A] as u8");

    assert_eq!(val, 42u8, "[0x2A] must decode as u8 = 42");
    assert_eq!(consumed, 1, "consuming [0x2A] must advance 1 byte");

    // Also verify the forward direction
    let enc = encode_to_vec(&42u8).expect("encode 42u8");
    assert_eq!(enc, vec![0x2A], "42u8 must encode as [0x2A]");
}

// Test 19: Wire format stability: [0x01, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F] = Some("hello")
#[test]
fn test_wire_stability_some_hello() {
    // 0x01 = Some tag, 0x05 = len 5, then "hello" UTF-8 bytes
    let wire: Vec<u8> = vec![0x01, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F];
    let (val, _): (Option<String>, _) = decode_from_slice(&wire).expect("decode Some(\"hello\")");

    assert_eq!(
        val,
        Some("hello".to_string()),
        "wire bytes must decode as Some(\"hello\")"
    );

    // Verify forward direction
    let enc = encode_to_vec(&Some("hello".to_string())).expect("encode Some(\"hello\")");
    assert_eq!(
        enc, wire,
        "Some(\"hello\") must encode as [0x01, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F]"
    );
}

// Test 20: i32(-1) zigzag: encodes as [0x01] (zigzag maps -1 to 1)
#[test]
fn test_i32_neg1_zigzag_encoding() {
    let enc = encode_to_vec(&(-1i32)).expect("encode -1i32");
    assert_eq!(enc, vec![0x01u8], "-1i32 must zigzag-encode as [0x01]");
    assert_eq!(enc.len(), 1, "-1i32 must encode in 1 byte via zigzag");

    let (v, _): (i32, _) = decode_from_slice(&enc).expect("decode -1i32");
    assert_eq!(v, -1i32, "zigzag [0x01] must decode back to -1i32");
}

// Test 21: i32(1) zigzag: encodes as [0x02] (zigzag maps 1 to 2)
#[test]
fn test_i32_pos1_zigzag_encoding() {
    let enc = encode_to_vec(&1i32).expect("encode 1i32");
    assert_eq!(enc, vec![0x02u8], "1i32 must zigzag-encode as [0x02]");
    assert_eq!(enc.len(), 1, "1i32 must encode in 1 byte via zigzag");

    let (v, _): (i32, _) = decode_from_slice(&enc).expect("decode 1i32");
    assert_eq!(v, 1i32, "zigzag [0x02] must decode back to 1i32");
}

// Test 22: i32(-128) zigzag: encodes as [0xFB, 0xFF, 0x00] (zigzag maps -128 to 255)
// Zigzag: -128 -> 2*|-128| - 1 = 255, which just exceeds 1-byte varint limit (250),
// so it uses the 3-byte varint form: [0xFB, 0xFF, 0x00]
#[test]
fn test_i32_neg128_zigzag_encoding() {
    let enc = encode_to_vec(&(-128i32)).expect("encode -128i32");
    // Zigzag(-128) = 255, varint(255) = [0xFB, 0xFF, 0x00] (3-byte form)
    assert_eq!(
        enc,
        vec![0xFBu8, 0xFF, 0x00],
        "-128i32 must zigzag-encode as [0xFB, 0xFF, 0x00]"
    );
    assert_eq!(
        enc.len(),
        3,
        "-128i32 must encode in 3 bytes (zigzag=255 exceeds 1-byte varint)"
    );

    let (v, _): (i32, _) = decode_from_slice(&enc).expect("decode -128i32");
    assert_eq!(
        v, -128i32,
        "zigzag [0xFB, 0xFF, 0x00] must decode back to -128i32"
    );
}
