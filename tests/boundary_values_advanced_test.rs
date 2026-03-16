//! Advanced boundary value tests covering varint encoding transitions, float bit patterns,
//! zero-sized collections, fixed-size arrays, NonZero types, and composite types at extremes.
//!
//! These tests deliberately exercise values at or immediately adjacent to every encoding
//! threshold so that any regression in the varint scheme or primitive serialisation is caught.
//! They do NOT duplicate the basic MIN/MAX roundtrip coverage already present in
//! `numeric_boundary_test.rs`; instead each test targets a distinct encoding property.

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
use oxicode::{config, decode_from_slice, encode_to_vec, encoded_size};

mod boundary_values_advanced {
    use super::*;
    use core::num::{NonZeroU64, NonZeroU8};

    // -------------------------------------------------------------------------
    // Test 1 – u8: interior boundaries (0, 1, 127, 128, 254, 255)
    //
    // u8 is always encoded as a single byte regardless of value.  We check
    // roundtrip fidelity at every interesting bit-boundary within the type.
    // -------------------------------------------------------------------------
    #[test]
    fn test_01_u8_interior_boundaries() {
        for v in [0u8, 1, 127, 128, 254, 255] {
            let enc = encode_to_vec(&v).expect("encode u8");
            assert_eq!(
                enc.len(),
                1,
                "u8 must always encode as 1 byte, got {} for value {}",
                enc.len(),
                v
            );
            let (dec, consumed): (u8, _) = decode_from_slice(&enc).expect("decode u8");
            assert_eq!(v, dec, "u8 roundtrip failed for {}", v);
            assert_eq!(consumed, 1, "u8 must consume exactly 1 byte");
        }
    }

    // -------------------------------------------------------------------------
    // Test 2 – u16: varint transition at 250 → 251 and upper boundary
    //
    // Values 0–250 encode as 1 byte.  251 is the first u16 value that crosses
    // into the 3-byte range (marker + u16 LE).  We also probe 252, 255, 256,
    // 65534, and 65535 to cover the full 3-byte window.
    // -------------------------------------------------------------------------
    #[test]
    fn test_02_u16_varint_boundaries() {
        // 1-byte territory
        for v in [0u16, 1, 250] {
            let enc = encode_to_vec(&v).expect("encode u16 1-byte");
            assert_eq!(enc.len(), 1, "u16 value {} must encode as 1 byte", v);
            let (dec, _): (u16, _) = decode_from_slice(&enc).expect("decode u16");
            assert_eq!(v, dec);
        }
        // 3-byte territory (marker byte + u16 LE)
        for v in [251u16, 252, 255, 256, 65534, 65535] {
            let enc = encode_to_vec(&v).expect("encode u16 3-byte");
            assert_eq!(enc.len(), 3, "u16 value {} must encode as 3 bytes", v);
            let (dec, _): (u16, _) = decode_from_slice(&enc).expect("decode u16");
            assert_eq!(v, dec);
        }
    }

    // -------------------------------------------------------------------------
    // Test 3 – u32: spans all three varint windows (1, 3, 5 bytes)
    //
    // 0 and 250 → 1 byte; 251 and 65535 → 3 bytes; 65536, u32::MAX-1, u32::MAX
    // → 5 bytes.
    // -------------------------------------------------------------------------
    #[test]
    fn test_03_u32_varint_boundaries() {
        // 1-byte window
        for v in [0u32, 250] {
            let enc = encode_to_vec(&v).expect("encode u32");
            assert_eq!(enc.len(), 1, "u32 {} must be 1 byte", v);
            let (dec, _): (u32, _) = decode_from_slice(&enc).expect("decode u32");
            assert_eq!(v, dec);
        }
        // 3-byte window
        for v in [251u32, 65535] {
            let enc = encode_to_vec(&v).expect("encode u32");
            assert_eq!(enc.len(), 3, "u32 {} must be 3 bytes", v);
            let (dec, _): (u32, _) = decode_from_slice(&enc).expect("decode u32");
            assert_eq!(v, dec);
        }
        // 5-byte window
        for v in [65536u32, u32::MAX - 1, u32::MAX] {
            let enc = encode_to_vec(&v).expect("encode u32");
            assert_eq!(enc.len(), 5, "u32 {} must be 5 bytes", v);
            let (dec, _): (u32, _) = decode_from_slice(&enc).expect("decode u32");
            assert_eq!(v, dec);
        }
    }

    // -------------------------------------------------------------------------
    // Test 4 – u64: all four varint windows (1, 3, 5, 9 bytes)
    //
    // The 9-byte window starts at u32::MAX + 1.  We probe both sides of every
    // window boundary.
    // -------------------------------------------------------------------------
    #[test]
    fn test_04_u64_varint_all_windows() {
        // 1-byte: 0 and 250
        for v in [0u64, 250] {
            let enc = encode_to_vec(&v).expect("encode u64");
            assert_eq!(enc.len(), 1, "u64 {} must be 1 byte", v);
            let (dec, _): (u64, _) = decode_from_slice(&enc).expect("decode u64");
            assert_eq!(v, dec);
        }
        // 3-byte: 251 and 65535
        for v in [251u64, 65535] {
            let enc = encode_to_vec(&v).expect("encode u64");
            assert_eq!(enc.len(), 3, "u64 {} must be 3 bytes", v);
            let (dec, _): (u64, _) = decode_from_slice(&enc).expect("decode u64");
            assert_eq!(v, dec);
        }
        // 5-byte: 65536 and u32::MAX
        for v in [65536u64, u32::MAX as u64] {
            let enc = encode_to_vec(&v).expect("encode u64");
            assert_eq!(enc.len(), 5, "u64 {} must be 5 bytes", v);
            let (dec, _): (u64, _) = decode_from_slice(&enc).expect("decode u64");
            assert_eq!(v, dec);
        }
        // 9-byte: u32::MAX + 1 and u64::MAX
        for v in [u32::MAX as u64 + 1, u64::MAX] {
            let enc = encode_to_vec(&v).expect("encode u64");
            assert_eq!(enc.len(), 9, "u64 {} must be 9 bytes", v);
            let (dec, _): (u64, _) = decode_from_slice(&enc).expect("decode u64");
            assert_eq!(v, dec);
        }
    }

    // -------------------------------------------------------------------------
    // Test 5 – u128: zero, u64::MAX, and u128::MAX roundtrip
    //
    // u128 uses a specialised 17-byte fixed encoding (marker + 16 LE bytes).
    // We verify the three extreme values roundtrip without data loss.
    // -------------------------------------------------------------------------
    #[test]
    fn test_05_u128_extremes() {
        for v in [0u128, u64::MAX as u128, u128::MAX] {
            let enc = encode_to_vec(&v).expect("encode u128");
            let (dec, _): (u128, _) = decode_from_slice(&enc).expect("decode u128");
            assert_eq!(v, dec, "u128 roundtrip failed for {}", v);
            // encoded_size must match actual bytes
            let sz = encoded_size(&v).expect("encoded_size u128");
            assert_eq!(sz, enc.len(), "encoded_size mismatch for u128 {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // Test 6 – i8: full set of interesting values (-128, -1, 0, 1, 127)
    //
    // Signed integers use zigzag encoding; we verify that both extremes and the
    // zero-cross values roundtrip correctly and that encoded_size is consistent.
    // -------------------------------------------------------------------------
    #[test]
    fn test_06_i8_boundary_values() {
        for v in [i8::MIN, -1i8, 0, 1, i8::MAX] {
            let enc = encode_to_vec(&v).expect("encode i8");
            let (dec, _): (i8, _) = decode_from_slice(&enc).expect("decode i8");
            assert_eq!(v, dec, "i8 roundtrip failed for {}", v);
            let sz = encoded_size(&v).expect("encoded_size i8");
            assert_eq!(sz, enc.len(), "encoded_size mismatch for i8 {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // Test 7 – i16: boundary values (MIN, -1, 0, 1, MAX)
    // -------------------------------------------------------------------------
    #[test]
    fn test_07_i16_boundary_values() {
        for v in [i16::MIN, -1i16, 0, 1, i16::MAX] {
            let enc = encode_to_vec(&v).expect("encode i16");
            let (dec, _): (i16, _) = decode_from_slice(&enc).expect("decode i16");
            assert_eq!(v, dec, "i16 roundtrip failed for {}", v);
            let sz = encoded_size(&v).expect("encoded_size i16");
            assert_eq!(sz, enc.len(), "encoded_size mismatch for i16 {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // Test 8 – i32: boundary values (MIN, -1, 0, 1, MAX)
    // -------------------------------------------------------------------------
    #[test]
    fn test_08_i32_boundary_values() {
        for v in [i32::MIN, -1i32, 0, 1, i32::MAX] {
            let enc = encode_to_vec(&v).expect("encode i32");
            let (dec, _): (i32, _) = decode_from_slice(&enc).expect("decode i32");
            assert_eq!(v, dec, "i32 roundtrip failed for {}", v);
            let sz = encoded_size(&v).expect("encoded_size i32");
            assert_eq!(sz, enc.len(), "encoded_size mismatch for i32 {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // Test 9 – i64: boundary values (MIN, -1, 0, 1, MAX)
    // -------------------------------------------------------------------------
    #[test]
    fn test_09_i64_boundary_values() {
        for v in [i64::MIN, -1i64, 0, 1, i64::MAX] {
            let enc = encode_to_vec(&v).expect("encode i64");
            let (dec, _): (i64, _) = decode_from_slice(&enc).expect("decode i64");
            assert_eq!(v, dec, "i64 roundtrip failed for {}", v);
            let sz = encoded_size(&v).expect("encoded_size i64");
            assert_eq!(sz, enc.len(), "encoded_size mismatch for i64 {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // Test 10 – i128: MIN, 0, and MAX
    // -------------------------------------------------------------------------
    #[test]
    fn test_10_i128_boundary_values() {
        for v in [i128::MIN, 0i128, i128::MAX] {
            let enc = encode_to_vec(&v).expect("encode i128");
            let (dec, _): (i128, _) = decode_from_slice(&enc).expect("decode i128");
            assert_eq!(v, dec, "i128 roundtrip failed for {}", v);
            let sz = encoded_size(&v).expect("encoded_size i128");
            assert_eq!(sz, enc.len(), "encoded_size mismatch for i128 {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // Test 11 – f32: 0.0, -0.0, f32::MIN, f32::MAX (bit-exact comparison)
    //
    // -0.0 and +0.0 compare equal by IEEE 754 == but differ in their bit
    // patterns; we use to_bits() to verify the sign bit is preserved.
    // f32::MIN and f32::MAX are the most-negative and most-positive finite
    // values respectively.
    // -------------------------------------------------------------------------
    #[test]
    fn test_11_f32_zero_and_extremes() {
        let pos_zero: f32 = 0.0_f32;
        let neg_zero: f32 = -0.0_f32;

        // Positive zero
        let enc = encode_to_vec(&pos_zero).expect("encode f32 +0.0");
        let (dec, _): (f32, _) = decode_from_slice(&enc).expect("decode f32 +0.0");
        assert_eq!(
            pos_zero.to_bits(),
            dec.to_bits(),
            "f32 +0.0 bit pattern not preserved"
        );

        // Negative zero – sign bit must survive the round-trip
        let enc = encode_to_vec(&neg_zero).expect("encode f32 -0.0");
        let (dec, _): (f32, _) = decode_from_slice(&enc).expect("decode f32 -0.0");
        assert_eq!(
            neg_zero.to_bits(),
            dec.to_bits(),
            "f32 -0.0 sign bit not preserved"
        );
        // Confirm they differ at the bit level
        assert_ne!(
            pos_zero.to_bits(),
            neg_zero.to_bits(),
            "sanity: +0.0 and -0.0 must differ"
        );

        // f32::MIN (most-negative finite) and f32::MAX (most-positive finite)
        for v in [f32::MIN, f32::MAX] {
            let enc = encode_to_vec(&v).expect("encode f32 extreme");
            let (dec, _): (f32, _) = decode_from_slice(&enc).expect("decode f32 extreme");
            assert_eq!(
                v.to_bits(),
                dec.to_bits(),
                "f32 {} bit pattern not preserved",
                v
            );
        }
    }

    // -------------------------------------------------------------------------
    // Test 12 – f64: 0.0, -0.0, f64::MIN, f64::MAX (bit-exact comparison)
    // -------------------------------------------------------------------------
    #[test]
    fn test_12_f64_zero_and_extremes() {
        let pos_zero: f64 = 0.0_f64;
        let neg_zero: f64 = -0.0_f64;

        // Positive zero
        let enc = encode_to_vec(&pos_zero).expect("encode f64 +0.0");
        let (dec, _): (f64, _) = decode_from_slice(&enc).expect("decode f64 +0.0");
        assert_eq!(
            pos_zero.to_bits(),
            dec.to_bits(),
            "f64 +0.0 bit pattern not preserved"
        );

        // Negative zero
        let enc = encode_to_vec(&neg_zero).expect("encode f64 -0.0");
        let (dec, _): (f64, _) = decode_from_slice(&enc).expect("decode f64 -0.0");
        assert_eq!(
            neg_zero.to_bits(),
            dec.to_bits(),
            "f64 -0.0 sign bit not preserved"
        );
        assert_ne!(
            pos_zero.to_bits(),
            neg_zero.to_bits(),
            "sanity: +0.0 and -0.0 must differ"
        );

        // f64::MIN and f64::MAX
        for v in [f64::MIN, f64::MAX] {
            let enc = encode_to_vec(&v).expect("encode f64 extreme");
            let (dec, _): (f64, _) = decode_from_slice(&enc).expect("decode f64 extreme");
            assert_eq!(
                v.to_bits(),
                dec.to_bits(),
                "f64 {} bit pattern not preserved",
                v
            );
        }
    }

    // -------------------------------------------------------------------------
    // Test 13 – Varint size verification for u64 across all window boundaries
    //
    // This test explicitly verifies the four encoding widths that the oxicode
    // varint scheme defines for u64:
    //   0–250         → 1 byte  (value fits in single byte)
    //   251–65535     → 3 bytes (marker 0xFB + 2 LE bytes)
    //   65536–2^32-1  → 5 bytes (marker 0xFC + 4 LE bytes)
    //   2^32–2^64-1   → 9 bytes (marker 0xFD + 8 LE bytes)
    // -------------------------------------------------------------------------
    #[test]
    fn test_13_varint_size_u64_all_boundaries() {
        struct Case {
            value: u64,
            expected_bytes: usize,
            label: &'static str,
        }

        let cases = [
            Case {
                value: 250,
                expected_bytes: 1,
                label: "250",
            },
            Case {
                value: 251,
                expected_bytes: 3,
                label: "251",
            },
            Case {
                value: 65535,
                expected_bytes: 3,
                label: "65535",
            },
            Case {
                value: 65536,
                expected_bytes: 5,
                label: "65536",
            },
            Case {
                value: u32::MAX as u64,
                expected_bytes: 5,
                label: "u32::MAX",
            },
            Case {
                value: u32::MAX as u64 + 1,
                expected_bytes: 9,
                label: "u32::MAX+1",
            },
            Case {
                value: u64::MAX,
                expected_bytes: 9,
                label: "u64::MAX",
            },
        ];

        for c in &cases {
            let enc = encode_to_vec(&c.value).expect("encode u64");
            assert_eq!(
                enc.len(),
                c.expected_bytes,
                "u64 value {} ({}) must encode as {} bytes, got {}",
                c.value,
                c.label,
                c.expected_bytes,
                enc.len()
            );
            // encoded_size must agree with the actual buffer length
            let sz = encoded_size(&c.value).expect("encoded_size u64");
            assert_eq!(sz, enc.len(), "encoded_size mismatch for u64 {}", c.label);
            // roundtrip
            let (dec, _): (u64, _) = decode_from_slice(&enc).expect("decode u64");
            assert_eq!(c.value, dec, "u64 roundtrip failed for {}", c.label);
        }
    }

    // -------------------------------------------------------------------------
    // Test 14 – Vec<u8> at length boundaries: 0, 250, 251 items
    //
    // A Vec<T> is encoded as varint(length) followed by each item.  At length
    // boundaries the prefix changes size, so total encoded length must reflect
    // the correct prefix size.
    //
    //  - empty (0 items): 1-byte length prefix (0x00) → total 1 byte
    //  - 250 items:       1-byte length prefix        → total 251 bytes
    //  - 251 items:       3-byte length prefix        → total 254 bytes
    // -------------------------------------------------------------------------
    #[test]
    fn test_14_vec_u8_length_boundaries() {
        // Empty vec → 1-byte length prefix + 0 data bytes = 1 byte total
        let empty: Vec<u8> = vec![];
        let enc_empty = encode_to_vec(&empty).expect("encode empty vec");
        assert_eq!(enc_empty.len(), 1, "empty Vec<u8> must encode as 1 byte");
        let (dec_empty, _): (Vec<u8>, _) = decode_from_slice(&enc_empty).expect("decode empty vec");
        assert_eq!(empty, dec_empty, "empty vec roundtrip failed");

        // 250-item vec → 1-byte prefix + 250 × 1 byte = 251 bytes total
        let v250: Vec<u8> = (0u8..250).collect();
        let enc250 = encode_to_vec(&v250).expect("encode 250-item vec");
        assert_eq!(
            enc250.len(),
            251,
            "250-item Vec<u8> must encode as 251 bytes (1-byte prefix + 250 data), got {}",
            enc250.len()
        );
        let (dec250, _): (Vec<u8>, _) = decode_from_slice(&enc250).expect("decode 250-item vec");
        assert_eq!(v250, dec250, "250-item vec roundtrip failed");

        // 251-item vec → 3-byte prefix + 251 × 1 byte = 254 bytes total
        let v251: Vec<u8> = (0u8..=250).collect(); // 0..=250 gives 251 elements
        let enc251 = encode_to_vec(&v251).expect("encode 251-item vec");
        assert_eq!(
            enc251.len(),
            254,
            "251-item Vec<u8> must encode as 254 bytes (3-byte prefix + 251 data), got {}",
            enc251.len()
        );
        let (dec251, _): (Vec<u8>, _) = decode_from_slice(&enc251).expect("decode 251-item vec");
        assert_eq!(v251, dec251, "251-item vec roundtrip failed");
    }

    // -------------------------------------------------------------------------
    // Test 15 – String at length boundaries: "", 250 chars, 251 chars
    //
    // A String is encoded as varint(byte_length) followed by the UTF-8 bytes.
    // For ASCII-only strings char count == byte count, so the same prefix-size
    // transitions apply.
    // -------------------------------------------------------------------------
    #[test]
    fn test_15_string_length_boundaries() {
        // Empty string → 1-byte prefix (0x00) + 0 data = 1 byte
        let empty = String::new();
        let enc_empty = encode_to_vec(&empty).expect("encode empty string");
        assert_eq!(enc_empty.len(), 1, "empty String must encode as 1 byte");
        let (dec_empty, _): (String, _) =
            decode_from_slice(&enc_empty).expect("decode empty string");
        assert_eq!(empty, dec_empty);

        // 250-char ASCII string → 1-byte prefix + 250 bytes = 251 bytes
        let s250: String = "x".repeat(250);
        let enc250 = encode_to_vec(&s250).expect("encode 250-char string");
        assert_eq!(
            enc250.len(),
            251,
            "250-char String must encode as 251 bytes, got {}",
            enc250.len()
        );
        let (dec250, _): (String, _) = decode_from_slice(&enc250).expect("decode 250-char string");
        assert_eq!(s250, dec250, "250-char string roundtrip failed");

        // 251-char ASCII string → 3-byte prefix + 251 bytes = 254 bytes
        let s251: String = "y".repeat(251);
        let enc251 = encode_to_vec(&s251).expect("encode 251-char string");
        assert_eq!(
            enc251.len(),
            254,
            "251-char String must encode as 254 bytes, got {}",
            enc251.len()
        );
        let (dec251, _): (String, _) = decode_from_slice(&enc251).expect("decode 251-char string");
        assert_eq!(s251, dec251, "251-char string roundtrip failed");
    }

    // -------------------------------------------------------------------------
    // Test 16 – Array [u8; 0]: zero-length fixed array roundtrip
    //
    // A fixed-size array has no length prefix; its encoded form is exactly
    // N × sizeof(element) bytes.  A zero-length array encodes as 0 bytes.
    // -------------------------------------------------------------------------
    #[test]
    fn test_16_array_zero_length_roundtrip() {
        let arr: [u8; 0] = [];
        let enc = encode_to_vec(&arr).expect("encode [u8; 0]");
        assert_eq!(
            enc.len(),
            0,
            "[u8; 0] must encode as 0 bytes, got {}",
            enc.len()
        );
        let (dec, consumed): ([u8; 0], _) = decode_from_slice(&enc).expect("decode [u8; 0]");
        assert_eq!(arr, dec, "[u8; 0] roundtrip failed");
        assert_eq!(consumed, 0, "[u8; 0] must consume 0 bytes");
    }

    // -------------------------------------------------------------------------
    // Test 17 – Array [u8; 255]: full-range fixed array roundtrip
    //
    // Contains every possible u8 value (0 through 254 and 255 implicitly via
    // the full 255-element initialisation).  Verifies no data corruption occurs
    // across the array boundary and that each element is stored unmodified.
    // -------------------------------------------------------------------------
    #[test]
    fn test_17_array_255_length_roundtrip() {
        let mut arr = [0u8; 255];
        for (i, b) in arr.iter_mut().enumerate() {
            *b = i as u8;
        }
        let enc = encode_to_vec(&arr).expect("encode [u8; 255]");
        assert_eq!(
            enc.len(),
            255,
            "[u8; 255] must encode as 255 bytes, got {}",
            enc.len()
        );
        let (dec, _): ([u8; 255], _) = decode_from_slice(&enc).expect("decode [u8; 255]");
        assert_eq!(arr, dec, "[u8; 255] roundtrip failed");
    }

    // -------------------------------------------------------------------------
    // Test 18 – Array [u64; 16] all elements u64::MAX
    //
    // Every element is the maximum u64 value, which requires 9 bytes each in
    // the oxicode varint scheme.  The expected encoded size is 16 × 9 = 144 bytes.
    // -------------------------------------------------------------------------
    #[test]
    fn test_18_array_u64_max_all_elements() {
        let arr = [u64::MAX; 16];
        let enc = encode_to_vec(&arr).expect("encode [u64::MAX; 16]");
        // Each u64::MAX encodes as 9 bytes; 16 × 9 = 144
        assert_eq!(
            enc.len(),
            144,
            "[u64::MAX; 16] must encode as 144 bytes (16 × 9), got {}",
            enc.len()
        );
        let (dec, _): ([u64; 16], _) = decode_from_slice(&enc).expect("decode [u64::MAX; 16]");
        assert_eq!(arr, dec, "[u64::MAX; 16] roundtrip failed");

        let sz = encoded_size(&arr).expect("encoded_size [u64::MAX; 16]");
        assert_eq!(sz, enc.len(), "encoded_size mismatch for [u64::MAX; 16]");
    }

    // -------------------------------------------------------------------------
    // Test 19 – Tuple at extreme type boundaries: (u8::MAX, u64::MAX, i64::MIN)
    //
    // Tuples are encoded field-by-field with no framing overhead.  The total
    // size must equal the sum of the individual field sizes.
    // -------------------------------------------------------------------------
    #[test]
    fn test_19_tuple_extreme_type_boundaries() {
        let tup = (u8::MAX, u64::MAX, i64::MIN);

        let enc = encode_to_vec(&tup).expect("encode tuple");
        let (dec, _): ((u8, u64, i64), _) = decode_from_slice(&enc).expect("decode tuple");
        assert_eq!(
            tup, dec,
            "tuple (u8::MAX, u64::MAX, i64::MIN) roundtrip failed"
        );

        // Verify total encoded size equals the sum of individual sizes
        let sz_u8 = encoded_size(&u8::MAX).expect("encoded_size u8::MAX");
        let sz_u64 = encoded_size(&u64::MAX).expect("encoded_size u64::MAX");
        let sz_i64 = encoded_size(&i64::MIN).expect("encoded_size i64::MIN");
        let expected_total = sz_u8 + sz_u64 + sz_i64;

        let sz = encoded_size(&tup).expect("encoded_size tuple");
        assert_eq!(
            sz, expected_total,
            "tuple encoded_size {} must equal sum of fields {}",
            sz, expected_total
        );
        assert_eq!(
            enc.len(),
            expected_total,
            "encoded byte len must match field sum"
        );
    }

    // -------------------------------------------------------------------------
    // Test 20 – usize::MAX roundtrip with encoded_size verification
    //
    // On 64-bit platforms usize is the same width as u64, so usize::MAX equals
    // u64::MAX and must encode in 9 bytes.  We verify both the roundtrip and
    // the size prediction from encoded_size().
    // -------------------------------------------------------------------------
    #[test]
    fn test_20_usize_max_with_size_verification() {
        let v: usize = usize::MAX;
        let enc = encode_to_vec(&v).expect("encode usize::MAX");

        // On any 64-bit platform usize::MAX == u64::MAX → 9 bytes
        #[cfg(target_pointer_width = "64")]
        assert_eq!(
            enc.len(),
            9,
            "usize::MAX on 64-bit must encode as 9 bytes, got {}",
            enc.len()
        );

        let (dec, consumed): (usize, _) = decode_from_slice(&enc).expect("decode usize::MAX");
        assert_eq!(v, dec, "usize::MAX roundtrip failed");
        assert_eq!(
            consumed,
            enc.len(),
            "consumed bytes must equal encoded length"
        );

        let sz = encoded_size(&v).expect("encoded_size usize::MAX");
        assert_eq!(
            sz,
            enc.len(),
            "encoded_size must match actual encoded length for usize::MAX"
        );
    }

    // -------------------------------------------------------------------------
    // Test 21 – NonZeroU8 boundary values: 1 and u8::MAX
    //
    // NonZeroU8 is encoded like its inner u8 value.  We verify roundtrip
    // fidelity at both ends of the legal NonZeroU8 range (1 .. 255 inclusive).
    // -------------------------------------------------------------------------
    #[test]
    fn test_21_nonzero_u8_boundaries() {
        let one = NonZeroU8::new(1).expect("NonZeroU8::new(1)");
        let max = NonZeroU8::new(u8::MAX).expect("NonZeroU8::new(u8::MAX)");

        for v in [one, max] {
            let enc = encode_to_vec(&v).expect("encode NonZeroU8");
            assert_eq!(
                enc.len(),
                1,
                "NonZeroU8 must encode as 1 byte, got {} for {}",
                enc.len(),
                v
            );
            let (dec, _): (NonZeroU8, _) = decode_from_slice(&enc).expect("decode NonZeroU8");
            assert_eq!(v, dec, "NonZeroU8 roundtrip failed for {}", v);

            // The encoded form must be byte-identical to encoding the inner u8
            let u8_enc = encode_to_vec(&v.get()).expect("encode inner u8");
            assert_eq!(
                enc, u8_enc,
                "NonZeroU8 encoding must match inner u8 encoding for {}",
                v
            );
        }
    }

    // -------------------------------------------------------------------------
    // Test 22 – NonZeroU64 boundary values: 1 and u64::MAX
    //
    // NonZeroU64 encodes as its inner u64.  At value 1 (encoded as 1 byte) and
    // u64::MAX (encoded as 9 bytes) we verify byte-identity with the
    // plain u64 encoding.
    // -------------------------------------------------------------------------
    #[test]
    fn test_22_nonzero_u64_boundaries() {
        let one = NonZeroU64::new(1).expect("NonZeroU64::new(1)");
        let max = NonZeroU64::new(u64::MAX).expect("NonZeroU64::new(u64::MAX)");

        // value 1 → 1 byte
        {
            let enc = encode_to_vec(&one).expect("encode NonZeroU64(1)");
            assert_eq!(
                enc.len(),
                1,
                "NonZeroU64(1) must encode as 1 byte, got {}",
                enc.len()
            );
            let (dec, _): (NonZeroU64, _) = decode_from_slice(&enc).expect("decode NonZeroU64(1)");
            assert_eq!(one, dec, "NonZeroU64(1) roundtrip failed");
            let u64_enc = encode_to_vec(&one.get()).expect("encode inner u64(1)");
            assert_eq!(
                enc, u64_enc,
                "NonZeroU64(1) encoding must match inner u64(1)"
            );
        }

        // value u64::MAX → 9 bytes
        {
            let enc = encode_to_vec(&max).expect("encode NonZeroU64(u64::MAX)");
            assert_eq!(
                enc.len(),
                9,
                "NonZeroU64(u64::MAX) must encode as 9 bytes, got {}",
                enc.len()
            );
            let (dec, _): (NonZeroU64, _) =
                decode_from_slice(&enc).expect("decode NonZeroU64(u64::MAX)");
            assert_eq!(max, dec, "NonZeroU64(u64::MAX) roundtrip failed");
            let u64_enc = encode_to_vec(&max.get()).expect("encode inner u64(u64::MAX)");
            assert_eq!(
                enc, u64_enc,
                "NonZeroU64(u64::MAX) encoding must match inner u64(u64::MAX)"
            );
        }

        // encoded_size must agree with actual bytes for both
        let sz_one = encoded_size(&one).expect("encoded_size NonZeroU64(1)");
        assert_eq!(sz_one, 1, "encoded_size for NonZeroU64(1) must be 1");
        let sz_max = encoded_size(&max).expect("encoded_size NonZeroU64(u64::MAX)");
        assert_eq!(sz_max, 9, "encoded_size for NonZeroU64(u64::MAX) must be 9");
    }

    // -------------------------------------------------------------------------
    // Additional consistency test: encoded_size agrees with encode_to_vec
    // across every test case that uses config::standard() implicitly
    // -------------------------------------------------------------------------
    #[test]
    fn test_encoded_size_config_consistency() {
        // Spot-check a handful of values using explicit config to make sure
        // encoded_size_with_config and encode_to_vec_with_config stay in sync.
        let cfg = config::standard();

        macro_rules! check {
            ($val:expr, $ty:ty) => {{
                let v: $ty = $val;
                let enc = oxicode::encode_to_vec_with_config(&v, cfg).expect("encode");
                let sz = oxicode::encoded_size_with_config(&v, cfg).expect("encoded_size");
                assert_eq!(
                    sz,
                    enc.len(),
                    "config encoded_size mismatch for {}",
                    stringify!($ty)
                );
            }};
        }

        check!(0u8, u8);
        check!(255u8, u8);
        check!(0u64, u64);
        check!(u64::MAX, u64);
        check!(i64::MIN, i64);
        check!(i64::MAX, i64);
        check!(0.0f32, f32);
        check!(f32::MAX, f32);
        check!(0.0f64, f64);
        check!(f64::MIN, f64);
    }
}
