//! WP-G1 hardening: `#[oxicode(bytes)]` must work for byte containers other than `Vec<u8>`
//! (finding derive-audit#2, TODO line 434) and must not be silently dropped under `BorrowDecode`.

#![cfg(all(feature = "alloc", feature = "derive"))]
use oxicode::{config, BorrowDecode, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct OwnedByteFields {
    #[oxicode(bytes)]
    boxed: Box<[u8]>,
    #[oxicode(bytes)]
    vec: Vec<u8>,
    #[oxicode(bytes)]
    array: [u8; 4],
}

#[test]
fn bytes_decode_into_non_vec_containers_roundtrips() {
    let value = OwnedByteFields {
        boxed: vec![1u8, 2, 3].into_boxed_slice(),
        vec: vec![9, 8, 7, 6, 5],
        array: [10, 20, 30, 40],
    };

    let encoded = oxicode::encode_to_vec_with_config(&value, config::standard()).expect("encode");
    let (decoded, _): (OwnedByteFields, usize) =
        oxicode::decode_from_slice_with_config(&encoded, config::standard()).expect("decode");

    assert_eq!(decoded, value);
}

#[test]
fn bytes_array_length_mismatch_is_rejected_not_panicked() {
    // Encode a 4-byte array; then hand the byte payload to a struct whose array field is a
    // different length. The TryFrom<Vec<u8>> conversion must surface an error, not panic.
    #[derive(Encode)]
    struct Src {
        #[oxicode(bytes)]
        array: [u8; 4],
    }
    #[derive(Decode)]
    #[allow(dead_code)]
    struct Dst {
        #[oxicode(bytes)]
        array: [u8; 8],
    }

    let encoded = oxicode::encode_to_vec_with_config(
        &Src {
            array: [1, 2, 3, 4],
        },
        config::standard(),
    )
    .expect("encode");
    let result: Result<(Dst, usize), _> =
        oxicode::decode_from_slice_with_config(&encoded, config::standard());
    assert!(result.is_err(), "array length mismatch must error");
}

// --- BorrowDecode: the `bytes` attribute must be honored, not silently dropped ---

#[derive(Encode, BorrowDecode, PartialEq, Debug)]
struct BorrowedByteField<'a> {
    #[oxicode(bytes)]
    raw: &'a [u8],
    tail: u32,
}

#[test]
fn bytes_borrow_decode_zero_copy_slice_roundtrips() {
    let value = BorrowedByteField {
        raw: &[3, 1, 4, 1, 5, 9],
        tail: 42,
    };
    let encoded = oxicode::encode_to_vec_with_config(&value, config::standard()).expect("encode");
    let (decoded, _): (BorrowedByteField, usize) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config::standard())
            .expect("borrow decode");
    assert_eq!(decoded, value);
    // The decoded slice must point back into the input buffer (zero-copy).
    assert!(decoded.raw.as_ptr() >= encoded.as_ptr());
}

#[derive(Encode, BorrowDecode, PartialEq, Debug)]
struct BorrowedOwnedBytes<'a> {
    #[oxicode(bytes)]
    owned: Vec<u8>,
    borrowed: &'a str,
}

#[test]
fn bytes_borrow_decode_into_owned_container_roundtrips() {
    let value = BorrowedOwnedBytes {
        owned: vec![7, 7, 7],
        borrowed: "hi",
    };
    let encoded = oxicode::encode_to_vec_with_config(&value, config::standard()).expect("encode");
    let (decoded, _): (BorrowedOwnedBytes, usize) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config::standard())
            .expect("borrow decode");
    assert_eq!(decoded, value);
}

#[test]
fn bytes_owned_and_borrow_decode_agree_on_wire() {
    // The BorrowDecode `bytes` branch must read the exact same wire layout the owned Decode path
    // reads, so an owned-encoded value decodes identically through both entry points.
    #[derive(Encode, Decode, BorrowDecode, PartialEq, Debug)]
    struct Both {
        #[oxicode(bytes)]
        data: Vec<u8>,
    }
    let value = Both {
        data: vec![0, 1, 2, 3, 4],
    };
    let encoded = oxicode::encode_to_vec_with_config(&value, config::standard()).expect("encode");

    let (owned, _): (Both, usize) =
        oxicode::decode_from_slice_with_config(&encoded, config::standard()).expect("owned");
    let (borrowed, _): (Both, usize) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, config::standard())
            .expect("borrowed");
    assert_eq!(owned, value);
    assert_eq!(borrowed, value);
}
