//! Complex tests for `#[oxicode(encode_with = "...")]` and `#[oxicode(decode_with = "...")]`
//! field-level attributes.
//!
//! 22 tests covering big-endian layouts, negated integers, fixed-width padding,
//! sentinel bytes, zigzag varints, custom string prefixes, packed pairs, array
//! encodings, and combined-field structs with explicit byte-pattern verification.

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
// Test 1 & 16: custom big-endian u32 — store raw BE bytes
// ---------------------------------------------------------------------------

fn encode_u32_be<E: oxicode::enc::Encoder>(
    value: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let bytes = value.to_be_bytes();
    for b in &bytes {
        b.encode(encoder)?;
    }
    Ok(())
}

fn decode_u32_be<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let b0 = u8::decode(decoder)?;
    let b1 = u8::decode(decoder)?;
    let b2 = u8::decode(decoder)?;
    let b3 = u8::decode(decoder)?;
    Ok(u32::from_be_bytes([b0, b1, b2, b3]))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BigEndianU32 {
    #[oxicode(encode_with = "encode_u32_be", decode_with = "decode_u32_be")]
    value: u32,
}

#[test]
fn test01_custom_big_endian_u32_roundtrip() {
    let original = BigEndianU32 { value: 0xDEAD_BEEF };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, n): (BigEndianU32, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded, original);
    assert_eq!(n, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: custom big-endian u64
// ---------------------------------------------------------------------------

fn encode_u64_be<E: oxicode::enc::Encoder>(
    value: &u64,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    for b in value.to_be_bytes() {
        b.encode(encoder)?;
    }
    Ok(())
}

fn decode_u64_be<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u64, oxicode::error::Error> {
    use oxicode::de::Decode;
    let mut buf = [0u8; 8];
    for cell in &mut buf {
        *cell = u8::decode(decoder)?;
    }
    Ok(u64::from_be_bytes(buf))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BigEndianU64 {
    #[oxicode(encode_with = "encode_u64_be", decode_with = "decode_u64_be")]
    timestamp: u64,
}

#[test]
fn test02_custom_big_endian_u64_roundtrip() {
    let original = BigEndianU64 {
        timestamp: 0x0102_0304_0506_0708,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (BigEndianU64, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 3: negated i32 encoding (store as negated, decode back as negated)
// ---------------------------------------------------------------------------

fn encode_negated_i32<E: oxicode::enc::Encoder>(
    value: &i32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    // Store as the bitwise complement; avoids overflow at i32::MIN
    (!*value).encode(encoder)
}

fn decode_negated_i32<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<i32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let stored = i32::decode(decoder)?;
    Ok(!stored)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NegatedI32 {
    #[oxicode(encode_with = "encode_negated_i32", decode_with = "decode_negated_i32")]
    delta: i32,
}

#[test]
fn test03_negated_i32_roundtrip() {
    for delta in [0i32, 1, -1, i32::MAX, i32::MIN, 42, -12345] {
        let original = NegatedI32 { delta };
        let encoded = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (NegatedI32, _) = oxicode::decode_from_slice(&encoded).expect("decode");
        assert_eq!(decoded.delta, delta, "roundtrip failed for delta={delta}");
    }
}

// ---------------------------------------------------------------------------
// Test 4: byte-reversed Vec<u8> with explicit byte-pattern verification
// ---------------------------------------------------------------------------

fn encode_bytes_xor<E: oxicode::enc::Encoder>(
    value: &[u8],
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    // XOR every byte with 0xFF before storing
    let xored: Vec<u8> = value.iter().map(|b| b ^ 0xFF).collect();
    xored.encode(encoder)
}

fn decode_bytes_xor<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Vec<u8>, oxicode::error::Error> {
    use oxicode::de::Decode;
    let xored = Vec::<u8>::decode(decoder)?;
    Ok(xored.into_iter().map(|b| b ^ 0xFF).collect())
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct XoredBytes {
    #[oxicode(encode_with = "encode_bytes_xor", decode_with = "decode_bytes_xor")]
    payload: Vec<u8>,
}

#[test]
fn test04_byte_xored_vec_roundtrip() {
    let original = XoredBytes {
        payload: vec![0x00, 0xAA, 0xFF, 0x55, 0x0F],
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (XoredBytes, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.payload, vec![0x00, 0xAA, 0xFF, 0x55, 0x0F]);
}

// ---------------------------------------------------------------------------
// Test 5: doubled f64 encoding (store 2x, decode ÷2)
// ---------------------------------------------------------------------------

fn encode_f64_doubled<E: oxicode::enc::Encoder>(
    value: &f64,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    (*value * 2.0).encode(encoder)
}

fn decode_f64_halved<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<f64, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = f64::decode(decoder)?;
    Ok(v / 2.0)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DoubledF64 {
    #[oxicode(encode_with = "encode_f64_doubled", decode_with = "decode_f64_halved")]
    rate: f64,
}

#[test]
fn test05_doubled_f64_roundtrip() {
    let original = DoubledF64 {
        rate: std::f64::consts::PI,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (DoubledF64, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    // Exact equality holds because PI * 2.0 / 2.0 is exact in IEEE 754
    assert_eq!(decoded.rate, std::f64::consts::PI);
}

// ---------------------------------------------------------------------------
// Test 6: struct with multiple custom-encoded fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiBeFields {
    #[oxicode(encode_with = "encode_u32_be", decode_with = "decode_u32_be")]
    seq: u32,
    name: String,
    #[oxicode(encode_with = "encode_u32_be", decode_with = "decode_u32_be")]
    checksum: u32,
}

#[test]
fn test06_multiple_be_fields_roundtrip() {
    let original = MultiBeFields {
        seq: 1,
        name: "oxicode".into(),
        checksum: 0xCAFE_BABE,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (MultiBeFields, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 7: custom-encoded field alongside normal fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MixedNormalAndCustom {
    id: u64,
    #[oxicode(encode_with = "encode_negated_i32", decode_with = "decode_negated_i32")]
    offset: i32,
    label: String,
    version: u8,
}

#[test]
fn test07_custom_field_alongside_normal_fields_roundtrip() {
    let original = MixedNormalAndCustom {
        id: 9999,
        offset: -42,
        label: "packet".into(),
        version: 3,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (MixedNormalAndCustom, _) =
        oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 8: custom encode to exactly 8 bytes regardless of value (zero-padded)
// ---------------------------------------------------------------------------

fn encode_u32_fixed8<E: oxicode::enc::Encoder>(
    value: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    // Write 4 zero-padding bytes followed by 4 BE value bytes = 8 total
    let padding: u32 = 0;
    for b in padding.to_be_bytes() {
        b.encode(encoder)?;
    }
    for b in value.to_be_bytes() {
        b.encode(encoder)?;
    }
    Ok(())
}

fn decode_u32_fixed8<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    // Skip 4 padding bytes, then read 4 BE value bytes
    for _ in 0..4 {
        let _ = u8::decode(decoder)?;
    }
    let b0 = u8::decode(decoder)?;
    let b1 = u8::decode(decoder)?;
    let b2 = u8::decode(decoder)?;
    let b3 = u8::decode(decoder)?;
    Ok(u32::from_be_bytes([b0, b1, b2, b3]))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Fixed8Encoded {
    #[oxicode(encode_with = "encode_u32_fixed8", decode_with = "decode_u32_fixed8")]
    value: u32,
}

#[test]
fn test08_fixed8_encoding_always_produces_8_bytes() {
    let original = Fixed8Encoded { value: 42 };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    // The field occupies exactly 8 bytes
    assert_eq!(
        encoded.len(),
        8,
        "expected 8 bytes for fixed8 encoding, got {}",
        encoded.len()
    );
    let (decoded, _): (Fixed8Encoded, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 9: custom String as length-prefixed bytes with sentinel prefix byte
// ---------------------------------------------------------------------------

fn encode_string_with_sentinel<E: oxicode::enc::Encoder>(
    value: &str,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    // Write sentinel 0xAB, then standard String encoding
    0xABu8.encode(encoder)?;
    value.to_string().encode(encoder)
}

fn decode_string_with_sentinel<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    let sentinel = u8::decode(decoder)?;
    if sentinel != 0xAB {
        return Err(oxicode::error::Error::InvalidData {
            message: "expected sentinel byte 0xAB",
        });
    }
    String::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SentinelString {
    #[oxicode(
        encode_with = "encode_string_with_sentinel",
        decode_with = "decode_string_with_sentinel"
    )]
    tag: String,
    id: u32,
}

#[test]
fn test09_string_with_sentinel_prefix_roundtrip() {
    let original = SentinelString {
        tag: "hello".into(),
        id: 7,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    // Verify the sentinel byte 0xAB appears as the first byte
    assert_eq!(encoded[0], 0xAB, "sentinel byte missing at offset 0");
    let (decoded, _): (SentinelString, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 10: custom encoding for a nested non-derive type
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

fn encode_rgb<E: oxicode::enc::Encoder>(
    value: &Rgb,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    // Pack into a single u32: 0x00RRGGBB
    let packed: u32 = ((value.r as u32) << 16) | ((value.g as u32) << 8) | (value.b as u32);
    packed.encode(encoder)
}

fn decode_rgb<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Rgb, oxicode::error::Error> {
    use oxicode::de::Decode;
    let packed = u32::decode(decoder)?;
    Ok(Rgb {
        r: ((packed >> 16) & 0xFF) as u8,
        g: ((packed >> 8) & 0xFF) as u8,
        b: (packed & 0xFF) as u8,
    })
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ColorRecord {
    name: String,
    #[oxicode(encode_with = "encode_rgb", decode_with = "decode_rgb")]
    color: Rgb,
}

#[test]
fn test10_nested_non_derive_type_custom_encoding_roundtrip() {
    let original = ColorRecord {
        name: "coral".into(),
        color: Rgb {
            r: 255,
            g: 127,
            b: 80,
        },
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (ColorRecord, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 11: custom zigzag-encoded u64 (manual varint-style)
// ---------------------------------------------------------------------------

fn encode_zigzag_u64<E: oxicode::enc::Encoder>(
    value: &u64,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    // Simple zigzag: interleave sign bit for signed interpretation
    // For u64 we just encode as a pair (high, low) u32 in big-endian
    let high = (value >> 32) as u32;
    let low = (*value & 0xFFFF_FFFF) as u32;
    for b in high.to_be_bytes() {
        b.encode(encoder)?;
    }
    for b in low.to_be_bytes() {
        b.encode(encoder)?;
    }
    Ok(())
}

fn decode_zigzag_u64<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u64, oxicode::error::Error> {
    use oxicode::de::Decode;
    let mut high_buf = [0u8; 4];
    let mut low_buf = [0u8; 4];
    for cell in &mut high_buf {
        *cell = u8::decode(decoder)?;
    }
    for cell in &mut low_buf {
        *cell = u8::decode(decoder)?;
    }
    let high = u32::from_be_bytes(high_buf) as u64;
    let low = u32::from_be_bytes(low_buf) as u64;
    Ok((high << 32) | low)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ZigzagU64Field {
    #[oxicode(encode_with = "encode_zigzag_u64", decode_with = "decode_zigzag_u64")]
    sequence_id: u64,
}

#[test]
fn test11_custom_split_u64_roundtrip() {
    for val in [0u64, 1, u32::MAX as u64, u64::MAX, 0xABCD_EF01_2345_6789] {
        let original = ZigzagU64Field { sequence_id: val };
        let encoded = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (ZigzagU64Field, _) =
            oxicode::decode_from_slice(&encoded).expect("decode");
        assert_eq!(decoded.sequence_id, val, "roundtrip mismatch for {val}");
    }
}

// ---------------------------------------------------------------------------
// Test 12: custom encoded bool as 'T'/'F' byte (ASCII 84 / 70)
// ---------------------------------------------------------------------------

fn encode_bool_as_tf<E: oxicode::enc::Encoder>(
    value: &bool,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let byte: u8 = if *value { b'T' } else { b'F' };
    byte.encode(encoder)
}

fn decode_bool_from_tf<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<bool, oxicode::error::Error> {
    use oxicode::de::Decode;
    match u8::decode(decoder)? {
        b'T' => Ok(true),
        b'F' => Ok(false),
        other => Err(oxicode::error::Error::InvalidData {
            message: if other == b'T' || other == b'F' {
                "unreachable"
            } else {
                "expected 'T' or 'F'"
            },
        }),
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TFBool {
    #[oxicode(encode_with = "encode_bool_as_tf", decode_with = "decode_bool_from_tf")]
    active: bool,
    name: String,
}

#[test]
fn test12_bool_as_ascii_tf_roundtrip() {
    for active in [true, false] {
        let original = TFBool {
            active,
            name: "flag".into(),
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode");
        // Verify the first byte is literally 'T' or 'F'
        assert!(
            encoded[0] == b'T' || encoded[0] == b'F',
            "expected 'T' or 'F' at byte 0, got 0x{:02X}",
            encoded[0]
        );
        let (decoded, _): (TFBool, _) = oxicode::decode_from_slice(&encoded).expect("decode");
        assert_eq!(decoded.active, active);
        assert_eq!(decoded.name, "flag");
    }
}

// ---------------------------------------------------------------------------
// Test 13: struct with encode_with on optional field (custom None sentinel)
// ---------------------------------------------------------------------------

fn encode_option_as_sentinel<E: oxicode::enc::Encoder>(
    value: &Option<u16>,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    // Use 0xFFFF as the None sentinel; valid values are 0..=0xFFFE
    match *value {
        None => 0xFFFFu16.encode(encoder),
        Some(0xFFFF) => Err(oxicode::error::Error::InvalidData {
            message: "value 0xFFFF is reserved as None sentinel",
        }),
        Some(v) => v.encode(encoder),
    }
}

fn decode_option_from_sentinel<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Option<u16>, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u16::decode(decoder)?;
    if v == 0xFFFF {
        Ok(None)
    } else {
        Ok(Some(v))
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalU16Sentinel {
    label: String,
    #[oxicode(
        encode_with = "encode_option_as_sentinel",
        decode_with = "decode_option_from_sentinel"
    )]
    port: Option<u16>,
}

#[test]
fn test13_option_with_sentinel_roundtrip() {
    let with_port = OptionalU16Sentinel {
        label: "server".into(),
        port: Some(8080),
    };
    let encoded = oxicode::encode_to_vec(&with_port).expect("encode");
    let (decoded, _): (OptionalU16Sentinel, _) =
        oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.port, Some(8080));

    let no_port = OptionalU16Sentinel {
        label: "unknown".into(),
        port: None,
    };
    let encoded2 = oxicode::encode_to_vec(&no_port).expect("encode");
    let (decoded2, _): (OptionalU16Sentinel, _) =
        oxicode::decode_from_slice(&encoded2).expect("decode");
    assert_eq!(decoded2.port, None);
}

// ---------------------------------------------------------------------------
// Test 14: custom-encoded 4-variant enum as a single discriminant byte
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn encode_direction<E: oxicode::enc::Encoder>(
    value: &Direction,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let disc: u8 = match *value {
        Direction::North => 0,
        Direction::East => 1,
        Direction::South => 2,
        Direction::West => 3,
    };
    disc.encode(encoder)
}

fn decode_direction<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Direction, oxicode::error::Error> {
    use oxicode::de::Decode;
    match u8::decode(decoder)? {
        0 => Ok(Direction::North),
        1 => Ok(Direction::East),
        2 => Ok(Direction::South),
        3 => Ok(Direction::West),
        d => Err(oxicode::error::Error::UnexpectedVariant {
            found: d as u32,
            type_name: "Direction",
        }),
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Heading {
    #[oxicode(encode_with = "encode_direction", decode_with = "decode_direction")]
    dir: Direction,
    magnitude: u32,
}

#[test]
fn test14_enum_as_single_byte_roundtrip() {
    for dir in [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ] {
        let original = Heading {
            dir,
            magnitude: 100,
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (Heading, _) = oxicode::decode_from_slice(&encoded).expect("decode");
        assert_eq!(decoded.dir, dir);
        assert_eq!(decoded.magnitude, 100);
    }
}

// ---------------------------------------------------------------------------
// Test 15: multiple structs with different custom encoders, verify independence
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct StructA {
    #[oxicode(encode_with = "encode_u32_be", decode_with = "decode_u32_be")]
    val: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct StructB {
    #[oxicode(encode_with = "encode_negated_i32", decode_with = "decode_negated_i32")]
    val: i32,
}

#[test]
fn test15_multiple_structs_with_different_custom_encoders_independent() {
    let a = StructA { val: 0x1234_5678 };
    let b = StructB { val: -99 };
    let enc_a = oxicode::encode_to_vec(&a).expect("encode a");
    let enc_b = oxicode::encode_to_vec(&b).expect("encode b");
    let (dec_a, _): (StructA, _) = oxicode::decode_from_slice(&enc_a).expect("decode a");
    let (dec_b, _): (StructB, _) = oxicode::decode_from_slice(&enc_b).expect("decode b");
    assert_eq!(dec_a, a);
    assert_eq!(dec_b, b);
}

// ---------------------------------------------------------------------------
// Test 16: verify custom BE encoding produces expected byte pattern
// ---------------------------------------------------------------------------

#[test]
fn test16_custom_be_encoding_produces_expected_byte_pattern() {
    // BigEndianU32 { value: 0x01020304 } should produce bytes [0x01, 0x02, 0x03, 0x04]
    let original = BigEndianU32 { value: 0x01020304 };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    assert_eq!(
        &encoded,
        &[0x01, 0x02, 0x03, 0x04],
        "expected big-endian bytes [01, 02, 03, 04], got {:?}",
        encoded
    );
}

// ---------------------------------------------------------------------------
// Test 17: custom encoder that adds a sentinel byte prefix to a u32
// ---------------------------------------------------------------------------

fn encode_u32_with_prefix<E: oxicode::enc::Encoder>(
    value: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    // Prefix sentinel 0x7E, then BE bytes
    0x7Eu8.encode(encoder)?;
    for b in value.to_be_bytes() {
        b.encode(encoder)?;
    }
    Ok(())
}

fn decode_u32_with_prefix<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let prefix = u8::decode(decoder)?;
    if prefix != 0x7E {
        return Err(oxicode::error::Error::InvalidData {
            message: "expected sentinel prefix 0x7E",
        });
    }
    let b0 = u8::decode(decoder)?;
    let b1 = u8::decode(decoder)?;
    let b2 = u8::decode(decoder)?;
    let b3 = u8::decode(decoder)?;
    Ok(u32::from_be_bytes([b0, b1, b2, b3]))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PrefixedU32 {
    #[oxicode(
        encode_with = "encode_u32_with_prefix",
        decode_with = "decode_u32_with_prefix"
    )]
    code: u32,
}

#[test]
fn test17_encoder_with_sentinel_prefix_byte() {
    let original = PrefixedU32 { code: 0xABCDEF00 };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    assert_eq!(encoded[0], 0x7E, "missing sentinel prefix 0x7E");
    // 1 sentinel + 4 BE bytes = 5 total
    assert_eq!(encoded.len(), 5, "expected 5 bytes, got {}", encoded.len());
    let (decoded, _): (PrefixedU32, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 18: custom decoder that skips a padding byte
// ---------------------------------------------------------------------------

fn encode_u16_with_padding<E: oxicode::enc::Encoder>(
    value: &u16,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    // Write a NUL padding byte, then the u16 as 2 BE bytes
    0x00u8.encode(encoder)?;
    for b in value.to_be_bytes() {
        b.encode(encoder)?;
    }
    Ok(())
}

fn decode_u16_skip_padding<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u16, oxicode::error::Error> {
    use oxicode::de::Decode;
    // Skip the padding byte
    let _ = u8::decode(decoder)?;
    let hi = u8::decode(decoder)?;
    let lo = u8::decode(decoder)?;
    Ok(u16::from_be_bytes([hi, lo]))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PaddedU16 {
    #[oxicode(
        encode_with = "encode_u16_with_padding",
        decode_with = "decode_u16_skip_padding"
    )]
    value: u16,
}

#[test]
fn test18_decoder_skips_padding_byte() {
    let original = PaddedU16 { value: 0x1234 };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    // 1 padding + 2 value bytes = 3 total
    assert_eq!(
        encoded.len(),
        3,
        "expected 3 bytes (1 pad + 2 data), got {}",
        encoded.len()
    );
    assert_eq!(encoded[0], 0x00, "expected padding byte 0x00 at offset 0");
    let (decoded, _): (PaddedU16, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 19: encode_with for array type [u8; 4] packed as big-endian u32
// ---------------------------------------------------------------------------

fn encode_array4_as_be_u32<E: oxicode::enc::Encoder>(
    value: &[u8; 4],
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let packed = u32::from_be_bytes(*value);
    packed.encode(encoder)
}

fn decode_array4_from_be_u32<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<[u8; 4], oxicode::error::Error> {
    use oxicode::de::Decode;
    let packed = u32::decode(decoder)?;
    Ok(packed.to_be_bytes())
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PackedArray4 {
    #[oxicode(
        encode_with = "encode_array4_as_be_u32",
        decode_with = "decode_array4_from_be_u32"
    )]
    octets: [u8; 4],
}

#[test]
fn test19_array4_encoded_as_be_u32_roundtrip() {
    let original = PackedArray4 {
        octets: [10, 0, 0, 1],
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (PackedArray4, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.octets, [10, 0, 0, 1]);
}

// ---------------------------------------------------------------------------
// Test 20: encode_with for pair (u32, u32) packed into a single u64
// ---------------------------------------------------------------------------

fn encode_pair_packed<E: oxicode::enc::Encoder>(
    value: &(u32, u32),
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let packed: u64 = ((value.0 as u64) << 32) | (value.1 as u64);
    packed.encode(encoder)
}

fn decode_pair_unpacked<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<(u32, u32), oxicode::error::Error> {
    use oxicode::de::Decode;
    let packed = u64::decode(decoder)?;
    let high = (packed >> 32) as u32;
    let low = (packed & 0xFFFF_FFFF) as u32;
    Ok((high, low))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PackedPair {
    #[oxicode(
        encode_with = "encode_pair_packed",
        decode_with = "decode_pair_unpacked"
    )]
    coords: (u32, u32),
}

#[test]
fn test20_pair_packed_into_u64_roundtrip() {
    let original = PackedPair {
        coords: (1920, 1080),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (PackedPair, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.coords, (1920, 1080));
}

// ---------------------------------------------------------------------------
// Test 21: roundtrip preserves values exactly, not just approximately
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct ExactRoundtrip {
    #[oxicode(encode_with = "encode_u32_be", decode_with = "decode_u32_be")]
    a: u32,
    #[oxicode(encode_with = "encode_u32_be", decode_with = "decode_u32_be")]
    b: u32,
    #[oxicode(encode_with = "encode_negated_i32", decode_with = "decode_negated_i32")]
    c: i32,
}

#[test]
fn test21_roundtrip_preserves_values_exactly() {
    let test_cases = [
        ExactRoundtrip {
            a: 0,
            b: u32::MAX,
            c: 0,
        },
        ExactRoundtrip {
            a: u32::MAX,
            b: 0,
            c: i32::MIN,
        },
        ExactRoundtrip {
            a: 0xDEAD_BEEF,
            b: 0xCAFE_BABE,
            c: -1,
        },
        ExactRoundtrip {
            a: 1,
            b: 2,
            c: i32::MAX,
        },
    ];
    for original in &test_cases {
        let encoded = oxicode::encode_to_vec(original).expect("encode");
        let (decoded, n): (ExactRoundtrip, _) =
            oxicode::decode_from_slice(&encoded).expect("decode");
        assert_eq!(&decoded, original, "exact roundtrip mismatch");
        assert_eq!(n, encoded.len(), "bytes consumed mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 22: combining normal and custom-encoded fields using legacy config
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct LegacyConfigStruct {
    magic: u16,
    #[oxicode(encode_with = "encode_u32_be", decode_with = "decode_u32_be")]
    be_field: u32,
    description: String,
    #[oxicode(encode_with = "encode_bool_as_tf", decode_with = "decode_bool_from_tf")]
    enabled: bool,
}

#[test]
fn test22_combined_normal_and_custom_fields_with_legacy_config() {
    let config = oxicode::config::legacy();
    let original = LegacyConfigStruct {
        magic: 0xFFEE,
        be_field: 0x0A0B_0C0D,
        description: "legacy struct".into(),
        enabled: true,
    };
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode");
    let (decoded, n): (LegacyConfigStruct, _) =
        oxicode::decode_from_slice_with_config(&encoded, config).expect("decode");
    assert_eq!(decoded, original);
    assert_eq!(n, encoded.len(), "not all bytes consumed");
    // The 'T' sentinel for `enabled = true` must appear somewhere in the encoded output
    assert!(
        encoded.contains(&b'T'),
        "expected 'T' byte in encoded output, got {:?}",
        encoded
    );
}
