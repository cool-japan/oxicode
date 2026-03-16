//! Config limit enforcement tests — `with_limit::<N>()` applies to DECODE side only.
//!
//! The limit is a byte limit enforced via `claim_bytes_read` / `claim_container_read`
//! during decoding of strings and collections.  Encoding is never limited.

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
    config, config::Config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config,
};

// ---------------------------------------------------------------------------
// 1. Standard config encodes and decodes String successfully
// ---------------------------------------------------------------------------

#[test]
fn test_standard_config_string_roundtrip() {
    let original = String::from("hello oxicode config limits");
    let enc = encode_to_vec_with_config(&original, config::standard())
        .expect("standard config: String encode must succeed");
    let (val, _): (String, usize) = decode_from_slice_with_config(&enc, config::standard())
        .expect("standard config: String decode must succeed");
    assert_eq!(val, original, "decoded String must match original");
}

// ---------------------------------------------------------------------------
// 2. Fixed-int encoding: u32 always 4 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_u32_always_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0u32, 1, 127, 128, 255, 256, 65535, u32::MAX] {
        let enc =
            encode_to_vec_with_config(&value, cfg).expect("fixed-int u32: encode must succeed");
        assert_eq!(
            enc.len(),
            4,
            "fixed-int u32 must always be 4 bytes; value={value}"
        );
        let (decoded, consumed): (u32, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("fixed-int u32: decode must succeed");
        assert_eq!(decoded, value, "decoded u32 must match; value={value}");
        assert_eq!(consumed, 4, "consumed must be 4 bytes; value={value}");
    }
}

// ---------------------------------------------------------------------------
// 3. Fixed-int encoding: u64 always 8 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_u64_always_8_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0u64, 1, 127, 128, 255, 256, 65535, u64::MAX] {
        let enc =
            encode_to_vec_with_config(&value, cfg).expect("fixed-int u64: encode must succeed");
        assert_eq!(
            enc.len(),
            8,
            "fixed-int u64 must always be 8 bytes; value={value}"
        );
        let (decoded, consumed): (u64, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("fixed-int u64: decode must succeed");
        assert_eq!(decoded, value, "decoded u64 must match; value={value}");
        assert_eq!(consumed, 8, "consumed must be 8 bytes; value={value}");
    }
}

// ---------------------------------------------------------------------------
// 4. Big-endian + fixed-int: u32(1) = [0, 0, 0, 1]
// ---------------------------------------------------------------------------

#[test]
fn test_big_endian_fixed_int_u32_one_exact_bytes() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&1u32, cfg)
        .expect("big-endian fixed u32(1): encode must succeed");
    assert_eq!(
        enc.as_slice(),
        &[0x00u8, 0x00, 0x00, 0x01],
        "big-endian fixed u32(1) must serialize as [0, 0, 0, 1]"
    );
    let (decoded, _): (u32, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("big-endian fixed u32(1): decode must succeed");
    assert_eq!(decoded, 1u32);
}

// ---------------------------------------------------------------------------
// 5. with_limit::<100>() decodes String of 50 chars successfully
// ---------------------------------------------------------------------------

#[test]
fn test_limit_100_decodes_50_char_string_ok() {
    // 50 ASCII chars → 50-byte body; limit=100 bytes → enough room
    let s: String = "a".repeat(50);
    let enc = encode_to_vec(&s).expect("encode 50-char String");
    let cfg = config::standard().with_limit::<100>();
    let (val, _): (String, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("limit=100 must decode 50-char String successfully");
    assert_eq!(val, s, "decoded String must match original");
}

// ---------------------------------------------------------------------------
// 6. with_limit::<10>() decode of String with 20 chars returns Err
// ---------------------------------------------------------------------------

#[test]
fn test_limit_10_rejects_20_char_string() {
    // 20 ASCII chars → 20-byte body; limit=10 bytes → exceeded
    let s: String = "b".repeat(20);
    let enc = encode_to_vec(&s).expect("encode 20-char String");
    let cfg = config::standard().with_limit::<10>();
    let result: Result<(String, usize), _> = decode_from_slice_with_config(&enc, cfg);
    assert!(
        result.is_err(),
        "limit=10 must reject a String whose body is 20 bytes"
    );
}

// ---------------------------------------------------------------------------
// 7. with_limit::<100>() decodes Vec<u8> of 50 elements successfully
// ---------------------------------------------------------------------------

#[test]
fn test_limit_100_decodes_50_element_vec_u8_ok() {
    // Vec<u8> with 50 elements → claims 50 bytes; limit=100 → enough room
    let data: Vec<u8> = (0u8..50).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u8> of 50 elements");
    let cfg = config::standard().with_limit::<100>();
    let (val, _): (Vec<u8>, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("limit=100 must decode Vec<u8> of 50 elements successfully");
    assert_eq!(val, data, "decoded Vec<u8> must match original");
}

// ---------------------------------------------------------------------------
// 8. with_limit::<10>() decode of Vec<u8> with 20 elements returns Err
// ---------------------------------------------------------------------------

#[test]
fn test_limit_10_rejects_20_element_vec_u8() {
    // Vec<u8> with 20 elements → claims 20 bytes; limit=10 → exceeded
    let data: Vec<u8> = (0u8..20).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u8> of 20 elements");
    let cfg = config::standard().with_limit::<10>();
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice_with_config(&enc, cfg);
    assert!(
        result.is_err(),
        "limit=10 must reject Vec<u8> whose payload is 20 bytes"
    );
}

// ---------------------------------------------------------------------------
// 9. with_limit::<1000>() decodes Vec<u32> of 100 elements successfully
// ---------------------------------------------------------------------------

#[test]
fn test_limit_1000_decodes_100_element_vec_u32_ok() {
    // Vec<u32> with 100 elements; each u32 is varint-encoded (1 byte for values < 251),
    // so payload ≈ 100 bytes; limit=1000 → plenty of room
    let data: Vec<u32> = (0u32..100).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u32> of 100 elements");
    let cfg = config::standard().with_limit::<1000>();
    let (val, _): (Vec<u32>, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("limit=1000 must decode Vec<u32> of 100 elements successfully");
    assert_eq!(val, data, "decoded Vec<u32> must match original");
}

// ---------------------------------------------------------------------------
// 10. Encode with limit config works same as encode without limit (limit is decode-only)
// ---------------------------------------------------------------------------

#[test]
fn test_encode_with_limit_config_same_bytes_as_no_limit() {
    let s: String = "encode limit is decode-only".to_string();
    let enc_no_limit =
        encode_to_vec_with_config(&s, config::standard()).expect("encode without limit");
    // tiny limit of 1 byte — encoding should still succeed because limit only affects decode
    let cfg_limited = config::standard().with_limit::<1>();
    let enc_with_limit = encode_to_vec_with_config(&s, cfg_limited)
        .expect("encode with limit=1 must succeed; limit only affects decode");
    assert_eq!(
        enc_no_limit, enc_with_limit,
        "encoding with or without a limit must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// 11. with_limit::<0>() decode of any non-empty String returns Err
// ---------------------------------------------------------------------------

#[test]
fn test_limit_0_rejects_any_nonempty_string() {
    let s = String::from("x"); // 1-byte body
    let enc = encode_to_vec(&s).expect("encode 1-char String");
    let cfg = config::standard().with_limit::<0>();
    let result: Result<(String, usize), _> = decode_from_slice_with_config(&enc, cfg);
    assert!(
        result.is_err(),
        "limit=0 must reject even a 1-char String (1-byte body exceeds 0-byte limit)"
    );
}

// ---------------------------------------------------------------------------
// 12. Default config has no limit — encodes/decodes 1000-char string
// ---------------------------------------------------------------------------

#[test]
fn test_default_config_no_limit_1000_char_string() {
    let s: String = "z".repeat(1000);
    let enc = encode_to_vec_with_config(&s, config::standard())
        .expect("standard config must encode 1000-char String");
    assert_eq!(
        config::standard().limit(),
        None,
        "standard config must have no limit"
    );
    let (val, _): (String, usize) = decode_from_slice_with_config(&enc, config::standard())
        .expect("standard config must decode 1000-char String");
    assert_eq!(val, s, "decoded 1000-char String must match original");
}

// ---------------------------------------------------------------------------
// 13. config::standard().with_fixed_int_encoding().with_big_endian() chain
// ---------------------------------------------------------------------------

#[test]
fn test_config_chain_fixed_int_then_big_endian() {
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    // u32 = 0x0A0B0C0D: big-endian → [0x0A, 0x0B, 0x0C, 0x0D]
    let value: u32 = 0x0A0B_0C0D;
    let enc = encode_to_vec_with_config(&value, cfg).expect("chained config: encode must succeed");
    assert_eq!(
        enc.as_slice(),
        &[0x0Au8, 0x0B, 0x0C, 0x0D],
        "chained fixed+BE must produce MSB-first bytes"
    );
    let (decoded, _): (u32, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("chained config: decode must succeed");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// 14. Two configs with same settings produce same encoded bytes
// ---------------------------------------------------------------------------

#[test]
fn test_two_equivalent_configs_produce_same_bytes() {
    let cfg_a = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let cfg_b = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value: u64 = 0x0102_0304_0506_0708u64;
    let enc_a = encode_to_vec_with_config(&value, cfg_a).expect("cfg_a: encode must succeed");
    let enc_b = encode_to_vec_with_config(&value, cfg_b).expect("cfg_b: encode must succeed");
    assert_eq!(
        enc_a, enc_b,
        "two configs with identical settings must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// 15. with_limit::<{ 1024 * 1024 }>() decode of Vec<String> with 3 items succeeds
// ---------------------------------------------------------------------------

#[test]
fn test_large_limit_decodes_vec_of_3_short_strings_ok() {
    // Three 1-byte strings; large limit → decode must succeed
    let items: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let enc = encode_to_vec(&items).expect("encode Vec<String> of 3 items");
    let cfg = config::standard().with_limit::<{ 1024 * 1024 }>();
    let (val, _): (Vec<String>, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("large limit must decode Vec<String> of 3 short items successfully");
    assert_eq!(val, items, "decoded Vec<String> must match original");
}

// ---------------------------------------------------------------------------
// 16. with_limit::<2>() decode of Vec<String> with 3 items fails
// ---------------------------------------------------------------------------

#[test]
fn test_limit_2_rejects_vec_of_3_strings() {
    // Vec<String> with 3 items; even the shortest 1-char strings require 3+ bytes of body.
    // A limit of 2 bytes cannot accommodate all three.
    let items: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let enc = encode_to_vec(&items).expect("encode Vec<String> of 3 items");
    let cfg = config::standard().with_limit::<2>();
    let result: Result<(Vec<String>, usize), _> = decode_from_slice_with_config(&enc, cfg);
    assert!(
        result.is_err(),
        "limit=2 must reject Vec<String> with 3 items (body > 2 bytes)"
    );
}

// ---------------------------------------------------------------------------
// 17. Standard config: i32(-1) encodes with zigzag (varint, 1 byte)
// ---------------------------------------------------------------------------

#[test]
fn test_standard_config_i32_minus_one_zigzag_1_byte() {
    // With zigzag varint, -1 maps to 1 (zigzag(−1) = 1), which encodes as a single byte.
    let value: i32 = -1;
    let enc = encode_to_vec_with_config(&value, config::standard())
        .expect("encode i32(-1) with standard config");
    assert_eq!(
        enc.len(),
        1,
        "standard config encodes i32(-1) as 1 byte via zigzag varint; got {} bytes",
        enc.len()
    );
    let (decoded, _): (i32, usize) = decode_from_slice_with_config(&enc, config::standard())
        .expect("decode i32(-1) with standard config");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// 18. Standard config: Option<String> Some/None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_standard_config_option_string_some_none_roundtrip() {
    // Some("hello")
    let some_val: Option<String> = Some(String::from("hello"));
    let enc_some = encode_to_vec_with_config(&some_val, config::standard())
        .expect("encode Option<String> Some");
    let (decoded_some, _): (Option<String>, usize) =
        decode_from_slice_with_config(&enc_some, config::standard())
            .expect("decode Option<String> Some");
    assert_eq!(decoded_some, some_val, "Some roundtrip must match");

    // None
    let none_val: Option<String> = None;
    let enc_none = encode_to_vec_with_config(&none_val, config::standard())
        .expect("encode Option<String> None");
    let (decoded_none, _): (Option<String>, usize) =
        decode_from_slice_with_config(&enc_none, config::standard())
            .expect("decode Option<String> None");
    assert_eq!(decoded_none, none_val, "None roundtrip must match");
}

// ---------------------------------------------------------------------------
// 19. Fixed-int: f32 still 4 bytes (floats unaffected by int encoding)
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_f32_is_still_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0.0f32, 1.0, -1.0, f32::MAX, f32::MIN, f32::INFINITY] {
        let enc = encode_to_vec_with_config(&value, cfg)
            .expect("fixed-int config: f32 encode must succeed");
        assert_eq!(
            enc.len(),
            4,
            "f32 must always be 4 bytes regardless of int encoding; value={value}"
        );
        let (decoded, _): (f32, usize) = decode_from_slice_with_config(&enc, cfg)
            .expect("fixed-int config: f32 decode must succeed");
        // Use bit comparison to handle special floats (infinity, etc.)
        assert_eq!(
            decoded.to_bits(),
            value.to_bits(),
            "decoded f32 bits must match; value={value}"
        );
    }
}

// ---------------------------------------------------------------------------
// 20. Fixed-int: f64 still 8 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_f64_is_still_8_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0.0f64, 1.0, -1.0, f64::MAX, f64::MIN, f64::INFINITY] {
        let enc = encode_to_vec_with_config(&value, cfg)
            .expect("fixed-int config: f64 encode must succeed");
        assert_eq!(
            enc.len(),
            8,
            "f64 must always be 8 bytes regardless of int encoding; value={value}"
        );
        let (decoded, _): (f64, usize) = decode_from_slice_with_config(&enc, cfg)
            .expect("fixed-int config: f64 decode must succeed");
        assert_eq!(
            decoded.to_bits(),
            value.to_bits(),
            "decoded f64 bits must match; value={value}"
        );
    }
}

// ---------------------------------------------------------------------------
// 21. bool encoding: 1 byte regardless of config
// ---------------------------------------------------------------------------

#[test]
fn test_bool_encoding_always_1_byte() {
    for value in [true, false] {
        // Standard config
        let enc_std = encode_to_vec_with_config(&value, config::standard())
            .expect("standard config: bool encode must succeed");
        assert_eq!(
            enc_std.len(),
            1,
            "bool ({value}) must be 1 byte with standard config"
        );
        let (dec_std, _): (bool, usize) =
            decode_from_slice_with_config(&enc_std, config::standard())
                .expect("standard config: bool decode must succeed");
        assert_eq!(dec_std, value, "bool ({value}) standard roundtrip");

        // Fixed-int + big-endian config
        let cfg_be = config::standard()
            .with_fixed_int_encoding()
            .with_big_endian();
        let enc_be = encode_to_vec_with_config(&value, cfg_be)
            .expect("BE fixed config: bool encode must succeed");
        assert_eq!(
            enc_be.len(),
            1,
            "bool ({value}) must be 1 byte with BE fixed config"
        );
        let (dec_be, _): (bool, usize) = decode_from_slice_with_config(&enc_be, cfg_be)
            .expect("BE fixed config: bool decode must succeed");
        assert_eq!(dec_be, value, "bool ({value}) BE fixed roundtrip");

        // Default encode_to_vec / decode_from_slice
        let enc_default = encode_to_vec(&value).expect("default: bool encode must succeed");
        assert_eq!(
            enc_default.len(),
            1,
            "bool ({value}) must be 1 byte with default encode"
        );
        let (dec_default, _): (bool, usize) =
            decode_from_slice(&enc_default).expect("default: bool decode must succeed");
        assert_eq!(dec_default, value, "bool ({value}) default roundtrip");
    }
}

// ---------------------------------------------------------------------------
// 22. Config chaining: .with_fixed_int_encoding() then encode/decode u64::MAX
// ---------------------------------------------------------------------------

#[test]
fn test_config_chain_fixed_int_encode_decode_u64_max() {
    // With fixed-int encoding u64::MAX always occupies exactly 8 bytes.
    // Chain multiple builder calls to confirm fluent API works end-to-end.
    let cfg = config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();
    let value: u64 = u64::MAX;
    let enc = encode_to_vec_with_config(&value, cfg)
        .expect("fixed-int config: u64::MAX encode must succeed");
    assert_eq!(
        enc.len(),
        8,
        "fixed-int u64::MAX must be exactly 8 bytes; got {}",
        enc.len()
    );
    // With little-endian, u64::MAX = all 0xFF bytes
    assert_eq!(
        enc.as_slice(),
        &[0xFFu8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
        "fixed-int LE u64::MAX must be 8 x 0xFF bytes"
    );
    let (decoded, consumed): (u64, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("fixed-int config: u64::MAX decode must succeed");
    assert_eq!(decoded, value, "decoded u64 must be u64::MAX");
    assert_eq!(consumed, 8, "consumed must be exactly 8 bytes");
}
