//! Primitive types property-based roundtrip tests using proptest
//! (split from proptest_test.rs).
//!
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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

fn roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(value: &T) {
    let encoded = encode_to_vec(value).expect("encode failed");
    let (decoded, bytes_read): (T, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(value, &decoded, "roundtrip failed");
    assert_eq!(bytes_read, encoded.len(), "bytes_read mismatch");
}

proptest! {
    #[test]
    fn prop_roundtrip_u8(v: u8) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_u16(v: u16) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_u32(v: u32) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_u64(v: u64) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_u128(v: u128) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_i8(v: i8) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_i16(v: i16) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_i32(v: i32) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_i64(v: i64) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_i128(v: i128) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_f32(v: f32) {
        // Skip NaN because NaN != NaN
        prop_assume!(!v.is_nan());
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_f64(v: f64) {
        prop_assume!(!v.is_nan());
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_bool(v: bool) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_char(v: char) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_string(v: String) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_vec_u8(v: Vec<u8>) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_vec_u64(v: Vec<u64>) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_option_u32(v: Option<u32>) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_option_string(v: Option<String>) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_tuple_u32_string(a: u32, b: String) {
        roundtrip(&(a, b));
    }

    #[test]
    fn prop_roundtrip_tuple_i64_f64_bool(a: i64, b: f64, c: bool) {
        prop_assume!(!b.is_nan());
        roundtrip(&(a, b, c));
    }

    #[test]
    fn prop_roundtrip_nested_vec(v: Vec<Vec<u32>>) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_vec_string(v: Vec<String>) {
        roundtrip(&v);
    }

    // Varint edge cases: test values around encoding boundaries
    // 0-250: single byte; 251=u16; 252=u32; 253=u64; 254=u128
    #[test]
    fn prop_roundtrip_u64_varint_boundary(
        v in prop_oneof![
            0u64..=250,        // single byte range
            251u64..=65535,    // u16 range (tag 251)
            65536u64..=4294967295,  // u32 range (tag 252)
            any::<u64>(),      // full range including u64 (tag 253)
        ]
    ) {
        roundtrip(&v);
    }

    #[test]
    fn prop_roundtrip_i64_zigzag_boundary(
        v in prop_oneof![
            -125i64..=125,       // zigzag single byte
            126i64..=32767,      // zigzag u16
            any::<i64>(),        // full range
        ]
    ) {
        roundtrip(&v);
    }
}

// Range<u32>
proptest! {
    #[test]
    fn prop_roundtrip_range_u32(start in 0u32..1000u32, end in 0u32..1000u32) {
        let range = start..end;
        let enc = encode_to_vec(&range).expect("encode");
        let (dec, _): (core::ops::Range<u32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(range, dec);
    }
}

// Bound<i32>
proptest! {
    #[test]
    fn prop_roundtrip_bound_i32(v in -1000i32..1000i32, kind in 0u8..3u8) {
        let bound: core::ops::Bound<i32> = match kind {
            0 => core::ops::Bound::Unbounded,
            1 => core::ops::Bound::Included(v),
            _ => core::ops::Bound::Excluded(v),
        };
        let enc = encode_to_vec(&bound).expect("encode");
        let (dec, _): (core::ops::Bound<i32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(bound, dec);
    }
}

// Wrapping<u32>
proptest! {
    #[test]
    fn prop_roundtrip_wrapping_u32(v in u32::MIN..u32::MAX) {
        let w = core::num::Wrapping(v);
        let enc = encode_to_vec(&w).expect("encode");
        let (dec, _): (core::num::Wrapping<u32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(w, dec);
    }
}

// Vec<(String, i32)> - test complex nested types
proptest! {
    #[test]
    fn prop_roundtrip_vec_tuple_string_i32(
        pairs in proptest::collection::vec(
            (proptest::string::string_regex("[a-z]{1,20}").unwrap(), -1000i32..1000i32),
            0..20
        )
    ) {
        let enc = encode_to_vec(&pairs).expect("encode");
        let (dec, _): (Vec<(String, i32)>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(pairs, dec);
    }
}

// encoded_size matches encode_to_vec length
proptest! {
    #[test]
    fn prop_encoded_size_matches_vec_len_u64(v in u64::MIN..u64::MAX) {
        let size = oxicode::encoded_size(&v).expect("size");
        let enc = encode_to_vec(&v).expect("encode");
        prop_assert_eq!(size, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_encoded_size_matches_vec_len_string(
        s in proptest::string::string_regex("[a-z]{0,100}").unwrap()
    ) {
        let size = oxicode::encoded_size(&s).expect("size");
        let enc = encode_to_vec(&s).expect("encode");
        prop_assert_eq!(size, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_encoded_size_vec_u32(data in proptest::collection::vec(0u32..u32::MAX, 0..100)) {
        let size = oxicode::encoded_size(&data).expect("size");
        let enc = encode_to_vec(&data).expect("encode");
        prop_assert_eq!(size, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_encode_decode_option_u32(v in proptest::option::of(0u32..u32::MAX)) {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, _): (Option<u32>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }
}

proptest! {
    #[test]
    fn prop_truncated_data_returns_error(
        data in proptest::collection::vec(0u32..1000u32, 1..20),
        truncate_at in 0usize..50usize
    ) {
        let enc = encode_to_vec(&data).expect("encode");
        if truncate_at < enc.len() {
            let truncated = &enc[..truncate_at];
            let result: Result<(Vec<u32>, _), _> = decode_from_slice(truncated);
            // Truncated data should return error, never panic
            prop_assert!(result.is_err());
        }
    }
}

// Verify encode_to_vec and encode_to_fixed_array produce the same bytes for u32
proptest! {
    #[test]
    fn prop_fixed_array_matches_vec_u32(v in 0u32..u32::MAX) {
        let vec_bytes = encode_to_vec(&v).expect("vec encode");
        if vec_bytes.len() <= 10 {
            let (arr, n): ([u8; 10], usize) =
                oxicode::encode_to_fixed_array(&v).expect("fixed encode");
            prop_assert_eq!(vec_bytes.len(), n, "fixed array written length must match vec length");
            prop_assert_eq!(&vec_bytes[..], &arr[..n], "fixed array bytes must match vec bytes");
        }
    }
}

// Verify Option::None always encodes to exactly the same bytes, independent of
// the concrete u32 value that would have been stored in Some(v).
proptest! {
    #[test]
    fn prop_option_none_always_same_bytes(_x in 0u32..u32::MAX) {
        let none: Option<u32> = None;
        let enc1 = encode_to_vec(&none).expect("encode");
        let enc2 = encode_to_vec(&none).expect("encode2");
        prop_assert_eq!(enc1, enc2);
    }
}

// =============================================================================
// Bincode byte-for-byte compatibility tests
// =============================================================================

proptest! {
    #[test]
    fn prop_bincode_compat_u32(v in 0u32..u32::MAX) {
        let oxicode_bytes = oxicode::encode_to_vec_with_config(&v, oxicode::config::standard())
            .expect("oxicode encode u32");
        let bincode_bytes = bincode::encode_to_vec(v, bincode::config::standard())
            .expect("bincode encode u32");
        prop_assert_eq!(oxicode_bytes, bincode_bytes, "Mismatch for u32 = {}", v);
    }
}

proptest! {
    #[test]
    fn prop_bincode_compat_i32(v in i32::MIN..i32::MAX) {
        let oxicode_bytes = oxicode::encode_to_vec_with_config(&v, oxicode::config::standard())
            .expect("oxicode encode i32");
        let bincode_bytes = bincode::encode_to_vec(v, bincode::config::standard())
            .expect("bincode encode i32");
        prop_assert_eq!(oxicode_bytes, bincode_bytes, "Mismatch for i32 = {}", v);
    }
}

proptest! {
    #[test]
    fn prop_bincode_compat_string(
        s in proptest::string::string_regex("[a-z0-9 ]{0,100}").unwrap()
    ) {
        let oxicode_bytes = oxicode::encode_to_vec_with_config(&s, oxicode::config::standard())
            .expect("oxicode encode string");
        let bincode_bytes = bincode::encode_to_vec(&s, bincode::config::standard())
            .expect("bincode encode string");
        prop_assert_eq!(oxicode_bytes, bincode_bytes, "Mismatch for string = {:?}", s);
    }
}

proptest! {
    #[test]
    fn prop_bincode_compat_bool(b: bool) {
        let oxicode_bytes = oxicode::encode_to_vec_with_config(&b, oxicode::config::standard())
            .expect("oxicode encode bool");
        let bincode_bytes = bincode::encode_to_vec(b, bincode::config::standard())
            .expect("bincode encode bool");
        prop_assert_eq!(oxicode_bytes, bincode_bytes);
    }
}

// =============================================================================
// Varint size invariants
// =============================================================================

// All u64 values in range 0-250 should encode as exactly 1 byte
proptest! {
    #[test]
    fn prop_varint_small_values_1_byte(v in 0u64..=250) {
        let enc = encode_to_vec(&v).expect("encode u64 small");
        prop_assert_eq!(enc.len(), 1, "Value {} should be 1 byte", v);
    }
}

// Encoded size always matches actual encoded length for i32
proptest! {
    #[test]
    fn prop_encoded_size_matches_i32(v in i32::MIN..i32::MAX) {
        let size = oxicode::encoded_size(&v).expect("encoded_size i32");
        let enc = encode_to_vec(&v).expect("encode i32");
        prop_assert_eq!(size, enc.len());
    }
}

// Encoded size always matches actual encoded length for bool
proptest! {
    #[test]
    fn prop_encoded_size_matches_bool(b: bool) {
        let size = oxicode::encoded_size(&b).expect("encoded_size bool");
        let enc = encode_to_vec(&b).expect("encode bool");
        prop_assert_eq!(size, enc.len());
    }
}

// u8 always encodes as exactly 1 byte; verify fixed-array matches vec
proptest! {
    #[test]
    fn prop_fixed_array_matches_vec_u8(v in 0u8..=u8::MAX) {
        let vec_bytes = encode_to_vec(&v).expect("vec encode u8");
        // u8 always fits in 1 byte — encode into a [u8; 1] fixed array
        let (arr, n): ([u8; 1], usize) =
            oxicode::encode_to_fixed_array(&v).expect("fixed encode u8");
        prop_assert_eq!(n, 1, "u8 must write exactly 1 byte");
        prop_assert_eq!(vec_bytes.len(), 1, "vec must also be 1 byte for u8");
        prop_assert_eq!(vec_bytes[0], arr[0]);
    }
}

proptest! {
    // Tuple types
    #[test]
    fn prop_tuple_2_roundtrip(a: u32, b: u64) {
        let t = (a, b);
        let enc = oxicode::encode_to_vec(&t).expect("encode");
        let (dec, _): ((u32, u64), _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(t, dec);
    }

    #[test]
    fn prop_tuple_3_roundtrip(a: u8, b: i32, c: bool) {
        let t = (a, b, c);
        let enc = oxicode::encode_to_vec(&t).expect("encode");
        let (dec, _): ((u8, i32, bool), _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(t, dec);
    }

    // Wrapping types
    #[test]
    fn prop_wrapping_i32_roundtrip(v: i32) {
        use std::num::Wrapping;
        let w = Wrapping(v);
        let enc = oxicode::encode_to_vec(&w).expect("encode");
        let (dec, _): (Wrapping<i32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(w, dec);
    }

    // Option wrapping
    #[test]
    fn prop_option_u64_roundtrip(v: Option<u64>) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (Option<u64>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }

    // char type
    #[test]
    fn prop_char_roundtrip(c: char) {
        let enc = oxicode::encode_to_vec(&c).expect("encode");
        let (dec, _): (char, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(c, dec);
    }

    // bool
    #[test]
    fn prop_bool_roundtrip(b: bool) {
        let enc = oxicode::encode_to_vec(&b).expect("encode");
        let (dec, _): (bool, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(b, dec);
    }

    // f32 (skip NaN since NaN != NaN)
    #[test]
    fn prop_f32_non_nan_roundtrip(v in proptest::num::f32::NORMAL | proptest::num::f32::ZERO | proptest::num::f32::INFINITE | proptest::num::f32::NEGATIVE | proptest::num::f32::SUBNORMAL) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (f32, _) = oxicode::decode_from_slice(&enc).expect("decode");
        // Use bits comparison to handle -0.0 == +0.0 edge case
        prop_assert_eq!(v.to_bits(), dec.to_bits());
    }

    // encoded_size for bool, char, tuples
    #[test]
    fn prop_encoded_size_matches_bool_new(v: bool) {
        let size = oxicode::encoded_size(&v).expect("encoded_size");
        let actual = oxicode::encode_to_vec(&v).expect("encode").len();
        prop_assert_eq!(size, actual);
    }

    #[test]
    fn prop_encoded_size_matches_char(c: char) {
        let size = oxicode::encoded_size(&c).expect("encoded_size");
        let actual = oxicode::encode_to_vec(&c).expect("encode").len();
        prop_assert_eq!(size, actual);
    }

    // hex roundtrip for i32
    #[test]
    fn prop_hex_roundtrip_i32(v: i32) {
        let hex = oxicode::encode_to_hex(&v).expect("encode_to_hex");
        let (dec, _): (i32, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(v, dec);
    }
}

proptest! {
    // i128/u128 roundtrip
    #[test]
    fn prop_i128_roundtrip(v: i128) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (i128, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }

    #[test]
    fn prop_u128_roundtrip(v: u128) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (u128, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }

    // isize/usize roundtrip
    #[test]
    fn prop_usize_roundtrip(v: usize) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (usize, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }

    #[test]
    fn prop_isize_roundtrip(v: isize) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (isize, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }

    // BTreeMap
    #[test]
    fn prop_btreemap_string_u32_roundtrip(map: std::collections::BTreeMap<String, u32>) {
        let enc = oxicode::encode_to_vec(&map).expect("encode");
        let (dec, _): (std::collections::BTreeMap<String, u32>, _) =
            oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(map, dec);
    }

    // Vec<Vec<u8>> nested
    #[test]
    fn prop_vec_vec_u8_roundtrip(v: Vec<Vec<u8>>) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (Vec<Vec<u8>>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }

    // encoded_size for u128
    #[test]
    fn prop_encoded_size_matches_u128(v: u128) {
        let size = oxicode::encoded_size(&v).expect("encoded_size");
        let actual = oxicode::encode_to_vec(&v).expect("encode").len();
        prop_assert_eq!(size, actual);
    }
}

// 1. prop_encoded_size_matches_vec_len: Vec<u8> up to 1000 elements
proptest! {
    #[test]
    fn prop_encoded_size_matches_vec_len(
        v in proptest::collection::vec(any::<u8>(), 0..=1000)
    ) {
        let size = oxicode::encoded_size(&v).expect("encoded_size");
        let enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(size, enc.len());
    }
}

// 2. prop_encoded_size_u32_matches
proptest! {
    #[test]
    fn prop_encoded_size_u32_matches(v: u32) {
        let size = oxicode::encoded_size(&v).expect("encoded_size");
        let enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(size, enc.len());
    }
}

// 3. prop_encoded_size_string_matches
proptest! {
    #[test]
    fn prop_encoded_size_string_matches(v: String) {
        let size = oxicode::encoded_size(&v).expect("encoded_size");
        let enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(size, enc.len());
    }
}

// 4. prop_encode_copy_matches_encode_to_vec_u64
proptest! {
    #[test]
    fn prop_encode_copy_matches_encode_to_vec_u64(v: u64) {
        let copy_enc = oxicode::encode_copy(v).expect("encode_copy");
        let ref_enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(copy_enc, ref_enc);
    }
}

// 5. prop_encode_copy_matches_encode_to_vec_i32
proptest! {
    #[test]
    fn prop_encode_copy_matches_encode_to_vec_i32(v: i32) {
        let copy_enc = oxicode::encode_copy(v).expect("encode_copy");
        let ref_enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(copy_enc, ref_enc);
    }
}

// 6. prop_hex_roundtrip_u64
proptest! {
    #[test]
    fn prop_hex_roundtrip_u64(v: u64) {
        let hex = oxicode::encode_to_hex(&v).expect("encode_to_hex");
        let (dec, _): (u64, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(v, dec);
    }
}

// 7. prop_hex_roundtrip_string_ascii: String limited to ASCII chars, max 100 chars
proptest! {
    #[test]
    fn prop_hex_roundtrip_string_ascii(
        s in proptest::string::string_regex("[\\x20-\\x7e]{0,100}").unwrap()
    ) {
        let hex = oxicode::encode_to_hex(&s).expect("encode_to_hex");
        let (dec, _): (String, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(s, dec);
    }
}

// 8. prop_encode_copy_i64
proptest! {
    #[test]
    fn prop_encode_copy_i64(v: i64) {
        let copy_enc = oxicode::encode_copy(v).expect("encode_copy");
        let ref_enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(copy_enc, ref_enc);
    }
}
