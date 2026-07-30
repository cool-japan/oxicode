//! Hardening test for TODO.md line 572 (WP-J1): `oxicode::decode_from_hex`
//! must reject any non-hex character, including a leading `+`/`-` sign that
//! `u8::from_str_radix` would otherwise silently accept (e.g.
//! `u8::from_str_radix("+f", 16) == Ok(15)`), contradicting its own
//! documented behavior ("Returns Err if the hex string contains non-hex
//! characters").

#![cfg(feature = "std")]

#[test]
fn decode_from_hex_rejects_leading_plus_sign() {
    // Previously accepted: u8::from_str_radix("+f", 16) == Ok(15).
    let result: Result<(u8, usize), _> = oxicode::decode_from_hex("+f");
    assert!(result.is_err(), "expected \"+f\" to be rejected as non-hex");

    let result: Result<(u8, usize), _> = oxicode::decode_from_hex("0+");
    assert!(result.is_err(), "expected \"0+\" to be rejected as non-hex");
}

#[test]
fn decode_from_hex_rejects_leading_minus_sign() {
    // Previously accepted: u8::from_str_radix("-2", 16) == Ok(-2 as u8 wrapping? actually Err for u8)
    // but for i8-style radix parsing on unsigned types, '-' is still consumed as a sign
    // character by the parser before failing on unsigned overflow in some cases; verify
    // oxicode rejects it outright regardless of the underlying integer's signedness handling.
    let result: Result<(u8, usize), _> = oxicode::decode_from_hex("-2");
    assert!(result.is_err(), "expected \"-2\" to be rejected as non-hex");
}

#[test]
fn decode_from_hex_rejects_other_non_hex_characters() {
    for bad in ["0x", "g1", "1 ", "  ", "!!", "0X", " f"] {
        let result: Result<(u8, usize), _> = oxicode::decode_from_hex(bad);
        assert!(
            result.is_err(),
            "expected {:?} to be rejected as non-hex",
            bad
        );
    }
}

#[test]
fn decode_from_hex_accepts_valid_hex_roundtrip() {
    let hex = oxicode::encode_to_hex(&0xABu8).expect("encode");
    let (value, consumed): (u8, usize) = oxicode::decode_from_hex(&hex).expect("decode");
    assert_eq!(value, 0xABu8);
    assert_eq!(consumed, 1);

    // Uppercase hex digits remain valid.
    let (value, _): (u8, usize) = oxicode::decode_from_hex("FF").expect("decode uppercase");
    assert_eq!(value, 0xFFu8);
}

#[test]
fn decode_from_hex_rejects_odd_length() {
    let result: Result<(u8, usize), _> = oxicode::decode_from_hex("abc");
    assert!(
        result.is_err(),
        "expected odd-length hex string to be rejected"
    );
}
