//! Advanced tests for `#[oxicode(encode_with = "fn")]` and `#[oxicode(decode_with = "fn")]`
//! field-level attributes. 22 tests covering a wide range of custom serialization patterns.

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

// ---------------------------------------------------------------------------
// Test 1: double/halve a u32
// ---------------------------------------------------------------------------

fn adv01_encode_doubled<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.saturating_mul(2).encode(encoder)
}

fn adv01_decode_halved<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v / 2)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvDoubled {
    #[oxicode(
        encode_with = "adv01_encode_doubled",
        decode_with = "adv01_decode_halved"
    )]
    amount: u32,
}

#[test]
fn test_adv01_double_halve_u32_roundtrip() {
    let original = AdvDoubled { amount: 100 };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvDoubled, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.amount, 100);
}

// ---------------------------------------------------------------------------
// Test 2: uppercase on encode, lowercase on decode
// ---------------------------------------------------------------------------

#[allow(clippy::ptr_arg)]
fn adv02_encode_upper<E: oxicode::enc::Encoder>(
    val: &String,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.to_uppercase().encode(encoder)
}

fn adv02_decode_lower<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    let s = String::decode(decoder)?;
    Ok(s.to_lowercase())
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvCaseStr {
    #[oxicode(encode_with = "adv02_encode_upper", decode_with = "adv02_decode_lower")]
    text: String,
}

#[test]
fn test_adv02_uppercase_string_roundtrip() {
    let original = AdvCaseStr {
        text: "hello".into(),
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvCaseStr, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    // upper → lower = identity for ASCII
    assert_eq!(decoded.text, "hello");
}

// ---------------------------------------------------------------------------
// Test 3: Option<u32> — None as 0, Some(n) as n+1
// ---------------------------------------------------------------------------

fn adv03_encode_opt<E: oxicode::enc::Encoder>(
    val: &Option<u32>,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let wire: u32 = match val {
        None => 0,
        Some(n) => n.saturating_add(1),
    };
    wire.encode(encoder)
}

fn adv03_decode_opt<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Option<u32>, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(if v == 0 { None } else { Some(v - 1) })
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvOptU32 {
    #[oxicode(encode_with = "adv03_encode_opt", decode_with = "adv03_decode_opt")]
    val: Option<u32>,
}

#[test]
fn test_adv03_option_u32_none_as_zero() {
    let none_case = AdvOptU32 { val: None };
    let bytes = oxicode::encode_to_vec(&none_case).expect("encode none");
    let (decoded, _): (AdvOptU32, _) = oxicode::decode_from_slice(&bytes).expect("decode none");
    assert_eq!(decoded.val, None);

    let some_case = AdvOptU32 { val: Some(42) };
    let bytes2 = oxicode::encode_to_vec(&some_case).expect("encode some");
    let (decoded2, _): (AdvOptU32, _) = oxicode::decode_from_slice(&bytes2).expect("decode some");
    assert_eq!(decoded2.val, Some(42));
}

// ---------------------------------------------------------------------------
// Test 4: u64 encoded as two u32s (hi/lo split)
// ---------------------------------------------------------------------------

fn adv04_encode_u64_split<E: oxicode::enc::Encoder>(
    val: &u64,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let hi = (*val >> 32) as u32;
    let lo = *val as u32;
    hi.encode(encoder)?;
    lo.encode(encoder)
}

fn adv04_decode_u64_split<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u64, oxicode::error::Error> {
    use oxicode::de::Decode;
    let hi = u32::decode(decoder)? as u64;
    let lo = u32::decode(decoder)? as u64;
    Ok((hi << 32) | lo)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvU64Split {
    #[oxicode(
        encode_with = "adv04_encode_u64_split",
        decode_with = "adv04_decode_u64_split"
    )]
    value: u64,
}

#[test]
fn test_adv04_u64_as_two_u32s_roundtrip() {
    let original = AdvU64Split {
        value: 0xDEAD_BEEF_CAFE_F00D,
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvU64Split, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.value, 0xDEAD_BEEF_CAFE_F00D);
}

// ---------------------------------------------------------------------------
// Test 5: bijective zero-skip (val + 1 on encode, val - 1 on decode)
// ---------------------------------------------------------------------------

fn adv05_encode_shift<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.saturating_add(1).encode(encoder)
}

fn adv05_decode_shift<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v.saturating_sub(1))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvZeroSkip {
    #[oxicode(encode_with = "adv05_encode_shift", decode_with = "adv05_decode_shift")]
    code: u32,
}

#[test]
fn test_adv05_bijective_zero_skip_roundtrip() {
    for val in [0u32, 1, 100, u32::MAX - 1] {
        let original = AdvZeroSkip { code: val };
        let bytes = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (AdvZeroSkip, _) = oxicode::decode_from_slice(&bytes).expect("decode");
        assert_eq!(decoded.code, val, "roundtrip failed for val={val}");
    }
}

// ---------------------------------------------------------------------------
// Test 6: Vec<u8> with manual length-prefix framing
// ---------------------------------------------------------------------------

fn adv06_encode_vec_framed<E: oxicode::enc::Encoder>(
    val: &[u8],
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    (val.len() as u32).encode(encoder)?;
    for &b in val {
        b.encode(encoder)?;
    }
    Ok(())
}

fn adv06_decode_vec_framed<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Vec<u8>, oxicode::error::Error> {
    use oxicode::de::Decode;
    let len = u32::decode(decoder)? as usize;
    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        out.push(u8::decode(decoder)?);
    }
    Ok(out)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvVecLenSep {
    #[oxicode(
        encode_with = "adv06_encode_vec_framed",
        decode_with = "adv06_decode_vec_framed"
    )]
    data: Vec<u8>,
}

#[test]
fn test_adv06_vec_u8_length_prefix_roundtrip() {
    let original = AdvVecLenSep {
        data: vec![10, 20, 30, 40, 50],
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvVecLenSep, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.data, vec![10, 20, 30, 40, 50]);
}

// ---------------------------------------------------------------------------
// Test 7: Multiple fields, one with custom encode
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvMultiOne {
    name: String,
    #[oxicode(
        encode_with = "adv01_encode_doubled",
        decode_with = "adv01_decode_halved"
    )]
    amount: u32,
    id: u64,
}

#[test]
fn test_adv07_multiple_fields_one_custom() {
    let original = AdvMultiOne {
        name: "test".into(),
        amount: 50,
        id: 999,
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvMultiOne, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.name, "test");
    assert_eq!(decoded.amount, 50);
    assert_eq!(decoded.id, 999);
}

// ---------------------------------------------------------------------------
// Test 8: Explicit encode_with/decode_with pair (both specified, xor-based)
// ---------------------------------------------------------------------------

fn adv08_encode_xor<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    (val ^ 0xFFFF_FFFF).encode(encoder)
}

fn adv08_decode_xor<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v ^ 0xFFFF_FFFF)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvExplicitPair {
    #[oxicode(encode_with = "adv08_encode_xor", decode_with = "adv08_decode_xor")]
    value: u32,
}

#[test]
fn test_adv08_encode_with_explicit_pair_roundtrip() {
    let original = AdvExplicitPair { value: 0xABCD_1234 };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvExplicitPair, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.value, 0xABCD_1234);
}

// ---------------------------------------------------------------------------
// Test 9: Duration encode — secs only, nanos discarded
// ---------------------------------------------------------------------------

fn adv09_encode_duration_secs<E: oxicode::enc::Encoder>(
    val: &std::time::Duration,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.as_secs().encode(encoder)
}

fn adv09_decode_duration_secs<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<std::time::Duration, oxicode::error::Error> {
    use oxicode::de::Decode;
    let secs = u64::decode(decoder)?;
    Ok(std::time::Duration::from_secs(secs))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvTimestamp {
    #[oxicode(
        encode_with = "adv09_encode_duration_secs",
        decode_with = "adv09_decode_duration_secs"
    )]
    ts: std::time::Duration,
    label: String,
}

#[test]
fn test_adv09_timestamp_encode_secs_only() {
    let original = AdvTimestamp {
        ts: std::time::Duration::new(100, 999_999_999),
        label: "event".into(),
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvTimestamp, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    // nanos are discarded
    assert_eq!(decoded.ts, std::time::Duration::from_secs(100));
    assert_eq!(decoded.label, "event");
}

// ---------------------------------------------------------------------------
// Test 10: Checksum embedded via encode_with (legacy fixed-int config)
// ---------------------------------------------------------------------------

fn adv10_encode_with_checksum<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let bytes = val.to_le_bytes();
    let checksum: u8 = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    val.encode(encoder)?;
    checksum.encode(encoder)
}

fn adv10_decode_with_checksum<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    let stored_cs = u8::decode(decoder)?;
    let bytes = v.to_le_bytes();
    let expected_cs: u8 = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    if stored_cs != expected_cs {
        return Err(oxicode::error::Error::InvalidData {
            message: "checksum mismatch",
        });
    }
    Ok(v)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvChecksum {
    #[oxicode(
        encode_with = "adv10_encode_with_checksum",
        decode_with = "adv10_decode_with_checksum"
    )]
    value: u32,
}

#[test]
fn test_adv10_checksum_in_encode_with() {
    let original = AdvChecksum { value: 0xDEAD_BEEF };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvChecksum, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.value, 0xDEAD_BEEF);
}

// ---------------------------------------------------------------------------
// Test 11: i32 as sign-magnitude (u8 sign + u32 abs)
// ---------------------------------------------------------------------------

fn adv11_encode_sign_mag<E: oxicode::enc::Encoder>(
    val: &i32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let sign: u8 = if *val < 0 { 1 } else { 0 };
    let abs: u32 = val.unsigned_abs();
    sign.encode(encoder)?;
    abs.encode(encoder)
}

fn adv11_decode_sign_mag<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<i32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let sign = u8::decode(decoder)?;
    let abs = u32::decode(decoder)?;
    let v = abs as i32;
    Ok(if sign == 1 { -v } else { v })
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvSignMag {
    #[oxicode(
        encode_with = "adv11_encode_sign_mag",
        decode_with = "adv11_decode_sign_mag"
    )]
    val: i32,
}

#[test]
fn test_adv11_negative_as_positive_roundtrip() {
    for v in [-42i32, 0, 100, i32::MAX, i32::MIN + 1] {
        let original = AdvSignMag { val: v };
        let bytes = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (AdvSignMag, _) = oxicode::decode_from_slice(&bytes).expect("decode");
        assert_eq!(decoded.val, v, "roundtrip failed for val={v}");
    }
}

// ---------------------------------------------------------------------------
// Test 12: String encoded as raw UTF-8 Vec<u8>
// ---------------------------------------------------------------------------

#[allow(clippy::ptr_arg)]
fn adv12_encode_str_as_bytes<E: oxicode::enc::Encoder>(
    val: &String,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.as_bytes().to_vec().encode(encoder)
}

fn adv12_decode_str_from_bytes<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    let bytes = Vec::<u8>::decode(decoder)?;
    String::from_utf8(bytes).map_err(|_| oxicode::error::Error::InvalidData {
        message: "invalid UTF-8 in string field",
    })
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvStrAsBytes {
    #[oxicode(
        encode_with = "adv12_encode_str_as_bytes",
        decode_with = "adv12_decode_str_from_bytes"
    )]
    text: String,
}

#[test]
fn test_adv12_string_as_utf8_bytes_roundtrip() {
    let original = AdvStrAsBytes {
        text: "Hello, 世界!".into(),
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvStrAsBytes, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.text, "Hello, 世界!");
}

// ---------------------------------------------------------------------------
// Test 13: Two fields with different encode_with functions
// ---------------------------------------------------------------------------

fn adv13_encode_str_lower<E: oxicode::enc::Encoder>(
    val: &str,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.to_lowercase().encode(encoder)
}

fn adv13_decode_str_pass<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    String::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvTwoCustom {
    #[oxicode(
        encode_with = "adv01_encode_doubled",
        decode_with = "adv01_decode_halved"
    )]
    amount: u32,
    #[oxicode(
        encode_with = "adv13_encode_str_lower",
        decode_with = "adv13_decode_str_pass"
    )]
    tag: String,
}

#[test]
fn test_adv13_two_fields_different_encode_fns() {
    let original = AdvTwoCustom {
        amount: 20,
        tag: "RUST".into(),
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvTwoCustom, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.amount, 20);
    assert_eq!(decoded.tag, "rust");
}

// ---------------------------------------------------------------------------
// Test 14: Nested struct — outer uses encode_with on inner struct field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvInner {
    id: u32,
    name: String,
}

fn adv14_encode_inner_as_id<E: oxicode::enc::Encoder>(
    val: &AdvInner,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.id.encode(encoder)
}

fn adv14_decode_inner_from_id<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<AdvInner, oxicode::error::Error> {
    use oxicode::de::Decode;
    let id = u32::decode(decoder)?;
    Ok(AdvInner {
        id,
        name: "unknown".into(),
    })
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvOuter {
    #[oxicode(
        encode_with = "adv14_encode_inner_as_id",
        decode_with = "adv14_decode_inner_from_id"
    )]
    inner: AdvInner,
    desc: String,
}

#[test]
fn test_adv14_nested_outer_uses_encode_with_on_inner_field() {
    let original = AdvOuter {
        inner: AdvInner {
            id: 77,
            name: "important".into(),
        },
        desc: "outer description".into(),
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvOuter, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    // inner.name is lost — only id is preserved
    assert_eq!(decoded.inner.id, 77);
    assert_eq!(decoded.inner.name, "unknown");
    assert_eq!(decoded.desc, "outer description");
}

// ---------------------------------------------------------------------------
// Test 15: f64 encoded as u64 bit pattern
// ---------------------------------------------------------------------------

fn adv15_encode_f64_bits<E: oxicode::enc::Encoder>(
    val: &f64,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.to_bits().encode(encoder)
}

fn adv15_decode_f64_bits<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<f64, oxicode::error::Error> {
    use oxicode::de::Decode;
    let bits = u64::decode(decoder)?;
    Ok(f64::from_bits(bits))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvF64Bits {
    #[oxicode(
        encode_with = "adv15_encode_f64_bits",
        decode_with = "adv15_decode_f64_bits"
    )]
    value: f64,
}

#[test]
fn test_adv15_f64_as_u64_bits_roundtrip() {
    let original = AdvF64Bits {
        value: std::f64::consts::PI,
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvF64Bits, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    // Bit-exact comparison via to_bits
    assert_eq!(decoded.value.to_bits(), std::f64::consts::PI.to_bits());
}

// ---------------------------------------------------------------------------
// Test 16: Vec<String> encoded as single "|"-joined string
// ---------------------------------------------------------------------------

fn adv16_encode_vec_str_joined<E: oxicode::enc::Encoder>(
    val: &[String],
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let joined = val.join("|");
    joined.encode(encoder)
}

fn adv16_decode_vec_str_split<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Vec<String>, oxicode::error::Error> {
    use oxicode::de::Decode;
    let joined = String::decode(decoder)?;
    if joined.is_empty() {
        Ok(vec![])
    } else {
        Ok(joined.split('|').map(String::from).collect())
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvVecStrJoined {
    #[oxicode(
        encode_with = "adv16_encode_vec_str_joined",
        decode_with = "adv16_decode_vec_str_split"
    )]
    tags: Vec<String>,
}

#[test]
fn test_adv16_vec_string_as_joined_roundtrip() {
    let original = AdvVecStrJoined {
        tags: vec!["foo".into(), "bar".into(), "baz".into()],
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvVecStrJoined, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.tags, vec!["foo", "bar", "baz"]);
}

// ---------------------------------------------------------------------------
// Test 17: Enum variant as u8 index
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Copy)]
enum Priority {
    Low,
    Medium,
    High,
}

fn adv17_encode_priority<E: oxicode::enc::Encoder>(
    val: &Priority,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let idx: u8 = match val {
        Priority::Low => 0,
        Priority::Medium => 1,
        Priority::High => 2,
    };
    idx.encode(encoder)
}

fn adv17_decode_priority<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Priority, oxicode::error::Error> {
    use oxicode::de::Decode;
    match u8::decode(decoder)? {
        0 => Ok(Priority::Low),
        1 => Ok(Priority::Medium),
        2 => Ok(Priority::High),
        other => Err(oxicode::error::Error::InvalidData {
            message: if other > 2 {
                "unknown Priority discriminant"
            } else {
                "unreachable"
            },
        }),
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvPriority {
    #[oxicode(
        encode_with = "adv17_encode_priority",
        decode_with = "adv17_decode_priority"
    )]
    priority: Priority,
    task: String,
}

#[test]
fn test_adv17_enum_variant_as_u8_index() {
    for prio in [Priority::Low, Priority::Medium, Priority::High] {
        let original = AdvPriority {
            priority: prio,
            task: "work".into(),
        };
        let bytes = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (AdvPriority, _) = oxicode::decode_from_slice(&bytes).expect("decode");
        assert_eq!(decoded.priority, prio);
        assert_eq!(decoded.task, "work");
    }
}

// ---------------------------------------------------------------------------
// Test 18: Padding bytes encode/decode
// ---------------------------------------------------------------------------

fn adv18_encode_with_padding<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.encode(encoder)?;
    [0u8; 4].encode(encoder)
}

fn adv18_decode_skip_padding<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    let _padding = <[u8; 4]>::decode(decoder)?;
    Ok(v)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvPadded {
    #[oxicode(
        encode_with = "adv18_encode_with_padding",
        decode_with = "adv18_decode_skip_padding"
    )]
    value: u32,
}

#[test]
fn test_adv18_padding_bytes_encode_decode() {
    let original = AdvPadded { value: 12345 };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvPadded, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.value, 12345);

    // Verify encoded size is larger than plain u32 in legacy (fixed-int) config.
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct PlainU32 {
        value: u32,
    }
    let config = oxicode::config::legacy();
    let custom_bytes =
        oxicode::encode_to_vec_with_config(&original, config).expect("encode custom");
    let plain_bytes = oxicode::encode_to_vec_with_config(&PlainU32 { value: 12345 }, config)
        .expect("encode plain");
    assert_eq!(
        custom_bytes.len(),
        plain_bytes.len() + 4,
        "padded encoding should be 4 bytes larger than plain"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Option<String> — None as empty string
// ---------------------------------------------------------------------------

fn adv19_encode_opt_str<E: oxicode::enc::Encoder>(
    val: &Option<String>,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let s: &str = match val {
        None => "",
        Some(ref s) => s.as_str(),
    };
    s.encode(encoder)
}

fn adv19_decode_opt_str<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Option<String>, oxicode::error::Error> {
    use oxicode::de::Decode;
    let s = String::decode(decoder)?;
    Ok(if s.is_empty() { None } else { Some(s) })
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvOptStr {
    #[oxicode(
        encode_with = "adv19_encode_opt_str",
        decode_with = "adv19_decode_opt_str"
    )]
    maybe: Option<String>,
    id: u32,
}

#[test]
fn test_adv19_option_string_none_as_empty() {
    let none_case = AdvOptStr { maybe: None, id: 1 };
    let bytes = oxicode::encode_to_vec(&none_case).expect("encode none");
    let (decoded, _): (AdvOptStr, _) = oxicode::decode_from_slice(&bytes).expect("decode none");
    assert_eq!(decoded.maybe, None);
    assert_eq!(decoded.id, 1);

    let some_case = AdvOptStr {
        maybe: Some("hello".into()),
        id: 2,
    };
    let bytes2 = oxicode::encode_to_vec(&some_case).expect("encode some");
    let (decoded2, _): (AdvOptStr, _) = oxicode::decode_from_slice(&bytes2).expect("decode some");
    assert_eq!(decoded2.maybe, Some("hello".into()));
    assert_eq!(decoded2.id, 2);
}

// ---------------------------------------------------------------------------
// Test 20: Tuple (u32, u32) field with swapped encode order
// ---------------------------------------------------------------------------

fn adv20_encode_pair_swapped<E: oxicode::enc::Encoder>(
    val: &(u32, u32),
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.1.encode(encoder)?;
    val.0.encode(encoder)
}

fn adv20_decode_pair_swapped<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<(u32, u32), oxicode::error::Error> {
    use oxicode::de::Decode;
    let b = u32::decode(decoder)?;
    let a = u32::decode(decoder)?;
    Ok((a, b))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvReorder {
    #[oxicode(
        encode_with = "adv20_encode_pair_swapped",
        decode_with = "adv20_decode_pair_swapped"
    )]
    pair: (u32, u32),
    label: String,
}

#[test]
fn test_adv20_reorder_fields_via_custom_encode() {
    let original = AdvReorder {
        pair: (10, 20),
        label: "swap".into(),
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvReorder, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    // swapped on wire then swapped back → original
    assert_eq!(decoded.pair, (10, 20));
    assert_eq!(decoded.label, "swap");
}

// ---------------------------------------------------------------------------
// Test 21: Generic struct with encode_with on a concrete String field
// ---------------------------------------------------------------------------

fn adv21_encode_tag_trimmed<E: oxicode::enc::Encoder>(
    val: &str,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.trim().encode(encoder)
}

fn adv21_decode_tag_pass<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    String::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvGeneric<T: Encode + Decode> {
    payload: T,
    #[oxicode(
        encode_with = "adv21_encode_tag_trimmed",
        decode_with = "adv21_decode_tag_pass"
    )]
    tag: String,
}

#[test]
fn test_adv21_generic_field_encode_with() {
    let original = AdvGeneric::<u64> {
        payload: 9999,
        tag: "  hello  ".into(),
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AdvGeneric<u64>, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.payload, 9999);
    // whitespace trimmed on encode, decoded as-is
    assert_eq!(decoded.tag, "hello");
}

// ---------------------------------------------------------------------------
// Test 22: Round-trip byte-exact match
// ---------------------------------------------------------------------------

fn adv22_encode_tripled<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.saturating_mul(3).encode(encoder)
}

fn adv22_decode_third<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v / 3)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvByteExact {
    #[oxicode(
        encode_with = "adv22_encode_tripled",
        decode_with = "adv22_decode_third"
    )]
    value: u32,
}

#[test]
fn test_adv22_roundtrip_byte_exact_match() {
    let original = AdvByteExact { value: 100 };
    let struct_bytes = oxicode::encode_to_vec(&original).expect("encode struct");

    // Manually encode 300u32 (= 100 * 3) — this is what encode_with writes on the wire.
    let manual_bytes = oxicode::encode_to_vec(&300u32).expect("encode manual");

    assert_eq!(
        struct_bytes, manual_bytes,
        "struct encoding should match manually encoded tripled value"
    );

    // Also verify round-trip correctness.
    let (decoded, n): (AdvByteExact, _) =
        oxicode::decode_from_slice(&struct_bytes).expect("decode");
    assert_eq!(decoded.value, 100);
    assert_eq!(n, struct_bytes.len());
}
