//! Tests for primitive type encoding and decoding

#![cfg(feature = "alloc")]
use oxicode::{
    config,
    de::{Decode, DecoderImpl, SliceReader},
    enc::{Encode, EncoderImpl, SliceWriter, VecWriter},
};

/// Test encoding and decoding with a given configuration
fn test_round_trip<T, C>(value: T, config: C)
where
    T: Encode + Decode + PartialEq + core::fmt::Debug,
    C: config::Config,
{
    // Encode
    let writer = VecWriter::new();
    let mut encoder = EncoderImpl::new(writer, config);
    value.encode(&mut encoder).expect("Encoding failed");
    let bytes = encoder.into_writer().into_vec();

    // Decode
    let reader = SliceReader::new(&bytes);
    let mut decoder = DecoderImpl::new(reader, config);
    let decoded = T::decode(&mut decoder).expect("Decoding failed");

    assert_eq!(value, decoded, "Round-trip failed for value: {:?}", value);
}

#[test]
fn test_bool() {
    test_round_trip(false, config::standard());
    test_round_trip(true, config::standard());
    test_round_trip(false, config::legacy());
    test_round_trip(true, config::legacy());
}

#[test]
fn test_u8() {
    test_round_trip(0u8, config::standard());
    test_round_trip(127u8, config::standard());
    test_round_trip(255u8, config::standard());
}

#[test]
fn test_u16() {
    let test_values = [0u16, 1, 127, 250, 251, 255, 256, 1000, 65535];
    for &val in &test_values {
        test_round_trip(val, config::standard());
        test_round_trip(val, config::legacy());
        test_round_trip(val, config::standard().with_big_endian());
    }
}

#[test]
fn test_u32() {
    let test_values = [
        0u32,
        1,
        127,
        250,
        251,
        255,
        256,
        65535,
        65536,
        1_000_000,
        u32::MAX,
    ];
    for &val in &test_values {
        test_round_trip(val, config::standard());
        test_round_trip(val, config::legacy());
        test_round_trip(val, config::standard().with_big_endian());
    }
}

#[test]
fn test_u64() {
    let test_values = [
        0u64,
        1,
        127,
        250,
        251,
        255,
        256,
        65535,
        65536,
        4_294_967_295,
        4_294_967_296,
        1_000_000_000_000,
        u64::MAX,
    ];
    for &val in &test_values {
        test_round_trip(val, config::standard());
        test_round_trip(val, config::legacy());
    }
}

#[test]
fn test_u128() {
    let test_values = [
        0u128,
        1,
        127,
        250,
        251,
        255,
        256,
        65535,
        65536,
        4_294_967_295,
        4_294_967_296,
        u64::MAX as u128,
        u64::MAX as u128 + 1,
        1_000_000_000_000_000_000_000,
        u128::MAX,
    ];
    for &val in &test_values {
        test_round_trip(val, config::standard());
        test_round_trip(val, config::legacy());
    }
}

#[test]
fn test_i8() {
    test_round_trip(-128i8, config::standard());
    test_round_trip(-1i8, config::standard());
    test_round_trip(0i8, config::standard());
    test_round_trip(1i8, config::standard());
    test_round_trip(127i8, config::standard());
}

#[test]
fn test_i16() {
    let test_values = [i16::MIN, -1000, -1, 0, 1, 1000, i16::MAX];
    for &val in &test_values {
        test_round_trip(val, config::standard());
        test_round_trip(val, config::legacy());
    }
}

#[test]
fn test_i32() {
    let test_values = [i32::MIN, -100000, -1, 0, 1, 100000, i32::MAX];
    for &val in &test_values {
        test_round_trip(val, config::standard());
        test_round_trip(val, config::legacy());
    }
}

#[test]
fn test_i64() {
    let test_values = [i64::MIN, -1_000_000_000, -1, 0, 1, 1_000_000_000, i64::MAX];
    for &val in &test_values {
        test_round_trip(val, config::standard());
        test_round_trip(val, config::legacy());
    }
}

#[test]
fn test_i128() {
    let test_values = [
        i128::MIN,
        -1_000_000_000_000_000,
        -1,
        0,
        1,
        1_000_000_000_000_000,
        i128::MAX,
    ];
    for &val in &test_values {
        test_round_trip(val, config::standard());
        test_round_trip(val, config::legacy());
    }
}

#[test]
fn test_f32() {
    test_round_trip(0.0f32, config::standard());
    test_round_trip(-1.5f32, config::standard());
    test_round_trip(std::f32::consts::PI, config::standard());
    test_round_trip(f32::INFINITY, config::standard());
    test_round_trip(f32::NEG_INFINITY, config::standard());
    test_round_trip(f32::MAX, config::standard());
    test_round_trip(f32::MIN, config::standard());
}

#[test]
fn test_f64() {
    test_round_trip(0.0f64, config::standard());
    test_round_trip(-1.5f64, config::standard());
    test_round_trip(std::f64::consts::PI, config::standard());
    test_round_trip(f64::INFINITY, config::standard());
    test_round_trip(f64::NEG_INFINITY, config::standard());
    test_round_trip(f64::MAX, config::standard());
    test_round_trip(f64::MIN, config::standard());
}

#[test]
fn test_char() {
    test_round_trip('a', config::standard());
    test_round_trip('Z', config::standard());
    test_round_trip('0', config::standard());
    test_round_trip('あ', config::standard()); // Japanese character
    test_round_trip('😀', config::standard()); // Emoji
    test_round_trip('\0', config::standard());
    test_round_trip('\n', config::standard());
}

#[test]
fn test_unit() {
    test_round_trip((), config::standard());
}

#[test]
fn test_usize_isize() {
    test_round_trip(0usize, config::standard());
    test_round_trip(1000usize, config::standard());
    test_round_trip(usize::MAX, config::standard());

    test_round_trip(0isize, config::standard());
    test_round_trip(-1000isize, config::standard());
    test_round_trip(1000isize, config::standard());
    test_round_trip(isize::MIN, config::standard());
    test_round_trip(isize::MAX, config::standard());
}

#[test]
fn test_varint_vs_fixed() {
    // Small values should be more compact with varint
    let small_value = 42u32;

    // Varint encoding
    let writer = VecWriter::new();
    let mut encoder = EncoderImpl::new(writer, config::standard());
    small_value.encode(&mut encoder).expect("Encoding failed");
    let varint_bytes = encoder.into_writer().into_vec();

    // Fixed encoding
    let writer = VecWriter::new();
    let mut encoder = EncoderImpl::new(writer, config::legacy());
    small_value.encode(&mut encoder).expect("Encoding failed");
    let fixed_bytes = encoder.into_writer().into_vec();

    assert_eq!(varint_bytes.len(), 1); // Single byte for 42
    assert_eq!(fixed_bytes.len(), 4); // 4 bytes for u32

    // Large values might be same or larger with varint
    let large_value = u32::MAX;

    let writer = VecWriter::new();
    let mut encoder = EncoderImpl::new(writer, config::standard());
    large_value.encode(&mut encoder).expect("Encoding failed");
    let varint_bytes = encoder.into_writer().into_vec();

    let writer = VecWriter::new();
    let mut encoder = EncoderImpl::new(writer, config::legacy());
    large_value.encode(&mut encoder).expect("Encoding failed");
    let fixed_bytes = encoder.into_writer().into_vec();

    assert_eq!(varint_bytes.len(), 5); // Tag byte + 4 bytes
    assert_eq!(fixed_bytes.len(), 4); // 4 bytes for u32
}

#[test]
fn test_endianness() {
    let value = 0x1234u16;

    // Little endian
    let writer = VecWriter::new();
    let mut encoder = EncoderImpl::new(writer, config::legacy().with_little_endian());
    value.encode(&mut encoder).expect("Encoding failed");
    let le_bytes = encoder.into_writer().into_vec();
    assert_eq!(le_bytes, &[0x34, 0x12]);

    // Big endian
    let writer = VecWriter::new();
    let mut encoder = EncoderImpl::new(writer, config::legacy().with_big_endian());
    value.encode(&mut encoder).expect("Encoding failed");
    let be_bytes = encoder.into_writer().into_vec();
    assert_eq!(be_bytes, &[0x12, 0x34]);
}

#[test]
fn test_slice_writer_exact_size() {
    let mut buffer = [0u8; 4];
    let config = config::legacy().with_little_endian();

    let writer = SliceWriter::new(&mut buffer);
    let mut encoder = EncoderImpl::new(writer, config);
    5u32.encode(&mut encoder).expect("Encoding failed");

    let bytes_written = encoder.into_writer().bytes_written();
    assert_eq!(bytes_written, 4);
    assert_eq!(buffer, [5, 0, 0, 0]);
}
