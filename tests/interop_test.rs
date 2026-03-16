//! Cross-format compatibility and interoperability tests.
//!
//! 20 tests covering config-level roundtrips, byte-level differences,
//! varint sizing, borrow-decode, encode_into_slice, encoded_size, and
//! byte-limit enforcement.

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
use oxicode::{config, decode_from_slice, encode_into_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Shared derive types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SmallRecord {
    id: u32,
    tag: u8,
    flag: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LargeRecord {
    id: u64,
    name: String,
    values: Vec<i32>,
    nested: SmallRecord,
    score: u16,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NestedCollections {
    matrix: Vec<Vec<u32>>,
    tags: Vec<String>,
    counts: Vec<u64>,
}

// ---------------------------------------------------------------------------
// Test 1: Encode with standard config, decode with standard config → roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_interop_standard_config_roundtrip() {
    let original = LargeRecord {
        id: 999_999,
        name: String::from("interop-standard"),
        values: vec![-10, 0, 10, 255, i32::MAX],
        nested: SmallRecord {
            id: 7,
            tag: 0xAB,
            flag: true,
        },
        score: 1234,
        active: false,
    };

    let cfg = config::standard();
    let bytes =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("standard config encode failed");
    let (decoded, consumed): (LargeRecord, usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("standard config decode failed");

    assert_eq!(
        original, decoded,
        "standard config roundtrip: value mismatch"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "standard config roundtrip: unconsumed bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 2: Encode with fixed_int config, decode with fixed_int config → roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_interop_fixed_int_config_roundtrip() {
    let original = SmallRecord {
        id: 65_535,
        tag: 255,
        flag: false,
    };

    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("fixed_int encode failed");
    let (decoded, consumed): (SmallRecord, usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("fixed_int decode failed");

    assert_eq!(
        original, decoded,
        "fixed_int config roundtrip: value mismatch"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "fixed_int config roundtrip: unconsumed bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 3: Encode with big_endian config, decode with big_endian config → roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_interop_big_endian_config_roundtrip() {
    let original: u64 = 0x0102_0304_0506_0708;
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let bytes =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("big_endian encode failed");
    let (decoded, consumed): (u64, usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("big_endian decode failed");

    assert_eq!(
        original, decoded,
        "big_endian config roundtrip: value mismatch"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "big_endian config roundtrip: unconsumed bytes"
    );
    // Byte order must be MSB-first
    assert_eq!(
        bytes.as_slice(),
        &[0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        "big_endian must serialize MSB-first"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Encode with legacy config (bincode compat), decode with legacy → roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_interop_legacy_config_roundtrip() {
    let original = LargeRecord {
        id: 0xDEAD_BEEF_CAFE_0001,
        name: String::from("legacy-compat"),
        values: vec![1, -1, 0, i32::MIN, i32::MAX],
        nested: SmallRecord {
            id: 42,
            tag: 0x00,
            flag: true,
        },
        score: u16::MAX,
        active: true,
    };

    let cfg = config::legacy();
    let bytes = oxicode::encode_to_vec_with_config(&original, cfg).expect("legacy encode failed");
    let (decoded, consumed): (LargeRecord, usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("legacy decode failed");

    assert_eq!(original, decoded, "legacy config roundtrip: value mismatch");
    assert_eq!(
        consumed,
        bytes.len(),
        "legacy config roundtrip: unconsumed bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 5: Standard and fixed_int configs produce different bytes for the same multi-byte int
// ---------------------------------------------------------------------------

#[test]
fn test_interop_standard_and_fixed_int_produce_different_bytes() {
    // For small values (< 251) standard varint is 1 byte, but fixed_int is still
    // the full width.  For u32 that's 4 bytes vs 1 byte — clearly different.
    let value: u32 = 42;
    let std_bytes = encode_to_vec(&value).expect("standard encode failed");
    let fix_bytes =
        oxicode::encode_to_vec_with_config(&value, config::standard().with_fixed_int_encoding())
            .expect("fixed_int encode failed");

    assert_ne!(
        std_bytes, fix_bytes,
        "standard and fixed_int configs must produce different byte sequences"
    );
    assert_eq!(std_bytes.len(), 1, "varint 42 must be 1 byte");
    assert_eq!(fix_bytes.len(), 4, "fixed_int u32 must be 4 bytes");
}

// ---------------------------------------------------------------------------
// Test 6: Little-endian and big-endian configs produce different bytes for multi-byte ints
// ---------------------------------------------------------------------------

#[test]
fn test_interop_endianness_configs_produce_different_bytes() {
    let value: u32 = 0x0102_0304; // distinct bytes in each position

    let le_bytes = oxicode::encode_to_vec_with_config(
        &value,
        config::standard()
            .with_little_endian()
            .with_fixed_int_encoding(),
    )
    .expect("LE encode failed");

    let be_bytes = oxicode::encode_to_vec_with_config(
        &value,
        config::standard()
            .with_big_endian()
            .with_fixed_int_encoding(),
    )
    .expect("BE encode failed");

    assert_ne!(
        le_bytes, be_bytes,
        "little-endian and big-endian must produce different byte sequences"
    );
    assert_eq!(
        le_bytes.as_slice(),
        &[0x04u8, 0x03, 0x02, 0x01],
        "LE must be LSB-first"
    );
    assert_eq!(
        be_bytes.as_slice(),
        &[0x01u8, 0x02, 0x03, 0x04],
        "BE must be MSB-first"
    );
}

// ---------------------------------------------------------------------------
// Test 7: Standard config u16 varint — small values (< 251) are 1 byte
// ---------------------------------------------------------------------------

#[test]
fn test_interop_standard_u16_small_values_are_one_byte() {
    let cfg = config::standard();
    for v in [0u16, 1, 100, 200, 250] {
        let bytes = oxicode::encode_to_vec_with_config(&v, cfg).expect("u16 small encode failed");
        assert_eq!(
            bytes.len(),
            1,
            "standard varint u16={v} (< 251) must be 1 byte, got {}",
            bytes.len()
        );
        let (decoded, _): (u16, usize) =
            oxicode::decode_from_slice_with_config(&bytes, cfg).expect("u16 small decode failed");
        assert_eq!(decoded, v, "roundtrip failed for u16={v}");
    }
}

// ---------------------------------------------------------------------------
// Test 8: Fixed int config u16 always 2 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_interop_fixed_int_u16_always_two_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for v in [0u16, 1, 100, 250, 251, 1000, u16::MAX] {
        let bytes = oxicode::encode_to_vec_with_config(&v, cfg).expect("fixed u16 encode failed");
        assert_eq!(
            bytes.len(),
            2,
            "fixed_int u16={v} must always be 2 bytes, got {}",
            bytes.len()
        );
        let (decoded, consumed): (u16, usize) =
            oxicode::decode_from_slice_with_config(&bytes, cfg).expect("fixed u16 decode failed");
        assert_eq!(decoded, v, "fixed_int u16 roundtrip failed for v={v}");
        assert_eq!(consumed, 2, "fixed_int u16 must consume exactly 2 bytes");
    }
}

// ---------------------------------------------------------------------------
// Test 9: Standard config Vec<u8> with small length has 1-byte length prefix
// ---------------------------------------------------------------------------

#[test]
fn test_interop_standard_small_vec_has_one_byte_length_prefix() {
    // Vec with 10 elements: varint(10) = 1 byte + 10 payload bytes = 11 total
    let payload: Vec<u8> = (1u8..=10).collect();
    let bytes = encode_to_vec(&payload).expect("vec encode failed");

    assert_eq!(
        bytes.len(),
        11,
        "Vec<u8> len=10 with standard config: expected 11 bytes (1 varint prefix + 10 data), got {}",
        bytes.len()
    );
    assert_eq!(bytes[0], 10u8, "first byte must be the varint length=10");

    let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&bytes).expect("vec decode failed");
    assert_eq!(decoded, payload, "Vec<u8> roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 10: Encode struct with standard config, verify exact byte layout
// ---------------------------------------------------------------------------

#[test]
fn test_interop_struct_exact_byte_layout() {
    // SmallRecord { id: 1, tag: 2, flag: true }
    // standard varint: id=1 → [1], tag=2 → [2], flag=true → [1]
    // Expected: [1, 2, 1]
    let record = SmallRecord {
        id: 1,
        tag: 2,
        flag: true,
    };
    let bytes = encode_to_vec(&record).expect("struct encode failed");
    assert_eq!(
        bytes,
        vec![1u8, 2u8, 1u8],
        "SmallRecord{{id:1,tag:2,flag:true}} must encode as [1, 2, 1]"
    );

    let (decoded, consumed): (SmallRecord, usize) =
        decode_from_slice(&bytes).expect("struct decode failed");
    assert_eq!(decoded, record, "struct exact layout roundtrip mismatch");
    assert_eq!(consumed, 3, "struct must consume exactly 3 bytes");
}

// ---------------------------------------------------------------------------
// Test 11: Decode from manually constructed bytes using known u64 varint format
// ---------------------------------------------------------------------------

#[test]
fn test_interop_manual_bytes_u64_varint_decode() {
    // u64 = 42 encodes as single byte [42] in standard varint
    let manual_bytes: &[u8] = &[42u8];
    let (decoded, consumed): (u64, usize) =
        decode_from_slice(manual_bytes).expect("manual u64 decode failed");
    assert_eq!(decoded, 42u64, "u64 from manual [42] must be 42");
    assert_eq!(consumed, 1, "single-byte varint must consume 1 byte");

    // u64 = 1000 encodes as [251, 0xE8, 0x03] (251 = u16 varint marker, 1000 = 0x03E8 LE)
    let manual_1000: &[u8] = &[251u8, 0xE8, 0x03];
    let (decoded_1000, consumed_1000): (u64, usize) =
        decode_from_slice(manual_1000).expect("manual u64 1000 decode failed");
    assert_eq!(
        decoded_1000, 1000u64,
        "u64 from manual [251,0xE8,0x03] must be 1000"
    );
    assert_eq!(consumed_1000, 3, "three-byte varint must consume 3 bytes");
}

// ---------------------------------------------------------------------------
// Test 12: BorrowDecode from slice — zero-copy string borrow
// ---------------------------------------------------------------------------

#[test]
fn test_interop_borrow_decode_zero_copy_str() {
    let original = String::from("zero-copy borrow decode test");
    let encoded = encode_to_vec(&original).expect("str encode failed");

    // borrow_decode_from_slice returns &str borrowing directly from `encoded`
    let (borrowed, consumed): (&str, usize) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode_from_slice failed");

    assert_eq!(
        borrowed,
        original.as_str(),
        "borrow-decoded &str must equal original"
    );
    assert_eq!(consumed, encoded.len(), "all bytes must be consumed");
}

// ---------------------------------------------------------------------------
// Test 13: encode_to_vec + decode_from_slice roundtrip for &[u8] (byte slice)
// ---------------------------------------------------------------------------

#[test]
fn test_interop_byte_slice_roundtrip() {
    let original: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF, 0x42];
    let encoded = encode_to_vec(&original).expect("byte slice encode failed");
    let (decoded, consumed): (Vec<u8>, usize) =
        decode_from_slice(&encoded).expect("byte slice decode failed");

    assert_eq!(decoded, original, "byte slice roundtrip: value mismatch");
    assert_eq!(
        consumed,
        encoded.len(),
        "byte slice roundtrip: unconsumed bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Encode with encode_into_slice, verify exact slice length used
// ---------------------------------------------------------------------------

#[test]
fn test_interop_encode_into_slice_exact_length() {
    let value: u32 = 250; // varint: 1 byte (250 < 251)
    let mut buf = [0u8; 32];
    let written =
        encode_into_slice(value, &mut buf, config::standard()).expect("encode_into_slice failed");

    assert_eq!(written, 1, "u32=250 varint must write exactly 1 byte");
    assert_eq!(buf[0], 250u8, "first byte must be 250");

    // Verify that the rest of the buffer is untouched (still zeroed)
    for &b in &buf[1..] {
        assert_eq!(b, 0u8, "bytes beyond written region must be zero");
    }

    // Roundtrip via the written region
    let (decoded, _): (u32, usize) =
        decode_from_slice(&buf[..written]).expect("decode from slice written region failed");
    assert_eq!(decoded, 250u32, "roundtrip via encode_into_slice mismatch");
}

// ---------------------------------------------------------------------------
// Test 15: encoded_size matches encode_to_vec length for various types
// ---------------------------------------------------------------------------

#[test]
fn test_interop_encoded_size_matches_vec_length() {
    macro_rules! check_size {
        ($val:expr, $label:expr) => {{
            let bytes = encode_to_vec(&$val).expect(concat!("encode_to_vec failed: ", $label));
            let size =
                oxicode::encoded_size(&$val).expect(concat!("encoded_size failed: ", $label));
            assert_eq!(
                size,
                bytes.len(),
                "encoded_size mismatch for {}: size={} vec_len={}",
                $label,
                size,
                bytes.len()
            );
        }};
    }

    check_size!(0u8, "u8=0");
    check_size!(255u8, "u8=255");
    check_size!(0u16, "u16=0");
    check_size!(1000u16, "u16=1000");
    check_size!(0u32, "u32=0");
    check_size!(u32::MAX, "u32::MAX");
    check_size!(0u64, "u64=0");
    check_size!(u64::MAX, "u64::MAX");
    check_size!(String::from("hello world"), "String");
    check_size!(vec![1u8, 2, 3, 4, 5], "Vec<u8>");
    check_size!(
        SmallRecord {
            id: 100,
            tag: 7,
            flag: true
        },
        "SmallRecord"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Decode with byte limit that is exactly the encoded size succeeds
// ---------------------------------------------------------------------------

#[test]
fn test_interop_byte_limit_exactly_encoded_size_succeeds() {
    // Encode a u32 without a limit to get reference bytes
    let value: u32 = 99;
    let reference_bytes = oxicode::encode_to_vec_with_config(&value, config::standard())
        .expect("reference encode failed");

    // reference_bytes.len() is 1 (varint 99 < 251).
    // Use with_limit::<1> — exactly the encoded size.
    let limit_cfg = config::standard().with_limit::<1>();
    let result: Result<(u32, usize), _> =
        oxicode::decode_from_slice_with_config(&reference_bytes, limit_cfg);

    assert!(
        result.is_ok(),
        "decode with limit == encoded_size must succeed; got err={:?}",
        result.err()
    );
    let (decoded, _) = result.expect("decode with exact limit failed");
    assert_eq!(
        decoded, value,
        "value mismatch after decode with exact limit"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Decode with byte limit 1 less than the claimed payload size fails
// ---------------------------------------------------------------------------

#[test]
fn test_interop_byte_limit_one_less_than_encoded_size_fails() {
    // Encode a Vec<u8> of 10 elements without a limit.
    // standard encode: 1-byte varint length + 10 bytes payload = 11 bytes.
    // The limit is enforced via claim_container_read which claims size_of::<u8>() * 10 = 10 bytes.
    // with_limit::<9>: claiming 10 bytes against a limit of 9 → 10 > 9 → LimitExceeded.
    let data: Vec<u8> = (1u8..=10).collect();
    let reference_bytes = encode_to_vec(&data).expect("reference encode failed");

    assert_eq!(
        reference_bytes.len(),
        11,
        "expected 11 encoded bytes for 10-element Vec<u8>"
    );

    // Limit = 9, claimed = 10 → must fail.
    let limit_cfg = config::standard().with_limit::<9>();
    let result: Result<(Vec<u8>, usize), _> =
        oxicode::decode_from_slice_with_config(&reference_bytes, limit_cfg);

    assert!(
        result.is_err(),
        "decode with limit (9) one less than claimed payload bytes (10) must fail"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Large struct encode→decode preserves all fields exactly
// ---------------------------------------------------------------------------

#[test]
fn test_interop_large_struct_roundtrip_all_fields() {
    let original = LargeRecord {
        id: u64::MAX,
        name: String::from("large-struct-test-field-preservation"),
        values: (-500i32..=500).collect(),
        nested: SmallRecord {
            id: u32::MAX,
            tag: 0xFF,
            flag: false,
        },
        score: 0xBEEF,
        active: true,
    };

    let bytes = encode_to_vec(&original).expect("large struct encode failed");
    let (decoded, consumed): (LargeRecord, usize) =
        decode_from_slice(&bytes).expect("large struct decode failed");

    assert_eq!(original.id, decoded.id, "id field mismatch");
    assert_eq!(original.name, decoded.name, "name field mismatch");
    assert_eq!(original.values, decoded.values, "values field mismatch");
    assert_eq!(original.nested, decoded.nested, "nested field mismatch");
    assert_eq!(original.score, decoded.score, "score field mismatch");
    assert_eq!(original.active, decoded.active, "active field mismatch");
    assert_eq!(consumed, bytes.len(), "all bytes must be consumed");
}

// ---------------------------------------------------------------------------
// Test 19: Nested collections roundtrip with default config
// ---------------------------------------------------------------------------

#[test]
fn test_interop_nested_collections_roundtrip() {
    let original = NestedCollections {
        matrix: vec![
            vec![1u32, 2, 3],
            vec![],
            vec![u32::MAX, 0, 42],
            vec![100, 200, 300, 400],
        ],
        tags: vec![
            String::from("alpha"),
            String::from("beta"),
            String::new(),
            String::from("delta"),
        ],
        counts: vec![0u64, 1, 250, 251, 65535, u64::MAX],
    };

    let bytes = encode_to_vec(&original).expect("nested collections encode failed");
    let (decoded, consumed): (NestedCollections, usize) =
        decode_from_slice(&bytes).expect("nested collections decode failed");

    assert_eq!(original.matrix, decoded.matrix, "matrix field mismatch");
    assert_eq!(original.tags, decoded.tags, "tags field mismatch");
    assert_eq!(original.counts, decoded.counts, "counts field mismatch");
    assert_eq!(consumed, bytes.len(), "all bytes must be consumed");
}

// ---------------------------------------------------------------------------
// Test 20: Same data encoded twice produces identical bytes (determinism)
// ---------------------------------------------------------------------------

#[test]
fn test_interop_encoding_is_deterministic() {
    let value = LargeRecord {
        id: 123_456_789,
        name: String::from("determinism-check"),
        values: vec![-1, 0, 1, 100, -100],
        nested: SmallRecord {
            id: 55,
            tag: 0xAB,
            flag: true,
        },
        score: 9999,
        active: false,
    };

    let first = encode_to_vec(&value).expect("first encode failed");
    let second = encode_to_vec(&value).expect("second encode failed");

    assert_eq!(
        first, second,
        "encoding must be deterministic: two encodes of the same value must produce identical bytes"
    );

    // Also verify with non-default config
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let first_cfg =
        oxicode::encode_to_vec_with_config(&value, cfg).expect("first cfg encode failed");
    let second_cfg =
        oxicode::encode_to_vec_with_config(&value, cfg).expect("second cfg encode failed");
    assert_eq!(
        first_cfg, second_cfg,
        "big-endian fixed-int encoding must also be deterministic"
    );
}
