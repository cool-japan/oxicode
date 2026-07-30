//! WP-G1 hardening: `#[oxicode(tag_type = "u64")]` must honor explicit discriminants above
//! `u32::MAX` instead of capping/truncating at u32 (findings derive-audit#4 + panic-safety-audit#2
//! derive part, TODO lines 444 / 239).

#![cfg(all(feature = "alloc", feature = "derive"))]
use oxicode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
#[oxicode(tag_type = "u64")]
enum WideTag {
    #[oxicode(variant = 5_000_000_000)]
    Above(u32),
    #[oxicode(variant = 1)]
    Small,
    #[oxicode(variant = 18_000_000_000_000_000_000)]
    Enormous(u8),
}

#[test]
fn u64_discriminant_above_u32_max_roundtrips() {
    for value in [WideTag::Above(123), WideTag::Small, WideTag::Enormous(9)] {
        let encoded =
            oxicode::encode_to_vec_with_config(&value, config::standard()).expect("encode");
        let (decoded, _): (WideTag, usize) =
            oxicode::decode_from_slice_with_config(&encoded, config::standard()).expect("decode");
        assert_eq!(decoded, value);
    }
}

#[test]
fn u64_discriminant_writes_eight_byte_fixed_tag() {
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = oxicode::encode_to_vec_with_config(&WideTag::Above(0), cfg).expect("encode");
    // 8-byte little-endian tag == 5_000_000_000, then the u32 payload (4 bytes).
    assert_eq!(&encoded[..8], &5_000_000_000u64.to_le_bytes());
    let (decoded, _): (WideTag, usize) =
        oxicode::decode_from_slice_with_config(&encoded, cfg).expect("decode");
    assert_eq!(decoded, WideTag::Above(0));
}

#[test]
fn default_u32_tag_wire_layout_is_unchanged() {
    // A default-tag enum must still emit a 4-byte fixed discriminant (byte-for-byte parity with
    // the pre-hardening behavior).
    #[derive(Encode, Decode, PartialEq, Debug)]
    enum Normal {
        A,
        B(u16),
    }
    let cfg = config::standard().with_fixed_int_encoding();
    let a = oxicode::encode_to_vec_with_config(&Normal::A, cfg).expect("encode");
    assert_eq!(a, 0u32.to_le_bytes().to_vec());
    let b = oxicode::encode_to_vec_with_config(&Normal::B(7), cfg).expect("encode");
    let mut expected = 1u32.to_le_bytes().to_vec();
    expected.extend_from_slice(&7u16.to_le_bytes());
    assert_eq!(b, expected);
}
