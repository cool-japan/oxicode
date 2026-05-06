//! Tests for `BorrowDecode<'de> for &'de [T]` where `T: BorrowableSliceElement`.
//!
//! Validates zero-copy slice borrowing for Pod-like primitive types under
//! `IntEncoding::Fixed` + native-endian configuration.

/// Build a Fixint + little-endian config for most tests.
fn fixint_le() -> oxicode::config::Configuration<
    oxicode::config::LittleEndian,
    oxicode::config::Fixint,
    oxicode::config::NoLimit,
> {
    oxicode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding()
}

#[test]
fn borrow_slice_u16_roundtrip() {
    let original: Vec<u16> = vec![0, 1, 256, 65535, 0x1234];
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode u16");
    let (decoded, consumed): (&[u16], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode u16");
    assert_eq!(decoded, original.as_slice(), "u16 roundtrip failed");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn borrow_slice_u32_roundtrip() {
    let original: Vec<u32> = vec![0, 1, u32::MAX / 2, u32::MAX];
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode u32");
    let (decoded, consumed): (&[u32], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode u32");
    assert_eq!(decoded, original.as_slice(), "u32 roundtrip failed");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn borrow_slice_u64_roundtrip() {
    let original: Vec<u64> = vec![0, 42, u64::MAX / 2, u64::MAX];
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode u64");
    let (decoded, consumed): (&[u64], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode u64");
    assert_eq!(decoded, original.as_slice(), "u64 roundtrip failed");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn borrow_slice_i16_roundtrip() {
    let original: Vec<i16> = vec![i16::MIN, -1, 0, 1, i16::MAX];
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode i16");
    let (decoded, consumed): (&[i16], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode i16");
    assert_eq!(decoded, original.as_slice(), "i16 roundtrip failed");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn borrow_slice_i32_roundtrip() {
    let original: Vec<i32> = vec![i32::MIN, -1, 0, 1, i32::MAX];
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode i32");
    let (decoded, consumed): (&[i32], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode i32");
    assert_eq!(decoded, original.as_slice(), "i32 roundtrip failed");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn borrow_slice_i64_roundtrip() {
    let original: Vec<i64> = vec![i64::MIN, -1, 0, 1, i64::MAX];
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode i64");
    let (decoded, consumed): (&[i64], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode i64");
    assert_eq!(decoded, original.as_slice(), "i64 roundtrip failed");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn borrow_slice_f32_roundtrip() {
    let original: Vec<f32> = vec![0.0_f32, 1.0, -1.0, f32::NEG_INFINITY, f32::NAN];
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode f32");
    let (decoded, consumed): (&[f32], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode f32");
    assert_eq!(consumed, encoded.len());
    // NaN != NaN, so compare element by element using bits
    assert_eq!(decoded.len(), original.len());
    for (a, b) in decoded.iter().zip(original.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f32 element mismatch");
    }
}

#[test]
fn borrow_slice_f64_roundtrip() {
    let original: Vec<f64> = vec![0.0_f64, 1.0, -1.0, f64::NEG_INFINITY, f64::NAN];
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode f64");
    let (decoded, consumed): (&[f64], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode f64");
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), original.len());
    for (a, b) in decoded.iter().zip(original.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 element mismatch");
    }
}

#[test]
fn borrow_slice_empty_u32() {
    let original: Vec<u32> = vec![];
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode empty");
    let (decoded, consumed): (&[u32], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode empty");
    assert!(decoded.is_empty(), "expected empty slice");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn borrow_slice_rejects_varint_encoding() {
    let original: Vec<u32> = vec![1, 2, 3];
    let config_varint = oxicode::config::standard().with_variable_int_encoding();
    let encoded =
        oxicode::encode_to_vec_with_config(&original, config_varint).expect("encode varint");
    let result: oxicode::Result<(&[u32], usize)> =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config_varint);
    assert!(
        result.is_err(),
        "should reject varint encoding, got: {:?}",
        result
    );
    let err_msg = format!("{:?}", result.unwrap_err());
    assert!(
        err_msg.contains("IntEncoding::Fixed") || err_msg.contains("varint"),
        "error should mention encoding: {}",
        err_msg
    );
}

#[test]
#[cfg(target_endian = "little")]
fn borrow_slice_rejects_cross_endian_on_le_host() {
    let original: Vec<u32> = vec![1, 2, 3];
    let config_be = oxicode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let encoded = oxicode::encode_to_vec_with_config(&original, config_be).expect("encode BE");
    let result: oxicode::Result<(&[u32], usize)> =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config_be);
    assert!(
        result.is_err(),
        "should reject BE encoding on LE host, got: {:?}",
        result
    );
    let err_msg = format!("{:?}", result.unwrap_err());
    assert!(
        err_msg.contains("endian") || err_msg.contains("native"),
        "error should mention endianness: {}",
        err_msg
    );
}

#[test]
#[cfg(target_endian = "big")]
fn borrow_slice_rejects_cross_endian_on_be_host() {
    let original: Vec<u32> = vec![1, 2, 3];
    let config_le = oxicode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();
    let encoded = oxicode::encode_to_vec_with_config(&original, config_le).expect("encode LE");
    let result: oxicode::Result<(&[u32], usize)> =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config_le);
    assert!(
        result.is_err(),
        "should reject LE encoding on BE host, got: {:?}",
        result
    );
    let err_msg = format!("{:?}", result.unwrap_err());
    assert!(
        err_msg.contains("endian") || err_msg.contains("native"),
        "error should mention endianness: {}",
        err_msg
    );
}

#[test]
fn borrow_slice_alignment_failure() {
    // Build a valid Fixint-encoded Vec<u32> payload.
    let original: Vec<u32> = vec![0xAABBCCDD_u32, 0x11223344_u32];
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode");

    // Prepend a single 0x00 byte to shift the data start by 1.
    // The buffer is now: [0x00, <encoded data>]
    // The encoded data starts at buf[1] which is offset 1 from the heap base.
    // Under Fixint: length occupies 8 bytes, so slice data is at buf[1+8] = buf[9].
    // If buf's heap allocation is 8-aligned, buf[9] is at addr % 8 == 1 — not 4-aligned.
    let mut misaligned = vec![0x00u8];
    misaligned.extend_from_slice(&encoded);

    // Decode from the misaligned slice (skip the 0x00 prefix).
    let result: oxicode::Result<(&[u32], usize)> =
        oxicode::borrow_decode_from_slice_with_config(&misaligned[1..], config);

    // On most allocators: the heap address is ≥ 8-aligned.
    // buf[1..] starts at addr+1, data at addr+1+8 = addr+9; addr+9 % 4 == 1 (not aligned).
    // We expect an alignment error. On unusual allocators it might still pass —
    // accept either outcome but if it succeeds, verify the values are correct.
    match result {
        Err(e) => {
            let err_msg = format!("{:?}", e);
            assert!(
                err_msg.contains("align") || err_msg.contains("InvalidData"),
                "alignment error should mention alignment: {}",
                err_msg
            );
        }
        Ok((decoded, _)) => {
            // If the allocator happens to be 16-aligned and the data ends up aligned,
            // the roundtrip should still produce correct values.
            assert_eq!(
                decoded,
                original.as_slice(),
                "alignment passed but values wrong"
            );
        }
    }
}

#[test]
fn borrow_slice_truncation() {
    // Manually construct a buffer that claims 8 u32 elements but only has bytes for 3.
    let config = fixint_le();
    // Encode a slice of 8 u32s, then truncate to only hold 3 elements' bytes.
    let original: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode");
    // Encoded = 8-byte length + 8 * 4 = 40 bytes data. Truncate to 8 + 12 = 20 bytes (3 elements).
    let truncated = &encoded[..8 + 3 * 4];
    let result: oxicode::Result<(&[u32], usize)> =
        oxicode::borrow_decode_from_slice_with_config(truncated, config);
    assert!(result.is_err(), "should fail on truncated buffer");
}

#[test]
fn borrow_slice_length_overflow() {
    // Construct a buffer with a length value that causes len * size_of::<u32>() to overflow.
    // u64::MAX / 4 * 4 would be u64::MAX — use u64::MAX / 2 as length for u32 (4-byte).
    let config = fixint_le();
    // Encode just the length, no actual data bytes.
    let len_val: u64 = usize::MAX as u64 / 2;
    let len_bytes = len_val.to_le_bytes();
    // Under Fixint, u64 is 8 bytes LE.
    let buf: Vec<u8> = len_bytes.to_vec();

    let result: oxicode::Result<(&[u32], usize)> =
        oxicode::borrow_decode_from_slice_with_config(&buf, config);
    assert!(result.is_err(), "should fail on overflow length");
}

#[test]
fn borrow_slice_u8_existing_path_unaffected() {
    // The existing concrete &[u8] impl should still work after adding the generic impl.
    let original: Vec<u8> = vec![1, 2, 3, 4, 5, 255];
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode u8");
    let (decoded, consumed): (&[u8], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode u8");
    assert_eq!(decoded, original.as_slice(), "existing &[u8] path broken");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn borrow_slice_str_existing_path_unaffected() {
    let original = String::from("hello zero-copy world");
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode str");
    let (decoded, consumed): (&str, _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode str");
    assert_eq!(decoded, original.as_str(), "existing &str path broken");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn borrow_slice_i8_existing_path_unaffected() {
    let original: Vec<i8> = vec![-128_i8, -1, 0, 1, 127];
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode i8");
    let (decoded, consumed): (&[i8], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode i8");
    assert_eq!(decoded, original.as_slice(), "existing &[i8] path broken");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn borrow_slice_u32_large() {
    let original: Vec<u32> = (0u32..1000).collect();
    let config = fixint_le();
    let encoded = oxicode::encode_to_vec_with_config(&original, config).expect("encode 1000");
    let (decoded, consumed): (&[u32], _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config).expect("decode 1000");
    assert_eq!(decoded, original.as_slice(), "large slice roundtrip failed");
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 1000);
}
