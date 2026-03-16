//! Exact binary encoding format tests for OxiCode.
//!
//! Verifies byte-level output complementing (not duplicating) format_spec_test.rs.
//! Coverage:
//!   - u8 value 42, u16/u32 varint 1000/256
//!   - i32 zigzag for -1, -128, +1
//!   - String "hello" with 5-byte varint length
//!   - String::from("") (owned empty string)
//!   - Option<u32> None and Some(42)
//!   - Vec<u16> empty, Vec<u8> two elements
//!   - [u8; 5] fixed array (no length prefix)
//!   - 3-tuple (u8, u8, u8)
//!   - Enum variant 0, 1, 250, 251 exact byte sequences
//!   - f32 PI, f64 E IEEE-754 LE bytes
//!   - u64=1 with fixed_int config (8 bytes LE)
//!   - u32=1 with big_endian + fixed_int config (4 bytes BE)

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
use oxicode::{config, encode_to_vec, encode_to_vec_with_config, Decode, Encode};

// ---------------------------------------------------------------------------
// Derive helpers
// ---------------------------------------------------------------------------

/// A simple 3-variant enum for discriminant 0 / 1 tests.
#[derive(Debug, PartialEq, Encode, Decode)]
enum SmallEnum {
    A,
    B,
    C,
}

/// A macro that builds a unit enum whose variants are named V000, V001, … V251
/// (252 variants total) so we can encode V250 (discriminant 250, 1-byte varint)
/// and V251 (discriminant 251, 3-byte varint).
macro_rules! unit_enum_252 {
    () => {
        #[derive(Debug, PartialEq, Encode, Decode)]
        #[allow(dead_code)]
        enum Enum252 {
            V000,
            V001,
            V002,
            V003,
            V004,
            V005,
            V006,
            V007,
            V008,
            V009,
            V010,
            V011,
            V012,
            V013,
            V014,
            V015,
            V016,
            V017,
            V018,
            V019,
            V020,
            V021,
            V022,
            V023,
            V024,
            V025,
            V026,
            V027,
            V028,
            V029,
            V030,
            V031,
            V032,
            V033,
            V034,
            V035,
            V036,
            V037,
            V038,
            V039,
            V040,
            V041,
            V042,
            V043,
            V044,
            V045,
            V046,
            V047,
            V048,
            V049,
            V050,
            V051,
            V052,
            V053,
            V054,
            V055,
            V056,
            V057,
            V058,
            V059,
            V060,
            V061,
            V062,
            V063,
            V064,
            V065,
            V066,
            V067,
            V068,
            V069,
            V070,
            V071,
            V072,
            V073,
            V074,
            V075,
            V076,
            V077,
            V078,
            V079,
            V080,
            V081,
            V082,
            V083,
            V084,
            V085,
            V086,
            V087,
            V088,
            V089,
            V090,
            V091,
            V092,
            V093,
            V094,
            V095,
            V096,
            V097,
            V098,
            V099,
            V100,
            V101,
            V102,
            V103,
            V104,
            V105,
            V106,
            V107,
            V108,
            V109,
            V110,
            V111,
            V112,
            V113,
            V114,
            V115,
            V116,
            V117,
            V118,
            V119,
            V120,
            V121,
            V122,
            V123,
            V124,
            V125,
            V126,
            V127,
            V128,
            V129,
            V130,
            V131,
            V132,
            V133,
            V134,
            V135,
            V136,
            V137,
            V138,
            V139,
            V140,
            V141,
            V142,
            V143,
            V144,
            V145,
            V146,
            V147,
            V148,
            V149,
            V150,
            V151,
            V152,
            V153,
            V154,
            V155,
            V156,
            V157,
            V158,
            V159,
            V160,
            V161,
            V162,
            V163,
            V164,
            V165,
            V166,
            V167,
            V168,
            V169,
            V170,
            V171,
            V172,
            V173,
            V174,
            V175,
            V176,
            V177,
            V178,
            V179,
            V180,
            V181,
            V182,
            V183,
            V184,
            V185,
            V186,
            V187,
            V188,
            V189,
            V190,
            V191,
            V192,
            V193,
            V194,
            V195,
            V196,
            V197,
            V198,
            V199,
            V200,
            V201,
            V202,
            V203,
            V204,
            V205,
            V206,
            V207,
            V208,
            V209,
            V210,
            V211,
            V212,
            V213,
            V214,
            V215,
            V216,
            V217,
            V218,
            V219,
            V220,
            V221,
            V222,
            V223,
            V224,
            V225,
            V226,
            V227,
            V228,
            V229,
            V230,
            V231,
            V232,
            V233,
            V234,
            V235,
            V236,
            V237,
            V238,
            V239,
            V240,
            V241,
            V242,
            V243,
            V244,
            V245,
            V246,
            V247,
            V248,
            V249,
            V250,
            V251,
        }
    };
}

unit_enum_252!();

// IEEE-754 LE byte sequences for PI and E (pre-computed to stay within MSRV 1.70).
// std::f32::consts::PI.to_bits() = 0x40490FDB  → LE = [0xDB, 0x0F, 0x49, 0x40]
// std::f64::consts::E.to_bits()  = 0x4005BF0A8B145769 → LE = [0x69, 0x57, 0x14, 0x8B, 0x0A, 0xBF, 0x05, 0x40]
const PI_F32_BYTES: [u8; 4] = [0xDB, 0x0F, 0x49, 0x40];
const E_F64_BYTES: [u8; 8] = [0x69, 0x57, 0x14, 0x8B, 0x0A, 0xBF, 0x05, 0x40];

// ---------------------------------------------------------------------------
// 1. u8 value 42 → [42]
// ---------------------------------------------------------------------------
#[test]
fn test_u8_42() {
    let bytes = encode_to_vec(&42u8).expect("encode u8 42");
    assert_eq!(bytes, &[42]);
}

// ---------------------------------------------------------------------------
// 2. u16 1000 → [251, 232, 3]  (varint 3-byte: marker 251 + LE u16 1000)
//    1000 = 0x03E8; LE u16 bytes = [0xE8, 0x03] = [232, 3]
// ---------------------------------------------------------------------------
#[test]
fn test_u16_1000_varint() {
    let bytes = encode_to_vec(&1000u16).expect("encode u16 1000");
    assert_eq!(bytes, &[251, 232, 3]);
}

// ---------------------------------------------------------------------------
// 3. u32 256 → [251, 0, 1]  (varint 3-byte: marker 251 + LE u16 256)
//    256 = 0x0100; LE u16 bytes = [0x00, 0x01]
// ---------------------------------------------------------------------------
#[test]
fn test_u32_256_varint() {
    let bytes = encode_to_vec(&256u32).expect("encode u32 256");
    assert_eq!(bytes, &[251, 0, 1]);
}

// ---------------------------------------------------------------------------
// 4. i32 -1 zigzag → [1]
//    zigzag(-1) = ((-1i32 as u32) << 1) ^ ((-1i32 >> 31) as u32)
//               = 0xFFFFFFFE ^ 0xFFFFFFFF = 1  → varint [1]
// ---------------------------------------------------------------------------
#[test]
fn test_i32_neg1_zigzag() {
    let bytes = encode_to_vec(&(-1i32)).expect("encode i32 -1");
    assert_eq!(bytes, &[1]);
}

// ---------------------------------------------------------------------------
// 5. i32 -128 zigzag → [251, 255, 0]
//    zigzag(-128) = ((-128i32 as u32) << 1) ^ ((-128i32 >> 31) as u32)
//                = 0xFFFFFF00 ^ 0xFFFFFFFF = 0xFF = 255
//    255 > 250, so encoded as 3-byte varint: marker 251 + LE u16(255) = [255, 0]
// ---------------------------------------------------------------------------
#[test]
fn test_i32_neg128_zigzag() {
    let bytes = encode_to_vec(&(-128i32)).expect("encode i32 -128");
    // zigzag(-128) = 255, which > 250 → 3-byte varint [251, 255, 0]
    assert_eq!(bytes, &[251, 255, 0]);
}

// ---------------------------------------------------------------------------
// 6. i32 +1 zigzag → [2]
//    zigzag(1) = (1u32 << 1) ^ (0u32) = 2  → varint [2]
// ---------------------------------------------------------------------------
#[test]
fn test_i32_pos1_zigzag() {
    let bytes = encode_to_vec(&(1i32)).expect("encode i32 +1");
    assert_eq!(bytes, &[2]);
}

// ---------------------------------------------------------------------------
// 7. String "hello" → [5, 104, 101, 108, 108, 111]
//    Length 5 as varint (1 byte), then UTF-8 bytes for 'h','e','l','l','o'
// ---------------------------------------------------------------------------
#[test]
fn test_string_hello_encoding() {
    let bytes = encode_to_vec(&"hello").expect("encode &str hello");
    assert_eq!(bytes, &[5, b'h', b'e', b'l', b'l', b'o']);
}

// ---------------------------------------------------------------------------
// 8. String::from("") → [0]
//    Owned empty string: varint length 0, no payload bytes
// ---------------------------------------------------------------------------
#[test]
fn test_string_owned_empty_encoding() {
    let s = String::from("");
    let bytes = encode_to_vec(&s).expect("encode String empty");
    assert_eq!(bytes, &[0u8]);
}

// ---------------------------------------------------------------------------
// 9. Option::<u32>::None → [0]
// ---------------------------------------------------------------------------
#[test]
fn test_option_none_u32() {
    let v: Option<u32> = None;
    let bytes = encode_to_vec(&v).expect("encode Option<u32> None");
    assert_eq!(bytes, &[0]);
}

// ---------------------------------------------------------------------------
// 10. Option::<u32>::Some(42) → [1, 42]
//     Tag byte 1 = Some, then varint(42) = [42]
// ---------------------------------------------------------------------------
#[test]
fn test_option_some_u32_42() {
    let v: Option<u32> = Some(42);
    let bytes = encode_to_vec(&v).expect("encode Option<u32> Some(42)");
    assert_eq!(bytes, &[1, 42]);
}

// ---------------------------------------------------------------------------
// 11. Vec::<u16>::new() → [0]
//     Empty Vec of u16: varint length 0
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u16_empty_encoding() {
    let v: Vec<u16> = Vec::new();
    let bytes = encode_to_vec(&v).expect("encode Vec<u16> empty");
    assert_eq!(bytes, &[0]);
}

// ---------------------------------------------------------------------------
// 12. vec![100u8, 200u8] → [2, 100, 200]
//     Varint length 2, then two u8 values
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_two_elements() {
    let v: Vec<u8> = vec![100, 200];
    let bytes = encode_to_vec(&v).expect("encode Vec<u8> [100,200]");
    assert_eq!(bytes, &[2, 100, 200]);
}

// ---------------------------------------------------------------------------
// 13. [u8; 5] = [10, 20, 30, 40, 50] → [10, 20, 30, 40, 50]
//     Fixed-size array: no length prefix, elements written directly
// ---------------------------------------------------------------------------
#[test]
fn test_array_u8_5_no_length_prefix() {
    let v: [u8; 5] = [10, 20, 30, 40, 50];
    let bytes = encode_to_vec(&v).expect("encode [u8; 5]");
    assert_eq!(bytes, &[10, 20, 30, 40, 50]);
}

// ---------------------------------------------------------------------------
// 14. (1u8, 2u8, 3u8) → [1, 2, 3]
//     3-tuple: fields in order, no separators or length
// ---------------------------------------------------------------------------
#[test]
fn test_tuple_three_u8() {
    let v = (1u8, 2u8, 3u8);
    let bytes = encode_to_vec(&v).expect("encode (u8,u8,u8)");
    assert_eq!(bytes, &[1, 2, 3]);
}

// ---------------------------------------------------------------------------
// 15. Enum unit variant 0 (SmallEnum::A) → [0]
// ---------------------------------------------------------------------------
#[test]
fn test_enum_variant_0_encoding() {
    let bytes = encode_to_vec(&SmallEnum::A).expect("encode SmallEnum::A");
    assert_eq!(bytes, &[0]);
}

// ---------------------------------------------------------------------------
// 16. Enum unit variant 1 (SmallEnum::B) → [1]
// ---------------------------------------------------------------------------
#[test]
fn test_enum_variant_1_encoding() {
    let bytes = encode_to_vec(&SmallEnum::B).expect("encode SmallEnum::B");
    assert_eq!(bytes, &[1]);
}

// ---------------------------------------------------------------------------
// 17. Enum unit variant 250 (Enum252::V250) → [250]
//     Discriminant 250 ≤ 250, encoded as single varint byte
// ---------------------------------------------------------------------------
#[test]
fn test_enum_variant_250_one_byte() {
    let bytes = encode_to_vec(&Enum252::V250).expect("encode Enum252::V250");
    assert_eq!(bytes, &[250]);
}

// ---------------------------------------------------------------------------
// 18. Enum unit variant 251 (Enum252::V251) → [251, 251, 0]
//     Discriminant 251 is in 251–65535 range: 3-byte varint
//     marker 251, then LE u16 251 = [0xFB, 0x00] = [251, 0]
// ---------------------------------------------------------------------------
#[test]
fn test_enum_variant_251_three_bytes() {
    let bytes = encode_to_vec(&Enum252::V251).expect("encode Enum252::V251");
    assert_eq!(bytes, &[251, 251, 0]);
}

// ---------------------------------------------------------------------------
// 19. f32 PI → 4-byte LE IEEE-754
//     std::f32::consts::PI.to_bits() = 0x40490FDB
//     LE bytes = [0xDB, 0x0F, 0x49, 0x40] = [219, 15, 73, 64]
// ---------------------------------------------------------------------------
#[test]
fn test_f32_pi_exact_bytes() {
    let bytes = encode_to_vec(&std::f32::consts::PI).expect("encode f32 PI");
    assert_eq!(bytes.as_slice(), PI_F32_BYTES.as_slice());
}

// ---------------------------------------------------------------------------
// 20. f64 E → 8-byte LE IEEE-754
//     std::f64::consts::E.to_bits() = 0x4005BF0A8B145769
//     LE bytes = [0x69, 0x57, 0x14, 0x8B, 0x0A, 0xBF, 0x05, 0x40]
// ---------------------------------------------------------------------------
#[test]
fn test_f64_e_exact_bytes() {
    let bytes = encode_to_vec(&std::f64::consts::E).expect("encode f64 E");
    assert_eq!(bytes.as_slice(), E_F64_BYTES.as_slice());
}

// ---------------------------------------------------------------------------
// 21. u64 1 with fixed_int encoding → [1, 0, 0, 0, 0, 0, 0, 0]
//     8-byte little-endian, no varint compression
// ---------------------------------------------------------------------------
#[test]
fn test_fixed_int_u64_1_le() {
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&1u64, cfg).expect("encode u64 1 fixed_int");
    assert_eq!(bytes, &[1, 0, 0, 0, 0, 0, 0, 0]);
}

// ---------------------------------------------------------------------------
// 22. u32 1 with big_endian + fixed_int encoding → [0, 0, 0, 1]
//     4-byte big-endian, no varint compression
// ---------------------------------------------------------------------------
#[test]
fn test_big_endian_fixed_u32_1() {
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let bytes = encode_to_vec_with_config(&1u32, cfg).expect("encode u32 1 BE fixed_int");
    assert_eq!(bytes, &[0, 0, 0, 1]);
}
