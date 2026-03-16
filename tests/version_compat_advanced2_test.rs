//! Binary wire format stability and backward compatibility tests.
//!
//! These tests pin down the exact byte sequences produced for primitive types
//! so that any accidental change to the wire format is immediately caught.

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
use oxicode::{decode_from_slice, encode_to_vec};

// ---------------------------------------------------------------------------
// 1. u8 value 0 → [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_wire_u8_zero() {
    let enc = encode_to_vec(&0u8).expect("encode u8 0");
    assert_eq!(enc, vec![0x00u8], "u8(0) must encode to a single zero byte");
    let (val, _): (u8, _) = decode_from_slice(&enc).expect("decode u8 0");
    assert_eq!(val, 0u8);
}

// ---------------------------------------------------------------------------
// 2. u8 value 1 → [0x01]
// ---------------------------------------------------------------------------
#[test]
fn test_wire_u8_one() {
    let enc = encode_to_vec(&1u8).expect("encode u8 1");
    assert_eq!(enc, vec![0x01u8], "u8(1) must encode to [0x01]");
    let (val, _): (u8, _) = decode_from_slice(&enc).expect("decode u8 1");
    assert_eq!(val, 1u8);
}

// ---------------------------------------------------------------------------
// 3. u8 value 127 → [0x7F]
// ---------------------------------------------------------------------------
#[test]
fn test_wire_u8_127() {
    let enc = encode_to_vec(&127u8).expect("encode u8 127");
    assert_eq!(enc, vec![0x7Fu8], "u8(127) must encode to [0x7F]");
    let (val, _): (u8, _) = decode_from_slice(&enc).expect("decode u8 127");
    assert_eq!(val, 127u8);
}

// ---------------------------------------------------------------------------
// 4. u8 value 255 → [0xFF]
//    u8 is always written as a raw single byte (no varint treatment).
// ---------------------------------------------------------------------------
#[test]
fn test_wire_u8_255() {
    let enc = encode_to_vec(&255u8).expect("encode u8 255");
    assert_eq!(enc, vec![0xFFu8], "u8(255) must encode to [0xFF]");
    let (val, _): (u8, _) = decode_from_slice(&enc).expect("decode u8 255");
    assert_eq!(val, 255u8);
}

// ---------------------------------------------------------------------------
// 5. u16 value 0 → varint single byte [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_wire_u16_zero() {
    let enc = encode_to_vec(&0u16).expect("encode u16 0");
    assert_eq!(enc, vec![0x00u8], "u16(0) varint must encode to [0x00]");
    assert_eq!(enc.len(), 1, "u16(0) must be 1 byte");
    let (val, _): (u16, _) = decode_from_slice(&enc).expect("decode u16 0");
    assert_eq!(val, 0u16);
}

// ---------------------------------------------------------------------------
// 6. u16 value 255 → varint 3-byte form: [0xFB, 0xFF, 0x00]
//    255 > 250 (SINGLE_BYTE_MAX), so the U16_BYTE marker (0xFB) is written
//    followed by the 2-byte little-endian representation: 0xFF, 0x00.
// ---------------------------------------------------------------------------
#[test]
fn test_wire_u16_255() {
    let enc = encode_to_vec(&255u16).expect("encode u16 255");
    assert_eq!(
        enc,
        vec![0xFBu8, 0xFFu8, 0x00u8],
        "u16(255) varint must be [0xFB, 0xFF, 0x00]"
    );
    assert_eq!(enc.len(), 3, "u16(255) varint must be 3 bytes");
    let (val, _): (u16, _) = decode_from_slice(&enc).expect("decode u16 255");
    assert_eq!(val, 255u16);
}

// ---------------------------------------------------------------------------
// 7. u32 value 0 → varint [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_wire_u32_zero() {
    let enc = encode_to_vec(&0u32).expect("encode u32 0");
    assert_eq!(enc, vec![0x00u8], "u32(0) varint must encode to [0x00]");
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32 0");
    assert_eq!(val, 0u32);
}

// ---------------------------------------------------------------------------
// 8. u32 value 1 → varint [0x01]
// ---------------------------------------------------------------------------
#[test]
fn test_wire_u32_one() {
    let enc = encode_to_vec(&1u32).expect("encode u32 1");
    assert_eq!(enc, vec![0x01u8], "u32(1) varint must encode to [0x01]");
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32 1");
    assert_eq!(val, 1u32);
}

// ---------------------------------------------------------------------------
// 9. u32 value 250 → varint [0xFA]  (250 == SINGLE_BYTE_MAX, still single byte)
// ---------------------------------------------------------------------------
#[test]
fn test_wire_u32_250() {
    let enc = encode_to_vec(&250u32).expect("encode u32 250");
    assert_eq!(
        enc,
        vec![0xFAu8],
        "u32(250) varint must encode to [0xFA] (single byte)"
    );
    assert_eq!(enc.len(), 1, "u32(250) must be 1 byte");
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32 250");
    assert_eq!(val, 250u32);
}

// ---------------------------------------------------------------------------
// 10. u32 value 251 → varint 3-byte form: [0xFB, 0xFB, 0x00]
//     251 > 250 but <= u16::MAX, so U16_BYTE marker (0xFB) + LE u16 of 251.
//     251 in LE u16 = [0xFB, 0x00]. So the full encoding = [0xFB, 0xFB, 0x00].
// ---------------------------------------------------------------------------
#[test]
fn test_wire_u32_251() {
    let enc = encode_to_vec(&251u32).expect("encode u32 251");
    assert_eq!(
        enc,
        vec![0xFBu8, 0xFBu8, 0x00u8],
        "u32(251) varint must be [0xFB, 0xFB, 0x00]"
    );
    assert_eq!(enc.len(), 3, "u32(251) varint must be 3 bytes");
    let (val, _): (u32, _) = decode_from_slice(&enc).expect("decode u32 251");
    assert_eq!(val, 251u32);
}

// ---------------------------------------------------------------------------
// 11. bool true → [0x01]  (encoded as u8)
// ---------------------------------------------------------------------------
#[test]
fn test_wire_bool_true() {
    let enc = encode_to_vec(&true).expect("encode bool true");
    assert_eq!(enc, vec![0x01u8], "bool(true) must encode to [0x01]");
    assert_eq!(enc.len(), 1);
    let (val, _): (bool, _) = decode_from_slice(&enc).expect("decode bool true");
    assert!(val);
}

// ---------------------------------------------------------------------------
// 12. bool false → [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_wire_bool_false() {
    let enc = encode_to_vec(&false).expect("encode bool false");
    assert_eq!(enc, vec![0x00u8], "bool(false) must encode to [0x00]");
    assert_eq!(enc.len(), 1);
    let (val, _): (bool, _) = decode_from_slice(&enc).expect("decode bool false");
    assert!(!val);
}

// ---------------------------------------------------------------------------
// 13. String empty → [0x00]  (varint u64 length 0)
// ---------------------------------------------------------------------------
#[test]
fn test_wire_string_empty() {
    let s = String::new();
    let enc = encode_to_vec(&s).expect("encode empty String");
    assert_eq!(
        enc,
        vec![0x00u8],
        "empty String must encode to a single 0x00 length prefix"
    );
    let (val, _): (String, _) = decode_from_slice(&enc).expect("decode empty String");
    assert_eq!(val, "");
}

// ---------------------------------------------------------------------------
// 14. String "A" → [0x01, 0x41]  (length 1 as varint, then ASCII 'A')
// ---------------------------------------------------------------------------
#[test]
fn test_wire_string_single_char() {
    let s = String::from("A");
    let enc = encode_to_vec(&s).expect("encode String \"A\"");
    assert_eq!(
        enc,
        vec![0x01u8, 0x41u8],
        "String(\"A\") must encode to [0x01, 0x41]"
    );
    let (val, _): (String, _) = decode_from_slice(&enc).expect("decode String \"A\"");
    assert_eq!(val, "A");
}

// ---------------------------------------------------------------------------
// 15. Vec<u8> empty → [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_wire_vec_u8_empty() {
    let v: Vec<u8> = Vec::new();
    let enc = encode_to_vec(&v).expect("encode empty Vec<u8>");
    assert_eq!(enc, vec![0x00u8], "empty Vec<u8> must encode to [0x00]");
    let (val, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode empty Vec<u8>");
    assert!(val.is_empty());
}

// ---------------------------------------------------------------------------
// 16. Vec<u8> with one byte 0xFF → [0x01, 0xFF]
// ---------------------------------------------------------------------------
#[test]
fn test_wire_vec_u8_single_byte() {
    let v: Vec<u8> = vec![0xFF];
    let enc = encode_to_vec(&v).expect("encode Vec<u8> [0xFF]");
    assert_eq!(
        enc,
        vec![0x01u8, 0xFFu8],
        "Vec<u8>([0xFF]) must encode to [0x01, 0xFF]"
    );
    let (val, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode Vec<u8> [0xFF]");
    assert_eq!(val, vec![0xFFu8]);
}

// ---------------------------------------------------------------------------
// 17. Option<u8> None → [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_wire_option_u8_none() {
    let v: Option<u8> = None;
    let enc = encode_to_vec(&v).expect("encode Option<u8> None");
    assert_eq!(enc, vec![0x00u8], "Option<u8>(None) must encode to [0x00]");
    let (val, _): (Option<u8>, _) = decode_from_slice(&enc).expect("decode Option<u8> None");
    assert_eq!(val, None);
}

// ---------------------------------------------------------------------------
// 18. Option<u8> Some(42) → [0x01, 0x2A]
//     Variant tag 1u8 then the raw u8 value 42 = 0x2A.
// ---------------------------------------------------------------------------
#[test]
fn test_wire_option_u8_some() {
    let v: Option<u8> = Some(42);
    let enc = encode_to_vec(&v).expect("encode Option<u8> Some(42)");
    assert_eq!(
        enc,
        vec![0x01u8, 0x2Au8],
        "Option<u8>(Some(42)) must encode to [0x01, 0x2A]"
    );
    let (val, _): (Option<u8>, _) = decode_from_slice(&enc).expect("decode Option<u8> Some(42)");
    assert_eq!(val, Some(42u8));
}

// ---------------------------------------------------------------------------
// 19. [u8; 3] value [1, 2, 3] → [0x01, 0x02, 0x03]
//     Fixed-size arrays have no length prefix; each element is encoded in order.
// ---------------------------------------------------------------------------
#[test]
fn test_wire_fixed_array_u8() {
    let arr: [u8; 3] = [1, 2, 3];
    let enc = encode_to_vec(&arr).expect("encode [u8; 3]");
    assert_eq!(
        enc,
        vec![0x01u8, 0x02u8, 0x03u8],
        "[u8; 3]([1,2,3]) must encode to [0x01, 0x02, 0x03] with no length prefix"
    );
    assert_eq!(enc.len(), 3, "fixed array must not add a length prefix");
    let (val, _): ([u8; 3], _) = decode_from_slice(&enc).expect("decode [u8; 3]");
    assert_eq!(val, [1u8, 2, 3]);
}

// ---------------------------------------------------------------------------
// 20. u64 value 0 → [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_wire_u64_zero() {
    let enc = encode_to_vec(&0u64).expect("encode u64 0");
    assert_eq!(enc, vec![0x00u8], "u64(0) varint must encode to [0x00]");
    assert_eq!(enc.len(), 1);
    let (val, _): (u64, _) = decode_from_slice(&enc).expect("decode u64 0");
    assert_eq!(val, 0u64);
}

// ---------------------------------------------------------------------------
// 21. i8 value -1 → raw byte 0xFF  (i8 is NOT zigzag-encoded; cast to u8)
// ---------------------------------------------------------------------------
#[test]
fn test_wire_i8_minus_one() {
    let enc = encode_to_vec(&(-1i8)).expect("encode i8 -1");
    assert_eq!(
        enc,
        vec![0xFFu8],
        "i8(-1) must encode as raw byte 0xFF (no zigzag, direct u8 cast)"
    );
    assert_eq!(enc.len(), 1);
    let (val, _): (i8, _) = decode_from_slice(&enc).expect("decode i8 -1");
    assert_eq!(val, -1i8);
}

// ---------------------------------------------------------------------------
// 22. i32 value -1 → zigzag varint [0x01]
//     Zigzag maps -1 to 1 (formula: ((-1i32 as u32).wrapping_shl(1)) ^ ((-1 >> 31) as u32)
//     = 0xFFFFFFFE ^ 0xFFFFFFFF = 1). 1 <= 250 → single byte [0x01].
// ---------------------------------------------------------------------------
#[test]
fn test_wire_i32_minus_one() {
    let enc = encode_to_vec(&(-1i32)).expect("encode i32 -1");
    assert_eq!(
        enc,
        vec![0x01u8],
        "i32(-1) zigzag varint must encode to [0x01] (zigzag maps -1 → 1)"
    );
    assert_eq!(enc.len(), 1);
    let (val, _): (i32, _) = decode_from_slice(&enc).expect("decode i32 -1");
    assert_eq!(val, -1i32);
}
