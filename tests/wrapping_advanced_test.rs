//! Advanced tests for Wrapping<T> and Reverse<T> encoding in OxiCode.

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
use oxicode::{config, decode_from_slice, encode_to_vec, encode_to_vec_with_config};
use oxicode_derive::{Decode, Encode};
use std::cmp::Reverse;
use std::num::Wrapping;

// ===== 1. Wrapping<u8> boundary roundtrips =====

#[test]
fn test_wrapping_advanced_wrapping_u8_zero_roundtrip() {
    let original = Wrapping(0u8);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u8>(0) failed");
    let (decoded, _): (Wrapping<u8>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<u8>(0) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 0u8);
}

#[test]
fn test_wrapping_advanced_wrapping_u8_max_roundtrip() {
    let original = Wrapping(255u8);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u8>(255) failed");
    let (decoded, _): (Wrapping<u8>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<u8>(255) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 255u8);
}

// ===== 2. Wrapping<u16> boundary roundtrips =====

#[test]
fn test_wrapping_advanced_wrapping_u16_zero_roundtrip() {
    let original = Wrapping(0u16);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u16>(0) failed");
    let (decoded, _): (Wrapping<u16>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<u16>(0) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 0u16);
}

#[test]
fn test_wrapping_advanced_wrapping_u16_max_roundtrip() {
    let original = Wrapping(65535u16);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u16>(65535) failed");
    let (decoded, _): (Wrapping<u16>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<u16>(65535) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 65535u16);
}

// ===== 3. Wrapping<u32> boundary roundtrips =====

#[test]
fn test_wrapping_advanced_wrapping_u32_zero_roundtrip() {
    let original = Wrapping(0u32);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u32>(0) failed");
    let (decoded, _): (Wrapping<u32>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<u32>(0) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 0u32);
}

#[test]
fn test_wrapping_advanced_wrapping_u32_max_roundtrip() {
    let original = Wrapping(u32::MAX);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u32>(MAX) failed");
    let (decoded, _): (Wrapping<u32>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<u32>(MAX) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, u32::MAX);
}

// ===== 4. Wrapping<u64> boundary roundtrips =====

#[test]
fn test_wrapping_advanced_wrapping_u64_zero_roundtrip() {
    let original = Wrapping(0u64);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u64>(0) failed");
    let (decoded, _): (Wrapping<u64>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<u64>(0) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 0u64);
}

#[test]
fn test_wrapping_advanced_wrapping_u64_max_roundtrip() {
    let original = Wrapping(u64::MAX);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u64>(MAX) failed");
    let (decoded, _): (Wrapping<u64>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<u64>(MAX) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, u64::MAX);
}

// ===== 5. Wrapping<i8> boundary roundtrips =====

#[test]
fn test_wrapping_advanced_wrapping_i8_min_roundtrip() {
    let original = Wrapping(-128i8);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<i8>(-128) failed");
    let (decoded, _): (Wrapping<i8>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<i8>(-128) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, -128i8);
}

#[test]
fn test_wrapping_advanced_wrapping_i8_max_roundtrip() {
    let original = Wrapping(127i8);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<i8>(127) failed");
    let (decoded, _): (Wrapping<i8>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<i8>(127) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 127i8);
}

// ===== 6. Wrapping<i16> boundary roundtrips =====

#[test]
fn test_wrapping_advanced_wrapping_i16_min_roundtrip() {
    let original = Wrapping(-32768i16);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<i16>(-32768) failed");
    let (decoded, _): (Wrapping<i16>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<i16>(-32768) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, -32768i16);
}

#[test]
fn test_wrapping_advanced_wrapping_i16_max_roundtrip() {
    let original = Wrapping(32767i16);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<i16>(32767) failed");
    let (decoded, _): (Wrapping<i16>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<i16>(32767) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 32767i16);
}

// ===== 7. Wrapping<i32> boundary roundtrips =====

#[test]
fn test_wrapping_advanced_wrapping_i32_min_roundtrip() {
    let original = Wrapping(i32::MIN);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<i32>(MIN) failed");
    let (decoded, _): (Wrapping<i32>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<i32>(MIN) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, i32::MIN);
}

#[test]
fn test_wrapping_advanced_wrapping_i32_max_roundtrip() {
    let original = Wrapping(i32::MAX);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<i32>(MAX) failed");
    let (decoded, _): (Wrapping<i32>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<i32>(MAX) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, i32::MAX);
}

// ===== 8. Wrapping<i64> boundary roundtrips =====

#[test]
fn test_wrapping_advanced_wrapping_i64_min_roundtrip() {
    let original = Wrapping(i64::MIN);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<i64>(MIN) failed");
    let (decoded, _): (Wrapping<i64>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<i64>(MIN) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, i64::MIN);
}

#[test]
fn test_wrapping_advanced_wrapping_i64_max_roundtrip() {
    let original = Wrapping(i64::MAX);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<i64>(MAX) failed");
    let (decoded, _): (Wrapping<i64>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<i64>(MAX) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, i64::MAX);
}

// ===== 9. Wrapping<u8> addition overflow (wraps at 256) =====

#[test]
fn test_wrapping_advanced_wrapping_u8_addition_overflow() {
    // 200u8 + 100u8 = 300, wraps to 300 - 256 = 44
    let a = Wrapping(200u8);
    let b = Wrapping(100u8);
    let sum = a + b;
    assert_eq!(sum.0, 44u8, "wrapping addition must wrap at 256");

    let encoded = encode_to_vec(&sum).expect("encode wrapping sum failed");
    let (decoded, _): (Wrapping<u8>, _) =
        decode_from_slice(&encoded).expect("decode wrapping sum failed");

    assert_eq!(decoded.0, 44u8);
    // Confirm continued arithmetic from decoded value preserves wrap semantics
    let again = decoded + Wrapping(212u8); // 44 + 212 = 256 => wraps to 0
    assert_eq!(again.0, 0u8);
}

// ===== 10. Wrapping<i32> subtraction underflow behavior =====

#[test]
fn test_wrapping_advanced_wrapping_i32_subtraction_underflow() {
    // i32::MIN - 1 wraps to i32::MAX
    let a = Wrapping(i32::MIN);
    let b = Wrapping(1i32);
    let diff = a - b;
    assert_eq!(
        diff.0,
        i32::MAX,
        "wrapping subtraction must underflow to i32::MAX"
    );

    let encoded = encode_to_vec(&diff).expect("encode wrapping diff failed");
    let (decoded, _): (Wrapping<i32>, _) =
        decode_from_slice(&encoded).expect("decode wrapping diff failed");

    assert_eq!(decoded.0, i32::MAX);
    // Further wrap: i32::MAX + 1 = i32::MIN
    let further = decoded + Wrapping(1i32);
    assert_eq!(further.0, i32::MIN);
}

// ===== 11. Vec<Wrapping<u32>> roundtrip =====

#[test]
fn test_wrapping_advanced_vec_wrapping_u32_roundtrip() {
    let original: Vec<Wrapping<u32>> = vec![
        Wrapping(0u32),
        Wrapping(1u32),
        Wrapping(42u32),
        Wrapping(u32::MAX / 2),
        Wrapping(u32::MAX),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Wrapping<u32>> failed");
    let (decoded, _): (Vec<Wrapping<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Wrapping<u32>> failed");
    assert_eq!(original, decoded);
}

// ===== 12. Option<Wrapping<u64>> Some and None roundtrip =====

#[test]
fn test_wrapping_advanced_option_wrapping_u64_some_roundtrip() {
    let original: Option<Wrapping<u64>> = Some(Wrapping(u64::MAX));
    let encoded = encode_to_vec(&original).expect("encode Option<Wrapping<u64>>(Some) failed");
    let (decoded, _): (Option<Wrapping<u64>>, _) =
        decode_from_slice(&encoded).expect("decode Option<Wrapping<u64>>(Some) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded, Some(Wrapping(u64::MAX)));
}

#[test]
fn test_wrapping_advanced_option_wrapping_u64_none_roundtrip() {
    let original: Option<Wrapping<u64>> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Wrapping<u64>>(None) failed");
    let (decoded, _): (Option<Wrapping<u64>>, _) =
        decode_from_slice(&encoded).expect("decode Option<Wrapping<u64>>(None) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded, None);
}

// ===== 13. Reverse<u32> boundary roundtrips =====

#[test]
fn test_wrapping_advanced_reverse_u32_zero_roundtrip() {
    let original = Reverse(0u32);
    let encoded = encode_to_vec(&original).expect("encode Reverse<u32>(0) failed");
    let (decoded, _): (Reverse<u32>, _) =
        decode_from_slice(&encoded).expect("decode Reverse<u32>(0) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 0u32);
}

#[test]
fn test_wrapping_advanced_reverse_u32_max_roundtrip() {
    let original = Reverse(u32::MAX);
    let encoded = encode_to_vec(&original).expect("encode Reverse<u32>(MAX) failed");
    let (decoded, _): (Reverse<u32>, _) =
        decode_from_slice(&encoded).expect("decode Reverse<u32>(MAX) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, u32::MAX);
}

// ===== 14. Reverse<i64> boundary roundtrips =====

#[test]
fn test_wrapping_advanced_reverse_i64_min_roundtrip() {
    let original = Reverse(i64::MIN);
    let encoded = encode_to_vec(&original).expect("encode Reverse<i64>(MIN) failed");
    let (decoded, _): (Reverse<i64>, _) =
        decode_from_slice(&encoded).expect("decode Reverse<i64>(MIN) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, i64::MIN);
}

#[test]
fn test_wrapping_advanced_reverse_i64_max_roundtrip() {
    let original = Reverse(i64::MAX);
    let encoded = encode_to_vec(&original).expect("encode Reverse<i64>(MAX) failed");
    let (decoded, _): (Reverse<i64>, _) =
        decode_from_slice(&encoded).expect("decode Reverse<i64>(MAX) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, i64::MAX);
}

// ===== 15. Reverse<String> roundtrip =====

#[test]
fn test_wrapping_advanced_reverse_string_roundtrip() {
    let original = Reverse(String::from("oxicode reverse encoding test"));
    let encoded = encode_to_vec(&original).expect("encode Reverse<String> failed");
    let (decoded, _): (Reverse<String>, _) =
        decode_from_slice(&encoded).expect("decode Reverse<String> failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, "oxicode reverse encoding test");
}

// ===== 16. Vec<Reverse<u8>> roundtrip =====

#[test]
fn test_wrapping_advanced_vec_reverse_u8_roundtrip() {
    let original: Vec<Reverse<u8>> = vec![
        Reverse(0u8),
        Reverse(1u8),
        Reverse(127u8),
        Reverse(128u8),
        Reverse(255u8),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Reverse<u8>> failed");
    let (decoded, _): (Vec<Reverse<u8>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Reverse<u8>> failed");
    assert_eq!(original, decoded);
}

// ===== 17. Wrapping<u32> with fixed_int_encoding config (4 bytes) =====

#[test]
fn test_wrapping_advanced_wrapping_u32_fixed_int_encoding() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = Wrapping(0xDEAD_BEEFu32);
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Wrapping<u32> fixed_int failed");
    // u32 with fixed int encoding is always 4 bytes
    assert_eq!(
        encoded.len(),
        4,
        "Wrapping<u32> with fixed_int must be 4 bytes"
    );
    let (decoded, consumed): (Wrapping<u32>, _) =
        oxicode::decode_from_slice_with_config(&encoded, cfg)
            .expect("decode Wrapping<u32> fixed_int failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, 4);
}

// ===== 18. Reverse<u32> with fixed_int_encoding config =====

#[test]
fn test_wrapping_advanced_reverse_u32_fixed_int_encoding() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = Reverse(0xCAFE_BABEu32);
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Reverse<u32> fixed_int failed");
    // u32 with fixed int encoding is always 4 bytes
    assert_eq!(
        encoded.len(),
        4,
        "Reverse<u32> with fixed_int must be 4 bytes"
    );
    let (decoded, consumed): (Reverse<u32>, _) =
        oxicode::decode_from_slice_with_config(&encoded, cfg)
            .expect("decode Reverse<u32> fixed_int failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, 4);
}

// ===== 19. Byte count verification for Wrapping<u8>(0) and Reverse<u8> =====

#[test]
fn test_wrapping_advanced_byte_count_wrapping_u8_zero_is_one_byte() {
    // Wrapping<u8>(0) is just a u8, so standard varint encoding of 0 is 1 byte.
    let original = Wrapping(0u8);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u8>(0) failed");
    assert_eq!(
        encoded.len(),
        1,
        "Wrapping<u8>(0) must encode to exactly 1 byte"
    );
    let (decoded, consumed): (Wrapping<u8>, _) =
        decode_from_slice(&encoded).expect("decode Wrapping<u8>(0) byte count failed");
    assert_eq!(decoded.0, 0u8);
    assert_eq!(consumed, 1);
}

#[test]
fn test_wrapping_advanced_byte_count_reverse_u8_is_one_byte() {
    // Reverse<u8>(255) encodes its inner u8, which is 1 byte.
    let original = Reverse(255u8);
    let encoded = encode_to_vec(&original).expect("encode Reverse<u8>(255) failed");
    assert_eq!(
        encoded.len(),
        1,
        "Reverse<u8>(255) must encode to exactly 1 byte"
    );
    let (decoded, consumed): (Reverse<u8>, _) =
        decode_from_slice(&encoded).expect("decode Reverse<u8>(255) byte count failed");
    assert_eq!(decoded.0, 255u8);
    assert_eq!(consumed, 1);
}

// ===== 20. Separate Wrapping and Reverse used in tandem =====
//
// Nested Wrapping<Reverse<u32>> is NOT valid because Reverse<T> does not implement
// the numeric traits required by Wrapping<T> (Add, Sub, etc.).
// Instead we demonstrate that Wrapping<u32> and Reverse<u32> can independently
// encode the same inner value and that their encoded bytes are identical
// (both are transparent wrappers delegating to the inner type).

#[test]
fn test_wrapping_advanced_wrapping_and_reverse_transparent_encoding() {
    let inner_value = 0xABCD_1234u32;
    let w = Wrapping(inner_value);
    let r = Reverse(inner_value);

    let w_bytes = encode_to_vec(&w).expect("encode Wrapping<u32> transparent failed");
    let r_bytes = encode_to_vec(&r).expect("encode Reverse<u32> transparent failed");

    // Both wrappers are fully transparent: encoded bytes must be identical to the
    // raw u32 encoding, and to each other.
    let raw_bytes = encode_to_vec(&inner_value).expect("encode raw u32 failed");
    assert_eq!(
        w_bytes, raw_bytes,
        "Wrapping<u32> must be byte-for-byte identical to u32"
    );
    assert_eq!(
        r_bytes, raw_bytes,
        "Reverse<u32> must be byte-for-byte identical to u32"
    );
    assert_eq!(
        w_bytes, r_bytes,
        "Wrapping and Reverse encoding must match for same inner value"
    );

    // Verify decode of each wrapper independently
    let (w_dec, _): (Wrapping<u32>, _) =
        decode_from_slice(&w_bytes).expect("decode Wrapping<u32> transparent failed");
    let (r_dec, _): (Reverse<u32>, _) =
        decode_from_slice(&r_bytes).expect("decode Reverse<u32> transparent failed");

    assert_eq!(w_dec.0, inner_value);
    assert_eq!(r_dec.0, inner_value);
}

// ===== 21. Struct with Wrapping field roundtrip (derive) =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct SensorReading {
    id: u32,
    counter: Wrapping<u32>,
    temperature_raw: Wrapping<i16>,
}

#[test]
fn test_wrapping_advanced_struct_with_wrapping_field_roundtrip() {
    let original = SensorReading {
        id: 42u32,
        // Simulate an overflowed hardware counter: u32::MAX wraps to 0 after next tick
        counter: Wrapping(u32::MAX),
        // Simulate a raw signed sensor value at its negative extreme
        temperature_raw: Wrapping(i16::MIN),
    };
    let encoded = encode_to_vec(&original).expect("encode SensorReading failed");
    let (decoded, _): (SensorReading, _) =
        decode_from_slice(&encoded).expect("decode SensorReading failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.counter.0, u32::MAX);
    assert_eq!(decoded.temperature_raw.0, i16::MIN);
    // Verify wrap arithmetic still works on decoded counter
    let next_tick = decoded.counter + Wrapping(1u32);
    assert_eq!(next_tick.0, 0u32);
}

// ===== 22. Struct with Reverse field roundtrip (derive) =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct LeaderboardEntry {
    player_id: u64,
    // Storing score as Reverse so lower-score entries sort last in a max-heap
    score: Reverse<u64>,
    rank_label: String,
}

#[test]
fn test_wrapping_advanced_struct_with_reverse_field_roundtrip() {
    let original = LeaderboardEntry {
        player_id: 999_999u64,
        score: Reverse(1_000_000u64),
        rank_label: String::from("Champion"),
    };
    let encoded = encode_to_vec(&original).expect("encode LeaderboardEntry failed");
    let (decoded, _): (LeaderboardEntry, _) =
        decode_from_slice(&encoded).expect("decode LeaderboardEntry failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.score.0, 1_000_000u64);
    assert_eq!(decoded.rank_label, "Champion");
    // Verify Reverse ordering: a lower score should compare as Greater under Reverse ordering
    assert!(
        Reverse(500u64) > Reverse(1000u64),
        "Reverse ordering: lower inner is Greater"
    );
}
