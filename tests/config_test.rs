//! Tests for the configuration system.
//!
//! Verifies that different configs produce distinct byte representations,
//! that each config roundtrips correctly, and that byte/size limits are enforced.

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
    config,
    config::{Config, Endianness, IntEncoding},
    decode_from_slice_with_config, encode_to_vec_with_config,
};

// ---------------------------------------------------------------------------
// Byte-format differences
// ---------------------------------------------------------------------------

/// Standard uses varint, legacy uses fixed-width: encoded bytes must differ.
#[test]
fn test_standard_vs_legacy_byte_difference() {
    let value: u32 = 1000;
    let standard_bytes =
        encode_to_vec_with_config(&value, config::standard()).expect("standard encode");
    let legacy_bytes = encode_to_vec_with_config(&value, config::legacy()).expect("legacy encode");

    // Legacy always emits 4 bytes for u32; standard uses varint (2 bytes for 1000).
    assert_ne!(standard_bytes, legacy_bytes);
    assert_eq!(legacy_bytes.len(), 4, "legacy u32 must be 4 bytes");
}

/// Small value (< 251) encodes as 1 byte with standard but 4 bytes with legacy.
#[test]
fn test_standard_vs_legacy_small_value() {
    let value: u32 = 42;
    let standard_bytes =
        encode_to_vec_with_config(&value, config::standard()).expect("standard encode");
    let legacy_bytes = encode_to_vec_with_config(&value, config::legacy()).expect("legacy encode");

    assert_eq!(standard_bytes.len(), 1, "varint 42 must be 1 byte");
    assert_eq!(legacy_bytes.len(), 4, "legacy u32 must be 4 bytes");
    assert_ne!(standard_bytes, legacy_bytes);
}

// ---------------------------------------------------------------------------
// Roundtrip correctness
// ---------------------------------------------------------------------------

/// Standard config encodes and decodes u32 correctly.
#[test]
fn test_standard_config_roundtrip_u32() {
    let value: u32 = 99_999;
    let bytes = encode_to_vec_with_config(&value, config::standard()).expect("encode");
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&bytes, config::standard()).expect("decode");
    assert_eq!(decoded, value);
    assert_eq!(consumed, bytes.len());
}

/// Legacy config encodes and decodes u32 correctly.
#[test]
fn test_legacy_config_roundtrip_u32() {
    let value: u32 = 99_999;
    let bytes = encode_to_vec_with_config(&value, config::legacy()).expect("encode");
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&bytes, config::legacy()).expect("decode");
    assert_eq!(decoded, value);
    assert_eq!(consumed, bytes.len());
}

/// Standard config roundtrips a String.
#[test]
fn test_standard_config_roundtrip_string() {
    let value = String::from("hello, oxicode");
    let bytes = encode_to_vec_with_config(&value, config::standard()).expect("encode");
    let (decoded, _): (String, usize) =
        decode_from_slice_with_config(&bytes, config::standard()).expect("decode");
    assert_eq!(decoded, value);
}

/// Legacy config roundtrips a String.
#[test]
fn test_legacy_config_roundtrip_string() {
    let value = String::from("hello, oxicode");
    let bytes = encode_to_vec_with_config(&value, config::legacy()).expect("encode");
    let (decoded, _): (String, usize) =
        decode_from_slice_with_config(&bytes, config::legacy()).expect("decode");
    assert_eq!(decoded, value);
}

/// Standard config roundtrips a Vec<u64>.
#[test]
fn test_standard_config_roundtrip_vec_u64() {
    let value: Vec<u64> = vec![0, 1, 127, 128, 255, 256, 65535, u64::MAX];
    let bytes = encode_to_vec_with_config(&value, config::standard()).expect("encode");
    let (decoded, consumed): (Vec<u64>, usize) =
        decode_from_slice_with_config(&bytes, config::standard()).expect("decode");
    assert_eq!(decoded, value);
    assert_eq!(consumed, bytes.len());
}

/// Legacy config roundtrips a Vec<u64>.
#[test]
fn test_legacy_config_roundtrip_vec_u64() {
    let value: Vec<u64> = vec![0, 1, 127, 128, 255, 256, 65535, u64::MAX];
    let bytes = encode_to_vec_with_config(&value, config::legacy()).expect("encode");
    let (decoded, consumed): (Vec<u64>, usize) =
        decode_from_slice_with_config(&bytes, config::legacy()).expect("decode");
    assert_eq!(decoded, value);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Cross-config incompatibility
// ---------------------------------------------------------------------------

/// Bytes produced by one config cannot generally be decoded by the other.
/// For a value that has different encoding widths the decoded result differs.
#[test]
fn test_cross_config_incompatibility_u32() {
    // 300 requires 2 varint bytes in standard, but exactly 4 bytes in legacy.
    let value: u32 = 300;
    let legacy_bytes = encode_to_vec_with_config(&value, config::legacy()).expect("legacy encode");

    // Attempt to decode legacy bytes using standard config.
    // The result should not equal 300 OR an error should occur — either way no panic.
    let std_result: Result<(u32, usize), _> =
        decode_from_slice_with_config(&legacy_bytes, config::standard());
    if let Ok((decoded, _)) = std_result {
        // The decoded value will differ because varint interprets the 4 bytes differently.
        assert_ne!(
            decoded, value,
            "cross-config decode must not silently produce the correct value"
        );
    }
    // An error is also acceptable — just confirm no panic occurred.
}

/// Bytes produced by standard config are incompatible with legacy for values > 250.
#[test]
fn test_cross_config_incompatibility_standard_to_legacy() {
    let value: u32 = 1000;
    let standard_bytes =
        encode_to_vec_with_config(&value, config::standard()).expect("standard encode");

    // Legacy expects 4 bytes; standard encoding of 1000 is only 2 bytes.
    // Decoding should either fail or produce a wrong value.
    let legacy_result: Result<(u32, usize), _> =
        decode_from_slice_with_config(&standard_bytes, config::legacy());
    if let Ok((decoded, _)) = legacy_result {
        assert_ne!(
            decoded, value,
            "cross-config must not produce correct value"
        );
    }
}

// ---------------------------------------------------------------------------
// Endianness variants
// ---------------------------------------------------------------------------

/// Big-endian config roundtrips correctly.
#[test]
fn test_big_endian_config_roundtrip() {
    let value: u32 = 0xDEAD_BEEF;
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec_with_config(&value, cfg).expect("big-endian encode");
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("big-endian decode");
    assert_eq!(decoded, value);
    assert_eq!(consumed, bytes.len());
}

/// Little-endian config roundtrips correctly.
#[test]
fn test_little_endian_config_roundtrip() {
    let value: u32 = 0xDEAD_BEEF;
    let cfg = config::standard().with_little_endian();
    let bytes = encode_to_vec_with_config(&value, cfg).expect("little-endian encode");
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("little-endian decode");
    assert_eq!(decoded, value);
    assert_eq!(consumed, bytes.len());
}

/// Big-endian and little-endian produce different byte sequences for multi-byte values.
#[test]
fn test_big_vs_little_endian_bytes_differ() {
    let value: u32 = 0x0102_0304; // distinct bytes in each position
    let big_bytes = encode_to_vec_with_config(
        &value,
        config::standard()
            .with_big_endian()
            .with_fixed_int_encoding(),
    )
    .expect("big-endian encode");
    let little_bytes = encode_to_vec_with_config(
        &value,
        config::standard()
            .with_little_endian()
            .with_fixed_int_encoding(),
    )
    .expect("little-endian encode");
    assert_ne!(big_bytes, little_bytes, "endianness must change byte order");
}

// ---------------------------------------------------------------------------
// Fixed-int vs variable-int encoding
// ---------------------------------------------------------------------------

/// Fixed-int encoding always produces 4 bytes for u32.
#[test]
fn test_fixed_int_encoding_u32_length() {
    for value in [0u32, 1, 127, 128, 255, 256, 65535, u32::MAX] {
        let cfg = config::standard().with_fixed_int_encoding();
        let bytes = encode_to_vec_with_config(&value, cfg).expect("fixed encode");
        assert_eq!(
            bytes.len(),
            4,
            "fixed u32 must always be 4 bytes; value={value}"
        );
    }
}

/// Variable-int encoding is compact for small values.
#[test]
fn test_variable_int_encoding_small_values() {
    let cfg = config::standard().with_variable_int_encoding();
    for value in 0u64..=250 {
        let bytes = encode_to_vec_with_config(&value, cfg).expect("varint encode");
        assert_eq!(
            bytes.len(),
            1,
            "varint values 0-250 must be 1 byte; value={value}"
        );
    }
}

// ---------------------------------------------------------------------------
// Byte-limit enforcement
// ---------------------------------------------------------------------------

/// with_limit builds a config that compiles and can be used for encode/decode.
/// The limit type is part of the config type signature; small payloads within
/// the limit succeed.
#[test]
fn test_write_limit_config_small_payload_succeeds() {
    let value: u32 = 42;
    let cfg = config::standard().with_limit::<64>();
    let result = encode_to_vec_with_config(&value, cfg);
    assert!(result.is_ok(), "small payload within limit must succeed");
    // Roundtrip check
    let bytes = result.expect("encode");
    let (decoded, _): (u32, usize) = decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(decoded, value);
}

/// with_no_limit removes a previously set limit type.
#[test]
fn test_no_limit_config_compiles_and_works() {
    let value: u32 = 1234;
    let cfg = config::standard().with_limit::<64>().with_no_limit();
    let bytes = encode_to_vec_with_config(&value, cfg).expect("encode");
    let (decoded, _): (u32, usize) = decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// Config consistency: encode then decode with the same config is identity
// ---------------------------------------------------------------------------

#[test]
fn test_config_consistency_standard() {
    let value: u64 = 123_456_789;
    let cfg = config::standard();
    let bytes = encode_to_vec_with_config(&value, cfg).expect("encode");
    let (decoded, _): (u64, usize) = decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(decoded, value);
}

#[test]
fn test_config_consistency_legacy() {
    let value: u64 = 123_456_789;
    let cfg = config::legacy();
    let bytes = encode_to_vec_with_config(&value, cfg).expect("encode");
    let (decoded, _): (u64, usize) = decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(decoded, value);
}

#[test]
fn test_config_consistency_big_endian_fixed() {
    let value: i32 = -1_000_000;
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&value, cfg).expect("encode");
    let (decoded, _): (i32, usize) = decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// Varint size trade-offs
// ---------------------------------------------------------------------------

/// Varint (standard) produces <= bytes compared to fixed (legacy) for small u64.
#[test]
fn test_varint_encodes_smaller_for_small_values() {
    let small: u64 = 1;
    let standard_bytes =
        encode_to_vec_with_config(&small, config::standard()).expect("standard encode");
    let legacy_bytes = encode_to_vec_with_config(&small, config::legacy()).expect("legacy encode");
    // varint should use <= bytes for small value
    assert!(
        standard_bytes.len() <= legacy_bytes.len(),
        "varint should use <= bytes for small value; varint={}, fixed={}",
        standard_bytes.len(),
        legacy_bytes.len()
    );
}

/// For u64::MAX, varint may use more bytes than fixed-width encoding.
#[test]
fn test_varint_encodes_larger_for_large_values() {
    let large: u64 = u64::MAX;
    let standard_bytes =
        encode_to_vec_with_config(&large, config::standard()).expect("standard encode");
    let legacy_bytes = encode_to_vec_with_config(&large, config::legacy()).expect("legacy encode");
    // varint for u64::MAX requires 9 bytes, fixed-width legacy uses 8
    assert!(
        standard_bytes.len() >= legacy_bytes.len(),
        "varint u64::MAX should use >= bytes than fixed; varint={}, fixed={}",
        standard_bytes.len(),
        legacy_bytes.len()
    );
    // Ensure roundtrip still works
    let (decoded, _): (u64, usize) =
        decode_from_slice_with_config(&standard_bytes, config::standard()).expect("decode");
    assert_eq!(decoded, large);
}

// ---------------------------------------------------------------------------
// Byte-order verification (exact byte values)
// ---------------------------------------------------------------------------

/// Big-endian fixed-int: 0x01020304 must be serialised as [0x01, 0x02, 0x03, 0x04].
#[test]
fn test_big_endian_fixed_int_byte_order() {
    let val: u32 = 0x0102_0304;
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode");
    assert_eq!(
        bytes,
        &[0x01, 0x02, 0x03, 0x04],
        "big-endian fixed u32 must be MSB-first"
    );
}

/// Little-endian fixed-int: 0x01020304 must be serialised as [0x04, 0x03, 0x02, 0x01].
#[test]
fn test_little_endian_fixed_int_byte_order() {
    let val: u32 = 0x0102_0304;
    let cfg = config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode");
    assert_eq!(
        bytes,
        &[0x04, 0x03, 0x02, 0x01],
        "little-endian fixed u32 must be LSB-first"
    );
}

// ---------------------------------------------------------------------------
// Variable-int encoding: large value overhead
// ---------------------------------------------------------------------------

/// Varint encoding of u64::MAX must produce exactly 9 bytes.
#[test]
fn test_variable_int_encoding_large_values() {
    let cfg = config::standard().with_variable_int_encoding();
    let bytes = encode_to_vec_with_config(&u64::MAX, cfg).expect("encode");
    // bincode varint: 0xFF marker (1 byte) + 8 raw bytes = 9 bytes total
    assert_eq!(
        bytes.len(),
        9,
        "varint u64::MAX must occupy 9 bytes; got {}",
        bytes.len()
    );
}

// ---------------------------------------------------------------------------
// Various with_limit configs
// ---------------------------------------------------------------------------

/// with_limit::<128> succeeds for small payloads and roundtrips.
#[test]
fn test_limit_config_128_small_payload() {
    let value: u32 = 99;
    let cfg = config::standard().with_limit::<128>();
    let bytes = encode_to_vec_with_config(&value, cfg).expect("encode");
    let (decoded, _): (u32, usize) = decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(decoded, value);
}

/// with_limit::<1024> succeeds for medium payloads.
#[test]
fn test_limit_config_1024_medium_payload() {
    let value: Vec<u8> = (0u8..100).collect();
    let cfg = config::standard().with_limit::<1024>();
    let bytes = encode_to_vec_with_config(&value, cfg).expect("encode");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(decoded, value);
}

/// with_limit::<65536> succeeds for larger payloads.
#[test]
fn test_limit_config_65536_large_payload() {
    let value: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    let cfg = config::standard().with_limit::<65536>();
    let bytes = encode_to_vec_with_config(&value, cfg).expect("encode");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// Config trait method accessors
// ---------------------------------------------------------------------------

/// cfg.limit() returns None for standard, Some(N) for with_limit::<N>, None after with_no_limit.
#[test]
fn test_config_limit_method_returns_correct_value() {
    let std_cfg = config::standard();
    assert_eq!(std_cfg.limit(), None, "standard has no limit");

    let limited = config::standard().with_limit::<64>();
    assert_eq!(
        limited.limit(),
        Some(64),
        "with_limit::<64> must report Some(64)"
    );

    let unlimited = config::standard().with_limit::<64>().with_no_limit();
    assert_eq!(unlimited.limit(), None, "with_no_limit must restore None");
}

/// cfg.endianness() returns the correct variant.
#[test]
fn test_config_endianness_method() {
    let le = config::standard();
    assert_eq!(
        le.endianness(),
        Endianness::Little,
        "standard is little-endian"
    );

    let be = config::standard().with_big_endian();
    assert_eq!(
        be.endianness(),
        Endianness::Big,
        "with_big_endian must be Big"
    );

    let back_le = config::standard().with_big_endian().with_little_endian();
    assert_eq!(
        back_le.endianness(),
        Endianness::Little,
        "switching back to little-endian must work"
    );
}

/// cfg.int_encoding() returns the correct variant.
#[test]
fn test_config_int_encoding_method() {
    let varint_cfg = config::standard();
    assert_eq!(
        varint_cfg.int_encoding(),
        IntEncoding::Variable,
        "standard uses variable int encoding"
    );

    let fixed_cfg = config::standard().with_fixed_int_encoding();
    assert_eq!(
        fixed_cfg.int_encoding(),
        IntEncoding::Fixed,
        "with_fixed_int_encoding must report Fixed"
    );

    let back_varint = config::standard()
        .with_fixed_int_encoding()
        .with_variable_int_encoding();
    assert_eq!(
        back_varint.int_encoding(),
        IntEncoding::Variable,
        "switching back to variable must work"
    );
}

// ===========================================================================
// NEW TESTS (appended)
// ===========================================================================

// ---------------------------------------------------------------------------
// 1. Fixed int encoding: u32 always encodes to 4 bytes
// ---------------------------------------------------------------------------

/// Fixed int encoding produces exactly 4 bytes for every u32 value.
#[test]
fn test_new_fixed_int_u32_always_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0u32, 1, 127, 128, 255, 256, 65535, u32::MAX] {
        let bytes = encode_to_vec_with_config(&value, cfg).expect("fixed u32 encode");
        assert_eq!(
            bytes.len(),
            4,
            "fixed-int u32 must be 4 bytes regardless of value; value={value}"
        );
        let (decoded, consumed): (u32, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("fixed u32 decode");
        assert_eq!(decoded, value);
        assert_eq!(consumed, 4);
    }
}

// ---------------------------------------------------------------------------
// 2. Fixed int encoding: i64 always encodes to 8 bytes
// ---------------------------------------------------------------------------

/// Fixed int encoding produces exactly 8 bytes for every i64 value.
#[test]
fn test_new_fixed_int_i64_always_8_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0i64, 1, -1, 127, -128, i64::MIN, i64::MAX] {
        let bytes = encode_to_vec_with_config(&value, cfg).expect("fixed i64 encode");
        assert_eq!(
            bytes.len(),
            8,
            "fixed-int i64 must be 8 bytes regardless of value; value={value}"
        );
        let (decoded, _): (i64, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("fixed i64 decode");
        assert_eq!(decoded, value);
    }
}

// ---------------------------------------------------------------------------
// 3. Varint encoding: small u64 (< 251) encodes to 1 byte
// ---------------------------------------------------------------------------

/// All u64 values in [0, 250] should encode to exactly 1 byte with varint.
#[test]
fn test_new_varint_small_u64_is_1_byte() {
    let cfg = config::standard().with_variable_int_encoding();
    for value in 0u64..=250 {
        let bytes = encode_to_vec_with_config(&value, cfg).expect("varint encode");
        assert_eq!(
            bytes.len(),
            1,
            "varint u64 value {value} (< 251) must encode to 1 byte"
        );
        let (decoded, _): (u64, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("varint decode");
        assert_eq!(decoded, value);
    }
}

// ---------------------------------------------------------------------------
// 4. Big endian: u32 = 256 encodes as [0x00, 0x00, 0x01, 0x00]
// ---------------------------------------------------------------------------

/// With big-endian fixed encoding, u32 = 256 must serialize as [0x00, 0x00, 0x01, 0x00].
#[test]
fn test_new_big_endian_u32_256_exact_bytes() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&256u32, cfg).expect("be encode");
    assert_eq!(
        bytes,
        &[0x00, 0x00, 0x01, 0x00],
        "big-endian u32 = 256 must be [0x00, 0x00, 0x01, 0x00]"
    );
    let (decoded, _): (u32, usize) = decode_from_slice_with_config(&bytes, cfg).expect("be decode");
    assert_eq!(decoded, 256u32);
}

// ---------------------------------------------------------------------------
// 5. Little endian (default): u32 = 256 encodes as [0x00, 0x01, 0x00, 0x00]
// ---------------------------------------------------------------------------

/// With little-endian fixed encoding, u32 = 256 must serialize as [0x00, 0x01, 0x00, 0x00].
#[test]
fn test_new_little_endian_u32_256_exact_bytes() {
    let cfg = config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&256u32, cfg).expect("le encode");
    assert_eq!(
        bytes,
        &[0x00, 0x01, 0x00, 0x00],
        "little-endian u32 = 256 must be [0x00, 0x01, 0x00, 0x00]"
    );
    let (decoded, _): (u32, usize) = decode_from_slice_with_config(&bytes, cfg).expect("le decode");
    assert_eq!(decoded, 256u32);
}

// ---------------------------------------------------------------------------
// 6. Legacy config compatibility: roundtrip
// ---------------------------------------------------------------------------

/// Legacy config is compatible with bincode 1.x: little-endian fixed-int.
/// Roundtrip must recover the original for various types.
#[test]
fn test_new_legacy_config_compatibility_roundtrip() {
    let cfg = config::legacy();

    // u32 roundtrip
    let original_u32: u32 = 0xDEAD_BEEF;
    let bytes_u32 = encode_to_vec_with_config(&original_u32, cfg).expect("legacy u32 encode");
    assert_eq!(bytes_u32.len(), 4, "legacy u32 must be 4 bytes");
    let (decoded_u32, _): (u32, usize) =
        decode_from_slice_with_config(&bytes_u32, cfg).expect("legacy u32 decode");
    assert_eq!(decoded_u32, original_u32);

    // String roundtrip
    let original_str = String::from("legacy compat");
    let bytes_str = encode_to_vec_with_config(&original_str, cfg).expect("legacy str encode");
    let (decoded_str, _): (String, usize) =
        decode_from_slice_with_config(&bytes_str, cfg).expect("legacy str decode");
    assert_eq!(decoded_str, original_str);

    // Tuple roundtrip
    let original_tuple: (u32, u64) = (42, 9999);
    let bytes_tuple = encode_to_vec_with_config(&original_tuple, cfg).expect("legacy tuple encode");
    let (decoded_tuple, _): ((u32, u64), usize) =
        decode_from_slice_with_config(&bytes_tuple, cfg).expect("legacy tuple decode");
    assert_eq!(decoded_tuple, original_tuple);
}

// ---------------------------------------------------------------------------
// 7. Fixed int with big endian combination
// ---------------------------------------------------------------------------

/// Combining big-endian and fixed-int produces correct byte-exact output and roundtrips.
#[test]
fn test_new_fixed_int_big_endian_combination() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    // Verify byte order for u32 = 0x01020304
    let bytes = encode_to_vec_with_config(&0x01020304u32, cfg).expect("encode");
    assert_eq!(bytes, &[0x01, 0x02, 0x03, 0x04]);

    // Roundtrip i32 negative value
    let neg: i32 = -12345;
    let enc_neg = encode_to_vec_with_config(&neg, cfg).expect("neg encode");
    assert_eq!(enc_neg.len(), 4);
    let (dec_neg, _): (i32, usize) =
        decode_from_slice_with_config(&enc_neg, cfg).expect("neg decode");
    assert_eq!(dec_neg, neg);

    // Roundtrip u64
    let large: u64 = 0x0102_0304_0506_0708;
    let enc_large = encode_to_vec_with_config(&large, cfg).expect("large encode");
    assert_eq!(enc_large, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    let (dec_large, _): (u64, usize) =
        decode_from_slice_with_config(&enc_large, cfg).expect("large decode");
    assert_eq!(dec_large, large);
}

// ---------------------------------------------------------------------------
// 8. Limit config: encode with limit that's large enough succeeds
// ---------------------------------------------------------------------------

/// Encoding within the byte limit must succeed and round-trip correctly.
#[test]
fn test_new_limit_config_large_enough_succeeds() {
    // u32 with varint encoding is at most 5 bytes; limit of 16 is sufficient.
    let cfg = config::standard().with_limit::<16>();
    let value: u32 = 12345;
    let bytes = encode_to_vec_with_config(&value, cfg).expect("encode within limit");
    assert!(bytes.len() <= 16, "encoded size must not exceed limit");
    let (decoded, _): (u32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode within limit");
    assert_eq!(decoded, value);
    assert_eq!(cfg.limit(), Some(16), "limit() must report the set limit");
}

// ---------------------------------------------------------------------------
// 9. Limit config: decode with limit smaller than data fails with LimitExceeded
// ---------------------------------------------------------------------------

/// Decoding a long Vec<u8> with a limit smaller than the payload must fail.
#[test]
fn test_new_limit_config_decode_exceeds_limit_fails() {
    // Encode a 50-byte Vec without any limit so we get valid bytes.
    let data: Vec<u8> = (0u8..50).collect();
    let unlimited_bytes =
        encode_to_vec_with_config(&data, config::standard()).expect("unlimited encode");

    // Now decode those bytes with a very small limit (4 bytes) — must fail.
    let small_cfg = config::standard().with_limit::<4>();
    let result: Result<(Vec<u8>, usize), _> =
        decode_from_slice_with_config(&unlimited_bytes, small_cfg);
    assert!(
        result.is_err(),
        "decoding a 50-byte Vec with a 4-byte limit must fail"
    );
}

// ---------------------------------------------------------------------------
// 10. Zero limit: fails immediately
// ---------------------------------------------------------------------------

/// A zero-byte limit should reject even the smallest container payload.
#[test]
fn test_new_zero_limit_fails() {
    let cfg = config::standard().with_limit::<0>();
    assert_eq!(cfg.limit(), Some(0), "zero limit must be Some(0)");
    // Vec<u8> decode claims `len` bytes via claim_bytes_read.
    // Even a 1-element Vec claims 1 byte, which exceeds the 0-byte limit.
    let one_element: Vec<u8> = vec![42];
    let encoded =
        encode_to_vec_with_config(&one_element, config::standard()).expect("unlimited encode");
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "decoding a Vec<u8> with a zero-byte limit must fail"
    );
}

// ---------------------------------------------------------------------------
// 11. Varint config matches standard default config for small integers
// ---------------------------------------------------------------------------

/// Explicit with_variable_int_encoding() is byte-for-byte identical to standard() default.
#[test]
fn test_new_varint_matches_standard_default() {
    let std_cfg = config::standard();
    let varint_cfg = config::standard().with_variable_int_encoding();

    for value in [0u64, 1, 42, 100, 250] {
        let std_bytes = encode_to_vec_with_config(&value, std_cfg).expect("std encode");
        let varint_bytes = encode_to_vec_with_config(&value, varint_cfg).expect("varint encode");
        assert_eq!(
            std_bytes, varint_bytes,
            "standard and explicit varint must produce identical bytes for value={value}"
        );
    }
}

// ---------------------------------------------------------------------------
// 12. Config is Copy: can pass by value multiple times
// ---------------------------------------------------------------------------

/// Configuration must be Copy so the same config value can be reused.
#[test]
fn test_new_config_is_copy() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    // Using cfg multiple times — this verifies Copy semantics compile and work.
    let bytes1 = encode_to_vec_with_config(&1u32, cfg).expect("first encode");
    let bytes2 = encode_to_vec_with_config(&1u32, cfg).expect("second encode");
    assert_eq!(bytes1, bytes2, "identical encodes must produce same bytes");

    // Use cfg in both encode and decode without cloning.
    let (decoded, _): (u32, usize) = decode_from_slice_with_config(&bytes1, cfg).expect("decode");
    assert_eq!(decoded, 1u32);
}

// ---------------------------------------------------------------------------
// 13. Config changes affect encoding of Vec length prefix
// ---------------------------------------------------------------------------

/// Standard (varint) vs legacy (fixed-int) must produce different Vec length prefixes.
/// For a Vec with <= 250 elements the varint prefix is 1 byte; legacy uses 8 bytes (u64 fixint).
#[test]
fn test_new_config_affects_vec_length_prefix() {
    let data: Vec<u8> = vec![0xAAu8; 10]; // 10-element Vec

    let std_bytes = encode_to_vec_with_config(&data, config::standard()).expect("std encode");
    let legacy_bytes = encode_to_vec_with_config(&data, config::legacy()).expect("legacy encode");

    // With varint the length "10" is 1 byte; with fixint it's 8 bytes (u64).
    // Payload (10 u8 values) is 10 bytes in both. Total: std=11, legacy=18.
    assert_ne!(
        std_bytes.len(),
        legacy_bytes.len(),
        "standard and legacy encode Vec with different length prefix widths"
    );
    assert!(
        std_bytes.len() < legacy_bytes.len(),
        "varint length prefix must be shorter than fixed-int for small Vec"
    );

    // Both roundtrip correctly.
    let (dec_std, _): (Vec<u8>, usize) =
        decode_from_slice_with_config(&std_bytes, config::standard()).expect("std decode");
    let (dec_legacy, _): (Vec<u8>, usize) =
        decode_from_slice_with_config(&legacy_bytes, config::legacy()).expect("legacy decode");
    assert_eq!(dec_std, data);
    assert_eq!(dec_legacy, data);
}

// ---------------------------------------------------------------------------
// 14. with_no_limit() config allows large data
// ---------------------------------------------------------------------------

/// A config produced by with_no_limit() must handle arbitrarily large payloads without error.
#[test]
fn test_new_no_limit_allows_large_data() {
    let cfg = config::standard().with_limit::<64>().with_no_limit();
    assert_eq!(cfg.limit(), None, "with_no_limit must report None");

    // Encode a Vec large enough to exceed the former 64-byte limit.
    let large_data: Vec<u8> = (0u8..=255).cycle().take(512).collect();
    let bytes = encode_to_vec_with_config(&large_data, cfg).expect("large data encode");
    assert!(
        bytes.len() > 64,
        "encoded data must exceed the former limit"
    );

    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("large data decode");
    assert_eq!(decoded, large_data);
}

// ---------------------------------------------------------------------------
// 15. Config roundtrip: encode and decode with same config gives back original
// ---------------------------------------------------------------------------

/// Encoding then decoding with the same config must be identity for all config variants.
#[test]
fn test_new_config_roundtrip_identity() {
    // Standard
    let v_std: (u32, String, Vec<u8>) = (42, String::from("roundtrip"), vec![1, 2, 3]);
    let b_std = encode_to_vec_with_config(&v_std, config::standard()).expect("std encode");
    let (d_std, _): ((u32, String, Vec<u8>), usize) =
        decode_from_slice_with_config(&b_std, config::standard()).expect("std decode");
    assert_eq!(d_std, v_std);

    // Legacy
    let v_legacy: (u32, String, Vec<u8>) = (99, String::from("legacy"), vec![7, 8, 9]);
    let b_legacy = encode_to_vec_with_config(&v_legacy, config::legacy()).expect("legacy encode");
    let (d_legacy, _): ((u32, String, Vec<u8>), usize) =
        decode_from_slice_with_config(&b_legacy, config::legacy()).expect("legacy decode");
    assert_eq!(d_legacy, v_legacy);

    // Big-endian fixed-int
    let cfg_be = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let v_be: (i32, u64) = (-100, 999_999);
    let b_be = encode_to_vec_with_config(&v_be, cfg_be).expect("be encode");
    let (d_be, consumed_be): ((i32, u64), usize) =
        decode_from_slice_with_config(&b_be, cfg_be).expect("be decode");
    assert_eq!(d_be, v_be);
    assert_eq!(consumed_be, b_be.len());
}
