//! Advanced encode/decode roundtrip property and invariant tests (set 2).
//!
//! Each test focuses on a specific guarantee or byte-level property of the
//! OxiCode encoding format: identity after roundtrip, determinism, consumed-bytes
//! invariants, concrete byte layouts, and structural encoding rules.

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ---------------------------------------------------------------------------
// Shared struct / enum definitions
// ---------------------------------------------------------------------------

/// Simple struct used in field-order and nested-encoding tests.
#[derive(Encode, Decode, Debug, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

/// Nested struct to verify flat serialisation of all fields.
#[derive(Encode, Decode, Debug, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

/// Enum with a unit variant and a tuple variant.
#[derive(Encode, Decode, Debug, PartialEq)]
enum Command {
    Quit,
    Move(u32, u32),
}

/// Large struct with many fields used for completeness roundtrip.
#[derive(Encode, Decode, Debug, PartialEq)]
struct Wide {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: i8,
    f: i16,
    g: i32,
    h: i64,
    i: bool,
    j: u8,
}

/// Mixed-type struct combining scalar, string, vec and optional fields.
#[derive(Encode, Decode, Debug, PartialEq)]
struct Mixed {
    id: u32,
    label: String,
    flags: Vec<bool>,
    score: Option<u64>,
}

// ---------------------------------------------------------------------------
// 1. encode then decode is identity for u32
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_identity_u32() {
    let val: u32 = 987_654;
    let enc = encode_to_vec(&val).expect("encode u32 failed");
    let (dec, _): (u32, usize) = decode_from_slice(&enc).expect("decode u32 failed");
    assert_eq!(dec, val, "roundtrip must preserve u32 value");
}

// ---------------------------------------------------------------------------
// 2. encode then decode is identity for String
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_identity_string() {
    let val = String::from("OxiCode roundtrip test");
    let enc = encode_to_vec(&val).expect("encode String failed");
    let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode String failed");
    assert_eq!(dec, val, "roundtrip must preserve String value");
}

// ---------------------------------------------------------------------------
// 3. encode then decode is identity for Vec<u8>
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_identity_vec_u8() {
    let val: Vec<u8> = vec![0u8, 1, 127, 128, 200, 255];
    let enc = encode_to_vec(&val).expect("encode Vec<u8> failed");
    let (dec, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8> failed");
    assert_eq!(dec, val, "roundtrip must preserve Vec<u8> value");
}

// ---------------------------------------------------------------------------
// 4. encode output is deterministic (encode twice produces same bytes)
// ---------------------------------------------------------------------------

#[test]
fn test_encode_deterministic() {
    let val: u64 = 123_456_789_000u64;
    let enc1 = encode_to_vec(&val).expect("first encode failed");
    let enc2 = encode_to_vec(&val).expect("second encode failed");
    assert_eq!(
        enc1, enc2,
        "encoding the same value twice must yield identical bytes"
    );
}

// ---------------------------------------------------------------------------
// 5. encode with standard config produces same bytes as bare encode_to_vec
// ---------------------------------------------------------------------------

#[test]
fn test_standard_config_matches_default() {
    let val: u32 = 42;
    let enc_default = encode_to_vec(&val).expect("encode_to_vec failed");
    let enc_explicit = encode_to_vec_with_config(&val, config::standard())
        .expect("encode_to_vec_with_config failed");
    assert_eq!(
        enc_default, enc_explicit,
        "encode_to_vec and encode_to_vec_with_config(standard) must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// 6. consumed bytes in decode == encoded length (invariant)
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_equals_encoded_length() {
    // Verify for several representative types
    let val_u32: u32 = 300;
    let enc = encode_to_vec(&val_u32).expect("encode u32");
    let (_, consumed): (u32, usize) = decode_from_slice(&enc).expect("decode u32");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length"
    );

    let val_str = String::from("consumed-bytes-check");
    let enc2 = encode_to_vec(&val_str).expect("encode String");
    let (_, consumed2): (String, usize) = decode_from_slice(&enc2).expect("decode String");
    assert_eq!(
        consumed2,
        enc2.len(),
        "consumed must equal encoded length for String"
    );

    let val_vec: Vec<u32> = vec![1, 2, 3, 4, 5];
    let enc3 = encode_to_vec(&val_vec).expect("encode Vec<u32>");
    let (_, consumed3): (Vec<u32>, usize) = decode_from_slice(&enc3).expect("decode Vec<u32>");
    assert_eq!(
        consumed3,
        enc3.len(),
        "consumed must equal encoded length for Vec<u32>"
    );
}

// ---------------------------------------------------------------------------
// 7. u8 value 0 encodes to exactly [0] (single raw byte, no varint header)
// ---------------------------------------------------------------------------

#[test]
fn test_u8_zero_encodes_to_single_byte_zero() {
    // u8 impl writes &[*self] directly — no varint overhead.
    let enc = encode_to_vec(&0u8).expect("encode u8=0");
    assert_eq!(
        enc,
        vec![0u8],
        "u8 value 0 must encode as the single byte 0x00"
    );
}

// ---------------------------------------------------------------------------
// 8. u8 value 127 encodes to exactly [127]
// ---------------------------------------------------------------------------

#[test]
fn test_u8_127_encodes_to_single_byte_127() {
    let enc = encode_to_vec(&127u8).expect("encode u8=127");
    assert_eq!(
        enc,
        vec![127u8],
        "u8 value 127 must encode as the single byte 0x7F"
    );
}

// ---------------------------------------------------------------------------
// 9. u8 value 128 encodes to exactly [128] (u8 is always 1 byte, no varint)
// ---------------------------------------------------------------------------

#[test]
fn test_u8_128_encodes_to_single_byte_128() {
    // u8 is always serialised as exactly one raw byte.
    // Varint is only applied to u16/u32/u64 etc., NOT to u8.
    let enc = encode_to_vec(&128u8).expect("encode u8=128");
    assert_eq!(
        enc,
        vec![128u8],
        "u8 value 128 must encode as the single byte 0x80 — u8 uses no varint wrapping"
    );
    let (dec, _): (u8, usize) = decode_from_slice(&enc).expect("decode u8=128");
    assert_eq!(dec, 128u8);
}

// ---------------------------------------------------------------------------
// 10. String "hello" encodes to [5, 'h','e','l','l','o']
//     Length prefix is u64 varint; 5 fits in 1 byte (<=250).
// ---------------------------------------------------------------------------

#[test]
fn test_string_hello_exact_bytes() {
    let enc = encode_to_vec(&String::from("hello")).expect("encode 'hello'");
    // length 5 as u64 varint = [5], then UTF-8 bytes of "hello"
    assert_eq!(
        enc,
        vec![5u8, b'h', b'e', b'l', b'l', b'o'],
        "String 'hello' must encode as [5, 104, 101, 108, 108, 111]"
    );
}

// ---------------------------------------------------------------------------
// 11. Vec<u8> [1,2,3] encodes to [3, 1, 2, 3]
// ---------------------------------------------------------------------------

#[test]
fn test_vec_u8_exact_bytes() {
    let val: Vec<u8> = vec![1u8, 2, 3];
    let enc = encode_to_vec(&val).expect("encode Vec<u8> [1,2,3]");
    // length 3 as u64 varint = [3], then the three raw u8 bytes
    assert_eq!(
        enc,
        vec![3u8, 1, 2, 3],
        "Vec<u8> [1,2,3] must encode as [3, 1, 2, 3]"
    );
}

// ---------------------------------------------------------------------------
// 12. bool true = [1], bool false = [0]
// ---------------------------------------------------------------------------

#[test]
fn test_bool_exact_bytes() {
    let enc_true = encode_to_vec(&true).expect("encode true");
    let enc_false = encode_to_vec(&false).expect("encode false");
    assert_eq!(enc_true, vec![1u8], "bool true must encode as [1]");
    assert_eq!(enc_false, vec![0u8], "bool false must encode as [0]");
}

// ---------------------------------------------------------------------------
// 13. None encodes to [0], Some(42u32) encodes to [1, varint(42)]
// ---------------------------------------------------------------------------

#[test]
fn test_option_exact_bytes() {
    let none: Option<u32> = None;
    let some42: Option<u32> = Some(42);

    let enc_none = encode_to_vec(&none).expect("encode None");
    assert_eq!(enc_none, vec![0u8], "None must encode as [0]");

    let enc_some = encode_to_vec(&some42).expect("encode Some(42)");
    // discriminant = 1u8, then 42u32 as varint (42<=250 → single byte [42])
    assert_eq!(
        enc_some,
        vec![1u8, 42u8],
        "Some(42u32) must encode as [1, 42]"
    );

    // Roundtrip check
    let (dec_none, _): (Option<u32>, usize) = decode_from_slice(&enc_none).expect("decode None");
    let (dec_some, _): (Option<u32>, usize) = decode_from_slice(&enc_some).expect("decode Some");
    assert_eq!(dec_none, None);
    assert_eq!(dec_some, Some(42u32));
}

// ---------------------------------------------------------------------------
// 14. Empty Vec<u32> encodes to [0] (length 0 as u64 varint)
// ---------------------------------------------------------------------------

#[test]
fn test_empty_vec_u32_exact_bytes() {
    let val: Vec<u32> = vec![];
    let enc = encode_to_vec(&val).expect("encode empty Vec<u32>");
    assert_eq!(
        enc,
        vec![0u8],
        "empty Vec<u32> must encode as [0] (varint length prefix only)"
    );
    let (dec, _): (Vec<u32>, usize) = decode_from_slice(&enc).expect("decode empty Vec<u32>");
    assert_eq!(dec, val);
}

// ---------------------------------------------------------------------------
// 15. Struct fields are encoded in declaration order
// ---------------------------------------------------------------------------

#[test]
fn test_struct_field_declaration_order() {
    // Point { x: u32, y: u32 } — both fit in 1-byte varint
    let p = Point { x: 10, y: 20 };
    let enc = encode_to_vec(&p).expect("encode Point");

    // Manually construct expected bytes: varint(10) + varint(20)
    let enc_x = encode_to_vec(&10u32).expect("encode x=10");
    let enc_y = encode_to_vec(&20u32).expect("encode y=20");
    let mut expected = Vec::new();
    expected.extend_from_slice(&enc_x);
    expected.extend_from_slice(&enc_y);

    assert_eq!(
        enc, expected,
        "Point fields must appear in declaration order: x then y"
    );

    let (dec, _): (Point, usize) = decode_from_slice(&enc).expect("decode Point");
    assert_eq!(dec, p);
}

// ---------------------------------------------------------------------------
// 16. Enum: unit variant = discriminant only; tuple variant = discriminant + fields
// ---------------------------------------------------------------------------

#[test]
fn test_enum_unit_and_tuple_variant_encoding() {
    // Command::Quit is variant index 0 (u32 varint)
    let enc_quit = encode_to_vec(&Command::Quit).expect("encode Quit");
    let enc_disc_0 = encode_to_vec(&0u32).expect("encode discriminant 0");
    assert_eq!(
        enc_quit, enc_disc_0,
        "unit variant Quit must encode as its discriminant only"
    );

    // Command::Move(5, 6) is variant index 1 followed by two u32 varint fields
    let enc_move = encode_to_vec(&Command::Move(5, 6)).expect("encode Move(5,6)");
    let enc_disc_1 = encode_to_vec(&1u32).expect("encode discriminant 1");
    let enc_5 = encode_to_vec(&5u32).expect("encode 5");
    let enc_6 = encode_to_vec(&6u32).expect("encode 6");
    let mut expected_move = Vec::new();
    expected_move.extend_from_slice(&enc_disc_1);
    expected_move.extend_from_slice(&enc_5);
    expected_move.extend_from_slice(&enc_6);
    assert_eq!(
        enc_move, expected_move,
        "tuple variant Move must encode as discriminant then fields"
    );

    // Roundtrip
    let (dec_quit, _): (Command, usize) = decode_from_slice(&enc_quit).expect("decode Quit");
    let (dec_move, _): (Command, usize) = decode_from_slice(&enc_move).expect("decode Move");
    assert_eq!(dec_quit, Command::Quit);
    assert_eq!(dec_move, Command::Move(5, 6));
}

// ---------------------------------------------------------------------------
// 17. Nested struct encodes as flat sequence of all leaf fields
// ---------------------------------------------------------------------------

#[test]
fn test_nested_struct_flat_encoding() {
    let line = Line {
        start: Point { x: 1, y: 2 },
        end: Point { x: 3, y: 4 },
    };
    let enc_line = encode_to_vec(&line).expect("encode Line");

    // Build expected from individual u32 varint encodings in field order
    let bytes: Vec<u8> = [1u32, 2, 3, 4]
        .iter()
        .flat_map(|v| encode_to_vec(v).expect("encode field"))
        .collect();
    assert_eq!(
        enc_line, bytes,
        "nested struct must encode as a flat sequence of all leaf field values"
    );

    let (dec, _): (Line, usize) = decode_from_slice(&enc_line).expect("decode Line");
    assert_eq!(dec, line);
}

// ---------------------------------------------------------------------------
// 18. Double roundtrip: encode -> decode -> encode -> decode matches original
// ---------------------------------------------------------------------------

#[test]
fn test_double_roundtrip_consistency() {
    let original = Mixed {
        id: 7,
        label: String::from("double-trip"),
        flags: vec![true, false, true],
        score: Some(999u64),
    };

    let enc1 = encode_to_vec(&original).expect("first encode");
    let (mid, _): (Mixed, usize) = decode_from_slice(&enc1).expect("first decode");
    assert_eq!(mid, original, "first roundtrip must preserve value");

    let enc2 = encode_to_vec(&mid).expect("second encode");
    assert_eq!(
        enc1, enc2,
        "second encode must produce identical bytes to first"
    );

    let (final_val, _): (Mixed, usize) = decode_from_slice(&enc2).expect("second decode");
    assert_eq!(
        final_val, original,
        "second roundtrip must still match original"
    );
}

// ---------------------------------------------------------------------------
// 19. Large struct: all fields preserved after roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_large_struct_all_fields_preserved() {
    let original = Wide {
        a: 255u8,
        b: 60_000u16,
        c: 3_000_000_000u32,
        d: 9_000_000_000_000_000_000u64,
        e: -128i8,
        f: -32768i16,
        g: -2_000_000_000i32,
        h: -9_000_000_000_000_000_000i64,
        i: true,
        j: 42u8,
    };
    let enc = encode_to_vec(&original).expect("encode Wide");
    let (dec, consumed): (Wide, usize) = decode_from_slice(&enc).expect("decode Wide");
    assert_eq!(
        dec, original,
        "all fields in Wide must be identical after roundtrip"
    );
    assert_eq!(consumed, enc.len(), "all encoded bytes must be consumed");
}

// ---------------------------------------------------------------------------
// 20. Zero value for all numeric types encodes correctly (roundtrip)
// ---------------------------------------------------------------------------

#[test]
fn test_zero_values_all_numeric_types_roundtrip() {
    macro_rules! check_zero_roundtrip {
        ($t:ty) => {{
            let z: $t = 0 as $t;
            let enc = encode_to_vec(&z).expect(concat!("encode zero ", stringify!($t)));
            assert!(
                !enc.is_empty(),
                concat!("zero ", stringify!($t), " must encode to non-empty bytes")
            );
            let (dec, consumed): ($t, usize) =
                decode_from_slice(&enc).expect(concat!("decode zero ", stringify!($t)));
            assert_eq!(
                dec, z,
                concat!("zero roundtrip failed for ", stringify!($t))
            );
            assert_eq!(
                consumed,
                enc.len(),
                concat!("consumed mismatch for zero ", stringify!($t))
            );
        }};
    }
    check_zero_roundtrip!(u8);
    check_zero_roundtrip!(u16);
    check_zero_roundtrip!(u32);
    check_zero_roundtrip!(u64);
    check_zero_roundtrip!(i8);
    check_zero_roundtrip!(i16);
    check_zero_roundtrip!(i32);
    check_zero_roundtrip!(i64);
    check_zero_roundtrip!(f32);
    check_zero_roundtrip!(f64);
}

// ---------------------------------------------------------------------------
// 21. MAX values for u8, u16, u32 all roundtrip correctly
// ---------------------------------------------------------------------------

#[test]
fn test_max_values_unsigned_roundtrip() {
    // u8::MAX = 255 — always 1 byte
    let enc_u8_max = encode_to_vec(&u8::MAX).expect("encode u8::MAX");
    assert_eq!(enc_u8_max.len(), 1, "u8::MAX must encode to exactly 1 byte");
    let (dec_u8, _): (u8, usize) = decode_from_slice(&enc_u8_max).expect("decode u8::MAX");
    assert_eq!(dec_u8, u8::MAX);

    // u16::MAX = 65535 — varint: tag 251 + 2 bytes little-endian
    let enc_u16_max = encode_to_vec(&u16::MAX).expect("encode u16::MAX");
    assert_eq!(
        enc_u16_max.len(),
        3,
        "u16::MAX varint must be 3 bytes (tag + 2)"
    );
    assert_eq!(enc_u16_max[0], 251u8, "u16 varint tag byte must be 251");
    let (dec_u16, _): (u16, usize) = decode_from_slice(&enc_u16_max).expect("decode u16::MAX");
    assert_eq!(dec_u16, u16::MAX);

    // u32::MAX = 4294967295 — varint: tag 252 + 4 bytes little-endian
    let enc_u32_max = encode_to_vec(&u32::MAX).expect("encode u32::MAX");
    assert_eq!(
        enc_u32_max.len(),
        5,
        "u32::MAX varint must be 5 bytes (tag + 4)"
    );
    assert_eq!(enc_u32_max[0], 252u8, "u32 varint tag byte must be 252");
    let (dec_u32, _): (u32, usize) = decode_from_slice(&enc_u32_max).expect("decode u32::MAX");
    assert_eq!(dec_u32, u32::MAX);
}

// ---------------------------------------------------------------------------
// 22. Mixed type roundtrip: struct with u32, String, Vec<bool>, Option<u64>
// ---------------------------------------------------------------------------

#[test]
fn test_mixed_type_struct_roundtrip() {
    let original = Mixed {
        id: 1_000_000u32,
        label: String::from("binary-serialization"),
        flags: vec![false, true, false, true, true],
        score: Some(u64::MAX / 2),
    };
    let enc = encode_to_vec(&original).expect("encode Mixed");
    assert!(
        !enc.is_empty(),
        "Mixed struct must produce non-empty encoding"
    );
    let (dec, consumed): (Mixed, usize) = decode_from_slice(&enc).expect("decode Mixed");
    assert_eq!(dec.id, original.id, "id field must roundtrip correctly");
    assert_eq!(
        dec.label, original.label,
        "label field must roundtrip correctly"
    );
    assert_eq!(
        dec.flags, original.flags,
        "flags field must roundtrip correctly"
    );
    assert_eq!(
        dec.score, original.score,
        "score field must roundtrip correctly"
    );
    assert_eq!(consumed, enc.len(), "all encoded bytes must be consumed");

    // Also verify with explicit standard config — must be byte-for-byte identical
    let enc_cfg =
        encode_to_vec_with_config(&original, config::standard()).expect("encode Mixed with config");
    assert_eq!(
        enc, enc_cfg,
        "standard config must produce identical bytes to default"
    );

    let (dec_cfg, _): (Mixed, usize) = decode_from_slice_with_config(&enc_cfg, config::standard())
        .expect("decode Mixed with config");
    assert_eq!(
        dec_cfg, original,
        "decode_from_slice_with_config must reproduce original Mixed"
    );
}
