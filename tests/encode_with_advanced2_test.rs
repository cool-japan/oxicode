//! Advanced tests (set 2) for `#[oxicode(encode_with = "fn")]` and
//! `#[oxicode(decode_with = "fn")]` field-level attributes. 22 tests covering
//! string transformations, numeric remapping, Vec manipulation, multi-field
//! structs, roundtrip verification, and config-variant usage.

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
// Test 1: String encoded with prefix "enc:" prepended
// ---------------------------------------------------------------------------

#[allow(clippy::ptr_arg)]
fn b01_encode_prefix<E: oxicode::enc::Encoder>(
    val: &String,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    format!("enc:{val}").encode(encoder)
}

fn b01_decode_strip_prefix<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    let s = String::decode(decoder)?;
    Ok(s.strip_prefix("enc:").unwrap_or(&s).to_owned())
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B01PrefixStr {
    #[oxicode(
        encode_with = "b01_encode_prefix",
        decode_with = "b01_decode_strip_prefix"
    )]
    name: String,
}

#[test]
fn test_b01_string_prefix_roundtrip() {
    let original = B01PrefixStr {
        name: "oxicode".into(),
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B01PrefixStr, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.name, "oxicode");
}

// ---------------------------------------------------------------------------
// Test 2: String encoded with suffix "::end" appended
// ---------------------------------------------------------------------------

#[allow(clippy::ptr_arg)]
fn b02_encode_suffix<E: oxicode::enc::Encoder>(
    val: &String,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    format!("{val}::end").encode(encoder)
}

fn b02_decode_strip_suffix<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    let s = String::decode(decoder)?;
    Ok(s.strip_suffix("::end").unwrap_or(&s).to_owned())
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B02SuffixStr {
    #[oxicode(
        encode_with = "b02_encode_suffix",
        decode_with = "b02_decode_strip_suffix"
    )]
    label: String,
}

#[test]
fn test_b02_string_suffix_roundtrip() {
    let original = B02SuffixStr {
        label: "rust".into(),
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B02SuffixStr, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.label, "rust");
}

// ---------------------------------------------------------------------------
// Test 3: u32 negated via wrapping arithmetic
// ---------------------------------------------------------------------------

fn b03_encode_wrapping_neg<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.wrapping_neg().encode(encoder)
}

fn b03_decode_wrapping_neg<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v.wrapping_neg())
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B03NegatedU32 {
    #[oxicode(
        encode_with = "b03_encode_wrapping_neg",
        decode_with = "b03_decode_wrapping_neg"
    )]
    count: u32,
}

#[test]
fn test_b03_u32_wrapping_negate_roundtrip() {
    for val in [0u32, 1, 255, 1000, u32::MAX] {
        let original = B03NegatedU32 { count: val };
        let bytes = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (B03NegatedU32, _) = oxicode::decode_from_slice(&bytes).expect("decode");
        assert_eq!(decoded.count, val, "roundtrip failed for val={val}");
    }
}

// ---------------------------------------------------------------------------
// Test 4: Vec<u32> reversed on encode, reversed again on decode
// ---------------------------------------------------------------------------

fn b04_encode_vec_reversed<E: oxicode::enc::Encoder>(
    val: &[u32],
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let mut reversed = val.to_vec();
    reversed.reverse();
    reversed.encode(encoder)
}

fn b04_decode_vec_reversed<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Vec<u32>, oxicode::error::Error> {
    use oxicode::de::Decode;
    let mut v = Vec::<u32>::decode(decoder)?;
    v.reverse();
    Ok(v)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B04ReversedVec {
    #[oxicode(
        encode_with = "b04_encode_vec_reversed",
        decode_with = "b04_decode_vec_reversed"
    )]
    items: Vec<u32>,
}

#[test]
fn test_b04_vec_u32_reversed_roundtrip() {
    let original = B04ReversedVec {
        items: vec![1, 2, 3, 4, 5],
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B04ReversedVec, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.items, vec![1, 2, 3, 4, 5]);
}

// ---------------------------------------------------------------------------
// Test 5: Vec<i32> sorted ascending on encode, passthrough on decode
// ---------------------------------------------------------------------------

fn b05_encode_vec_sorted<E: oxicode::enc::Encoder>(
    val: &[i32],
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let mut sorted = val.to_vec();
    sorted.sort();
    sorted.encode(encoder)
}

fn b05_decode_vec_passthrough<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Vec<i32>, oxicode::error::Error> {
    use oxicode::de::Decode;
    Vec::<i32>::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B05SortedVec {
    #[oxicode(
        encode_with = "b05_encode_vec_sorted",
        decode_with = "b05_decode_vec_passthrough"
    )]
    values: Vec<i32>,
}

#[test]
fn test_b05_vec_i32_sorted_on_encode() {
    let original = B05SortedVec {
        values: vec![5, 1, 3, 2, 4],
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B05SortedVec, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    // Values come back sorted because encode sorted them
    assert_eq!(decoded.values, vec![1, 2, 3, 4, 5]);
}

// ---------------------------------------------------------------------------
// Test 6: i64 encoded as absolute value, decoded as positive
// ---------------------------------------------------------------------------

fn b06_encode_abs_i64<E: oxicode::enc::Encoder>(
    val: &i64,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.unsigned_abs().encode(encoder)
}

fn b06_decode_abs_i64<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<i64, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u64::decode(decoder)?;
    Ok(v as i64)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B06AbsI64 {
    #[oxicode(encode_with = "b06_encode_abs_i64", decode_with = "b06_decode_abs_i64")]
    delta: i64,
}

#[test]
fn test_b06_i64_absolute_value_encoding() {
    let original = B06AbsI64 { delta: -9999 };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B06AbsI64, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    // Absolute value of -9999 is 9999; decoded as positive
    assert_eq!(decoded.delta, 9999);
}

// ---------------------------------------------------------------------------
// Test 7: Multiple fields — two with custom encode, one plain
// ---------------------------------------------------------------------------

#[allow(clippy::ptr_arg)]
fn b07_encode_upper<E: oxicode::enc::Encoder>(
    val: &String,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.to_uppercase().encode(encoder)
}

fn b07_decode_lower<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    let s = String::decode(decoder)?;
    Ok(s.to_lowercase())
}

fn b07_encode_mul3<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.saturating_mul(3).encode(encoder)
}

fn b07_decode_div3<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v / 3)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B07MultiCustom {
    #[oxicode(encode_with = "b07_encode_upper", decode_with = "b07_decode_lower")]
    username: String,
    id: u64,
    #[oxicode(encode_with = "b07_encode_mul3", decode_with = "b07_decode_div3")]
    score: u32,
}

#[test]
fn test_b07_two_custom_fields_one_plain_roundtrip() {
    let original = B07MultiCustom {
        username: "alice".into(),
        id: 42,
        score: 100,
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B07MultiCustom, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.username, "alice");
    assert_eq!(decoded.id, 42);
    assert_eq!(decoded.score, 100);
}

// ---------------------------------------------------------------------------
// Test 8: u8 encoded as its bit-reversed counterpart
// ---------------------------------------------------------------------------

fn b08_encode_bit_rev<E: oxicode::enc::Encoder>(
    val: &u8,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.reverse_bits().encode(encoder)
}

fn b08_decode_bit_rev<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u8, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u8::decode(decoder)?;
    Ok(v.reverse_bits())
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B08BitReversed {
    #[oxicode(encode_with = "b08_encode_bit_rev", decode_with = "b08_decode_bit_rev")]
    flags: u8,
}

#[test]
fn test_b08_u8_bit_reversed_roundtrip() {
    for v in [0u8, 1, 0b10101010, 0xFF] {
        let original = B08BitReversed { flags: v };
        let bytes = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (B08BitReversed, _) = oxicode::decode_from_slice(&bytes).expect("decode");
        assert_eq!(decoded.flags, v, "roundtrip failed for flags={v:#010b}");
    }
}

// ---------------------------------------------------------------------------
// Test 9: String trimmed and uppercased on encode, decoded as-is
// ---------------------------------------------------------------------------

fn b09_encode_trim_upper<E: oxicode::enc::Encoder>(
    val: &str,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.trim().to_uppercase().encode(encoder)
}

fn b09_decode_passthrough<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    String::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B09TrimUpper {
    #[oxicode(
        encode_with = "b09_encode_trim_upper",
        decode_with = "b09_decode_passthrough"
    )]
    keyword: String,
}

#[test]
fn test_b09_trim_and_uppercase_on_encode() {
    let original = B09TrimUpper {
        keyword: "  hello world  ".into(),
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B09TrimUpper, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.keyword, "HELLO WORLD");
}

// ---------------------------------------------------------------------------
// Test 10: u16 multiplied by 10 on encode, divided by 10 on decode
// ---------------------------------------------------------------------------

fn b10_encode_mul10<E: oxicode::enc::Encoder>(
    val: &u16,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.saturating_mul(10).encode(encoder)
}

fn b10_decode_div10<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u16, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u16::decode(decoder)?;
    Ok(v / 10)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B10ScaledU16 {
    #[oxicode(encode_with = "b10_encode_mul10", decode_with = "b10_decode_div10")]
    percentage: u16,
}

#[test]
fn test_b10_u16_scaled_by_10_roundtrip() {
    for pct in [0u16, 1, 50, 100, 6553] {
        let original = B10ScaledU16 { percentage: pct };
        let bytes = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (B10ScaledU16, _) = oxicode::decode_from_slice(&bytes).expect("decode");
        assert_eq!(
            decoded.percentage, pct,
            "roundtrip failed for percentage={pct}"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 11: Struct with encode_with using legacy config
// ---------------------------------------------------------------------------

fn b11_encode_shifted<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.wrapping_add(0x1000).encode(encoder)
}

fn b11_decode_shifted<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v.wrapping_sub(0x1000))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B11ShiftedLegacy {
    #[oxicode(encode_with = "b11_encode_shifted", decode_with = "b11_decode_shifted")]
    address: u32,
}

#[test]
fn test_b11_shifted_value_with_legacy_config() {
    let config = oxicode::config::legacy();
    let original = B11ShiftedLegacy { address: 0x4000 };
    let bytes = oxicode::encode_to_vec_with_config(&original, config).expect("encode legacy");
    let (decoded, _): (B11ShiftedLegacy, _) =
        oxicode::decode_from_slice_with_config(&bytes, config).expect("decode legacy");
    assert_eq!(decoded.address, 0x4000);
}

// ---------------------------------------------------------------------------
// Test 12: Vec<u8> deduplicated (consecutive duplicates removed) on encode
// ---------------------------------------------------------------------------

fn b12_encode_dedup<E: oxicode::enc::Encoder>(
    val: &[u8],
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let deduped: Vec<u8> = val
        .windows(2)
        .filter(|w| w[0] != w[1])
        .map(|w| w[0])
        .chain(val.last().copied())
        .collect();
    deduped.encode(encoder)
}

fn b12_decode_passthrough<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Vec<u8>, oxicode::error::Error> {
    use oxicode::de::Decode;
    Vec::<u8>::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B12DedupVec {
    #[oxicode(
        encode_with = "b12_encode_dedup",
        decode_with = "b12_decode_passthrough"
    )]
    run: Vec<u8>,
}

#[test]
fn test_b12_vec_u8_dedup_on_encode() {
    let original = B12DedupVec {
        run: vec![1, 1, 2, 2, 2, 3, 1, 1],
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B12DedupVec, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.run, vec![1, 2, 3, 1]);
}

// ---------------------------------------------------------------------------
// Test 13: f32 encoded as fixed-point i32 (×1000)
// ---------------------------------------------------------------------------

fn b13_encode_f32_fixed<E: oxicode::enc::Encoder>(
    val: &f32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let fixed = (val * 1000.0) as i32;
    fixed.encode(encoder)
}

fn b13_decode_f32_fixed<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<f32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let fixed = i32::decode(decoder)?;
    Ok(fixed as f32 / 1000.0)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B13FixedPointF32 {
    #[oxicode(
        encode_with = "b13_encode_f32_fixed",
        decode_with = "b13_decode_f32_fixed"
    )]
    ratio: f32,
}

#[test]
fn test_b13_f32_fixed_point_1000_roundtrip() {
    let original = B13FixedPointF32 { ratio: 3.141 };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B13FixedPointF32, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    // Tolerance of 0.001 due to fixed-point rounding
    assert!(
        (decoded.ratio - 3.141).abs() < 0.001,
        "expected ~3.141, got {}",
        decoded.ratio
    );
}

// ---------------------------------------------------------------------------
// Test 14: String encoded as char count (length) then original — verify wire
// ---------------------------------------------------------------------------

#[allow(clippy::ptr_arg)]
fn b14_encode_with_char_count<E: oxicode::enc::Encoder>(
    val: &String,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let char_count = val.chars().count() as u32;
    char_count.encode(encoder)?;
    val.as_str().encode(encoder)
}

fn b14_decode_with_char_count<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    let _char_count = u32::decode(decoder)?;
    String::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B14CharCountPrefixed {
    #[oxicode(
        encode_with = "b14_encode_with_char_count",
        decode_with = "b14_decode_with_char_count"
    )]
    message: String,
}

#[test]
fn test_b14_string_char_count_prefix_roundtrip() {
    let original = B14CharCountPrefixed {
        message: "hello".into(),
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B14CharCountPrefixed, _) =
        oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.message, "hello");
}

// ---------------------------------------------------------------------------
// Test 15: Byte-exact verification — custom encode_with matches manual wire
// ---------------------------------------------------------------------------

fn b15_encode_doubled<E: oxicode::enc::Encoder>(
    val: &u64,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.saturating_mul(2).encode(encoder)
}

fn b15_decode_halved<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u64, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u64::decode(decoder)?;
    Ok(v / 2)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B15DoubledU64 {
    #[oxicode(encode_with = "b15_encode_doubled", decode_with = "b15_decode_halved")]
    metric: u64,
}

#[test]
fn test_b15_byte_exact_u64_doubled_wire_match() {
    let original = B15DoubledU64 { metric: 500 };
    let struct_bytes = oxicode::encode_to_vec(&original).expect("encode struct");
    // 500 * 2 = 1000 — this is what should appear on the wire
    let manual_bytes = oxicode::encode_to_vec(&1000u64).expect("encode manual");
    assert_eq!(
        struct_bytes, manual_bytes,
        "struct wire should match manually encoded doubled value"
    );
    let (decoded, n): (B15DoubledU64, _) =
        oxicode::decode_from_slice(&struct_bytes).expect("decode");
    assert_eq!(decoded.metric, 500);
    assert_eq!(n, struct_bytes.len());
}

// ---------------------------------------------------------------------------
// Test 16: Three fields all with different custom encode_with functions
// ---------------------------------------------------------------------------

#[allow(clippy::ptr_arg)]
fn b16_encode_rev_str<E: oxicode::enc::Encoder>(
    val: &String,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let rev: String = val.chars().rev().collect();
    rev.encode(encoder)
}

fn b16_decode_rev_str<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    let s = String::decode(decoder)?;
    Ok(s.chars().rev().collect())
}

fn b16_encode_neg_i32<E: oxicode::enc::Encoder>(
    val: &i32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.wrapping_neg().encode(encoder)
}

fn b16_decode_neg_i32<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<i32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = i32::decode(decoder)?;
    Ok(v.wrapping_neg())
}

fn b16_encode_swap_bytes_u16<E: oxicode::enc::Encoder>(
    val: &u16,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.swap_bytes().encode(encoder)
}

fn b16_decode_swap_bytes_u16<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u16, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u16::decode(decoder)?;
    Ok(v.swap_bytes())
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B16ThreeCustom {
    #[oxicode(encode_with = "b16_encode_rev_str", decode_with = "b16_decode_rev_str")]
    tag: String,
    #[oxicode(encode_with = "b16_encode_neg_i32", decode_with = "b16_decode_neg_i32")]
    offset: i32,
    #[oxicode(
        encode_with = "b16_encode_swap_bytes_u16",
        decode_with = "b16_decode_swap_bytes_u16"
    )]
    port: u16,
}

#[test]
fn test_b16_three_fields_all_custom_roundtrip() {
    let original = B16ThreeCustom {
        tag: "rust".into(),
        offset: -77,
        port: 8080,
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B16ThreeCustom, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.tag, "rust");
    assert_eq!(decoded.offset, -77);
    assert_eq!(decoded.port, 8080);
}

// ---------------------------------------------------------------------------
// Test 17: bool encoded as "Y"/"N" char
// ---------------------------------------------------------------------------

fn b17_encode_bool_char<E: oxicode::enc::Encoder>(
    val: &bool,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let c: u8 = if *val { b'Y' } else { b'N' };
    c.encode(encoder)
}

fn b17_decode_bool_char<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<bool, oxicode::error::Error> {
    use oxicode::de::Decode;
    let c = u8::decode(decoder)?;
    Ok(c == b'Y')
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B17BoolAsChar {
    #[oxicode(
        encode_with = "b17_encode_bool_char",
        decode_with = "b17_decode_bool_char"
    )]
    active: bool,
    name: String,
}

#[test]
fn test_b17_bool_encoded_as_yn_char() {
    for flag in [true, false] {
        let original = B17BoolAsChar {
            active: flag,
            name: "item".into(),
        };
        let bytes = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (B17BoolAsChar, _) = oxicode::decode_from_slice(&bytes).expect("decode");
        assert_eq!(decoded.active, flag);
        assert_eq!(decoded.name, "item");
    }
}

// ---------------------------------------------------------------------------
// Test 18: Vec<String> sorted descending on encode
// ---------------------------------------------------------------------------

fn b18_encode_vec_str_sorted_desc<E: oxicode::enc::Encoder>(
    val: &[String],
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let mut sorted = val.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    sorted.encode(encoder)
}

fn b18_decode_vec_str_passthrough<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Vec<String>, oxicode::error::Error> {
    use oxicode::de::Decode;
    Vec::<String>::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B18SortedDescVec {
    #[oxicode(
        encode_with = "b18_encode_vec_str_sorted_desc",
        decode_with = "b18_decode_vec_str_passthrough"
    )]
    words: Vec<String>,
}

#[test]
fn test_b18_vec_string_sorted_descending_on_encode() {
    let original = B18SortedDescVec {
        words: vec!["banana".into(), "apple".into(), "cherry".into()],
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B18SortedDescVec, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.words, vec!["cherry", "banana", "apple"]);
}

// ---------------------------------------------------------------------------
// Test 19: u32 encoded with standard config variant (standard())
// ---------------------------------------------------------------------------

fn b19_encode_rotl8<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.rotate_left(8).encode(encoder)
}

fn b19_decode_rotr8<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v.rotate_right(8))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B19RotatedU32 {
    #[oxicode(encode_with = "b19_encode_rotl8", decode_with = "b19_decode_rotr8")]
    value: u32,
}

#[test]
fn test_b19_u32_rotate_standard_config() {
    let config = oxicode::config::standard();
    let original = B19RotatedU32 { value: 0x12345678 };
    let bytes = oxicode::encode_to_vec_with_config(&original, config).expect("encode standard");
    let (decoded, _): (B19RotatedU32, _) =
        oxicode::decode_from_slice_with_config(&bytes, config).expect("decode standard");
    assert_eq!(decoded.value, 0x12345678);
}

// ---------------------------------------------------------------------------
// Test 20: Struct with encode_with and extra plain field — verify field order
// ---------------------------------------------------------------------------

fn b20_encode_masked_u32<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    (val & 0x00FF_00FF).encode(encoder)
}

fn b20_decode_masked_u32<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    u32::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B20MaskedField {
    prefix: String,
    #[oxicode(
        encode_with = "b20_encode_masked_u32",
        decode_with = "b20_decode_masked_u32"
    )]
    flags: u32,
    suffix: String,
}

#[test]
fn test_b20_masked_field_preserves_plain_fields() {
    let original = B20MaskedField {
        prefix: "start".into(),
        flags: 0xAABB_CCDD,
        suffix: "end".into(),
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (B20MaskedField, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    // Only lower byte of each half survives masking
    assert_eq!(decoded.flags, 0x00BB_00DD);
    assert_eq!(decoded.prefix, "start");
    assert_eq!(decoded.suffix, "end");
}

// ---------------------------------------------------------------------------
// Test 21: u32 parity bit prepended (even/odd parity as u8) on encode
// ---------------------------------------------------------------------------

fn b21_encode_parity<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let parity: u8 = (val.count_ones() % 2) as u8;
    parity.encode(encoder)?;
    val.encode(encoder)
}

fn b21_decode_parity<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let parity = u8::decode(decoder)?;
    let v = u32::decode(decoder)?;
    let expected_parity = (v.count_ones() % 2) as u8;
    if parity != expected_parity {
        return Err(oxicode::error::Error::InvalidData {
            message: "parity mismatch on decode",
        });
    }
    Ok(v)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B21ParityField {
    #[oxicode(encode_with = "b21_encode_parity", decode_with = "b21_decode_parity")]
    data: u32,
}

#[test]
fn test_b21_parity_bit_encode_decode_roundtrip() {
    for val in [0u32, 1, 0b1010_1010, u32::MAX, 12345678] {
        let original = B21ParityField { data: val };
        let bytes = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (B21ParityField, _) = oxicode::decode_from_slice(&bytes).expect("decode");
        assert_eq!(decoded.data, val, "roundtrip failed for data={val}");
    }
}

// ---------------------------------------------------------------------------
// Test 22: u32 encoded via big-endian bytes then decoded back
// ---------------------------------------------------------------------------

fn b22_encode_big_endian_u32<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    // Encode each byte of the big-endian representation individually
    let be_bytes = val.to_be_bytes();
    for &b in &be_bytes {
        b.encode(encoder)?;
    }
    Ok(())
}

fn b22_decode_big_endian_u32<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let b0 = u8::decode(decoder)? as u32;
    let b1 = u8::decode(decoder)? as u32;
    let b2 = u8::decode(decoder)? as u32;
    let b3 = u8::decode(decoder)? as u32;
    Ok((b0 << 24) | (b1 << 16) | (b2 << 8) | b3)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct B22BigEndianU32 {
    #[oxicode(
        encode_with = "b22_encode_big_endian_u32",
        decode_with = "b22_decode_big_endian_u32"
    )]
    magic: u32,
}

#[test]
fn test_b22_u32_big_endian_byte_order_roundtrip() {
    let original = B22BigEndianU32 { magic: 0xDEAD_CAFE };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, n): (B22BigEndianU32, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(decoded.magic, 0xDEAD_CAFE);
    assert_eq!(n, bytes.len());
    // The 4 bytes on the wire should be in big-endian order: 0xDE, 0xAD, 0xCA, 0xFE
    // (bytes slice position depends on preceding varint-len headers; check the tail)
    let tail: &[u8] = &bytes[bytes.len() - 4..];
    assert_eq!(tail, &[0xDE, 0xAD, 0xCA, 0xFE]);
}
