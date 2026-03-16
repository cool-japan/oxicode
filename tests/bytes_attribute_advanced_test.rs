//! Advanced tests for the `#[oxicode(bytes)]` field attribute on `Vec<u8>`.
//!
//! The attribute encodes a `Vec<u8>` as a length-prefixed raw byte sequence,
//! which is identical on the wire to the default `Vec<u8>` encoding.  These
//! tests cover roundtrips under various configurations, edge-case payloads,
//! nested / compound structs, and multi-field structs.

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
// Helper structs
// ---------------------------------------------------------------------------

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct SimpleBytes {
    #[oxicode(bytes)]
    data: Vec<u8>,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct BytesWithFields {
    id: u32,
    #[oxicode(bytes)]
    payload: Vec<u8>,
    label: String,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct MultiBytesFields {
    #[oxicode(bytes)]
    header: Vec<u8>,
    version: u8,
    #[oxicode(bytes)]
    body: Vec<u8>,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct InnerStruct {
    value: u64,
    #[oxicode(bytes)]
    checksum: Vec<u8>,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct OuterStruct {
    name: String,
    inner: InnerStruct,
}

// ---------------------------------------------------------------------------
// 1. Basic roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_basic_roundtrip() {
    let original = SimpleBytes {
        data: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (SimpleBytes, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 2. Empty Vec<u8> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_empty_vec_roundtrip() {
    let original = SimpleBytes { data: vec![] };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (SimpleBytes, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 3. 1000 bytes roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_1000_bytes_roundtrip() {
    let data: Vec<u8> = (0u16..1000).map(|i| (i % 256) as u8).collect();
    let original = SimpleBytes { data };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (SimpleBytes, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 4. Size comparison: bytes attr vs default Vec<u8> encoding
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_vs_default_encoding_size_equal() {
    // #[oxicode(bytes)] produces identical wire format to the default Vec<u8>
    // encoding: a varint length prefix followed by raw bytes.

    #[derive(Encode, Decode, Debug, PartialEq)]
    struct DefaultVec {
        data: Vec<u8>,
    }

    let payload: Vec<u8> = (0..64).collect();

    let with_attr = SimpleBytes {
        data: payload.clone(),
    };
    let without_attr = DefaultVec { data: payload };

    let enc_attr = encode_to_vec(&with_attr).expect("encode attr");
    let enc_default = encode_to_vec(&without_attr).expect("encode default");

    assert_eq!(
        enc_attr.len(),
        enc_default.len(),
        "#[oxicode(bytes)] and default Vec<u8> must produce the same wire size"
    );
    assert_eq!(enc_attr, enc_default, "wire bytes must be identical");
}

// ---------------------------------------------------------------------------
// 5. Struct with bytes field and other fields
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_struct_with_other_fields() {
    let original = BytesWithFields {
        id: 42,
        payload: vec![1, 2, 3, 4, 5],
        label: String::from("hello"),
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BytesWithFields, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 6. All-zeros bytes roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_all_zeros_roundtrip() {
    let original = SimpleBytes {
        data: vec![0x00; 128],
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (SimpleBytes, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 7. All-0xFF bytes roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_all_ff_roundtrip() {
    let original = SimpleBytes {
        data: vec![0xFF; 128],
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (SimpleBytes, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 8. Pseudo-random bytes roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_random_ish_bytes_roundtrip() {
    // Deterministic LFSR-style pattern — no rand dependency.
    let mut state: u32 = 0xACE1_ACEB;
    let data: Vec<u8> = (0..256)
        .map(|_| {
            // Xorshift32
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            (state & 0xFF) as u8
        })
        .collect();
    let original = SimpleBytes { data };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (SimpleBytes, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 9. Consumed bytes count is correct
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_decoded_bytes_count_correct() {
    let original = SimpleBytes {
        data: vec![10, 20, 30, 40, 50],
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (_, bytes_read): (SimpleBytes, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(
        bytes_read,
        encoded.len(),
        "all encoded bytes should be consumed"
    );
}

// ---------------------------------------------------------------------------
// 10. With fixed-int encoding config
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_with_fixed_int_encoding() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = BytesWithFields {
        id: 7,
        payload: vec![0xAA, 0xBB, 0xCC],
        label: String::from("fixed"),
    };
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode");
    let (decoded, _): (BytesWithFields, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 11. With big-endian config
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_with_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let original = BytesWithFields {
        id: 0x0102_0304,
        payload: vec![0x01, 0x02, 0x03],
        label: String::from("be"),
    };
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode");
    let (decoded, _): (BytesWithFields, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 12. Multiple #[oxicode(bytes)] fields in same struct
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_multiple_fields_same_struct() {
    let original = MultiBytesFields {
        header: vec![0xFF, 0xFE, 0xFD],
        version: 2,
        body: vec![0x00, 0x01, 0x02, 0x03],
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (MultiBytesFields, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 13. 16-byte (UUID-like) payload
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_16_bytes_uuid_like() {
    let original = SimpleBytes {
        data: vec![
            0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4,
            0x30, 0xc8,
        ],
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (SimpleBytes, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 14. 32-byte (hash-like) payload
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_32_bytes_hash_like() {
    let data: Vec<u8> = (0u8..32)
        .map(|i| i.wrapping_mul(7).wrapping_add(3))
        .collect();
    let original = SimpleBytes { data };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (SimpleBytes, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 15. 64-byte payload
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_64_bytes_roundtrip() {
    let data: Vec<u8> = (0u8..64).collect();
    let original = SimpleBytes { data };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (SimpleBytes, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 16. Bytes field inside a nested struct
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_nested_struct() {
    let original = OuterStruct {
        name: String::from("outer"),
        inner: InnerStruct {
            value: 0xDEAD_BEEF_CAFE_BABE,
            checksum: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        },
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (OuterStruct, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 17. bytes attr produces same-size output as normal Vec<u8> (compactness check)
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_size_identical_to_normal_vec_u8() {
    #[derive(Encode, Decode, Debug, PartialEq)]
    struct NormalVecStruct {
        id: u32,
        data: Vec<u8>,
        label: String,
    }

    let payload: Vec<u8> = vec![0xAB; 100];

    let with_attr = BytesWithFields {
        id: 1,
        payload: payload.clone(),
        label: String::from("attr"),
    };
    let without_attr = NormalVecStruct {
        id: 1,
        data: payload,
        label: String::from("attr"),
    };

    let enc_attr = encode_to_vec(&with_attr).expect("encode attr");
    let enc_normal = encode_to_vec(&without_attr).expect("encode normal");

    assert_eq!(enc_attr.len(), enc_normal.len(), "encoded sizes must match");
    assert_eq!(enc_attr, enc_normal, "wire bytes must be identical");
}

// ---------------------------------------------------------------------------
// 18. With limit config
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_with_limit_config() {
    let cfg = config::standard().with_limit::<65536>();
    let original = SimpleBytes {
        data: vec![0x11, 0x22, 0x33, 0x44],
    };
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode");
    let (decoded, _): (SimpleBytes, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 19. Data integrity — no corruption across multiple cycles
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_data_integrity_multiple_cycles() {
    let data: Vec<u8> = (0u8..=255).collect();
    let mut current = SimpleBytes { data };

    for cycle in 0..5 {
        let encoded = encode_to_vec(&current).expect("encode");
        let (next, _): (SimpleBytes, _) = decode_from_slice(&encoded)
            .unwrap_or_else(|e| panic!("decode failed on cycle {cycle}: {e}"));
        assert_eq!(current, next, "data must be identical after cycle {cycle}");
        current = next;
    }
}

// ---------------------------------------------------------------------------
// 20. Null bytes (0x00) in the middle
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_null_bytes_in_middle() {
    let original = SimpleBytes {
        data: vec![0x01, 0x02, 0x00, 0x00, 0x00, 0x03, 0x04],
    };
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (SimpleBytes, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
    assert_eq!(decoded.data[2], 0x00);
    assert_eq!(decoded.data[3], 0x00);
    assert_eq!(decoded.data[4], 0x00);
}

// ---------------------------------------------------------------------------
// 21. Struct with bytes field inside Vec<T>
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_struct_in_vec_roundtrip() {
    let original: Vec<SimpleBytes> = vec![
        SimpleBytes {
            data: vec![1, 2, 3],
        },
        SimpleBytes { data: vec![] },
        SimpleBytes {
            data: vec![0xFF; 8],
        },
        SimpleBytes {
            data: (0u8..16).collect(),
        },
    ];
    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (Vec<SimpleBytes>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 22. Encode/decode cycle — encoded bytes are identical on every call
// ---------------------------------------------------------------------------

#[test]
fn test_bytes_attr_encode_decode_cycle_identical_bytes() {
    let original = BytesWithFields {
        id: 0xCAFE,
        payload: vec![10, 20, 30, 40, 50, 60, 70, 80],
        label: String::from("cycle"),
    };

    let first_encoded = encode_to_vec(&original).expect("first encode");
    let (decoded, _): (BytesWithFields, _) =
        decode_from_slice(&first_encoded).expect("first decode");
    let second_encoded = encode_to_vec(&decoded).expect("second encode");

    assert_eq!(
        first_encoded, second_encoded,
        "re-encoded bytes must be identical to the original encoding"
    );
    assert_eq!(original, decoded);
}
