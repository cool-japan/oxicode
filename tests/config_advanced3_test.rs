//! Advanced configuration combination tests (set 3).
//!
//! Covers 22 distinct test scenarios exercising OxiCode config variants:
//! standard, legacy, fixed-int, big-endian, little-endian, with_limit, and combinations.

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
    encode_to_vec_with_config,
};

// ---------------------------------------------------------------------------
// 1. Standard config u32 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_standard_config_u32_roundtrip() {
    let value: u32 = 123_456;
    let enc = encode_to_vec_with_config(&value, config::standard()).expect("standard encode u32");
    let (val, _): (u32, usize) =
        decode_from_slice_with_config(&enc, config::standard()).expect("standard decode u32");
    assert_eq!(val, value);
}

// ---------------------------------------------------------------------------
// 2. Legacy config u32 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_legacy_config_u32_roundtrip() {
    let value: u32 = 987_654;
    let enc = encode_to_vec_with_config(&value, config::legacy()).expect("legacy encode u32");
    let (val, _): (u32, usize) =
        decode_from_slice_with_config(&enc, config::legacy()).expect("legacy decode u32");
    assert_eq!(val, value);
}

// ---------------------------------------------------------------------------
// 3. Fixed-int config: u32 is exactly 4 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_config_u32_size() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0u32, 1, 127, 255, 65535, u32::MAX] {
        let enc = encode_to_vec_with_config(&value, cfg).expect("fixed u32 encode");
        assert_eq!(
            enc.len(),
            4,
            "fixed-int u32 must be exactly 4 bytes; value={value}"
        );
    }
}

// ---------------------------------------------------------------------------
// 4. Fixed-int config: u64 is exactly 8 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_config_u64_size() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0u64, 1, 255, 65535, u32::MAX as u64, u64::MAX] {
        let enc = encode_to_vec_with_config(&value, cfg).expect("fixed u64 encode");
        assert_eq!(
            enc.len(),
            8,
            "fixed-int u64 must be exactly 8 bytes; value={value}"
        );
    }
}

// ---------------------------------------------------------------------------
// 5. Fixed-int config: u16 is exactly 2 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_config_u16_size() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0u16, 1, 127, 255, u16::MAX] {
        let enc = encode_to_vec_with_config(&value, cfg).expect("fixed u16 encode");
        assert_eq!(
            enc.len(),
            2,
            "fixed-int u16 must be exactly 2 bytes; value={value}"
        );
    }
}

// ---------------------------------------------------------------------------
// 6. Standard vs fixed-int small value: standard uses fewer bytes for u64=1
// ---------------------------------------------------------------------------

#[test]
fn test_standard_vs_fixed_int_small_value() {
    let value: u64 = 1;
    let std_enc =
        encode_to_vec_with_config(&value, config::standard()).expect("standard encode u64=1");
    let fixed_enc = encode_to_vec_with_config(&value, config::standard().with_fixed_int_encoding())
        .expect("fixed encode u64=1");
    // Varint for 1 is 1 byte; fixed u64 is 8 bytes.
    assert!(
        std_enc.len() < fixed_enc.len(),
        "varint must use fewer bytes than fixed-int for small u64=1; varint={}, fixed={}",
        std_enc.len(),
        fixed_enc.len()
    );
    assert_eq!(std_enc.len(), 1, "varint u64=1 must be 1 byte");
    assert_eq!(fixed_enc.len(), 8, "fixed u64=1 must be 8 bytes");
}

// ---------------------------------------------------------------------------
// 7. Standard vs fixed-int large value: u32::MAX
// ---------------------------------------------------------------------------

#[test]
fn test_standard_vs_fixed_int_max_value() {
    let value: u32 = u32::MAX;
    let std_enc =
        encode_to_vec_with_config(&value, config::standard()).expect("standard encode u32::MAX");
    let fixed_enc = encode_to_vec_with_config(&value, config::standard().with_fixed_int_encoding())
        .expect("fixed encode u32::MAX");
    // Varint u32::MAX (0xFFFFFFFF) needs 5 bytes; fixed is always 4 bytes.
    assert!(
        std_enc.len() >= fixed_enc.len(),
        "varint u32::MAX must use >= bytes than fixed; varint={}, fixed={}",
        std_enc.len(),
        fixed_enc.len()
    );
    assert_eq!(fixed_enc.len(), 4, "fixed u32 must be 4 bytes");
}

// ---------------------------------------------------------------------------
// 8. Big-endian u32 byte order: 0x01020304 encodes as [01,02,03,04]
// ---------------------------------------------------------------------------

#[test]
fn test_big_endian_u32_byte_order() {
    let val: u32 = 0x0102_0304;
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("big-endian encode");
    assert_eq!(
        enc,
        &[0x01, 0x02, 0x03, 0x04],
        "big-endian fixed u32 0x01020304 must be [01,02,03,04]"
    );
    let (decoded, _): (u32, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("big-endian decode");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 9. Little-endian u32 byte order: 0x01020304 encodes as [04,03,02,01]
// ---------------------------------------------------------------------------

#[test]
fn test_little_endian_u32_byte_order() {
    let val: u32 = 0x0102_0304;
    let cfg = config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("little-endian encode");
    assert_eq!(
        enc,
        &[0x04, 0x03, 0x02, 0x01],
        "little-endian fixed u32 0x01020304 must be [04,03,02,01]"
    );
    let (decoded, _): (u32, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("little-endian decode");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 10. Big-endian u64 byte order: first byte is 0x01
// ---------------------------------------------------------------------------

#[test]
fn test_big_endian_u64_byte_order() {
    let val: u64 = 0x0102_0304_0506_0708;
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("big-endian u64 encode");
    assert_eq!(enc.len(), 8, "fixed u64 must be 8 bytes");
    assert_eq!(enc[0], 0x01, "first byte must be 0x01 in big-endian");
    assert_eq!(
        enc,
        &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        "big-endian fixed u64 must be MSB-first"
    );
    let (decoded, _): (u64, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("big-endian u64 decode");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 11. Explicit little-endian config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_little_endian_explicit_config() {
    let value: u64 = 0xDEAD_BEEF_CAFE_BABE;
    let cfg = config::standard().with_little_endian();
    let enc = encode_to_vec_with_config(&value, cfg).expect("explicit LE encode");
    let (decoded, consumed): (u64, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("explicit LE decode");
    assert_eq!(decoded, value);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// 12. Config limit OK: small value within limit succeeds
// ---------------------------------------------------------------------------

#[test]
fn test_config_limit_ok() {
    let value: u32 = 42;
    let cfg = config::standard().with_limit::<100>();
    let enc = encode_to_vec_with_config(&value, cfg).expect("encode within limit");
    // Varint 42 is 1 byte, well within limit=100.
    assert!(enc.len() <= 100, "encoded size must be within limit");
    let (decoded, _): (u32, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode within limit");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// 13. Config limit exceeded: decoding a Vec with a 1-byte limit must fail
// ---------------------------------------------------------------------------

#[test]
fn test_config_limit_exceeded() {
    // Encode a Vec<u8> without any limit to get valid bytes.
    let data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let enc = encode_to_vec_with_config(&data, config::standard()).expect("unlimited encode");
    // Now attempt to decode those bytes with a limit of 1 byte, which is far too small.
    // The decoder calls claim_bytes_read for the payload length, which must exceed the 1-byte limit.
    let cfg = config::standard().with_limit::<1>();
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice_with_config(&enc, cfg);
    assert!(
        result.is_err(),
        "decoding a 5-element Vec<u8> with a 1-byte limit must fail"
    );
}

// ---------------------------------------------------------------------------
// 14. Standard config String roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_standard_config_string_roundtrip() {
    let value = String::from("OxiCode config string test");
    let enc =
        encode_to_vec_with_config(&value, config::standard()).expect("standard string encode");
    let (decoded, _): (String, usize) =
        decode_from_slice_with_config(&enc, config::standard()).expect("standard string decode");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// 15. Legacy config String roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_legacy_config_string_roundtrip() {
    let value = String::from("legacy config string");
    let enc = encode_to_vec_with_config(&value, config::legacy()).expect("legacy string encode");
    let (decoded, _): (String, usize) =
        decode_from_slice_with_config(&enc, config::legacy()).expect("legacy string decode");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// 16. Standard config Vec<u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_standard_vec_u32_roundtrip() {
    let value: Vec<u32> = vec![0, 1, 100, 1000, u32::MAX];
    let enc =
        encode_to_vec_with_config(&value, config::standard()).expect("standard Vec<u32> encode");
    let (decoded, _): (Vec<u32>, usize) =
        decode_from_slice_with_config(&enc, config::standard()).expect("standard Vec<u32> decode");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// 17. Fixed-int config Vec<u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_vec_u32_roundtrip() {
    let value: Vec<u32> = vec![10, 20, 30, 40, 50];
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&value, cfg).expect("fixed Vec<u32> encode");
    // Length prefix (u64 varint=1 byte because len=5) + 5*4 = 21 bytes with fixed-int for values,
    // but length is still encoded as varint with standard base. With fixed-int everything is fixed.
    // Actually with fixed_int_encoding the length prefix also becomes u64 fixed = 8 bytes + 5*4=20 => 28.
    let (decoded, _): (Vec<u32>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("fixed Vec<u32> decode");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// 18. Big-endian String roundtrip (endianness should not affect string bytes)
// ---------------------------------------------------------------------------

#[test]
fn test_big_endian_string_roundtrip() {
    let value = String::from("endian-neutral string");
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&value, cfg).expect("big-endian string encode");
    let (decoded, _): (String, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("big-endian string decode");
    assert_eq!(
        decoded, value,
        "string roundtrip must work with big-endian config"
    );
}

// ---------------------------------------------------------------------------
// 19. Standard config bool is 1 byte
// ---------------------------------------------------------------------------

#[test]
fn test_standard_config_bool_size() {
    let enc_true = encode_to_vec_with_config(&true, config::standard()).expect("bool true encode");
    let enc_false =
        encode_to_vec_with_config(&false, config::standard()).expect("bool false encode");
    assert_eq!(enc_true.len(), 1, "bool true must be 1 byte");
    assert_eq!(enc_false.len(), 1, "bool false must be 1 byte");
    assert_ne!(enc_true[0], enc_false[0], "true and false must differ");

    let (decoded_true, _): (bool, usize) =
        decode_from_slice_with_config(&enc_true, config::standard()).expect("bool true decode");
    let (decoded_false, _): (bool, usize) =
        decode_from_slice_with_config(&enc_false, config::standard()).expect("bool false decode");
    assert!(decoded_true);
    assert!(!decoded_false);
}

// ---------------------------------------------------------------------------
// 20. Legacy == standard().with_fixed_int_encoding() for u32
// ---------------------------------------------------------------------------

#[test]
fn test_legacy_config_produces_same_as_fixed_int() {
    let value: u32 = 999;
    let legacy_enc =
        encode_to_vec_with_config(&value, config::legacy()).expect("legacy encode u32=999");
    let fixed_enc = encode_to_vec_with_config(&value, config::standard().with_fixed_int_encoding())
        .expect("fixed encode u32=999");
    assert_eq!(
        legacy_enc, fixed_enc,
        "legacy() must produce same bytes as standard().with_fixed_int_encoding() for u32=999"
    );
}

// ---------------------------------------------------------------------------
// 21. Config clone/copy behavior: config can be used multiple times
// ---------------------------------------------------------------------------

#[test]
fn test_config_clone_copy_behavior() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    // Pass cfg by value multiple times to confirm it is Copy.
    let enc1 = encode_to_vec_with_config(&100u32, cfg).expect("first encode");
    let enc2 = encode_to_vec_with_config(&100u32, cfg).expect("second encode");
    assert_eq!(
        enc1, enc2,
        "same config used twice must produce identical bytes"
    );

    let (val1, _): (u32, usize) = decode_from_slice_with_config(&enc1, cfg).expect("first decode");
    let (val2, _): (u32, usize) = decode_from_slice_with_config(&enc2, cfg).expect("second decode");
    assert_eq!(val1, 100u32);
    assert_eq!(val2, 100u32);
}

// ---------------------------------------------------------------------------
// 22. Limit=1024 config roundtrip for multiple types
// ---------------------------------------------------------------------------

#[test]
fn test_limit_config_roundtrip_multiple_types() {
    let cfg = config::standard().with_limit::<1024>();

    // u32
    let u32_val: u32 = 77_777;
    let enc_u32 = encode_to_vec_with_config(&u32_val, cfg).expect("limit u32 encode");
    let (dec_u32, _): (u32, usize) =
        decode_from_slice_with_config(&enc_u32, cfg).expect("limit u32 decode");
    assert_eq!(dec_u32, u32_val);

    // String
    let str_val = String::from("limit config roundtrip");
    let enc_str = encode_to_vec_with_config(&str_val, cfg).expect("limit string encode");
    let (dec_str, _): (String, usize) =
        decode_from_slice_with_config(&enc_str, cfg).expect("limit string decode");
    assert_eq!(dec_str, str_val);

    // Vec<u8> (100 elements, well within 1024-byte limit)
    let vec_val: Vec<u8> = (0u8..100).collect();
    let enc_vec = encode_to_vec_with_config(&vec_val, cfg).expect("limit Vec<u8> encode");
    let (dec_vec, _): (Vec<u8>, usize) =
        decode_from_slice_with_config(&enc_vec, cfg).expect("limit Vec<u8> decode");
    assert_eq!(dec_vec, vec_val);

    // bool
    let enc_bool = encode_to_vec_with_config(&true, cfg).expect("limit bool encode");
    let (dec_bool, _): (bool, usize) =
        decode_from_slice_with_config(&enc_bool, cfg).expect("limit bool decode");
    assert!(dec_bool);

    // Use default-config encode_to_vec / decode_from_slice as a sanity check
    let default_enc = encode_to_vec(&42u32).expect("default encode");
    let (default_dec, _): (u32, usize) = decode_from_slice(&default_enc).expect("default decode");
    assert_eq!(default_dec, 42u32);
}
