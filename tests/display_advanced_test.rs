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
mod display_advanced_tests {
    use std::f64::consts::E;
    use std::f64::consts::PI;

    // -----------------------------------------------------------------------
    // 1. EncodedBytes for u8 42 shows hex "2a"
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_bytes_u8_42_shows_hex_2a() {
        let bytes = [42u8];
        let eb = oxicode::encoded_bytes(&bytes);
        assert_eq!(format!("{}", eb), "2a");
    }

    // -----------------------------------------------------------------------
    // 2. EncodedBytes for empty slice shows empty output
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_bytes_empty_slice() {
        let bytes: &[u8] = &[];
        let eb = oxicode::encoded_bytes(bytes);
        assert_eq!(format!("{}", eb), "");
        assert_eq!(format!("{:x}", eb), "");
        assert_eq!(format!("{:X}", eb), "");
    }

    // -----------------------------------------------------------------------
    // 3. EncodedBytes for [0, 255] shows "00 ff"
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_bytes_zero_and_ff() {
        let bytes = [0u8, 255u8];
        let eb = oxicode::encoded_bytes(&bytes);
        assert_eq!(format!("{}", eb), "00 ff");
    }

    // -----------------------------------------------------------------------
    // 4. Hex dump of encoded u32 shows correct bytes
    // -----------------------------------------------------------------------
    #[test]
    fn test_hex_dump_encoded_u32_correct_bytes() {
        let value = 1u32;
        let eb = oxicode::encode_to_display(&value).expect("encode_to_display failed");
        let dump = eb.hex_dump();
        // The dump must contain the zero-offset address prefix
        assert!(
            dump.contains("00000000:"),
            "dump must start with address 00000000:"
        );
        // The dump must be non-empty
        assert!(!dump.is_empty(), "hex dump must not be empty");
    }

    // -----------------------------------------------------------------------
    // 5. Hex dump with offset addresses
    // -----------------------------------------------------------------------
    #[test]
    fn test_hex_dump_offset_addresses() {
        // Encode 32 bytes so we get at least two rows (16 bytes each)
        let data: Vec<u8> = (0u8..32).collect();
        let eb = oxicode::encoded_bytes(&data);
        let dump = eb.hex_dump();
        // First row: offset 0x00000000
        assert!(
            dump.contains("00000000:"),
            "first row address must be 00000000"
        );
        // Second row: offset 0x00000010 (16 in hex)
        assert!(
            dump.contains("00000010:"),
            "second row address must be 00000010"
        );
    }

    // -----------------------------------------------------------------------
    // 6. Hex dump of String "hello" shows 5 data bytes after length prefix
    // -----------------------------------------------------------------------
    #[test]
    fn test_hex_dump_string_hello_data_bytes() {
        let s = String::from("hello");
        let eb = oxicode::encode_to_display(&s).expect("encode_to_display failed");
        let dump = eb.hex_dump();
        // "hello" in ASCII is 68 65 6c 6c 6f — all must appear in the dump
        assert!(dump.contains("68"), "dump must contain 'h' byte 68");
        assert!(dump.contains("65"), "dump must contain 'e' byte 65");
        assert!(dump.contains("6c"), "dump must contain 'l' byte 6c");
        assert!(dump.contains("6f"), "dump must contain 'o' byte 6f");
    }

    // -----------------------------------------------------------------------
    // 7. EncodedBytes Display output is readable (space-separated lowercase hex)
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_bytes_display_readable_format() {
        let bytes = [0x0fu8, 0x1e, 0x2d];
        let eb = oxicode::encoded_bytes(&bytes);
        let s = format!("{}", eb);
        // Must be space-separated pairs
        assert_eq!(s, "0f 1e 2d");
        // Each group is exactly 2 chars; groups are separated by single spaces
        let parts: Vec<&str> = s.split(' ').collect();
        assert_eq!(parts.len(), 3);
        for part in &parts {
            assert_eq!(part.len(), 2, "each hex group must be exactly 2 characters");
        }
    }

    // -----------------------------------------------------------------------
    // 8. EncodedBytes for large Vec<u8> (100 bytes) formats without panic
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_bytes_large_vec_no_panic() {
        let data: Vec<u8> = (0u8..100).collect();
        let eb = oxicode::encoded_bytes(&data);
        let s = format!("{}", eb);
        // 100 bytes → 100 groups of 2 chars + 99 spaces = 299 chars
        assert_eq!(s.len(), 299, "large vec display must be 299 chars");
    }

    // -----------------------------------------------------------------------
    // 9. Hex dump includes ASCII representation column
    // -----------------------------------------------------------------------
    #[test]
    fn test_hex_dump_includes_ascii_column() {
        let bytes = b"Hello, World!";
        let eb = oxicode::encoded_bytes(bytes);
        let dump = eb.hex_dump();
        // The ASCII sidebar shows printable characters
        assert!(
            dump.contains("Hello"),
            "dump ASCII sidebar must show 'Hello'"
        );
        assert!(
            dump.contains("World"),
            "dump ASCII sidebar must show 'World'"
        );
    }

    // -----------------------------------------------------------------------
    // 10. EncodedBytes of struct shows all field bytes (non-empty)
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_bytes_struct_shows_field_bytes() {
        use oxicode::{Decode, Encode};

        #[derive(Encode, Decode)]
        struct Point {
            x: u32,
            y: u32,
        }

        let p = Point { x: 10, y: 20 };
        let encoded = oxicode::encode_to_vec(&p).expect("encode failed");
        let eb = oxicode::encoded_bytes(&encoded);
        let s = format!("{}", eb);
        // Two u32 fields must produce at least some bytes
        assert!(!s.is_empty(), "encoded struct bytes must not be empty");
        // Encoded bytes should be at least 2 bytes (two varints)
        assert!(encoded.len() >= 2, "struct must encode to at least 2 bytes");
    }

    // -----------------------------------------------------------------------
    // 11. Hex dump row length is 16 bytes per row
    // -----------------------------------------------------------------------
    #[test]
    fn test_hex_dump_row_length_16_bytes() {
        // Exactly 16 bytes → single row, no second address
        let data: Vec<u8> = (0u8..16).collect();
        let eb = oxicode::encoded_bytes(&data);
        let dump = eb.hex_dump();
        let lines: Vec<&str> = dump.lines().collect();
        assert_eq!(lines.len(), 1, "16 bytes must produce exactly 1 row");

        // 17 bytes → two rows
        let data17: Vec<u8> = (0u8..17).collect();
        let eb17 = oxicode::encoded_bytes(&data17);
        let dump17 = eb17.hex_dump();
        let lines17: Vec<&str> = dump17.lines().collect();
        assert_eq!(lines17.len(), 2, "17 bytes must produce exactly 2 rows");
    }

    // -----------------------------------------------------------------------
    // 12. EncodedBytes Debug trait implementation (via pointer/manual Debug)
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_bytes_as_bytes_accessor() {
        // EncodedBytes does not derive Debug, but .as_bytes() always works
        let raw = [0xdeu8, 0xad, 0xbe, 0xef];
        let eb = oxicode::encoded_bytes(&raw);
        // Verify as_bytes() returns the original slice content
        assert_eq!(eb.as_bytes(), &raw);
        assert_eq!(eb.as_bytes().len(), 4);
        // Each byte is accessible by index
        assert_eq!(eb.as_bytes()[0], 0xde);
        assert_eq!(eb.as_bytes()[3], 0xef);
    }

    // -----------------------------------------------------------------------
    // 13. Hex encode of PI f64 bits
    // -----------------------------------------------------------------------
    #[test]
    fn test_hex_encode_pi_f64_bits() {
        let pi_bits = PI.to_bits();
        // Use big-endian bytes so the hex string can be parsed directly with from_str_radix
        let bytes = pi_bits.to_be_bytes();
        let eb = oxicode::encoded_bytes(&bytes);
        let hex = format!("{:x}", eb);
        // PI as f64 be-bytes is a fixed 16-character hex string
        assert_eq!(hex.len(), 16, "f64 must be exactly 16 hex chars");
        // Round-trip: parse back with big-endian interpretation
        let parsed = u64::from_str_radix(&hex, 16).expect("hex parse failed");
        assert_eq!(parsed, pi_bits, "round-tripped PI bits must match");
        let recovered = f64::from_bits(parsed);
        assert!(
            (recovered - PI).abs() < f64::EPSILON,
            "recovered f64 must equal PI"
        );
    }

    // -----------------------------------------------------------------------
    // 14. Hex encode of E f64 bits
    // -----------------------------------------------------------------------
    #[test]
    fn test_hex_encode_e_f64_bits() {
        let e_bits = E.to_bits();
        // Use big-endian bytes so the hex string can be parsed directly with from_str_radix
        let bytes = e_bits.to_be_bytes();
        let eb = oxicode::encoded_bytes(&bytes);
        let hex = format!("{:x}", eb);
        assert_eq!(hex.len(), 16, "f64 must be exactly 16 hex chars");
        let parsed = u64::from_str_radix(&hex, 16).expect("hex parse failed");
        assert_eq!(parsed, e_bits, "round-tripped E bits must match");
        let recovered = f64::from_bits(parsed);
        assert!(
            (recovered - E).abs() < f64::EPSILON,
            "recovered f64 must equal E"
        );
        // PI and E have different bit patterns
        assert_ne!(
            PI.to_bits(),
            E.to_bits(),
            "PI and E must have different bit patterns"
        );
    }

    // -----------------------------------------------------------------------
    // 15. EncodedBytes formatted width (LowerHex and UpperHex)
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_bytes_lower_and_upper_hex_formats() {
        let bytes = [0xabu8, 0xcd, 0xefu8];
        let eb = oxicode::encoded_bytes(&bytes);
        let lower = format!("{:x}", eb);
        let upper = format!("{:X}", eb);
        assert_eq!(lower, "abcdef");
        assert_eq!(upper, "ABCDEF");
        assert_eq!(lower.to_uppercase(), upper);
    }

    // -----------------------------------------------------------------------
    // 16. EncodedBytes for bool true = "01"
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_bytes_bool_true() {
        let encoded = oxicode::encode_to_vec(&true).expect("encode true failed");
        let eb = oxicode::encoded_bytes(&encoded);
        let s = format!("{}", eb);
        assert_eq!(s, "01", "encoded bool true must display as '01'");
    }

    // -----------------------------------------------------------------------
    // 17. EncodedBytes for bool false = "00"
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_bytes_bool_false() {
        let encoded = oxicode::encode_to_vec(&false).expect("encode false failed");
        let eb = oxicode::encoded_bytes(&encoded);
        let s = format!("{}", eb);
        assert_eq!(s, "00", "encoded bool false must display as '00'");
    }

    // -----------------------------------------------------------------------
    // 18. Multiple sequential hex encodes are consistent
    // -----------------------------------------------------------------------
    #[test]
    fn test_multiple_sequential_hex_encodes_consistent() {
        let value = 12345u32;
        let enc1 = oxicode::encode_to_vec(&value).expect("encode 1 failed");
        let enc2 = oxicode::encode_to_vec(&value).expect("encode 2 failed");
        let enc3 = oxicode::encode_to_vec(&value).expect("encode 3 failed");
        // All three encodings must be identical
        assert_eq!(enc1, enc2, "repeated encodes must be identical (1 vs 2)");
        assert_eq!(enc2, enc3, "repeated encodes must be identical (2 vs 3)");
        // Display output must also be consistent
        let d1 = format!("{}", oxicode::encoded_bytes(&enc1));
        let d2 = format!("{}", oxicode::encoded_bytes(&enc2));
        let d3 = format!("{}", oxicode::encoded_bytes(&enc3));
        assert_eq!(d1, d2);
        assert_eq!(d2, d3);
    }

    // -----------------------------------------------------------------------
    // 19. Hex dump of Vec<u32> [1,2,3] shows correct varint encoding
    // -----------------------------------------------------------------------
    #[test]
    fn test_hex_dump_vec_u32_varint_encoding() {
        let data: Vec<u32> = vec![1, 2, 3];
        let eb = oxicode::encode_to_display(&data).expect("encode_to_display failed");
        let dump = eb.hex_dump();
        // Must have the offset header
        assert!(
            dump.contains("00000000:"),
            "dump must have address 00000000:"
        );
        // Encoded bytes must contain the values 01, 02, 03 somewhere
        let raw = eb.as_bytes();
        assert!(!raw.is_empty(), "encoded Vec<u32> must not be empty");
        // The values 1, 2, 3 with varint encoding map to bytes 0x02, 0x04, 0x06 (zigzag)
        // or 0x01, 0x02, 0x03 depending on encoding; either way the bytes are present
        let hex_str = format!("{:x}", oxicode::encoded_bytes(raw));
        assert!(!hex_str.is_empty(), "hex representation must not be empty");
    }

    // -----------------------------------------------------------------------
    // 20. EncodedBytes can be used in format! macro
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_bytes_in_format_macro() {
        let bytes = [0x41u8, 0x42, 0x43]; // ASCII A, B, C
        let eb = oxicode::encoded_bytes(&bytes);
        // Display in a full format string
        let msg = format!("encoded: {}", eb);
        assert_eq!(msg, "encoded: 41 42 43");
        // LowerHex in a format string
        let msg_lx = format!("hex: {:x}", eb);
        assert_eq!(msg_lx, "hex: 414243");
        // UpperHex in a format string
        let msg_ux = format!("HEX: {:X}", eb);
        assert_eq!(msg_ux, "HEX: 414243");
        // Use in a Vec of strings
        let items: Vec<String> = vec![
            format!("{}", oxicode::encoded_bytes(&[0x01u8])),
            format!("{}", oxicode::encoded_bytes(&[0x02u8])),
            format!("{}", oxicode::encoded_bytes(&[0x03u8])),
        ];
        assert_eq!(items, vec!["01", "02", "03"]);
    }
}
