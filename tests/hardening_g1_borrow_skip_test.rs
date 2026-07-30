//! WP-G1 hardening: the `BorrowDecode` derive must honor variant-level `#[oxicode(skip)]` exactly
//! like `Decode` — a skipped variant gets no borrow-decode arm, so both derives accept the same
//! byte set (finding derive-audit#1, TODO line 429).

#![cfg(all(feature = "alloc", feature = "derive"))]
use oxicode::{config, BorrowDecode, Encode};

#[derive(Encode, BorrowDecode, PartialEq, Debug)]
#[allow(dead_code)] // `Legacy` is intentionally never constructed; it exists to be skipped.
enum Msg<'a> {
    Hello(&'a str), // natural discriminant 0
    #[oxicode(skip)]
    Legacy(u32), // discriminant 1 — skipped, aliases the successor below
    Bye(u32),       // natural discriminant 2
}

#[test]
fn borrow_decode_skip_variant_roundtrips_successor() {
    for value in [Msg::Hello("hi"), Msg::Bye(7)] {
        let encoded =
            oxicode::encode_to_vec_with_config(&value, config::standard()).expect("encode");
        let (decoded, _): (Msg, usize) =
            oxicode::borrow_decode_from_slice_with_config(&encoded, config::standard())
                .expect("borrow decode");
        assert_eq!(decoded, value);
    }
}

#[test]
fn borrow_decode_rejects_skipped_variant_discriminant() {
    // Discriminant 1 belongs to the skipped `Legacy` variant. Under the fix, `BorrowDecode`
    // filters skipped variants (like `Decode`), so no arm matches tag 1 and decoding errors
    // instead of materializing a "skipped" variant.
    let cfg = config::standard().with_fixed_int_encoding();
    let mut bytes = 1u32.to_le_bytes().to_vec(); // tag = 1 (Legacy)
    bytes.extend_from_slice(&0u32.to_le_bytes()); // a plausible u32 payload
    let result: Result<(Msg, usize), _> =
        oxicode::borrow_decode_from_slice_with_config(&bytes, cfg);
    assert!(
        result.is_err(),
        "a skipped variant's own discriminant must not decode under BorrowDecode"
    );
}
