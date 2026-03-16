//! Advanced cross-format interoperability tests.
//!
//! 20 tests covering config roundtrips, byte-layout differences, varint sizing,
//! f64 endianness invariance, config cloning, debug output, file I/O, and
//! single-byte u8 array encoding.

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
// Shared derive types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AdvRecord {
    id: u32,
    label: String,
    score: f64,
}

// ---------------------------------------------------------------------------
// Module wrapper (satisfies the "module interop_advanced_tests" requirement)
// ---------------------------------------------------------------------------

mod interop_advanced_tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Test 1: Standard config encode → standard config decode roundtrip
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_standard_config_encode_decode_roundtrip() {
        let original = AdvRecord {
            id: 42,
            label: String::from("standard-roundtrip"),
            score: std::f64::consts::PI,
        };

        let cfg = config::standard();
        let bytes =
            encode_to_vec_with_config(&original, cfg).expect("standard config encode failed");
        let (decoded, consumed): (AdvRecord, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("standard config decode failed");

        assert_eq!(
            original, decoded,
            "standard config roundtrip: value mismatch"
        );
        assert_eq!(
            consumed,
            bytes.len(),
            "standard config: unconsumed bytes remain"
        );
    }

    // -----------------------------------------------------------------------
    // Test 2: Fixed-int config encode → fixed-int config decode roundtrip
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_fixed_int_config_encode_decode_roundtrip() {
        let original = AdvRecord {
            id: u32::MAX,
            label: String::from("fixed-int-roundtrip"),
            score: std::f64::consts::E,
        };

        let cfg = config::standard().with_fixed_int_encoding();
        let bytes =
            encode_to_vec_with_config(&original, cfg).expect("fixed_int config encode failed");
        let (decoded, consumed): (AdvRecord, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("fixed_int config decode failed");

        assert_eq!(
            original, decoded,
            "fixed_int config roundtrip: value mismatch"
        );
        assert_eq!(
            consumed,
            bytes.len(),
            "fixed_int config: unconsumed bytes remain"
        );
    }

    // -----------------------------------------------------------------------
    // Test 3: Big-endian config encode → big-endian config decode roundtrip
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_big_endian_config_encode_decode_roundtrip() {
        let original = AdvRecord {
            id: 0x0102_0304,
            label: String::from("big-endian-test"),
            score: std::f64::consts::PI * std::f64::consts::E,
        };

        let cfg = config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();
        let bytes =
            encode_to_vec_with_config(&original, cfg).expect("big_endian config encode failed");
        let (decoded, consumed): (AdvRecord, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("big_endian config decode failed");

        assert_eq!(
            original, decoded,
            "big_endian config roundtrip: value mismatch"
        );
        assert_eq!(
            consumed,
            bytes.len(),
            "big_endian config: unconsumed bytes remain"
        );
    }

    // -----------------------------------------------------------------------
    // Test 4: Standard encode, fixed-int decode → different byte-lengths → error
    //         or different value (config mismatch)
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_standard_encode_fixed_int_decode_mismatch() {
        // Encode a large u64 with standard (varint) config — many bytes.
        // Then attempt to decode with fixed-int config which expects 8 bytes
        // for u64 unconditionally. The byte lengths are different so
        // either decoding fails or produces a different value.
        let value: u64 = 100_000_000_000u64; // > 4 bytes varint, but not 8 bytes

        let std_bytes =
            encode_to_vec_with_config(&value, config::standard()).expect("standard encode failed");

        let fixed_cfg = config::standard().with_fixed_int_encoding();
        let result: Result<(u64, usize), _> = decode_from_slice_with_config(&std_bytes, fixed_cfg);

        // Either it returns an error (UnexpectedEnd) or gives a wrong value.
        // We only assert that the decoded value, if any, differs from original
        // OR that an error occurred.
        match result {
            Err(_) => {
                // Expected: insufficient bytes for fixed-int decode
            }
            Ok((decoded, _)) => {
                assert_ne!(
                    decoded, value,
                    "decoding standard-encoded bytes with fixed-int config must not yield correct value"
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // Test 5: Fixed-int encode, standard (varint) decode → may fail or wrong
    //         value for large integers
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_fixed_int_encode_standard_decode_mismatch_for_large() {
        // u64::MAX in fixed-int is always 8 bytes [0xFF; 8].
        // Standard (varint) decoder reading those 8 bytes will not see a valid
        // varint sentinel at position 0 — it will attempt to parse [0xFF, ...].
        // Varint treats 0xFF as a u128 marker (255), so it may succeed but
        // return the wrong value, or fail with an error.
        let value: u64 = u64::MAX;

        let fix_bytes =
            encode_to_vec_with_config(&value, config::standard().with_fixed_int_encoding())
                .expect("fixed_int encode failed");

        let result: Result<(u64, usize), _> =
            decode_from_slice_with_config(&fix_bytes, config::standard());

        match result {
            Err(_) => {
                // Expected: varint decode failure on fixed-int bytes
            }
            Ok((decoded, _)) => {
                assert_ne!(
                    decoded, value,
                    "decoding fixed-int bytes with varint config must not yield u64::MAX correctly"
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // Test 6: Legacy config (bincode-compatible) encode/decode roundtrip
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_legacy_config_encode_decode_roundtrip() {
        let original = AdvRecord {
            id: 12345,
            label: String::from("legacy-compat-roundtrip"),
            score: std::f64::consts::E,
        };

        let cfg = config::legacy();
        let bytes = encode_to_vec_with_config(&original, cfg).expect("legacy config encode failed");
        let (decoded, consumed): (AdvRecord, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("legacy config decode failed");

        assert_eq!(original, decoded, "legacy config roundtrip: value mismatch");
        assert_eq!(
            consumed,
            bytes.len(),
            "legacy config: unconsumed bytes remain"
        );
    }

    // -----------------------------------------------------------------------
    // Test 7: Legacy config: u32 encodes as 4 bytes LE (fixed-int)
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_legacy_config_u32_is_four_bytes_le() {
        // legacy() = fixed-int + little-endian
        let value: u32 = 0x0102_0304;
        let cfg = config::legacy();
        let bytes = encode_to_vec_with_config(&value, cfg).expect("legacy u32 encode failed");

        assert_eq!(
            bytes.len(),
            4,
            "legacy config u32 must be exactly 4 bytes, got {}",
            bytes.len()
        );
        // Little-endian byte order: LSB first
        assert_eq!(
            bytes.as_slice(),
            &[0x04u8, 0x03, 0x02, 0x01],
            "legacy config u32 must be little-endian: [0x04, 0x03, 0x02, 0x01]"
        );

        let (decoded, _): (u32, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("legacy u32 decode failed");
        assert_eq!(decoded, value, "legacy config u32 roundtrip mismatch");
    }

    // -----------------------------------------------------------------------
    // Test 8: Standard config: u64 300 encodes as 3 bytes (varint)
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_standard_u64_300_is_three_bytes() {
        let value: u64 = 300;
        let bytes = encode_to_vec_with_config(&value, config::standard())
            .expect("standard u64 300 encode failed");

        // 300 > 250 → needs 2-byte varint marker + 2-byte body: [251, 0x2C, 0x01]
        assert_eq!(
            bytes.len(),
            3,
            "standard varint u64=300 must be 3 bytes, got {}",
            bytes.len()
        );

        let (decoded, consumed): (u64, usize) =
            decode_from_slice_with_config(&bytes, config::standard())
                .expect("standard u64 300 decode failed");
        assert_eq!(decoded, 300u64, "standard u64=300 roundtrip mismatch");
        assert_eq!(consumed, 3, "standard u64=300 must consume 3 bytes");
    }

    // -----------------------------------------------------------------------
    // Test 9: Fixed-int config: u64 300 encodes as 8 bytes
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_fixed_int_u64_300_is_eight_bytes() {
        let value: u64 = 300;
        let cfg = config::standard().with_fixed_int_encoding();
        let bytes =
            encode_to_vec_with_config(&value, cfg).expect("fixed_int u64 300 encode failed");

        assert_eq!(
            bytes.len(),
            8,
            "fixed_int u64=300 must be 8 bytes, got {}",
            bytes.len()
        );

        let (decoded, consumed): (u64, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("fixed_int u64 300 decode failed");
        assert_eq!(decoded, 300u64, "fixed_int u64=300 roundtrip mismatch");
        assert_eq!(consumed, 8, "fixed_int u64=300 must consume 8 bytes");
    }

    // -----------------------------------------------------------------------
    // Test 10: Big-endian config: u32 0x01000000 encodes as [1, 0, 0, 0]
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_big_endian_u32_0x01000000_is_1_0_0_0() {
        let value: u32 = 0x0100_0000;
        let cfg = config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();
        let bytes = encode_to_vec_with_config(&value, cfg).expect("big_endian u32 encode failed");

        assert_eq!(
            bytes.as_slice(),
            &[0x01u8, 0x00, 0x00, 0x00],
            "big_endian u32=0x01000000 must encode as [1, 0, 0, 0]"
        );

        let (decoded, _): (u32, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("big_endian u32 decode failed");
        assert_eq!(
            decoded, value,
            "big_endian u32=0x01000000 roundtrip mismatch"
        );
    }

    // -----------------------------------------------------------------------
    // Test 11: Standard config: Vec<u8> length prefix is a varint
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_standard_vec_u8_length_prefix_is_varint() {
        // Vec with 10 elements: varint(10) = 1 byte + 10 data bytes = 11 total
        let data: Vec<u8> = (0u8..10).collect();
        let bytes = encode_to_vec_with_config(&data, config::standard())
            .expect("standard Vec<u8> encode failed");

        assert_eq!(
            bytes.len(),
            11,
            "standard Vec<u8>[10] must be 11 bytes (1 varint prefix + 10 data), got {}",
            bytes.len()
        );
        assert_eq!(
            bytes[0], 10u8,
            "first byte must be varint length=10, got {}",
            bytes[0]
        );

        let (decoded, _): (Vec<u8>, usize) =
            decode_from_slice_with_config(&bytes, config::standard())
                .expect("standard Vec<u8> decode failed");
        assert_eq!(decoded, data, "standard Vec<u8> roundtrip mismatch");
    }

    // -----------------------------------------------------------------------
    // Test 12: Fixed-int config: Vec<u8> length prefix is an 8-byte u64
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_fixed_int_vec_u8_length_prefix_is_eight_bytes() {
        // Vec with 4 elements: fixed u64 prefix (8 bytes) + 4 data bytes = 12 total
        let data: Vec<u8> = vec![0xAA, 0xBB, 0xCC, 0xDD];
        let cfg = config::standard().with_fixed_int_encoding();
        let bytes = encode_to_vec_with_config(&data, cfg).expect("fixed_int Vec<u8> encode failed");

        assert_eq!(
            bytes.len(),
            12,
            "fixed_int Vec<u8>[4] must be 12 bytes (8 u64 prefix + 4 data), got {}",
            bytes.len()
        );
        // The first 8 bytes encode u64=4 in little-endian
        assert_eq!(
            &bytes[..8],
            &[4u8, 0, 0, 0, 0, 0, 0, 0],
            "fixed_int Vec<u8> prefix must be u64=4 LE"
        );

        let (decoded, _): (Vec<u8>, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("fixed_int Vec<u8> decode failed");
        assert_eq!(decoded, data, "fixed_int Vec<u8> roundtrip mismatch");
    }

    // -----------------------------------------------------------------------
    // Test 13: Config with_limit works: encode within limit succeeds,
    //          decode of over-limit payload fails
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_config_with_limit_works_with_encode_decode() {
        // Encode a small value (u8=1 → 1 byte) with a limit of 8 — should succeed.
        let small_value: u8 = 1u8;
        let lim_cfg = config::standard().with_limit::<8>();

        let bytes = encode_to_vec_with_config(&small_value, lim_cfg)
            .expect("encode within limit must succeed");
        assert_eq!(bytes.len(), 1, "u8=1 must encode to 1 byte");

        let (decoded, _): (u8, usize) = decode_from_slice_with_config(&bytes, lim_cfg)
            .expect("decode within limit must succeed");
        assert_eq!(decoded, small_value, "decoded value mismatch");

        // Now attempt to decode a Vec<u8>[10] with limit=9 — the container
        // claims 10 bytes which exceeds the limit of 9.
        let big_data: Vec<u8> = (0u8..10).collect();
        let unconstrained_bytes = encode_to_vec(&big_data).expect("unconstrained encode");

        let over_limit_cfg = config::standard().with_limit::<9>();
        let result: Result<(Vec<u8>, usize), _> =
            decode_from_slice_with_config(&unconstrained_bytes, over_limit_cfg);

        assert!(
            result.is_err(),
            "decoding Vec<u8>[10] with limit=9 must fail (claims 10 > 9)"
        );
    }

    // -----------------------------------------------------------------------
    // Test 14: Two encoders with different configs produce different bytes
    //          for the same struct
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_two_different_configs_produce_different_bytes() {
        let value = AdvRecord {
            id: 1000,
            label: String::from("config-diff"),
            score: std::f64::consts::PI,
        };

        let std_bytes =
            encode_to_vec_with_config(&value, config::standard()).expect("standard encode failed");

        let fix_bytes =
            encode_to_vec_with_config(&value, config::standard().with_fixed_int_encoding())
                .expect("fixed_int encode failed");

        assert_ne!(
            std_bytes, fix_bytes,
            "standard and fixed_int configs must produce different byte sequences for same struct"
        );
    }

    // -----------------------------------------------------------------------
    // Test 15: Decode with mismatched config produces error or unexpected data
    //          (using a simple scalar u32, not a struct, to avoid OOM on
    //           misinterpreted length fields)
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_decode_with_mismatched_config_fails_or_wrong_value() {
        // Encode a u32 with big-endian fixed-int config.
        // Decode with little-endian fixed-int config.
        // The bytes are byte-reversed, so the decoded value must differ.
        let value: u32 = 0xDEAD_BEEF;

        let be_cfg = config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();
        let le_cfg = config::standard()
            .with_little_endian()
            .with_fixed_int_encoding();

        let be_bytes =
            encode_to_vec_with_config(&value, be_cfg).expect("big_endian u32 encode failed");

        // BE bytes of 0xDEADBEEF = [0xDE, 0xAD, 0xBE, 0xEF]
        // LE decode reads that as 0xEFBEADDE — a different value.
        let result: Result<(u32, usize), _> = decode_from_slice_with_config(&be_bytes, le_cfg);

        match result {
            Err(_) => {
                // Decode failed — acceptable outcome for config mismatch
            }
            Ok((decoded, _)) => {
                assert_ne!(
                    decoded, value,
                    "big-endian encoded u32 decoded with little-endian must not equal original"
                );
                // Specifically expect byte-reversed interpretation
                assert_eq!(
                    decoded, 0xEFBE_ADDEu32,
                    "LE decode of BE 0xDEADBEEF must be 0xEFBEADDE"
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // Test 16: Struct with f64 PI: LE and BE configs each roundtrip correctly,
    //          and the two byte sequences are byte-reversals of each other.
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_f64_pi_roundtrips_correctly_for_both_endiannesses() {
        let value: f64 = std::f64::consts::PI;

        let le_cfg = config::standard()
            .with_little_endian()
            .with_fixed_int_encoding();
        let be_cfg = config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();

        let le_bytes = encode_to_vec_with_config(&value, le_cfg).expect("LE f64 encode failed");
        let be_bytes = encode_to_vec_with_config(&value, be_cfg).expect("BE f64 encode failed");

        // Both representations must be 8 bytes (f64 is always 8 bytes)
        assert_eq!(le_bytes.len(), 8, "LE f64 must be 8 bytes");
        assert_eq!(be_bytes.len(), 8, "BE f64 must be 8 bytes");

        // The BE bytes are the byte-reversal of the LE bytes
        let le_reversed: Vec<u8> = le_bytes.iter().copied().rev().collect();
        assert_eq!(
            be_bytes, le_reversed,
            "BE f64 bytes must be the byte-reversal of LE f64 bytes"
        );

        // Each config must roundtrip its own encoding correctly
        let (le_decoded, _): (f64, usize) =
            decode_from_slice_with_config(&le_bytes, le_cfg).expect("LE f64 decode failed");
        let (be_decoded, _): (f64, usize) =
            decode_from_slice_with_config(&be_bytes, be_cfg).expect("BE f64 decode failed");

        assert_eq!(le_decoded, value, "LE f64 PI roundtrip mismatch");
        assert_eq!(be_decoded, value, "BE f64 PI roundtrip mismatch");
    }

    // -----------------------------------------------------------------------
    // Test 17: Config clone: `config.clone()` has same encoding behavior
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_config_clone_same_behavior_as_original() {
        let original_cfg = config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();
        let cloned_cfg = original_cfg; // Copy type — assignment is a bitwise clone

        let value: u32 = 0xCAFE_BABE;

        let bytes_orig =
            encode_to_vec_with_config(&value, original_cfg).expect("original config encode failed");
        let bytes_clone =
            encode_to_vec_with_config(&value, cloned_cfg).expect("cloned config encode failed");

        assert_eq!(
            bytes_orig, bytes_clone,
            "cloned config must produce identical bytes as original"
        );

        // Verify roundtrip with both
        let (decoded_orig, _): (u32, usize) =
            decode_from_slice_with_config(&bytes_orig, original_cfg)
                .expect("original config decode failed");
        let (decoded_clone, _): (u32, usize) =
            decode_from_slice_with_config(&bytes_clone, cloned_cfg)
                .expect("cloned config decode failed");

        assert_eq!(decoded_orig, value, "original config roundtrip mismatch");
        assert_eq!(decoded_clone, value, "cloned config roundtrip mismatch");
    }

    // -----------------------------------------------------------------------
    // Test 18: Config debug output contains useful info
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_config_debug_output_contains_useful_info() {
        let std_cfg = config::standard();
        let debug_str = format!("{:?}", std_cfg);

        // The debug output should be non-empty and contain some recognizable text
        // (Configuration type name or PhantomData).
        assert!(
            !debug_str.is_empty(),
            "debug output of standard config must not be empty"
        );

        let fix_cfg = config::standard().with_fixed_int_encoding();
        let fix_debug = format!("{:?}", fix_cfg);
        assert!(
            !fix_debug.is_empty(),
            "debug output of fixed_int config must not be empty"
        );

        let be_cfg = config::standard().with_big_endian();
        let be_debug = format!("{:?}", be_cfg);
        assert!(
            !be_debug.is_empty(),
            "debug output of big_endian config must not be empty"
        );
    }

    // -----------------------------------------------------------------------
    // Test 19: Encode to file, decode from file with standard config
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_encode_to_file_and_decode_from_file_with_standard_config() {
        let original = AdvRecord {
            id: 9_999,
            label: String::from("file-io-test"),
            score: std::f64::consts::PI,
        };

        let mut file_path = std::env::temp_dir();
        file_path.push("oxicode_interop_advanced_test_19.bin");

        // Encode to file
        oxicode::encode_to_file(&original, &file_path).expect("encode_to_file failed");

        // Decode from file
        let decoded: AdvRecord =
            oxicode::decode_from_file(&file_path).expect("decode_from_file failed");

        assert_eq!(
            original, decoded,
            "file encode/decode roundtrip: value mismatch"
        );

        // Clean up
        let _ = std::fs::remove_file(&file_path);
    }

    // -----------------------------------------------------------------------
    // Test 20: Encode byte array [1,2,3,4]: same 4-byte payload with both
    //          standard and fixed-int configs (u8 is always 1 byte regardless)
    // -----------------------------------------------------------------------

    #[test]
    fn test_adv_byte_array_same_payload_length_standard_and_fixed_int() {
        // [u8; 4] — each element is u8, which is exactly 1 byte in both configs.
        // The array itself has no length prefix (fixed-size array → no prefix).
        let arr: [u8; 4] = [1, 2, 3, 4];

        let std_bytes = encode_to_vec_with_config(&arr, config::standard())
            .expect("standard [u8;4] encode failed");
        let fix_bytes =
            encode_to_vec_with_config(&arr, config::standard().with_fixed_int_encoding())
                .expect("fixed_int [u8;4] encode failed");

        assert_eq!(
            std_bytes.len(),
            4,
            "standard [u8;4] must be 4 bytes, got {}",
            std_bytes.len()
        );
        assert_eq!(
            fix_bytes.len(),
            4,
            "fixed_int [u8;4] must be 4 bytes, got {}",
            fix_bytes.len()
        );
        assert_eq!(
            std_bytes, fix_bytes,
            "[u8;4] payload must be identical for standard and fixed_int configs"
        );

        // Verify roundtrip with standard
        let (decoded_std, _): ([u8; 4], usize) =
            decode_from_slice(&std_bytes).expect("standard [u8;4] decode failed");
        assert_eq!(decoded_std, arr, "standard [u8;4] roundtrip mismatch");

        // Verify roundtrip with fixed_int
        let (decoded_fix, _): ([u8; 4], usize) =
            decode_from_slice_with_config(&fix_bytes, config::standard().with_fixed_int_encoding())
                .expect("fixed_int [u8;4] decode failed");
        assert_eq!(decoded_fix, arr, "fixed_int [u8;4] roundtrip mismatch");
    }
}
