//! Advanced tests (set 2) for `#[oxicode(encode_with = "fn")]` and
//! `#[oxicode(decode_with = "fn")]` field-level attributes.
//! 22 tests covering patterns complementary to encode_with_advanced_test.rs.

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
use oxicode::de::Decoder;
use oxicode::enc::Encoder;
use oxicode::error::Error;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Test 1: u8 stored as u16 (widening store)
// ---------------------------------------------------------------------------

fn b2_01_encode_u8_as_u16<E: Encoder>(val: &u8, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    (*val as u16).encode(encoder)
}

fn b2_01_decode_u16_as_u8<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u8, Error> {
    use oxicode::de::Decode;
    let wide = u16::decode(decoder)?;
    Ok(wide as u8)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2WidenU8 {
    #[oxicode(
        encode_with = "b2_01_encode_u8_as_u16",
        decode_with = "b2_01_decode_u16_as_u8"
    )]
    byte_val: u8,
}

#[test]
fn test_b2_01_u8_stored_as_u16_roundtrip() {
    for v in [0u8, 1, 127, 200, 255] {
        let original = B2WidenU8 { byte_val: v };
        let enc = encode_to_vec(&original).expect("encode u8-as-u16");
        let (decoded, _): (B2WidenU8, usize) = decode_from_slice(&enc).expect("decode u8-as-u16");
        assert_eq!(decoded.byte_val, v, "roundtrip failed for byte_val={v}");
    }
}

// ---------------------------------------------------------------------------
// Test 2: bool encoded as u8 (0/1) instead of native bool encoding
// ---------------------------------------------------------------------------

fn b2_02_encode_bool_as_u8<E: Encoder>(val: &bool, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    let byte: u8 = if *val { 1 } else { 0 };
    byte.encode(encoder)
}

fn b2_02_decode_u8_as_bool<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<bool, Error> {
    use oxicode::de::Decode;
    let byte = u8::decode(decoder)?;
    Ok(byte != 0)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2BoolAsU8 {
    #[oxicode(
        encode_with = "b2_02_encode_bool_as_u8",
        decode_with = "b2_02_decode_u8_as_bool"
    )]
    flag: bool,
    label: String,
}

#[test]
fn test_b2_02_bool_as_u8_roundtrip() {
    for flag in [false, true] {
        let original = B2BoolAsU8 {
            flag,
            label: "check".into(),
        };
        let enc = encode_to_vec(&original).expect("encode bool-as-u8");
        let (decoded, _): (B2BoolAsU8, usize) = decode_from_slice(&enc).expect("decode bool-as-u8");
        assert_eq!(decoded.flag, flag, "flag mismatch");
        assert_eq!(decoded.label, "check");
    }
}

// ---------------------------------------------------------------------------
// Test 3: i64 with absolute-value negation (abs on encode, restore sign on decode)
// ---------------------------------------------------------------------------

fn b2_03_encode_i64_abs<E: Encoder>(val: &i64, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    let sign: u8 = if *val < 0 { 1 } else { 0 };
    let abs_val: u64 = val.unsigned_abs();
    sign.encode(encoder)?;
    abs_val.encode(encoder)
}

fn b2_03_decode_i64_abs<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<i64, Error> {
    use oxicode::de::Decode;
    let sign = u8::decode(decoder)?;
    let abs_val = u64::decode(decoder)?;
    let magnitude = abs_val as i64;
    Ok(if sign == 1 { -magnitude } else { magnitude })
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2SignedAbs {
    #[oxicode(
        encode_with = "b2_03_encode_i64_abs",
        decode_with = "b2_03_decode_i64_abs"
    )]
    count: i64,
}

#[test]
fn test_b2_03_i64_abs_sign_roundtrip() {
    for v in [0i64, 1, -1, i64::MAX, i64::MIN + 1, -9999, 42000] {
        let original = B2SignedAbs { count: v };
        let enc = encode_to_vec(&original).expect("encode i64-abs");
        let (decoded, _): (B2SignedAbs, usize) = decode_from_slice(&enc).expect("decode i64-abs");
        assert_eq!(decoded.count, v, "i64 roundtrip failed for v={v}");
    }
}

// ---------------------------------------------------------------------------
// Test 4: String stored as reversed bytes
// ---------------------------------------------------------------------------

#[allow(clippy::ptr_arg)]
fn b2_04_encode_str_reversed<E: Encoder>(val: &String, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    let reversed: String = val.chars().rev().collect();
    reversed.encode(encoder)
}

fn b2_04_decode_str_reversed<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<String, Error> {
    use oxicode::de::Decode;
    let s = String::decode(decoder)?;
    Ok(s.chars().rev().collect())
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2ReversedStr {
    id: u32,
    #[oxicode(
        encode_with = "b2_04_encode_str_reversed",
        decode_with = "b2_04_decode_str_reversed"
    )]
    name: String,
}

#[test]
fn test_b2_04_string_reversed_on_wire_roundtrip() {
    let original = B2ReversedStr {
        id: 42,
        name: "Rust".into(),
    };
    let enc = encode_to_vec(&original).expect("encode reversed str");
    let (decoded, _): (B2ReversedStr, usize) =
        decode_from_slice(&enc).expect("decode reversed str");
    assert_eq!(decoded.id, 42);
    assert_eq!(decoded.name, "Rust");
}

// ---------------------------------------------------------------------------
// Test 5: Three custom fields in one struct
// ---------------------------------------------------------------------------

fn b2_05_encode_negate_u32<E: Encoder>(val: &u32, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    val.wrapping_neg().encode(encoder)
}

fn b2_05_decode_negate_u32<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u32, Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v.wrapping_neg())
}

fn b2_05_encode_upper<E: Encoder>(val: &String, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    val.to_uppercase().encode(encoder)
}

fn b2_05_decode_lower<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<String, Error> {
    use oxicode::de::Decode;
    let s = String::decode(decoder)?;
    Ok(s.to_lowercase())
}

fn b2_05_encode_bool_inv<E: Encoder>(val: &bool, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    (!val).encode(encoder)
}

fn b2_05_decode_bool_inv<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<bool, Error> {
    use oxicode::de::Decode;
    let v = bool::decode(decoder)?;
    Ok(!v)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2ThreeCustomFields {
    #[oxicode(
        encode_with = "b2_05_encode_negate_u32",
        decode_with = "b2_05_decode_negate_u32"
    )]
    score: u32,
    #[oxicode(encode_with = "b2_05_encode_upper", decode_with = "b2_05_decode_lower")]
    category: String,
    #[oxicode(
        encode_with = "b2_05_encode_bool_inv",
        decode_with = "b2_05_decode_bool_inv"
    )]
    active: bool,
}

#[test]
fn test_b2_05_three_custom_fields_roundtrip() {
    let original = B2ThreeCustomFields {
        score: 500,
        category: "alpha".into(),
        active: true,
    };
    let enc = encode_to_vec(&original).expect("encode three fields");
    let (decoded, _): (B2ThreeCustomFields, usize) =
        decode_from_slice(&enc).expect("decode three fields");
    assert_eq!(decoded.score, 500);
    assert_eq!(decoded.category, "alpha");
    assert_eq!(decoded.active, true);
}

// ---------------------------------------------------------------------------
// Test 6: Vec<u32> stored sorted ascending
// ---------------------------------------------------------------------------

fn b2_06_encode_sorted_vec<E: Encoder>(val: &Vec<u32>, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    let mut sorted = val.clone();
    sorted.sort_unstable();
    sorted.encode(encoder)
}

fn b2_06_decode_sorted_vec<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Vec<u32>, Error> {
    use oxicode::de::Decode;
    Vec::<u32>::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2SortedVec {
    #[oxicode(
        encode_with = "b2_06_encode_sorted_vec",
        decode_with = "b2_06_decode_sorted_vec"
    )]
    numbers: Vec<u32>,
}

#[test]
fn test_b2_06_vec_u32_stored_sorted_roundtrip() {
    let original = B2SortedVec {
        numbers: vec![30, 10, 20, 5, 25],
    };
    let enc = encode_to_vec(&original).expect("encode sorted vec");
    let (decoded, _): (B2SortedVec, usize) = decode_from_slice(&enc).expect("decode sorted vec");
    // After encode (sort) + decode, result is sorted
    assert_eq!(decoded.numbers, vec![5, 10, 20, 25, 30]);
}

// ---------------------------------------------------------------------------
// Test 7: u128 split into four u32 fields
// ---------------------------------------------------------------------------

fn b2_07_encode_u128_quad<E: Encoder>(val: &u128, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    let a = (*val >> 96) as u32;
    let b = (*val >> 64) as u32;
    let c = (*val >> 32) as u32;
    let d = *val as u32;
    a.encode(encoder)?;
    b.encode(encoder)?;
    c.encode(encoder)?;
    d.encode(encoder)
}

fn b2_07_decode_u128_quad<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u128, Error> {
    use oxicode::de::Decode;
    let a = u32::decode(decoder)? as u128;
    let b = u32::decode(decoder)? as u128;
    let c = u32::decode(decoder)? as u128;
    let d = u32::decode(decoder)? as u128;
    Ok((a << 96) | (b << 64) | (c << 32) | d)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2U128Quad {
    #[oxicode(
        encode_with = "b2_07_encode_u128_quad",
        decode_with = "b2_07_decode_u128_quad"
    )]
    big_id: u128,
}

#[test]
fn test_b2_07_u128_as_four_u32_roundtrip() {
    let original = B2U128Quad {
        big_id: 0x0102_0304_0506_0708_0900_0A0B_0C0D_0E0Fu128,
    };
    let enc = encode_to_vec(&original).expect("encode u128-quad");
    let (decoded, _): (B2U128Quad, usize) = decode_from_slice(&enc).expect("decode u128-quad");
    assert_eq!(
        decoded.big_id,
        0x0102_0304_0506_0708_0900_0A0B_0C0D_0E0Fu128
    );
}

// ---------------------------------------------------------------------------
// Test 8: (u8, u8, u8) tuple stored as a single u32 (packed RGB)
// ---------------------------------------------------------------------------

fn b2_08_encode_rgb_packed<E: Encoder>(val: &(u8, u8, u8), encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    let packed: u32 = ((val.0 as u32) << 16) | ((val.1 as u32) << 8) | (val.2 as u32);
    packed.encode(encoder)
}

fn b2_08_decode_rgb_packed<D: Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<(u8, u8, u8), Error> {
    use oxicode::de::Decode;
    let packed = u32::decode(decoder)?;
    let r = ((packed >> 16) & 0xFF) as u8;
    let g = ((packed >> 8) & 0xFF) as u8;
    let b_val = (packed & 0xFF) as u8;
    Ok((r, g, b_val))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2RgbColor {
    #[oxicode(
        encode_with = "b2_08_encode_rgb_packed",
        decode_with = "b2_08_decode_rgb_packed"
    )]
    color: (u8, u8, u8),
    name: String,
}

#[test]
fn test_b2_08_rgb_tuple_as_packed_u32_roundtrip() {
    let original = B2RgbColor {
        color: (255, 128, 0),
        name: "orange".into(),
    };
    let enc = encode_to_vec(&original).expect("encode rgb");
    let (decoded, _): (B2RgbColor, usize) = decode_from_slice(&enc).expect("decode rgb");
    assert_eq!(decoded.color, (255, 128, 0));
    assert_eq!(decoded.name, "orange");
}

// ---------------------------------------------------------------------------
// Test 9: Encode only (no decode_with) — encode_with alongside normal field
// ---------------------------------------------------------------------------

fn b2_09_encode_multiply_five<E: Encoder>(val: &u32, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    val.saturating_mul(5).encode(encoder)
}

fn b2_09_decode_divide_five<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u32, Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v / 5)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2MultiplyFive {
    regular: String,
    #[oxicode(
        encode_with = "b2_09_encode_multiply_five",
        decode_with = "b2_09_decode_divide_five"
    )]
    factor: u32,
    extra: u64,
}

#[test]
fn test_b2_09_multiply_five_with_regular_fields() {
    let original = B2MultiplyFive {
        regular: "hello".into(),
        factor: 7,
        extra: 9999,
    };
    let enc = encode_to_vec(&original).expect("encode multiply-five");
    let (decoded, _): (B2MultiplyFive, usize) =
        decode_from_slice(&enc).expect("decode multiply-five");
    assert_eq!(decoded.regular, "hello");
    assert_eq!(decoded.factor, 7);
    assert_eq!(decoded.extra, 9999);
}

// ---------------------------------------------------------------------------
// Test 10: Empty Vec<u8> roundtrip through custom framing
// ---------------------------------------------------------------------------

fn b2_10_encode_empty_framed<E: Encoder>(val: &Vec<u8>, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    (val.len() as u32).encode(encoder)?;
    for &b in val.iter() {
        b.encode(encoder)?;
    }
    Ok(())
}

fn b2_10_decode_empty_framed<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Vec<u8>, Error> {
    use oxicode::de::Decode;
    let len = u32::decode(decoder)? as usize;
    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        out.push(u8::decode(decoder)?);
    }
    Ok(out)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2EmptyVec {
    #[oxicode(
        encode_with = "b2_10_encode_empty_framed",
        decode_with = "b2_10_decode_empty_framed"
    )]
    payload: Vec<u8>,
}

#[test]
fn test_b2_10_empty_vec_custom_framing_roundtrip() {
    let original = B2EmptyVec { payload: vec![] };
    let enc = encode_to_vec(&original).expect("encode empty vec");
    let (decoded, _): (B2EmptyVec, usize) = decode_from_slice(&enc).expect("decode empty vec");
    assert_eq!(decoded.payload, vec![]);
}

// ---------------------------------------------------------------------------
// Test 11: f32 encoded as u32 bit pattern
// ---------------------------------------------------------------------------

fn b2_11_encode_f32_bits<E: Encoder>(val: &f32, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    val.to_bits().encode(encoder)
}

fn b2_11_decode_f32_bits<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<f32, Error> {
    use oxicode::de::Decode;
    let bits = u32::decode(decoder)?;
    Ok(f32::from_bits(bits))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2F32Bits {
    #[oxicode(
        encode_with = "b2_11_encode_f32_bits",
        decode_with = "b2_11_decode_f32_bits"
    )]
    temperature: f32,
    unit: String,
}

#[test]
fn test_b2_11_f32_as_u32_bits_roundtrip() {
    let val: f32 = 3.14_f32;
    let original = B2F32Bits {
        temperature: val,
        unit: "celsius".into(),
    };
    let enc = encode_to_vec(&original).expect("encode f32-bits");
    let (decoded, _): (B2F32Bits, usize) = decode_from_slice(&enc).expect("decode f32-bits");
    assert_eq!(
        decoded.temperature.to_bits(),
        val.to_bits(),
        "f32 bit pattern mismatch"
    );
    assert_eq!(decoded.unit, "celsius");
}

// ---------------------------------------------------------------------------
// Test 12: String trimmed + truncated to max 10 chars
// ---------------------------------------------------------------------------

#[allow(clippy::ptr_arg)]
fn b2_12_encode_truncated<E: Encoder>(val: &String, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    let trimmed = val.trim();
    let truncated: String = trimmed.chars().take(10).collect();
    truncated.encode(encoder)
}

fn b2_12_decode_passthrough<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<String, Error> {
    use oxicode::de::Decode;
    String::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2TruncatedStr {
    #[oxicode(
        encode_with = "b2_12_encode_truncated",
        decode_with = "b2_12_decode_passthrough"
    )]
    description: String,
}

#[test]
fn test_b2_12_string_trimmed_and_truncated() {
    let original = B2TruncatedStr {
        description: "  Hello World Extra  ".into(),
    };
    let enc = encode_to_vec(&original).expect("encode truncated");
    let (decoded, _): (B2TruncatedStr, usize) = decode_from_slice(&enc).expect("decode truncated");
    // trim + take(10) on "Hello World Extra" → "Hello Worl"
    assert_eq!(decoded.description, "Hello Worl");
}

// ---------------------------------------------------------------------------
// Test 13: u32 zero value encoded explicitly
// ---------------------------------------------------------------------------

fn b2_13_encode_or_sentinel<E: Encoder>(val: &u32, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    // sentinel: 0 is stored as u32::MAX on wire
    let wire = if *val == 0 { u32::MAX } else { *val };
    wire.encode(encoder)
}

fn b2_13_decode_or_sentinel<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u32, Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(if v == u32::MAX { 0 } else { v })
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2SentinelZero {
    #[oxicode(
        encode_with = "b2_13_encode_or_sentinel",
        decode_with = "b2_13_decode_or_sentinel"
    )]
    code: u32,
}

#[test]
fn test_b2_13_zero_encoded_as_sentinel_roundtrip() {
    for v in [0u32, 1, 100, u32::MAX - 1] {
        let original = B2SentinelZero { code: v };
        let enc = encode_to_vec(&original).expect("encode sentinel");
        let (decoded, _): (B2SentinelZero, usize) =
            decode_from_slice(&enc).expect("decode sentinel");
        assert_eq!(decoded.code, v, "sentinel roundtrip failed for v={v}");
    }
}

// ---------------------------------------------------------------------------
// Test 14: Struct with 5 fields — first and last use encode_with
// ---------------------------------------------------------------------------

fn b2_14_encode_inc<E: Encoder>(val: &u32, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    val.saturating_add(1).encode(encoder)
}

fn b2_14_decode_dec<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u32, Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v.saturating_sub(1))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2FiveFields {
    #[oxicode(encode_with = "b2_14_encode_inc", decode_with = "b2_14_decode_dec")]
    first: u32,
    a: String,
    b: u64,
    c: Vec<u8>,
    #[oxicode(encode_with = "b2_14_encode_inc", decode_with = "b2_14_decode_dec")]
    last: u32,
}

#[test]
fn test_b2_14_first_and_last_field_custom_encode() {
    let original = B2FiveFields {
        first: 10,
        a: "mid".into(),
        b: 123456,
        c: vec![1, 2, 3],
        last: 99,
    };
    let enc = encode_to_vec(&original).expect("encode five-fields");
    let (decoded, _): (B2FiveFields, usize) = decode_from_slice(&enc).expect("decode five-fields");
    assert_eq!(decoded.first, 10);
    assert_eq!(decoded.a, "mid");
    assert_eq!(decoded.b, 123456);
    assert_eq!(decoded.c, vec![1, 2, 3]);
    assert_eq!(decoded.last, 99);
}

// ---------------------------------------------------------------------------
// Test 15: Vec<String> deduplicated on encode
// ---------------------------------------------------------------------------

fn b2_15_encode_dedup<E: Encoder>(val: &Vec<String>, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    let mut seen = std::collections::HashSet::new();
    let deduped: Vec<String> = val
        .iter()
        .filter(|s| seen.insert((*s).clone()))
        .cloned()
        .collect();
    deduped.encode(encoder)
}

fn b2_15_decode_dedup<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Vec<String>, Error> {
    use oxicode::de::Decode;
    Vec::<String>::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2DedupStrings {
    #[oxicode(encode_with = "b2_15_encode_dedup", decode_with = "b2_15_decode_dedup")]
    items: Vec<String>,
}

#[test]
fn test_b2_15_vec_string_deduped_on_encode() {
    let original = B2DedupStrings {
        items: vec!["a".into(), "b".into(), "a".into(), "c".into(), "b".into()],
    };
    let enc = encode_to_vec(&original).expect("encode dedup");
    let (decoded, _): (B2DedupStrings, usize) = decode_from_slice(&enc).expect("decode dedup");
    // Only unique items survive, in first-occurrence order
    assert_eq!(decoded.items, vec!["a", "b", "c"]);
}

// ---------------------------------------------------------------------------
// Test 16: Custom error returned by decode_with (invalid data)
// ---------------------------------------------------------------------------

fn b2_16_encode_restricted<E: Encoder>(val: &u32, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    val.encode(encoder)
}

fn b2_16_decode_restricted<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u32, Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    if v > 1000 {
        Err(Error::InvalidData {
            message: "value exceeds limit of 1000",
        })
    } else {
        Ok(v)
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2RestrictedU32 {
    #[oxicode(
        encode_with = "b2_16_encode_restricted",
        decode_with = "b2_16_decode_restricted"
    )]
    bounded: u32,
}

#[test]
fn test_b2_16_decode_with_returns_error_on_invalid_data() {
    // Encode a value > 1000 (bypasses the restriction)
    let original = B2RestrictedU32 { bounded: 1001 };
    let enc = encode_to_vec(&original).expect("encode restricted");
    // Decoding should fail with an error
    let result: Result<(B2RestrictedU32, usize), _> = decode_from_slice(&enc);
    assert!(result.is_err(), "expected decode error for value > 1000");
}

// ---------------------------------------------------------------------------
// Test 17: [u8; 8] fixed array rotated left by 1 on encode
// ---------------------------------------------------------------------------

fn b2_17_encode_rotate_array<E: Encoder>(val: &[u8; 8], encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    let mut rotated = *val;
    rotated.rotate_left(1);
    rotated.encode(encoder)
}

fn b2_17_decode_rotate_array<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<[u8; 8], Error> {
    use oxicode::de::Decode;
    let mut arr = <[u8; 8]>::decode(decoder)?;
    arr.rotate_right(1);
    Ok(arr)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2RotatedArray {
    #[oxicode(
        encode_with = "b2_17_encode_rotate_array",
        decode_with = "b2_17_decode_rotate_array"
    )]
    key: [u8; 8],
}

#[test]
fn test_b2_17_fixed_array_rotated_roundtrip() {
    let original = B2RotatedArray {
        key: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
    };
    let enc = encode_to_vec(&original).expect("encode rotate-array");
    let (decoded, _): (B2RotatedArray, usize) =
        decode_from_slice(&enc).expect("decode rotate-array");
    assert_eq!(
        decoded.key,
        [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]
    );
}

// ---------------------------------------------------------------------------
// Test 18: Multiple encode_with on same logical type (different instances)
// ---------------------------------------------------------------------------

fn b2_18_encode_x2<E: Encoder>(val: &u32, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    val.saturating_mul(2).encode(encoder)
}

fn b2_18_decode_x2<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u32, Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v / 2)
}

fn b2_18_encode_x3<E: Encoder>(val: &u32, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    val.saturating_mul(3).encode(encoder)
}

fn b2_18_decode_x3<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u32, Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v / 3)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2TwoMultiplied {
    #[oxicode(encode_with = "b2_18_encode_x2", decode_with = "b2_18_decode_x2")]
    doubled: u32,
    #[oxicode(encode_with = "b2_18_encode_x3", decode_with = "b2_18_decode_x3")]
    tripled: u32,
}

#[test]
fn test_b2_18_two_fields_different_multipliers() {
    let original = B2TwoMultiplied {
        doubled: 10,
        tripled: 9,
    };
    let enc = encode_to_vec(&original).expect("encode two-multiplied");
    let (decoded, _): (B2TwoMultiplied, usize) =
        decode_from_slice(&enc).expect("decode two-multiplied");
    assert_eq!(decoded.doubled, 10);
    assert_eq!(decoded.tripled, 9);
}

// ---------------------------------------------------------------------------
// Test 19: SystemTime seconds-only encoding via encode_with
// ---------------------------------------------------------------------------

fn b2_19_encode_systime_secs<E: Encoder>(
    val: &std::time::SystemTime,
    encoder: &mut E,
) -> Result<(), Error> {
    use oxicode::enc::Encode;
    let secs = val
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    secs.encode(encoder)
}

fn b2_19_decode_systime_secs<D: Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<std::time::SystemTime, Error> {
    use oxicode::de::Decode;
    let secs = u64::decode(decoder)?;
    Ok(std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs))
}

#[derive(Debug, Encode, Decode)]
struct B2SystemTimeEncoded {
    #[oxicode(
        encode_with = "b2_19_encode_systime_secs",
        decode_with = "b2_19_decode_systime_secs"
    )]
    timestamp: std::time::SystemTime,
}

#[test]
fn test_b2_19_systemtime_seconds_only_roundtrip() {
    let secs = 1_700_000_000u64;
    let ts = std::time::UNIX_EPOCH + std::time::Duration::from_secs(secs);
    let original = B2SystemTimeEncoded { timestamp: ts };
    let enc = encode_to_vec(&original).expect("encode systime");
    let (decoded, _): (B2SystemTimeEncoded, usize) =
        decode_from_slice(&enc).expect("decode systime");
    let decoded_secs = decoded
        .timestamp
        .duration_since(std::time::UNIX_EPOCH)
        .expect("duration since epoch")
        .as_secs();
    assert_eq!(decoded_secs, secs);
}

// ---------------------------------------------------------------------------
// Test 20: encode_with on an Option field — Some wraps a complex type
// ---------------------------------------------------------------------------

fn b2_20_encode_option_u64<E: Encoder>(val: &Option<u64>, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    // Store as: flag byte + value (0 if None)
    match val {
        None => {
            0u8.encode(encoder)?;
            0u64.encode(encoder)
        }
        Some(v) => {
            1u8.encode(encoder)?;
            v.encode(encoder)
        }
    }
}

fn b2_20_decode_option_u64<D: Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Option<u64>, Error> {
    use oxicode::de::Decode;
    let flag = u8::decode(decoder)?;
    let v = u64::decode(decoder)?;
    Ok(if flag == 0 { None } else { Some(v) })
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2OptionU64 {
    #[oxicode(
        encode_with = "b2_20_encode_option_u64",
        decode_with = "b2_20_decode_option_u64"
    )]
    maybe_id: Option<u64>,
    label: String,
}

#[test]
fn test_b2_20_option_u64_custom_flag_byte_roundtrip() {
    let none_case = B2OptionU64 {
        maybe_id: None,
        label: "none".into(),
    };
    let enc = encode_to_vec(&none_case).expect("encode none-u64");
    let (decoded, _): (B2OptionU64, usize) = decode_from_slice(&enc).expect("decode none-u64");
    assert_eq!(decoded.maybe_id, None);
    assert_eq!(decoded.label, "none");

    let some_case = B2OptionU64 {
        maybe_id: Some(42_000_000),
        label: "some".into(),
    };
    let enc2 = encode_to_vec(&some_case).expect("encode some-u64");
    let (decoded2, _): (B2OptionU64, usize) = decode_from_slice(&enc2).expect("decode some-u64");
    assert_eq!(decoded2.maybe_id, Some(42_000_000));
    assert_eq!(decoded2.label, "some");
}

// ---------------------------------------------------------------------------
// Test 21: Generic struct with two type params, encode_with on second field
// ---------------------------------------------------------------------------

fn b2_21_encode_tag_prefix<E: Encoder>(val: &String, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    let prefixed = format!("tag:{}", val);
    prefixed.encode(encoder)
}

fn b2_21_decode_tag_strip<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<String, Error> {
    use oxicode::de::Decode;
    let s = String::decode(decoder)?;
    Ok(s.strip_prefix("tag:").unwrap_or(&s).to_string())
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2GenericTwo<A: Encode + Decode, B: Encode + Decode> {
    first: A,
    second: B,
    #[oxicode(
        encode_with = "b2_21_encode_tag_prefix",
        decode_with = "b2_21_decode_tag_strip"
    )]
    label: String,
}

#[test]
fn test_b2_21_generic_two_type_params_with_encode_with() {
    let original = B2GenericTwo::<u32, u64> {
        first: 7,
        second: 9999,
        label: "hello".into(),
    };
    let enc = encode_to_vec(&original).expect("encode generic-two");
    let (decoded, _): (B2GenericTwo<u32, u64>, usize) =
        decode_from_slice(&enc).expect("decode generic-two");
    assert_eq!(decoded.first, 7);
    assert_eq!(decoded.second, 9999);
    // prefix added on encode, stripped on decode
    assert_eq!(decoded.label, "hello");
}

// ---------------------------------------------------------------------------
// Test 22: Wire-format verification — manually verify what encode_with writes
// ---------------------------------------------------------------------------

fn b2_22_encode_be_u32<E: Encoder>(val: &u32, encoder: &mut E) -> Result<(), Error> {
    use oxicode::enc::Encode;
    // Store each byte of the big-endian representation as separate u8
    let be = val.to_be_bytes();
    be[0].encode(encoder)?;
    be[1].encode(encoder)?;
    be[2].encode(encoder)?;
    be[3].encode(encoder)
}

fn b2_22_decode_be_u32<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u32, Error> {
    use oxicode::de::Decode;
    let b0 = u8::decode(decoder)?;
    let b1 = u8::decode(decoder)?;
    let b2 = u8::decode(decoder)?;
    let b3 = u8::decode(decoder)?;
    Ok(u32::from_be_bytes([b0, b1, b2, b3]))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B2BeU32 {
    #[oxicode(
        encode_with = "b2_22_encode_be_u32",
        decode_with = "b2_22_decode_be_u32"
    )]
    value: u32,
}

#[test]
fn test_b2_22_big_endian_manual_wire_verification() {
    let value = 0x01_02_03_04u32;
    let original = B2BeU32 { value };
    let enc = encode_to_vec(&original).expect("encode be-u32");

    // The encoding writes 4 separate u8 bytes (varint-encoded), verify roundtrip
    let (decoded, consumed): (B2BeU32, usize) = decode_from_slice(&enc).expect("decode be-u32");
    assert_eq!(decoded.value, value);
    assert_eq!(consumed, enc.len(), "all bytes should be consumed");

    // Manually encode 4 separate u8 values and compare total byte content
    let manual: Vec<u8> = {
        let b = value.to_be_bytes();
        let mut parts = encode_to_vec(&b[0]).expect("enc b0");
        parts.extend(encode_to_vec(&b[1]).expect("enc b1"));
        parts.extend(encode_to_vec(&b[2]).expect("enc b2"));
        parts.extend(encode_to_vec(&b[3]).expect("enc b3"));
        parts
    };
    assert_eq!(
        enc, manual,
        "struct encoding should match manually encoded big-endian bytes"
    );
}
