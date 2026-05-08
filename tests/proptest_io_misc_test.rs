//! I/O, hex, misc type property-based roundtrip tests using proptest
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
use oxicode::{decode_from_slice, encode_to_vec};
use proptest::prelude::*;

// --- Task 3: Cow<str> roundtrip ---

proptest! {
    #[test]
    fn prop_roundtrip_cow_str(
        s in proptest::string::string_regex("[a-z0-9 ]{0,50}").unwrap()
    ) {
        use std::borrow::Cow;
        let cow: Cow<str> = Cow::Owned(s.clone());
        let enc = encode_to_vec(&cow).expect("encode");
        let (dec, _): (Cow<str>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(s, dec.as_ref());
    }
}

proptest! {
    #[test]
    fn prop_roundtrip_duration(secs in 0u64..1_000_000u64, nanos in 0u32..999_999_999u32) {
        let dur = std::time::Duration::new(secs, nanos);
        let enc = encode_to_vec(&dur).expect("encode");
        let (dec, _): (std::time::Duration, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(dur, dec);
    }
}

proptest! {
    // encode_to_writer into Vec<u8> (which implements Write) produces same bytes as encode_to_vec
    #[test]
    fn prop_encode_to_writer_matches_encode_to_vec_u64(v: u64) {
        let mut buf = Vec::new();
        let n = oxicode::encode_to_writer(&v, &mut buf).expect("encode_to_writer");
        let expected = encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(n, expected.len(), "bytes written must match vec length");
        prop_assert_eq!(buf, expected, "encode_to_writer must produce same bytes as encode_to_vec");
    }

    #[test]
    fn prop_encode_to_writer_matches_encode_to_vec_string(
        s in proptest::string::string_regex("[a-z0-9]{0,50}").unwrap()
    ) {
        let mut buf = Vec::new();
        let n = oxicode::encode_to_writer(&s, &mut buf).expect("encode_to_writer");
        let expected = encode_to_vec(&s).expect("encode_to_vec");
        prop_assert_eq!(n, expected.len());
        prop_assert_eq!(buf, expected);
    }

    #[test]
    fn prop_encode_to_writer_matches_encode_to_vec_vec_u32(v: Vec<u32>) {
        let mut buf = Vec::new();
        let n = oxicode::encode_to_writer(&v, &mut buf).expect("encode_to_writer");
        let expected = encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(n, expected.len());
        prop_assert_eq!(buf, expected);
    }

    // decode_from_reader: encode with encode_to_vec, decode via Cursor<&[u8]>
    #[test]
    fn prop_decode_from_reader_roundtrip_u64(v: u64) {
        use std::io::Cursor;
        let bytes = encode_to_vec(&v).expect("encode");
        let cursor = Cursor::new(&bytes);
        let (dec, n): (u64, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, v);
        prop_assert_eq!(n, bytes.len());
    }

    #[test]
    fn prop_decode_from_reader_roundtrip_string(
        s in proptest::string::string_regex("[a-z0-9 ]{0,50}").unwrap()
    ) {
        use std::io::Cursor;
        let bytes = encode_to_vec(&s).expect("encode");
        let cursor = Cursor::new(bytes.clone());
        let (dec, n): (String, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, s);
        prop_assert_eq!(n, bytes.len());
    }

    #[test]
    fn prop_decode_from_reader_roundtrip_vec_u8(v: Vec<u8>) {
        use std::io::Cursor;
        let bytes = encode_to_vec(&v).expect("encode");
        let cursor = Cursor::new(bytes.clone());
        let (dec, n): (Vec<u8>, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, v);
        prop_assert_eq!(n, bytes.len());
    }

    // encode_to_writer + decode_from_reader are inverses of each other end-to-end
    #[test]
    fn prop_writer_reader_roundtrip_i32(v: i32) {
        use std::io::Cursor;
        let mut buf = Vec::new();
        let n_written = oxicode::encode_to_writer(&v, &mut buf).expect("encode_to_writer");
        let cursor = Cursor::new(&buf);
        let (dec, n_read): (i32, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, v);
        prop_assert_eq!(n_written, n_read);
    }

    #[test]
    fn prop_writer_reader_roundtrip_option_string(v: Option<String>) {
        use std::io::Cursor;
        let mut buf = Vec::new();
        let n_written = oxicode::encode_to_writer(&v, &mut buf).expect("encode_to_writer");
        let cursor = Cursor::new(&buf);
        let (dec, n_read): (Option<String>, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, v);
        prop_assert_eq!(n_written, n_read);
    }
}

proptest! {
    #[test]
    fn prop_hex_roundtrip_u32(v: u32) {
        let hex = oxicode::encode_to_hex(&v).expect("encode_to_hex");
        let (dec, _): (u32, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(v, dec);
    }

    #[test]
    fn prop_hex_roundtrip_string(s: String) {
        let hex = oxicode::encode_to_hex(&s).expect("encode_to_hex");
        let (dec, _): (String, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(s, dec);
    }

    #[test]
    fn prop_hex_roundtrip_vec_u8(v: Vec<u8>) {
        let hex = oxicode::encode_to_hex(&v).expect("encode_to_hex");
        let (dec, _): (Vec<u8>, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(v, dec);
    }
}

proptest! {
    // Range types
    #[test]
    fn prop_range_u32_roundtrip(start: u32, end: u32) {
        let r = start..end;
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (core::ops::Range<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r, dec);
    }

    #[test]
    fn prop_range_from_u32_roundtrip(start: u32) {
        let r: core::ops::RangeFrom<u32> = start..;
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (core::ops::RangeFrom<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r.start, dec.start);
    }

    #[test]
    fn prop_range_to_u32_roundtrip(end: u32) {
        let r: core::ops::RangeTo<u32> = ..end;
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (core::ops::RangeTo<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r.end, dec.end);
    }

    // encoded_size matches actual len
    #[test]
    fn prop_encoded_size_matches_actual(v: Vec<u32>) {
        let size = oxicode::encoded_size(&v).expect("encoded_size");
        let actual = oxicode::encode_to_vec(&v).expect("encode").len();
        prop_assert_eq!(size, actual);
    }

    #[test]
    fn prop_encoded_size_matches_actual_string(s: String) {
        let size = oxicode::encoded_size(&s).expect("encoded_size");
        let actual = oxicode::encode_to_vec(&s).expect("encode").len();
        prop_assert_eq!(size, actual);
    }

    // encode_copy == encode_to_vec for Copy types
    #[test]
    fn prop_encode_copy_matches_ref_u64(v: u64) {
        let copy_enc = oxicode::encode_copy(v).expect("encode_copy");
        let ref_enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(copy_enc, ref_enc);
    }

    #[test]
    fn prop_encode_copy_matches_ref_i64(v: i64) {
        let copy_enc = oxicode::encode_copy(v).expect("encode_copy");
        let ref_enc = oxicode::encode_to_vec(&v).expect("encode_to_vec");
        prop_assert_eq!(copy_enc, ref_enc);
    }

    // hex roundtrip for integers
    #[test]
    fn prop_hex_roundtrip_i64(v: i64) {
        let hex = oxicode::encode_to_hex(&v).expect("encode_to_hex");
        let (dec, _): (i64, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(v, dec);
    }

    #[test]
    fn prop_hex_roundtrip_bool(v: bool) {
        let hex = oxicode::encode_to_hex(&v).expect("encode_to_hex");
        let (dec, _): (bool, _) = oxicode::decode_from_hex(&hex).expect("decode_from_hex");
        prop_assert_eq!(v, dec);
    }
}

// 10. prop_writer_reader_u32_roundtrip
proptest! {
    #[test]
    fn prop_writer_reader_u32_roundtrip(v: u32) {
        use std::io::Cursor;
        let mut buf = Vec::new();
        let n_written = oxicode::encode_to_writer(&v, &mut buf).expect("encode_to_writer");
        let cursor = Cursor::new(&buf);
        let (dec, n_read): (u32, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, v);
        prop_assert_eq!(n_written, n_read);
    }
}

// 11. prop_writer_reader_string_roundtrip
proptest! {
    #[test]
    fn prop_writer_reader_string_roundtrip(
        s in proptest::string::string_regex("[a-z0-9 ]{0,80}").unwrap()
    ) {
        use std::io::Cursor;
        let mut buf = Vec::new();
        let n_written = oxicode::encode_to_writer(&s, &mut buf).expect("encode_to_writer");
        let cursor = Cursor::new(&buf);
        let (dec, n_read): (String, usize) =
            oxicode::decode_from_reader(cursor).expect("decode_from_reader");
        prop_assert_eq!(dec, s);
        prop_assert_eq!(n_written, n_read);
    }
}

// 15. prop_option_roundtrip_bool: Option<bool>
proptest! {
    #[test]
    fn prop_option_roundtrip_bool(v: Option<bool>) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (Option<bool>, _) =
            oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }
}

// 1. prop_range_inclusive_u32_roundtrip - RangeInclusive<u32>
proptest! {
    #[test]
    fn prop_range_inclusive_u32_roundtrip(start: u32, end: u32) {
        let r = start..=end;
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (std::ops::RangeInclusive<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r, dec);
    }
}

// 2. prop_bound_i32_exhaustive_roundtrip - std::ops::Bound<i32>
proptest! {
    #[test]
    fn prop_bound_i32_exhaustive_roundtrip(v: i32, kind in 0u8..3u8) {
        use std::ops::Bound;
        let b: Bound<i32> = match kind {
            0 => Bound::Included(v),
            1 => Bound::Excluded(v),
            _ => Bound::Unbounded,
        };
        let enc = oxicode::encode_to_vec(&b).expect("encode");
        let (dec, _): (Bound<i32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(b, dec);
    }
}

// 3. prop_cell_u32_roundtrip - std::cell::Cell<u32>
proptest! {
    #[test]
    fn prop_cell_u32_roundtrip(v: u32) {
        use std::cell::Cell;
        let c = Cell::new(v);
        let enc = oxicode::encode_to_vec(&c).expect("encode");
        let (dec, _): (Cell<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(c.get(), dec.get());
    }
}

// 4. prop_wrapping_i64_roundtrip - Wrapping<i64>
proptest! {
    #[test]
    fn prop_wrapping_i64_roundtrip(v: i64) {
        use std::num::Wrapping;
        let w = Wrapping(v);
        let enc = oxicode::encode_to_vec(&w).expect("encode");
        let (dec, _): (Wrapping<i64>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(w, dec);
    }
}

// 5. prop_nonzero_u32_new_roundtrip - NonZeroU32
proptest! {
    #[test]
    fn prop_nonzero_u32_new_roundtrip(v in 1u32..=u32::MAX) {
        use std::num::NonZeroU32;
        let nz = NonZeroU32::new(v).expect("nonzero");
        let enc = oxicode::encode_to_vec(&nz).expect("encode");
        let (dec, _): (NonZeroU32, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(nz, dec);
    }
}

// 6. prop_char_from_u32_roundtrip - char from valid unicode scalar values
proptest! {
    #[test]
    fn prop_char_from_u32_roundtrip(
        raw in (0u32..=0x10FFFFu32).prop_filter("not surrogate", |x| !(*x >= 0xD800 && *x <= 0xDFFF))
    ) {
        let c = char::from_u32(raw).expect("valid char");
        let enc = oxicode::encode_to_vec(&c).expect("encode");
        let (dec, _): (char, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(c, dec);
    }
}

// 7. prop_i128_new_roundtrip - i128
proptest! {
    #[test]
    fn prop_i128_new_roundtrip(v in i128::MIN..=i128::MAX) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (i128, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }
}

// 8. prop_u128_new_roundtrip - u128
proptest! {
    #[test]
    fn prop_u128_new_roundtrip(v in u128::MIN..=u128::MAX) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (u128, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }
}

// 9. prop_f32_finite_roundtrip - f32 finite values
proptest! {
    #[test]
    fn prop_f32_finite_roundtrip(
        v in proptest::num::f32::ANY.prop_filter("finite", |x| x.is_finite())
    ) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (f32, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v.to_bits(), dec.to_bits());
    }
}

// 10. prop_f64_finite_roundtrip - f64 finite values
proptest! {
    #[test]
    fn prop_f64_finite_roundtrip(
        v in proptest::num::f64::ANY.prop_filter("finite", |x| x.is_finite())
    ) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (f64, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v.to_bits(), dec.to_bits());
    }
}

// 13. prop_box_str_ascii_roundtrip - Box<str> from ASCII string
proptest! {
    #[test]
    fn prop_box_str_ascii_roundtrip(
        s in proptest::string::string_regex("[a-zA-Z0-9 _.-]{0,100}").unwrap()
    ) {
        let b: Box<str> = s.clone().into_boxed_str();
        let enc = oxicode::encode_to_vec(&b).expect("encode");
        let (dec, _): (Box<str>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(b, dec);
    }
}

// 14. prop_result_ok_err_u32_string_roundtrip - Result<u32, String>
proptest! {
    #[test]
    fn prop_result_ok_err_u32_string_roundtrip(v: u32, s: String, is_ok: bool) {
        let r: Result<u32, String> = if is_ok { Ok(v) } else { Err(s) };
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (Result<u32, String>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r, dec);
    }
}

// 15. prop_path_ascii_roundtrip - PathBuf from ASCII string (unix only)
proptest! {
    #[test]
    fn prop_path_ascii_roundtrip(
        s in proptest::string::string_regex("[a-zA-Z0-9_/.-]{1,50}").unwrap()
    ) {
        #[cfg(unix)]
        {
            use std::path::PathBuf;
            let p = PathBuf::from(&s);
            let enc = oxicode::encode_to_vec(&p).expect("encode");
            let (dec, _): (PathBuf, _) = oxicode::decode_from_slice(&enc).expect("decode");
            prop_assert_eq!(p, dec);
        }
        #[cfg(not(unix))]
        {
            let _ = s;
        }
    }
}
